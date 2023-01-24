import cv2
import pytesseract


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
