use bytes::Bytes;
use clap::Parser;
use deadpool_lapin::{Manager, Pool, PoolError};
use env_logger::init;
use lapin::{options::*, types::FieldTable, BasicProperties, ConnectionProperties};
use log::info;
use std::result::Result as StdResult;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
};
use warp::{Filter, Rejection, Reply};

type WebResult<T> = StdResult<T, Rejection>;

type Cache = Arc<Mutex<Bytes>>;

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

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("BROADCAST service starting");
    let args = Args::parse();

    let rabbitmq_connection_pool = initialize_rabbit_mq_connection(&args.ampq_url).await;
    info!("connected to rabbitmq on {}", &args.ampq_url);

    let webserver_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.server_port);
    info!("broadcasting to {}", webserver_address);

    let cache = Arc::new(Mutex::new(Bytes::new()));

    let routes = initialize_webserver_routes().await;
    info!("webserver routes initialized");

    warp::serve(routes).run(webserver_address).await;
}
