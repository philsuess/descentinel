use clap::Parser;
use descentinel_types::ipc::{self, IpcError, Message};
use detect_card::{identify_card, load_overlord_keywords, OverlordCards};
use lapin::Connection;
use log::{error, info};
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::join;

#[derive(Error, Debug)]
pub enum DetectCardError {
    #[error("ipc error: {0}")]
    IpcError(#[from] ipc::IpcError),
    #[error("lapin error: {0}")]
    LapinError(#[from] lapin::Error),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Q_CARD_IMAGE"))]
    game_room_feed_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_DETECTED_OL_CARDS"))]
    detected_ol_cards_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_SHORT_LOG"))]
    short_log_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,

    #[arg(short, long, default_value_t = String::from("keywords_cards.json"))]
    overlord_cards_keywords_file: String,
}

#[tokio::main]
async fn main() -> Result<(), DetectCardError> {
    env_logger::init();
    info!("DETECT_CARD service starting");
    let args = Args::parse();

    let overlord_cards = Arc::new(load_overlord_keywords(&args.overlord_cards_keywords_file));
    let connection = ipc::create_connection(&args.ampq_url).await?;
    info!("Established connection to {}", args.ampq_url);

    let _ = join!(rabbitmq_listen(connection, &args, overlord_cards));

    Ok(())
}

async fn rabbitmq_listen(
    connection: Arc<Connection>,
    args: &Args,
    overlord_cards: Arc<OverlordCards>,
) -> Result<(), DetectCardError> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(connection.clone(), args, overlord_cards.clone()).await {
            Ok(_) => info!("connection to rabbitmq established"),
            Err(e) => error!(
                "error when trying to establish connection to rabbitmq: {}",
                e
            ),
        };
    }
}

async fn init_queues(connection: Arc<Connection>, args: &Args) -> Result<(), IpcError> {
    let _results = tokio::join!(
        ipc::declare_queue(connection.clone(), &args.game_room_feed_queue),
        ipc::declare_queue(connection.clone(), &args.detected_ol_cards_queue),
        ipc::declare_queue(connection.clone(), &args.short_log_queue)
    );
    info!(
        "Awaiting game room images from {}",
        args.game_room_feed_queue
    );
    info!(
        "Sending detected OL cards to {}",
        args.detected_ol_cards_queue
    );
    info!("Logging to {}", args.short_log_queue);
    Ok(())
}

async fn init_rabbitmq_listen(
    connection: Arc<Connection>,
    args: &Args,
    overlord_cards: Arc<OverlordCards>,
) -> Result<(), DetectCardError> {
    init_queues(connection.clone(), args).await?;

    let detect_ol_card = move |game_room_image: &Message| {
        handle_game_room_image(game_room_image, overlord_cards.clone())
    };

    let _results = tokio::join!(ipc::process_message_pipeline(
        connection.clone(),
        &args.game_room_feed_queue,
        &args.detected_ol_cards_queue,
        detect_ol_card,
    ));

    Ok(())
}

fn handle_game_room_image(
    game_room_image: &Message,
    overlord_cards: Arc<OverlordCards>,
) -> Option<Message> {
    if let Some(card_id) = identify_card(&game_room_image.content, overlord_cards.as_ref()) {
        return Some(Message {
            content: card_id.as_bytes().to_vec(),
        });
    }

    None
}
