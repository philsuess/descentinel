use clap::Parser;
use image::{ImageBuffer, ImageOutputFormat, Rgb};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Result,
};
use log::info;
use nokhwa::{
    native_api_backend,
    pixel_format::RgbFormat,
    query,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use std::io::Cursor;

fn init_camera() -> nokhwa::Camera {
    let backend = native_api_backend().unwrap();
    let devices = query(backend).unwrap();
    info!("There are {} available cameras.", devices.len());
    for device in devices {
        info!("{device}");
    }

    let index = CameraIndex::Index(0);
    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
    // make the camera
    return Camera::new(index, requested).unwrap();
}

fn capture_frame(camera: &mut nokhwa::Camera) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let frame = camera.frame().unwrap();
    //info!("captured single frame of length {}", frame.buffer().len());
    return frame.decode_image::<RgbFormat>().unwrap();
}

fn convert_to_bytes_buffer(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
        .unwrap();
    bytes
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Q_GAME_ROOM_FEED"))]
    destination_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_SHORT_LOG"))]
    short_log_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,

    #[arg(short, long, default_value_t = 1000)]
    images_interval_in_ms: u64,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("MONITOR service starting");
    let args = Args::parse();

    let mut camera = init_camera();
    camera.open_stream().unwrap();
    info!("Camera streaming...");

    async_global_executor::block_on(async {
        let conn = Connection::connect(&args.ampq_url, ConnectionProperties::default()).await?;
        info!("Established connection to {}", args.ampq_url);
        let channel_images = conn.create_channel().await?;
        channel_images
            .queue_declare(
                &args.destination_queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!("Output queue set to {}", args.destination_queue);

        let channel_short_logs = conn.create_channel().await?;
        channel_short_logs
            .queue_declare(
                &args.short_log_queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!("Logging queue set to {}", args.short_log_queue);
        send_over_queue(
            b"Camera streaming over MONITOR service",
            &channel_short_logs,
            &args.short_log_queue,
        )
        .await?;

        let pause_between_images = std::time::Duration::from_millis(args.images_interval_in_ms);
        loop {
            let image = capture_frame(&mut camera);
            //image.save("capture.jpeg").unwrap();
            let bytes = convert_to_bytes_buffer(&image);
            send_over_queue(&bytes, &channel_images, &args.destination_queue).await?;
            info!("image sent to {}", args.destination_queue);
            std::thread::sleep(pause_between_images);
        }
    })
    //producer.close().await;
}
