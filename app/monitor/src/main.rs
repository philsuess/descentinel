use image::{ImageBuffer, Rgb};
use log::info;
use nokhwa::{
    native_api_backend,
    pixel_format::RgbFormat,
    query,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

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

fn send_over_queue(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    image.save("capture.jpeg").unwrap();
    info!("sent");
}

fn main() {
    env_logger::init();
    info!("MONITOR service starting");

    let mut camera = init_camera();
    camera.open_stream().unwrap();

    let image = capture_frame(&mut camera);
    send_over_queue(&image);
}
