use clap::Parser;
use descentinel_types::ipc::{self, IpcError, Message};
use identify_game_scene::{convert_to_opencv_image, CardDetector};
use lapin::Connection;
use log::{error, info};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use thiserror::Error;
use tokio::join;

#[derive(Error, Debug)]
pub enum IdentifyGameSceneError {
    #[error("ipc error: {0}")]
    IpcError(#[from] ipc::IpcError),
    #[error("lapin error: {0}")]
    LapinError(#[from] lapin::Error),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Q_GAME_ROOM_FEED"))]
    game_room_feed_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_CARD_IMAGE"))]
    detected_cards_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_SHORT_LOG"))]
    short_log_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,
}

#[tokio::main]
async fn main() -> Result<(), IdentifyGameSceneError> {
    env_logger::init();
    info!("IDENTIFY_GAME_SCENE service starting");
    let args = Arc::new(Args::parse());

    let template_overlord_card =
        opencv::imgcodecs::imread("./OL_template.jpg", opencv::imgcodecs::IMREAD_COLOR).unwrap();
    let card_detector = Arc::new(Mutex::new(CardDetector::new(&template_overlord_card)));

    let conn = ipc::create_connection(&args.ampq_url).await?;
    info!("Established connection to {}", args.ampq_url);

    let _ = join!(rabbitmq_listen(conn, args, card_detector));

    Ok(())
}

async fn rabbitmq_listen(
    connection: Arc<Connection>,
    args: Arc<Args>,
    card_detector: Arc<Mutex<CardDetector>>,
) -> Result<(), IdentifyGameSceneError> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(connection.clone(), args.clone(), card_detector.clone()).await {
            Ok(_) => info!("connection to rabbitmq established"),
            Err(e) => error!(
                "error when trying to establish connection to rabbitmq: {}",
                e
            ),
        };
    }
}

async fn init_queues(connection: Arc<Connection>, args: Arc<Args>) -> Result<(), IpcError> {
    let _results = tokio::join!(
        ipc::declare_queue(connection.clone(), &args.game_room_feed_queue),
        ipc::declare_queue(connection.clone(), &args.detected_cards_queue),
        ipc::declare_queue(connection.clone(), &args.short_log_queue)
    );
    info!(
        "Awaiting game room images from {}",
        args.game_room_feed_queue
    );
    info!("Sending detected cards to {}", args.detected_cards_queue);
    info!("Logging to {}", args.short_log_queue);
    Ok(())
}

async fn init_rabbitmq_listen(
    connection: Arc<Connection>,
    args: Arc<Args>,
    card_detector: Arc<Mutex<CardDetector>>,
) -> Result<(), IdentifyGameSceneError> {
    let args_clone = args.clone();
    init_queues(connection.clone(), args.clone()).await?;

    let detect_card = move |game_room_image: &Message| {
        handle_game_room_image(game_room_image, card_detector.clone(), args.clone())
    };

    let game_room_feed_queue = &args_clone.game_room_feed_queue;
    let _results = tokio::join!(ipc::process_message_pipeline(
        connection.clone(),
        game_room_feed_queue,
        detect_card,
    ));

    Ok(())
}

fn handle_game_room_image(
    game_room_image: &Message,
    card_detector: Arc<Mutex<CardDetector>>,
    args: Arc<Args>,
) -> Vec<(String, Message)> {
    let mut card_detector = card_detector.lock().unwrap();
    let mut downstream_messages = Vec::new();
    let data_as_opencv_image = convert_to_opencv_image(&game_room_image.content);
    if data_as_opencv_image.is_ok()
        && card_detector
            .detect_card(&data_as_opencv_image.unwrap())
            .is_some()
    {
        downstream_messages.push((
            args.detected_cards_queue.clone(),
            Message {
                content: game_room_image.content.clone(),
            },
        ));

        let log_message = String::from("detected an OL card");
        info!("{}", &log_message);
        downstream_messages.push((
            args.short_log_queue.clone(),
            Message {
                content: log_message.as_bytes().to_vec(),
            },
        ));
    }

    downstream_messages
}
