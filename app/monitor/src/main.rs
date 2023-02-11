use log::info;
use nokhwa::{
    native_api_backend,
    pixel_format::RgbFormat,
    query,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

fn main() {
    env_logger::init();

    info!("MONITOR service starting");

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
    let mut camera = Camera::new(index, requested).unwrap();
    camera.open_stream().unwrap();

    // get a frame
    let frame = camera.frame().unwrap();
    info!("Captured Single Frame of {}", frame.buffer().len());
    // decode into an ImageBuffer
    let decoded = frame.decode_image::<RgbFormat>().unwrap();
    info!("Decoded Frame of {}", decoded.len());
    decoded.save("capture.jpeg").unwrap();
}
