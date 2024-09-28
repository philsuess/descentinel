use clap::Parser;
use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Consumer, Result,
};
use log::{error, info};
use opencv::prelude::{DescriptorMatcherTraitConst, Feature2DTrait};
use std::time::Duration;
use tokio::join;

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
async fn main() -> Result<()> {
    env_logger::init();
    info!("IDENTIFY_GAME_SCENE service starting");
    let args = Args::parse();

    let template_overlord_card =
        opencv::imgcodecs::imread("./OL_template.jpg", opencv::imgcodecs::IMREAD_COLOR).unwrap();
    let mut card_detector = CardDetector::new(&template_overlord_card);

    let conn = Connection::connect(&args.ampq_url, ConnectionProperties::default()).await?;
    info!("Established connection to {}", args.ampq_url);

    let _ = join!(rabbitmq_listen(&conn, &args, &mut card_detector));

    Ok(())
}

async fn rabbitmq_listen(
    connection: &Connection,
    args: &Args,
    card_detector: &mut CardDetector,
) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        info!("connecting rabbitmq consumer...");
        match init_rabbitmq_listen(connection, args, card_detector).await {
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
    card_detector: &mut CardDetector,
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

    let channel_detected_cards = connection.create_channel().await?;
    channel_detected_cards
        .queue_declare(
            &args.detected_cards_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!("Sending detected cards to {}", args.detected_cards_queue);

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
        &channel_detected_cards,
        &channel_short_logs,
        args,
        card_detector,
    );

    futures::join!(consume_future);

    Ok(())
}

async fn consume_game_room_feed(
    mut game_room_feed: Consumer,
    channel_detected_cards: &Channel,
    channel_logs: &Channel,
    args: &Args,
    card_detector: &mut CardDetector,
) {
    while let Some(delivery) = game_room_feed.next().await {
        let delivery = delivery.expect("error in consumer");
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
        //  info!("received {:?}", delivery);
        let data_as_opencv_image = convert_to_opencv_image(&delivery.data);
        if data_as_opencv_image.is_ok()
            && card_detector
                .detect_card(&data_as_opencv_image.unwrap())
                .is_some()
        {
            let _ = send_over_queue(
                &delivery.data,
                channel_detected_cards,
                &args.detected_cards_queue,
            )
            .await;
            let log_message = String::from("detected an OL card");
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

fn convert_to_opencv_image(card_image_buffer: &[u8]) -> opencv::Result<opencv::core::Mat> {
    opencv::imgcodecs::imdecode(
        &opencv::core::Vector::from_slice(card_image_buffer),
        opencv::imgcodecs::IMREAD_COLOR,
    )
}

#[derive(PartialEq)]
enum CardType {
    Overlord,
}

#[derive(Debug)]
struct CardDetector {
    sift: opencv::core::Ptr<opencv::features2d::SIFT>,
    bf: opencv::core::Ptr<opencv::features2d::BFMatcher>,
    overlord_descriptors_from_template: opencv::core::Mat,
}

impl CardDetector {
    fn new(overlord_card_template: &opencv::core::Mat) -> CardDetector {
        let mut sift = opencv::features2d::SIFT::create(0, 3, 0.04, 10., 1.6,false).unwrap();
        let mut ol_kp_template = opencv::core::Vector::<opencv::core::KeyPoint>::default();
        let mut ol_des_template = opencv::core::Mat::default();
        let mask = opencv::core::Mat::default();
        sift.detect_and_compute(
            &overlord_card_template,
            &mask,
            &mut ol_kp_template,
            &mut ol_des_template,
            false,
        )
        .unwrap();

        let bf = opencv::features2d::BFMatcher::create(opencv::core::NORM_L2, false).unwrap();

        CardDetector {
            sift,
            bf,
            overlord_descriptors_from_template: ol_des_template,
        }
    }

    fn detect_card(&mut self, image: &opencv::core::Mat) -> Option<CardType> {
        if self.score_against_overlord_card(image) > 5 {
            return Some(CardType::Overlord);
        }
        None
    }

    fn score_against_overlord_card(&mut self, image: &opencv::core::Mat) -> u32 {
        let mut test_image_kp = opencv::core::Vector::<opencv::core::KeyPoint>::default();
        let mut test_image_des = opencv::core::Mat::default();
        let mask = opencv::core::Mat::default();
        self.sift
            .detect_and_compute(
                &image,
                &mask,
                &mut test_image_kp,
                &mut test_image_des,
                false,
            )
            .unwrap();

        let mut matches =
            opencv::core::Vector::<opencv::core::Vector<opencv::core::DMatch>>::default();
        self.bf
            .knn_train_match(
                &self.overlord_descriptors_from_template,
                &test_image_des,
                &mut matches,
                2,
                &mask,
                false,
            )
            .unwrap();

        let mut good_match = opencv::core::Vector::<opencv::core::DMatch>::default();
        let quality_threshold = 0.5;
        for match_pair in matches {
            if match_pair.len() > 1
                && match_pair.get(0).unwrap().distance
                    < quality_threshold * match_pair.get(1).unwrap().distance
            {
                good_match.push(match_pair.get(0).unwrap());
            }
        }
        good_match.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlord_card_detection_works() {
        let template_overlord_card =
            opencv::imgcodecs::imread("./OL_template.jpg", opencv::imgcodecs::IMREAD_COLOR)
                .unwrap();
        let mut card_detector = CardDetector::new(&template_overlord_card);

        let test_ol_image = opencv::imgcodecs::imread(
            "./test_images/ExplodierendeRune.jpg_detected.jpg",
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();
        assert!(card_detector.detect_card(&test_ol_image) == Some(CardType::Overlord));

        let test_not_an_ol_image = opencv::imgcodecs::imread(
            "./test_images/Not_an_OL_card_01_cropped.jpg",
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();
        assert!(card_detector.detect_card(&test_not_an_ol_image).is_none());
    }
}
