use clap::Parser;
use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Consumer, Result,
};
use log::{error, info};
use serde::Deserialize;
use std::time::Duration;
use tesseract::Tesseract;
use tokio::join;

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
async fn main() -> Result<()> {
    env_logger::init();
    info!("DETECT_CARD service starting");
    let args = Args::parse();

    let overlord_cards = load_overlord_keywords(&args.overlord_cards_keywords_file);

    let conn = Connection::connect(&args.ampq_url, ConnectionProperties::default()).await?;
    info!("Established connection to {}", args.ampq_url);

    let _ = join!(rabbitmq_listen(&conn, &args, &overlord_cards));

    Ok(())
}

async fn rabbitmq_listen(
    connection: &Connection,
    args: &Args,
    overlord_cards: &OverlordCards,
) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(connection, args, overlord_cards).await {
            Ok(_) => info!("connection to rabbitmq established"),
            Err(e) => error!(
                "error when trying to establish connection to rabbitmq: {}",
                e
            ),
        };
    }
}

async fn init_rabbitmq_listen(
    connection: &Connection,
    args: &Args,
    overlord_cards: &OverlordCards,
) -> Result<()> {
    let channel_images = connection.create_channel().await?;
    channel_images
        .queue_declare(
            &args.game_room_feed_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!(
        "Awaiting game room images from {}",
        args.game_room_feed_queue
    );
    let game_room_images_consumer = channel_images
        .basic_consume(
            &args.game_room_feed_queue,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let channel_detected_ol_cards = connection.create_channel().await?;
    channel_detected_ol_cards
        .queue_declare(
            &args.detected_ol_cards_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!(
        "Sending detected OL cards to {}",
        args.detected_ol_cards_queue
    );

    let channel_short_logs = connection.create_channel().await?;
    channel_short_logs
        .queue_declare(
            &args.short_log_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!("Logging to {}", args.short_log_queue);

    let consume_future = consume_game_room_feed(
        game_room_images_consumer,
        overlord_cards,
        &channel_detected_ol_cards,
        &channel_short_logs,
        args,
    );

    futures::join!(consume_future);

    Ok(())
}

async fn consume_game_room_feed(
    mut game_room_feed: Consumer,
    overlord_cards: &OverlordCards,
    channel_detected_ol_cards: &Channel,
    channel_logs: &Channel,
    args: &Args,
) {
    while let Some(delivery) = game_room_feed.next().await {
        let delivery = delivery.expect("error in consumer");
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
        //  info!("received {:?}", delivery);
        if let Some(card_id) = identify_card(&delivery.data, overlord_cards) {
            let _ = send_over_queue(
                card_id.as_bytes(),
                channel_detected_ol_cards,
                &args.detected_ol_cards_queue,
            )
            .await;
            let mut log_message = String::from("detected OL card ");
            log_message.push_str(&card_id);
            info!("{}", &log_message);
            let _ =
                send_over_queue(log_message.as_bytes(), channel_logs, &args.short_log_queue).await;
        }
    }
}

async fn send_over_queue(payload: &[u8], channel: &Channel, queue_name: &str) -> Result<()> {
    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await?
        .await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct OverlordCards {
    cards: Vec<CardKeywords>,
}

impl OverlordCards {
    fn id_of_best_keywords_match(&self, card_text: &str) -> Option<String> {
        let winning_card = self
            .cards
            .iter()
            .reduce(|max_found, candidate| {
                if candidate.number_of_matches(card_text) > max_found.number_of_matches(card_text) {
                    candidate
                } else {
                    max_found
                }
            })
            .unwrap();
        if winning_card.number_of_matches(card_text) == 0 {
            None
        } else {
            Some(winning_card.id.clone())
        }
    }
}

#[derive(Deserialize, Debug)]
struct CardKeywords {
    id: String,
    keywords: Vec<String>,
}

impl CardKeywords {
    fn number_of_matches(&self, card_text: &str) -> u8 {
        self.keywords.iter().fold(0, |sum_matches, next_keyword| {
            if card_text.contains(next_keyword.as_str()) {
                return sum_matches + 1;
            }
            sum_matches
        })
    }
}

fn identify_card(card_image_buffer: &[u8], overlord_cards: &OverlordCards) -> Option<String> {
    let card_text = extract_card_text_from_buffer(card_image_buffer, "fra");
    overlord_cards.id_of_best_keywords_match(&card_text)
}

fn load_overlord_keywords(file_name: &str) -> OverlordCards {
    let file = std::fs::File::open(file_name).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).expect("file is not proper json")
}

fn extract_card_text_from_buffer(card_image_buffer: &[u8], language: &str) -> String {
    match Tesseract::new(None, Some(language))
        .unwrap()
        .set_image_from_mem(card_image_buffer)
        .unwrap()
        .recognize()
        .unwrap()
        .get_text()
    {
        Ok(card_text) => card_text,
        Err(_) => String::from("could not read card text"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, ImageOutputFormat, Rgb};
    use std::io::Cursor;

    fn convert_to_bytes_buffer(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
        let mut bytes = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
            .unwrap();
        bytes
    }

    #[test]
    fn overlord_cards_detection_from_file_works() {
        let card_text_baume_sombre =
            tesseract::ocr("test_images/BaumeSombre_02.jpg_detected.jpg", "fra").unwrap();
        println!("{}", card_text_baume_sombre);
        assert!(card_text_baume_sombre.contains("Baume"));

        let card_text_explodierende_rune =
            tesseract::ocr("test_images/ExplodierendeRune.jpg_detected.jpg", "fra").unwrap();
        println!("{}", card_text_explodierende_rune);
        assert!(card_text_explodierende_rune.contains("Schatztruhe"));
    }

    #[test]
    fn overlord_cards_detection_from_memory_works() {
        let card_image = image::open("test_images/BaumeSombre_02.jpg_detected.jpg")
            .unwrap()
            .to_rgb8();

        let card_text = extract_card_text_from_buffer(&convert_to_bytes_buffer(&card_image), "fra");
        println!("{}", card_text);
        assert!(card_text.contains("Baume"));
    }

    #[test]
    fn keywords_cards_json_file_is_ok() {
        let overlord_cards = load_overlord_keywords("keywords_cards.json");
        assert!(overlord_cards.cards.len() > 3);
    }

    #[test]
    fn known_overlord_cards_detection_works() {
        let overlord_cards = load_overlord_keywords("keywords_cards.json");

        let card_image = image::open("test_images/BaumeSombre_02.jpg_detected.jpg")
            .unwrap()
            .to_rgb8();
        assert_eq!(
            Some(String::from("dark_balm")),
            identify_card(&convert_to_bytes_buffer(&card_image), &overlord_cards)
        );
    }
}
