use clap::Parser;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Result,
};
use log::info;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

fn capture_image_from_v4l(stream: &mut Stream) -> Vec<u8> {
    let (buf, _meta) = stream.next().unwrap();
    buf.to_vec()
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
    #[arg(short, long, default_value_t = 0)]
    camera_preference_index: u32,

    #[arg(short, long, default_value_t = String::from("Q_GAME_ROOM_FEED"))]
    destination_queue: String,

    #[arg(short, long, default_value_t = String::from("Q_SHORT_LOG"))]
    short_log_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,

    #[arg(short, long, default_value_t = 2500)]
    images_interval_in_ms: u64,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("MONITOR service starting");
    let args = Args::parse();

    //let mut camera = init_camera(args.camera_preference_index);
    //camera.open_stream().unwrap();
    let mut dev = Device::new(0).expect("Failed to open device");

    // Let's say we want to explicitly request another format
    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = 1280;
    fmt.height = 720;
    fmt.fourcc = FourCC::new(b"MJPG");
    let fmt = dev.set_format(&fmt).expect("Failed to write format");

    // The actual format chosen by the device driver may differ from what we
    // requested! Print it out to get an idea of what is actually used now.
    info!("Format in use:\n{}", fmt);
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

        let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 4)
            .expect("Failed to create buffer stream");

        let pause_between_images = std::time::Duration::from_millis(args.images_interval_in_ms);
        loop {
            let image_as_bytes = capture_image_from_v4l(&mut stream);
            send_over_queue(&image_as_bytes, &channel_images, &args.destination_queue).await?;
            info!("image sent to {}", args.destination_queue);
            std::thread::sleep(pause_between_images);
        }
    })
    //producer.close().await;
}
