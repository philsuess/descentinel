use bytes::Bytes;
use clap::Parser;
use deadpool_lapin::{Manager, Pool, PoolError};
use futures::{future::join_all, join, StreamExt};
use lapin::Consumer;
use lapin::{options::*, types::FieldTable, ConnectionProperties};
use log::{error, info};
use std::result::Result as StdResult;
use std::time::Duration;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
};
use thiserror::Error as ThisError;
use warp::{Filter, Rejection, Reply};

type Cache = Arc<Mutex<Bytes>>;

type WebResult<T> = StdResult<T, Rejection>;
type RMQResult<T> = StdResult<T, PoolError>;
type Result<T> = StdResult<T, Error>;
type Connection = deadpool::managed::Object<deadpool_lapin::Manager>;

#[derive(ThisError, Debug)]
enum Error {
    #[error("rmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("rmq pool error: {0}")]
    RMQPoolError(#[from] PoolError),
}

impl warp::reject::Reject for Error {}

#[derive(Clone, Debug)]
struct DescentinelObject {
    queue_name: String,
    last_published_object: Cache,
}

impl DescentinelObject {
    fn new(queue_name: &str) -> DescentinelObject {
        DescentinelObject {
            queue_name: String::from(queue_name),
            last_published_object: Arc::new(Mutex::new(Bytes::new())),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Q_GAME_ROOM_FEED"))]
    game_room_image_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_SHORT_LOG"))]
    short_log_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_DETECTED_OL_CARDS"))]
    detected_ol_cards_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,

    #[arg(long, default_value_t = 3030)]
    server_port: u16,
}

async fn initialize_rabbit_mq_connection(ampqurl: &str) -> Pool {
    let manager = Manager::new(
        ampqurl,
        ConnectionProperties::default().with_executor(tokio_executor_trait::Tokio::current()),
    );
    deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("can create pool for rabbitmq connection")
}

async fn initialize_webserver_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health = warp::path!("health").and_then(health_handler);
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    health.or(hello)
}

async fn health_handler() -> WebResult<impl Reply> {
    Ok("OK")
}

async fn rabbitmq_listen(pool: Pool, descentinel_objects: &[DescentinelObject]) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(pool.clone(), &descentinel_objects).await {
            Ok(_) => info!("connection to rabbitmq established"),
            Err(e) => error!(
                "error when trying to establish connection to rabbitmq: {}",
                e
            ),
        };
    }
}

async fn rabbitmq_connection(pool: Pool) -> RMQResult<Connection> {
    let connection = pool.get().await?;
    Ok(connection)
}

async fn consume(mut consumer: Consumer) {
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
        info!("received {:?}", delivery);
    }
}

async fn init_rabbitmq_listen(pool: Pool, descentinel_objects: &[DescentinelObject]) -> Result<()> {
    let rmq_con = rabbitmq_connection(pool).await.map_err(|e| {
        error!("could not get rabbitmq connection: {}", e);
        e
    })?;
    let channel = rmq_con.create_channel().await?;

    let mut consume_futures = vec![];

    for descentinel_object in descentinel_objects {
        let queue = channel
            .queue_declare(
                &descentinel_object.queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!("Declared queue {:?}", queue);

        let mut consumer = channel
            .basic_consume(
                &descentinel_object.queue_name,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "rabbitmq consumer connected to {}, waiting for messages",
            &descentinel_object.queue_name
        );
        consume_futures.push(consume(consumer));
    }
    join_all(consume_futures).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("BROADCAST service starting");
    let args = Args::parse();

    let webserver_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.server_port);
    info!("broadcasting to {}", webserver_address);

    let cache = Arc::new(Mutex::new(Bytes::new()));

    let routes = initialize_webserver_routes().await;
    info!("webserver routes initialized");

    let rabbitmq_connection_pool = initialize_rabbit_mq_connection(&args.ampq_url).await;
    info!("connecting to rabbitmq on {}", &args.ampq_url);

    let descentinel_objects = vec![
        DescentinelObject::new(&args.game_room_image_queue),
        DescentinelObject::new(&args.short_log_queue),
        DescentinelObject::new(&args.detected_ol_cards_queue),
    ];

    let _ = join!(
        warp::serve(routes).run(webserver_address),
        rabbitmq_listen(rabbitmq_connection_pool.clone(), &descentinel_objects)
    );
    Ok(())
}
