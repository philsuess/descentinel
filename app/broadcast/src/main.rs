use clap::Parser;
use deadpool_lapin::{Manager, Pool, PoolError};
use futures::{future::join_all, join, StreamExt};
use lapin::Consumer;
use lapin::{options::*, types::FieldTable, ConnectionProperties};
use log::{error, info};
use std::{
    collections::HashMap,
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    result::Result as StdResult,
    sync::{Arc, Mutex},
    time::Duration,
    vec,
};
use thiserror::Error as ThisError;
use warp::{Filter, Rejection, Reply};

type Cache = Arc<Mutex<HashMap<String, Vec<u8>>>>;

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
    route_to_descentinel_object: Cache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);

    let health = warp::path!("health").and_then(health_handler).with(&cors);

    let descentinel_object = warp::path!("descentinel" / String)
        .and_then({
            let cache = route_to_descentinel_object.clone();
            move |descentinel_object| {
                let cache = cache.clone();
                async move {
                    let database = cache.lock().unwrap();
                    Ok::<_, Infallible>(format!("{:?}", database.get(&descentinel_object).unwrap()))
                }
            }
        })
        .with(&cors);
    health.or(descentinel_object)
}

async fn health_handler() -> WebResult<impl Reply> {
    Ok("OK")
}

async fn rabbitmq_listen(
    pool: Pool,
    route_to_queue_map: HashMap<String, String>,
    route_to_descentinel_object: Cache,
) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(
            pool.clone(),
            &route_to_queue_map,
            route_to_descentinel_object.clone(),
        )
        .await
        {
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

async fn consume(mut consumer: Consumer, route_name: &str, route_to_descentinel_object: Cache) {
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
        //info!("received {:?}", delivery);
        {
            let mut route_to_descentinel_object = route_to_descentinel_object.lock().unwrap();
            route_to_descentinel_object.insert(String::from(route_name), delivery.data);
        }
    }
}

async fn init_rabbitmq_listen(
    pool: Pool,
    route_to_queue_map: &HashMap<String, String>,
    route_to_descentinel_object: Cache,
) -> Result<()> {
    let rmq_con = rabbitmq_connection(pool).await.map_err(|e| {
        error!("could not get rabbitmq connection: {}", e);
        e
    })?;
    let channel = rmq_con.create_channel().await?;

    let mut consume_futures = vec![];

    for (route_name, queue_name) in route_to_queue_map {
        let queue = channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!("Declared queue {:?}", queue);

        let consumer = channel
            .basic_consume(
                &queue_name,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "rabbitmq consumer connected to {}, waiting for messages",
            &queue_name
        );
        consume_futures.push(consume(
            consumer,
            route_name,
            route_to_descentinel_object.clone(),
        ));
    }
    join_all(consume_futures).await;
    Ok(())
}

fn initialize_cached_descentinel_objects() -> Cache {
    let db = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut db = db.lock().unwrap();
        db.insert(String::from("game_room_image"), vec![0u8]);
        db.insert(String::from("log"), vec![0u8]);
        db.insert(String::from("detected_ol_card"), vec![0u8]);
    }
    db
}

fn initialize_routes_to_queue_map(args: &Args) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(
        String::from("game_room_image"),
        String::from(&args.game_room_image_queue),
    );
    map.insert(String::from("log"), String::from(&args.short_log_queue));
    map.insert(
        String::from("detected_ol_card"),
        String::from(&args.detected_ol_cards_queue),
    );
    map
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("BROADCAST service starting");
    let args = Args::parse();

    let webserver_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.server_port);
    info!("broadcasting to {}", webserver_address);

    let route_to_descentinel_object = initialize_cached_descentinel_objects();
    let route_to_queue_map = initialize_routes_to_queue_map(&args);

    let routes = initialize_webserver_routes(route_to_descentinel_object.clone()).await;
    info!("webserver routes initialized");

    let rabbitmq_connection_pool = initialize_rabbit_mq_connection(&args.ampq_url).await;
    info!("connecting to rabbitmq on {}", &args.ampq_url);

    let _ = join!(
        warp::serve(routes).run(webserver_address),
        rabbitmq_listen(
            rabbitmq_connection_pool.clone(),
            route_to_queue_map,
            route_to_descentinel_object.clone()
        )
    );
    Ok(())
}
