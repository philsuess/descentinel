from detect_card import extract_card_text
import cv2


def test_extract_card_test():
    image = cv2.imread("tests/BaumeSombre_02.jpg_detected.jpg")
    assert "Baume" in extract_card_text(image)

    image = cv2.imread("tests/BaumSombre.jpg_detected.jpg")
    assert "tous les pions effet" in extract_card_text(image)

    image = cv2.imread("tests/ExpodierendeRune.jpg_detected.jpg")
    assert "Kune" in extract_card_text(image)

    image = cv2.imread("tests/Ferrox.jpg_detected.jpg")
    assert "Ferrox" in extract_card_text(image)

    image = cv2.imread("tests/Höllenhunde.jpg_detected.jpg")
    assert "hund" in extract_card_text(image)

    image = cv2.imread("tests/RuneExplosive.jpg_detected.jpg")
    assert "coffre" in extract_card_text(image)
