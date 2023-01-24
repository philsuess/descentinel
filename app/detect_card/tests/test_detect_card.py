from detect_card import extract_card_text, OverlordCardsKeywordsMatcher
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
