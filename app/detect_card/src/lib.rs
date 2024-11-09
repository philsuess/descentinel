use serde::Deserialize;
use tesseract::Tesseract;

pub fn load_overlord_keywords(file_name: &str) -> OverlordCards {
    let file = std::fs::File::open(file_name).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).expect("file is not proper json")
}

#[derive(Deserialize, Debug)]
pub struct OverlordCards {
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

pub fn identify_card(card_image_buffer: &[u8], overlord_cards: &OverlordCards) -> Option<String> {
    let card_text = extract_card_text_from_buffer(card_image_buffer, "fra");
    overlord_cards.id_of_best_keywords_match(&card_text)
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
