from detect_card import extract_card_text
import cv2


def test_extract_card_test():
    image = cv2.imread("tests/Ferrox.jpg_detected.jpg")
    assert "Ferrox" in extract_card_text(image)
