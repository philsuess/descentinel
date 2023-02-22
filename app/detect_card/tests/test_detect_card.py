from overlord_card_match import (
    extract_card_text,
    OverlordCardsKeywordsMatcher,
    encode_image,
    decode_image,
)
from recognize_card import CardDetector, CardType
import cv2
import numpy as np


def test_detect_overlord_cards():
    template_overlord_image = cv2.imread("OL_template.jpg", cv2.IMREAD_GRAYSCALE)
    detector = CardDetector(template_overlord_card_cv2_image=template_overlord_image)

    image = cv2.imread("tests/BaumeSombre_02.jpg_detected.jpg")
    assert detector.detect(image) == CardType.OVERLORD

    image = cv2.imread("tests/BaumSombre.jpg_detected.jpg")
    assert detector.detect(image) == CardType.OVERLORD

    image = cv2.imread("tests/RuneExplosive.jpg_detected.jpg")
    assert detector.detect(image) == CardType.OVERLORD

    image = cv2.imread("tests/Not_an_OL_card_01_cropped.jpg")
    assert detector.detect(image) == CardType.NOT_RECOGNIZED


def test_extract_card():
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


def tests_cards_keywords():
    matcher = OverlordCardsKeywordsMatcher.from_file("./keywords_cards.json")
    assert (
        "Dark Balm"
        == matcher.search_card_in_database("les pions Baum Sombre pions effet")["id"]
    )


def test_identify_cards():
    matcher = OverlordCardsKeywordsMatcher.from_file("./keywords_cards.json")
    image = cv2.imread("tests/BaumeSombre_02.jpg_detected.jpg")
    assert "Dark Balm" == matcher.identify(image)["id"]

    image = cv2.imread("tests/BaumSombre.jpg_detected.jpg")
    assert "Dark Balm" == matcher.identify(image)["id"]

    image = cv2.imread("tests/ExpodierendeRune.jpg_detected.jpg")
    assert "Explosive Rune" == matcher.identify(image)["id"]

    image = cv2.imread("tests/Ferrox.jpg_detected.jpg")
    assert "Ferrox Tribe" == matcher.identify(image)["id"]

    image = cv2.imread("tests/Höllenhunde.jpg_detected.jpg")
    assert "Hell Hound Pack" == matcher.identify(image)["id"]

    image = cv2.imread("tests/RuneExplosive.jpg_detected.jpg")
    assert "Explosive Rune" == matcher.identify(image)["id"]


def test_images_encoding():
    def are_images_equal(img1, img2):
        err = np.sum((img1.astype("float") - img2.astype("float")) ** 2)
        err /= float(img1.shape[0] * img2.shape[1])
        print(err)
        return err < 0.2

    def run_test_for(image_path):
        image = cv2.imread(image_path)
        encoded = encode_image(image)
        decoded = decode_image(encoded)
        assert are_images_equal(image, decoded)

    run_test_for("tests/BaumeSombre_02.jpg_detected.jpg")
    run_test_for("tests/RuneExplosive.jpg_detected.jpg")
    run_test_for("tests/Höllenhunde.jpg_detected.jpg")
