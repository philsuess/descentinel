import cv2
import pytesseract
import json
from typing import Dict


def image_as_grayscale(cv2_image):
    return cv2.cvtColor(cv2_image, cv2.COLOR_BGR2GRAY)


def remove_noise(cv2_image):
    return cv2.medianBlur(cv2_image, 5)


def extract_card_text(cv2_image) -> str:
    scale_percent = 200  # percent of original size
    new_width = int(cv2_image.shape[1] * scale_percent / 100)
    new_height = int(cv2_image.shape[0] * scale_percent / 100)
    cv2_image = cv2.resize(
        cv2_image, (new_width, new_height), interpolation=cv2.INTER_AREA
    )

    # image_gray = image_as_grayscale(cv2_image)
    # denoised = remove_noise(image_gray)
    # cv2.imwrite(f"{image}_denoised.jpg", denoised)
    custom_config = r""
    return pytesseract.image_to_string(cv2_image, config=custom_config)

def encode_image(cv2_image):
    encode_param = [int(cv2.IMWRITE_JPEG_QUALITY), 100]
    _, buffered = cv2.imencode('.jpg', cv2_image, encode_param)
    return buffered

def decode_image(encoded):
    return cv2.imdecode(encoded, -1)


class OverlordCardsKeywordsMatcher:
    def __init__(self, card_keywords: Dict) -> None:
        self.overlord_cards = card_keywords

    @classmethod
    def from_file(cls, file_name: str):
        with open(file_name) as f:
            overlord_cards_json = json.load(f)
            return cls(overlord_cards_json["cards"])

    def search_card_in_database(self, card_text: str) -> Dict:
        def match_card_text_with_keywords(card_keywords: list[str]) -> int:
            keyword_matches = 0
            for keyword in card_keywords:
                if keyword in card_text:
                    # print(f"\t\tfound {keyword} in text")
                    keyword_matches = keyword_matches + 1
            return keyword_matches

        best_matching_card = {}
        highest_keyword_match = 0
        for card in self.overlord_cards:
            card_score = match_card_text_with_keywords(card["keywords"])
            # print(f"\t{card['id']} has score {card_score}")
            if card_score > highest_keyword_match:
                best_matching_card = card
                highest_keyword_match = card_score

        return best_matching_card

    def identify(self, cv2_image) -> Dict:
        text_from_card = extract_card_text(cv2_image=cv2_image)
        return self.search_card_in_database(text_from_card)
