use amqp::protocol::basic::BasicProperties;
use amqp::{Basic, Channel, Session, Table};
use clap::Parser;
use image::{ImageBuffer, ImageOutputFormat, Rgb};
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
    info!("Captured Single Frame of {}", frame.buffer().len());
    return frame.decode_image::<RgbFormat>().unwrap();
}

fn create_session_and_channel(ampq_url: &str) -> (Session, Channel) {
    let mut session = Session::open_url(ampq_url).unwrap();
    let channel = session.open_channel(1).unwrap();
    return (session, channel);
}

fn declare_queue(channel: &mut Channel, queue_name: &str) {
    channel
        .queue_declare(queue_name, false, false, false, false, false, Table::new())
        .unwrap();
}

fn send_over_queue(
    channel: &mut Channel,
    destination: &str,
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let mut bytes = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
        .unwrap();
    channel
        .basic_publish(
            "",
            destination,
            true,
            false,
            BasicProperties {
                content_type: Some("text".to_string()),
                ..Default::default()
            },
            bytes,
        )
        .unwrap();
    //image.save("capture.jpeg").unwrap();
    info!("image sent to {}", destination);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("QUEUE_IMAGES"))]
    destination_queue: String,

    #[arg(short, long, default_value_t = String::from("amqp://localhost:5672"))]
    ampq_url: String,

    #[arg(short, long, default_value_t = 1000)]
    images_interval_in_ms: u64,
}

fn main() {
    env_logger::init();
    info!("MONITOR service starting");
    let args = Args::parse();

    let mut camera = init_camera();
    camera.open_stream().unwrap();
    info!("Camera streaming...");

    let (_session, mut channel) = create_session_and_channel(&args.ampq_url);
    declare_queue(&mut channel, &args.destination_queue);
    info!("Output queue set to {}", args.destination_queue);

    let pause_between_images = std::time::Duration::from_millis(args.images_interval_in_ms);
    loop {
        let image = capture_frame(&mut camera);
        send_over_queue(&mut channel, &args.destination_queue, &image);
        std::thread::sleep(pause_between_images);
    }
}
