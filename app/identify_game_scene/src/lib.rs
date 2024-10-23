use opencv::prelude::{DescriptorMatcherTraitConst, Feature2DTrait};

pub fn convert_to_opencv_image(card_image_buffer: &[u8]) -> opencv::Result<opencv::core::Mat> {
    opencv::imgcodecs::imdecode(
        &opencv::core::Vector::from_slice(card_image_buffer),
        opencv::imgcodecs::IMREAD_COLOR,
    )
}

#[derive(PartialEq)]
pub enum CardType {
    Overlord,
}

#[derive(Debug)]
pub struct CardDetector {
    sift: opencv::core::Ptr<opencv::features2d::SIFT>,
    bf: opencv::core::Ptr<opencv::features2d::BFMatcher>,
    overlord_descriptors_from_template: opencv::core::Mat,
}

impl CardDetector {
    pub fn new(overlord_card_template: &opencv::core::Mat) -> CardDetector {
        let mut sift = opencv::features2d::SIFT::create(0, 3, 0.04, 10., 1.6, false).unwrap();
        let mut ol_kp_template = opencv::core::Vector::<opencv::core::KeyPoint>::default();
        let mut ol_des_template = opencv::core::Mat::default();
        let mask = opencv::core::Mat::default();
        sift.detect_and_compute(
            &overlord_card_template,
            &mask,
            &mut ol_kp_template,
            &mut ol_des_template,
            false,
        )
        .unwrap();

        let bf = opencv::features2d::BFMatcher::create(opencv::core::NORM_L2, false).unwrap();

        CardDetector {
            sift,
            bf,
            overlord_descriptors_from_template: ol_des_template,
        }
    }

    pub fn detect_card(&mut self, image: &opencv::core::Mat) -> Option<CardType> {
        if self.score_against_overlord_card(image) > 5 {
            return Some(CardType::Overlord);
        }
        None
    }

    fn score_against_overlord_card(&mut self, image: &opencv::core::Mat) -> u32 {
        let mut test_image_kp = opencv::core::Vector::<opencv::core::KeyPoint>::default();
        let mut test_image_des = opencv::core::Mat::default();
        let mask = opencv::core::Mat::default();
        self.sift
            .detect_and_compute(
                &image,
                &mask,
                &mut test_image_kp,
                &mut test_image_des,
                false,
            )
            .unwrap();

        let mut matches =
            opencv::core::Vector::<opencv::core::Vector<opencv::core::DMatch>>::default();
        self.bf
            .knn_train_match(
                &self.overlord_descriptors_from_template,
                &test_image_des,
                &mut matches,
                2,
                &mask,
                false,
            )
            .unwrap();

        let mut good_match = opencv::core::Vector::<opencv::core::DMatch>::default();
        let quality_threshold = 0.5;
        for match_pair in matches {
            if match_pair.len() > 1
                && match_pair.get(0).unwrap().distance
                    < quality_threshold * match_pair.get(1).unwrap().distance
            {
                good_match.push(match_pair.get(0).unwrap());
            }
        }
        good_match.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlord_card_detection_works() {
        let template_overlord_card =
            opencv::imgcodecs::imread("./OL_template.jpg", opencv::imgcodecs::IMREAD_COLOR)
                .unwrap();
        let mut card_detector = CardDetector::new(&template_overlord_card);

        let test_ol_image = opencv::imgcodecs::imread(
            "./test_images/ExplodierendeRune.jpg_detected.jpg",
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();
        assert!(card_detector.detect_card(&test_ol_image) == Some(CardType::Overlord));

        let test_not_an_ol_image = opencv::imgcodecs::imread(
            "./test_images/Not_an_OL_card_01_cropped.jpg",
            opencv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();
        assert!(card_detector.detect_card(&test_not_an_ol_image).is_none());
    }
}
