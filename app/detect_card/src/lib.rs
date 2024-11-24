use std::io::Cursor;

use image::{DynamicImage, ImageReader};
use log::info;
use rxing::{
    common::HybridBinarizer, qrcode::cpp_port::QrReader as Cpp_Qr_Reader, qrcode::QRCodeReader,
    BinaryBitmap, BufferedImageLuminanceSource, Reader,
};

type GameRoomImage = DynamicImage; //ImageBuffer<Luma<u8>, Vec<u8>>;

pub fn convert_to_grey_image(image_buffer: &[u8]) -> GameRoomImage {
    ImageReader::new(Cursor::new(image_buffer))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
}

pub fn identify_card_from(image: &GameRoomImage) -> Option<String> {
    let decoded_strings = get_all_codes_from(image);
    if let Some(ol_card) = decoded_strings
        .iter()
        .find(|&content| content.contains("overlordcard"))
    {
        return Some(ol_card.to_string());
    }
    None
}

fn get_all_codes_from(image: &GameRoomImage) -> Vec<String> {
    let mut decoded_strings = vec![];
    info!("got an image");

    let mut cpp_reader = Cpp_Qr_Reader;
    let cpp_result = cpp_reader.decode(&mut BinaryBitmap::new(HybridBinarizer::new(
        BufferedImageLuminanceSource::new(image.clone()),
    )));
    if let Ok(result) = cpp_result {
        decoded_strings.push(result.getText().to_string());
        info!("got {}", result.getText());
    }

    if !decoded_strings.is_empty() {
        return decoded_strings;
    }

    let mut reader = QRCodeReader::new();

    let result = reader.decode(&mut BinaryBitmap::new(HybridBinarizer::new(
        BufferedImageLuminanceSource::new(image.clone()),
    )));

    if let Ok(result) = result {
        decoded_strings.push(result.getText().to_string());
        info!("got {}", result.getText());
    }

    decoded_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_qr_detection_from_file_works() {
        let image = image::open("test_images/ol_doom.png").unwrap();
        let result = identify_card_from(&image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }

    #[test]
    fn twenty_mm_reading_scenario_works() {
        let image = image::open("test_images/ol_qr_20mm_focused.png").unwrap();
        let result = identify_card_from(&image);
        assert!(result == Some(String::from("overlordcard/doom")));

        let capture0 = image::open("test_images/capture_0.png").unwrap();
        let result0 = identify_card_from(&capture0);
        assert!(result0 == Some(String::from("overlordcard/doom")));
    }

    #[test]
    fn ten_mm_easy_reading_scenarios_works() {
        let focused_image = image::open("test_images/ol_qr_10mm_ultra_focused.png").unwrap();
        let result = identify_card_from(&focused_image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }

    #[test]
    fn ten_mm_reading_scenarios_works() {
        let focused_image = image::open("test_images/ol_qr_10mm_focused.png").unwrap();
        let result = identify_card_from(&focused_image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }
}
