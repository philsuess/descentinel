use std::io::Cursor;

use image::{io::Reader, ImageBuffer, Luma};
use log::info;
use quircs::{DecodeError, ExtractError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetectCardError {
    #[error("qr error: {0}")]
    QrCodeExtractionError(#[from] ExtractError),
    #[error("qr error: {0}")]
    QrCodeDecodingError(#[from] DecodeError),
}

type GameRoomImage = ImageBuffer<Luma<u8>, Vec<u8>>;

pub fn convert_to_grey_image(image_buffer: &[u8]) -> GameRoomImage {
    Reader::new(Cursor::new(image_buffer))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8()
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
    let mut decoder = quircs::Quirc::default();
    let codes = decoder.identify(image.width() as usize, image.height() as usize, image);

    let mut decoded_strings = vec![];
    for code in codes {
        let code = code.expect("failed to extract qr code");
        let decoded = code.decode().expect("failed to decode qr code");
        let found_message = String::from_utf8(decoded.payload).unwrap();
        info!("got {} from a decoding", found_message);
        decoded_strings.push(found_message);
    }
    decoded_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_qr_detection_from_file_works() {
        let image = image::open("test_images/ol_doom.png").unwrap().into_luma8();
        let result = identify_card_from(&image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }

    #[test]
    fn twenty_mm_reading_scenario_works() {
        let image = image::open("test_images/ol_qr_20mm_focused.png")
            .unwrap()
            .into_luma8();
        let result = identify_card_from(&image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }

    #[test]
    #[ignore = "qr scanning not good enough yet..."]
    fn ten_mm_easy_reading_scenarios_works() {
        let focused_image = image::open("test_images/ol_qr_10mm_ultra_focused.png")
            .unwrap()
            .into_luma8();
        let result = identify_card_from(&focused_image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }

    #[test]
    #[ignore = "qr scanning not good enough yet..."]
    fn ten_mm_reading_scenarios_works() {
        let focused_image = image::open("test_images/ol_qr_10mm_focused.png")
            .unwrap()
            .into_luma8();
        let result = identify_card_from(&focused_image);
        assert!(result == Some(String::from("overlordcard/doom")));
    }
}
