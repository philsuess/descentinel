import cv2
import numpy as np
from enum import Enum


class CardType(Enum):
    NOT_RECOGNIZED = -1
    OVERLORD = 1


class CardDetector:
    def __init__(self, template_overlord_card_cv2_image) -> None:
        self._train_overlord_card_detection(template_overlord_card_cv2_image)

    def _detect_and_compute(self, cv_gray_scaled_image):
        return self.sift.detectAndCompute(cv_gray_scaled_image, None)

    def _train_overlord_card_detection(self, template_overlord_card_cv2_image):
        self.sift = cv2.SIFT_create()
        self.bf = cv2.BFMatcher()
        self.ol_kp_template, self.ol_des_template = self.sift.detectAndCompute(
            template_overlord_card_cv2_image, None
        )

    def _find_good_matches(self, cv_gray_scaled_image):
        quality_threshold = 0.5
        kp_bs2, des_bs2 = self._detect_and_compute(cv_gray_scaled_image)
        matches = self.bf.knnMatch(self.ol_des_template, des_bs2, k=2)
        good = []
        for m, n in matches:
            if m.distance < quality_threshold * n.distance:
                good.append([m])

        return good, kp_bs2

    def detect(self, cv2_image) -> CardType:
        good_matches, kp = self._find_good_matches(cv2_image)

        if len(good_matches) > 5:
            dst_pts = np.float32([kp[m[0].trainIdx].pt for m in good_matches]).reshape(
                -1, 1, 2
            )
            bounding_rectangle = cv2.boundingRect(dst_pts)
            offset = 75
            x = bounding_rectangle[0] - offset
            y = bounding_rectangle[1] - offset
            w = bounding_rectangle[2] + offset
            h = bounding_rectangle[3] + offset
            image_cropped_to_OL_card = cv2_image[y : y + h, x : x + w].copy()
            return CardType.OVERLORD

        return CardType.NOT_RECOGNIZED
