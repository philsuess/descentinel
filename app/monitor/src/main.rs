use clap::Parser;
use descentinel_types::ipc;
use descentinel_types::ipc::Message;
use image::ImageReader;
use log::info;
use std::io::Cursor;
use thiserror::Error;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("ipc error: {0}")]
    IpcError(#[from] ipc::IpcError),
    #[error("lapin error: {0}")]
    LapinError(#[from] lapin::Error),
}

fn capture_image_from_v4l(stream: &mut Stream) -> Vec<u8> {
    let (buf, _meta) = stream.next().unwrap();
    buf.to_vec()
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

    #[arg(long, default_value_t = false)]
    save_captures_to_local_file: bool,

    #[arg(short, long, default_value_t = 0)]
    v4l_device_id: usize,
}

fn main() -> Result<(), MonitorError> {
    env_logger::init();
    info!("MONITOR service starting");
    let args = Args::parse();

    //let mut camera = init_camera(args.camera_preference_index);
    //camera.open_stream().unwrap();
    let dev = Device::new(args.v4l_device_id).expect("Failed to open device");

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
        let connection = ipc::create_connection(&args.ampq_url).await?;
        info!("Established connection to {}", args.ampq_url);
        ipc::declare_queue(connection.clone(), &args.destination_queue).await?;
        info!("Output queue set to {}", args.destination_queue);

        ipc::declare_queue(connection.clone(), &args.short_log_queue).await?;
        info!("Logging queue set to {}", args.short_log_queue);

        ipc::send_message(
            connection.clone(),
            &args.short_log_queue,
            &Message {
                content: "Camera streaming over MONITOR service".as_bytes().to_vec(),
                content_type: ipc::ContentType::Text,
            },
        )
        .await?;

        let mut stream = Stream::with_buffers(&dev, Type::VideoCapture, 4)
            .expect("Failed to create buffer stream");

        let pause_between_images = std::time::Duration::from_millis(args.images_interval_in_ms);
        loop {
            let image_as_bytes = capture_image_from_v4l(&mut stream);
            if args.save_captures_to_local_file {
                let file_name = "capture.png";
                let imgage_for_saving = ImageReader::new(Cursor::new(image_as_bytes.clone()))
                    .with_guessed_format()
                    .unwrap()
                    .decode()
                    .unwrap();
                let _ = imgage_for_saving.save(file_name);
                info!("wrote {} ", file_name);
            }
            ipc::send_message(
                connection.clone(),
                &args.destination_queue,
                &Message {
                    content: image_as_bytes,
                    content_type: ipc::ContentType::Text,
                },
            )
            .await?;
            info!("image sent to {}", args.destination_queue);
            std::thread::sleep(pause_between_images);
        }
    })
    //producer.close().await;
}
