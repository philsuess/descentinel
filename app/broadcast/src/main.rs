use clap::Parser;
use descentinel_types::ipc::{self, process_message_pipeline, IpcError, Message};
use futures_util::stream;
use lapin::Connection;
use log::{error, info};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::vec;
use thiserror::Error;
use tokio::join;
use tokio::sync::broadcast;
use warp::sse::Event;
use warp::Filter;

type Broadcaster = broadcast::Sender<(String, Message)>;

#[derive(Error, Debug)]
enum BroadcastError {
    #[error("rmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("ipc error: {0}")]
    IpcError(#[from] ipc::IpcError),
}

type SharedState = Arc<Mutex<HashMap<String, Vec<u8>>>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Q_GAME_ROOM_FEED"))]
    game_room_feed_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_SHORT_LOG"))]
    short_log_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_DETECTED_OL_CARDS"))]
    detected_ol_cards_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,

    #[arg(long, default_value_t = 3030)]
    server_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), BroadcastError> {
    env_logger::init();
    info!("BROADCAST service starting");
    let args = Arc::new(Args::parse());

    let connection = ipc::create_connection(&args.ampq_url).await?;
    info!("Established connection to {}", args.ampq_url);

    let _ = join!(rabbitmq_listen(connection, args));

    Ok(())
}

async fn rabbitmq_listen(
    connection: Arc<Connection>,
    args: Arc<Args>,
) -> Result<(), BroadcastError> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(connection.clone(), args.clone()).await {
            Ok(_) => info!("connection to rabbitmq established"),
            Err(e) => error!(
                "error when trying to establish connection to rabbitmq: {}",
                e
            ),
        };
    }
}

fn create_broadcaster() -> Broadcaster {
    broadcast::channel::<(String, Message)>(100).0 // Buffer of 100 messages
}

async fn init_rabbitmq_listen(
    connection: Arc<Connection>,
    args: Arc<Args>,
) -> Result<(), BroadcastError> {
    let args_clone = args.clone();
    let queues = vec![
        args_clone.game_room_feed_queue.to_string(),
        args_clone.detected_ol_cards_queue.to_string(),
        args.short_log_queue.to_string(),
    ];
    init_queues(connection.clone(), &queues).await?;
    info!(
        "Awaiting game room images from {}",
        args.game_room_feed_queue
    );
    info!(
        "Awaiting detected OL cards from {}",
        args.detected_ol_cards_queue
    );
    info!("Logging to {}", args.short_log_queue);

    let webserver_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), args.server_port);
    info!("broadcasting to {}", webserver_address);

    let broadcaster = create_broadcaster();

    // Shared state to store the latest message content from each queue
    let state: SharedState = Arc::new(Mutex::new(HashMap::new()));

    for queue in queues.clone() {
        let state_clone = state.clone();
        let connection_clone = connection.clone();
        let queue_name = queue.clone();
        let broadcaster_clone = broadcaster.clone();
        tokio::spawn(async move {
            if let Err(e) = consume_and_update_state(
                connection_clone,
                queue_name,
                state_clone,
                broadcaster_clone,
            )
            .await
            {
                error!("Error processing messages from queue {}: {:?}", queue, e);
            }
        });
    }

    // Start the warp server
    start_warp_server(
        args_clone,
        webserver_address,
        state,
        queues,
        broadcaster.clone(),
    )
    .await;

    Ok(())
}

async fn init_queues(connection: Arc<Connection>, queues: &[String]) -> Result<(), IpcError> {
    for queue in queues {
        ipc::declare_queue(connection.clone(), queue).await?;
    }
    Ok(())
}

async fn consume_and_update_state(
    connection: Arc<Connection>,
    queue_name: String,
    state: SharedState,
    broadcaster: Broadcaster,
) -> Result<(), BroadcastError> {
    let game_room_feed_queue = queue_name.clone();
    process_message_pipeline(
        connection,
        &game_room_feed_queue,
        move |message: &Message| {
            let mut state = state.lock().unwrap();
            state.insert(queue_name.clone(), message.content.clone());
            info!(
                "got {:?} for {}",
                message.content.clone(),
                queue_name.clone()
            );

            // Broadcast the queue name alongside the message
            if let Err(err) = broadcaster.send((queue_name.clone(), message.clone())) {
                error!("Failed to broadcast message: {:?}", err);
            }
            vec![]
        },
    )
    .await?;

    Ok(())
}

fn message_stream(
    broadcaster: broadcast::Receiver<(String, Message)>,
    queue_name: String,
) -> impl futures_util::Stream<Item = Result<Event, warp::Error>> {
    stream::unfold(broadcaster, move |mut rx| {
        let queue_name_clone = queue_name.clone();
        async move {
            while let Ok((msg_queue_name, message)) = rx.recv().await {
                if msg_queue_name == queue_name_clone {
                    // Filter by queue_name
                    let event = Event::default().data(serde_json::to_string(&message).unwrap());
                    return Some((Ok(event), rx));
                }
            }
            None // Receiver was closed or no matching messages
        }
    })
}

async fn start_warp_server(
    args: Arc<Args>,
    webserver_address: SocketAddr,
    state: SharedState,
    queues: Vec<String>,
    broadcaster: Broadcaster,
) {
    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);

    let state_clone = state.clone();
    let queues_clone_e = queues.clone();
    let queue_routes = warp::path::param() // Capture any path parameter
        .and(warp::get())
        .and_then(move |queue_name: String| {
            let state = state_clone.clone();

            let queues_clone_a = queues_clone_e.clone();
            let args_clone = args.clone();

            async move {
                if !queues_clone_a.contains(&queue_name) {
                    return Err(warp::reject::not_found());
                }

                let state = state.lock().unwrap();
                if queue_name.eq(&args_clone.game_room_feed_queue) {
                    let response = if let Some(image_as_bytes) = state.get(&queue_name) {
                        warp::reply::with_header(
                            image_as_bytes.clone(),
                            "Content-Type",
                            "image/png",
                        )
                    } else {
                        warp::reply::with_header(vec![], "Content-Type", "image/png")
                    };
                    Ok::<_, warp::Rejection>(response)
                } else {
                    let response = if let Some(content) = state.get(&queue_name) {
                        warp::reply::with_header(
                            content.clone(),
                            "Content-Type",
                            "application/text",
                        )
                    } else {
                        warp::reply::with_header(vec![], "Content-Type", "application/text")
                    };
                    Ok::<_, warp::Rejection>(response)
                }
            }
        });

    let queues_clone_d = queues.clone();
    let sse_route = warp::path::param() // Capture any path parameter
        .and(warp::get())
        .and_then({
            let broadcaster = broadcaster.clone();
            let queues_clone_b = queues_clone_d.clone(); // Clone queues here to avoid moving it
            move |queue_name: String| {
                let broadcaster = broadcaster.clone();
                let queues_clone_c = queues_clone_b.clone(); // Clone queues inside the closure

                async move {
                    if !queues_clone_c.contains(&queue_name) {
                        return Err(warp::reject::not_found());
                    }

                    let rx = broadcaster.subscribe();
                    let stream = message_stream(rx, queue_name.clone()); // Pass queue_name to filter messages
                    Ok::<_, warp::Rejection>(warp::sse::reply(
                        warp::sse::keep_alive().stream(stream),
                    ))
                }
            }
        });

    // Define a default route for the root path
    let default_route = warp::any().map(|| warp::reply::json(&"No queue specified".to_string()));

    // Combine queue_routes and default_route
    //let routes = queue_routes.or(default_route).with(cors);
    let routes = sse_route.with(cors);

    warp::serve(routes).run(webserver_address).await;
}
