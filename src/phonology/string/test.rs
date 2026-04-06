use crate::phonology::feature::FeatureState::*;
use crate::phonology::syllable::SyllableFeatures;
use crate::phonology::{segment::SegmentFeatures, string::PhonoString};
use crate::ud3tree;

const A_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA,
    NA, NEG, POS, NEG, POS, NEG, NEG, NA,
]);
const K_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA,
    NA, NEG, POS, POS, NEG, NA, NA, NA,
]);
const I_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA,
    NA, NEG, POS, POS, NEG, POS, NEG, POS,
]);

const UNSTRESSED: SyllableFeatures = SyllableFeatures::new([NEG]);
const STRESSED: SyllableFeatures = SyllableFeatures::new([POS]);

#[test]
fn string_replace_in_syl() {
    // start with ['kai.ka]
    // replace first 3 segments with [ia]
    // end with ['ia.ka]

    let string = PhonoString::new(ud3tree![
        () => [
            STRESSED => [K_SEG, A_SEG, I_SEG],
            UNSTRESSED => [K_SEG, A_SEG]
        ]
    ]);

    let replacement = PhonoString::new(ud3tree!(
        () => [STRESSED => [I_SEG, A_SEG]]
    ));

    let expected = PhonoString::new(ud3tree!(
        () => [
            STRESSED => [I_SEG, A_SEG],
            UNSTRESSED => [K_SEG, A_SEG],
        ]
    ));

    let string = string.clone().replace_range(0..3, replacement).unwrap();
    assert_eq!(string, expected,);
}

#[test]
fn string_replace_across_syl_bound() {
    // start with ['kai.ka]
    // replace segments [2,4) with [ia]
    // end with ['kaiaa]
    // syllable boundary in source is replaced

    let string = PhonoString::new(ud3tree!(
        () => [
            STRESSED => [K_SEG, A_SEG, I_SEG],
            UNSTRESSED => [K_SEG, A_SEG],
        ]
    ));

    let replacement = PhonoString::new(ud3tree!(
        () => [STRESSED => [I_SEG, A_SEG]]
    ));

    let expected = PhonoString::new(ud3tree![
        () => [
            STRESSED => [K_SEG, A_SEG, I_SEG, A_SEG, A_SEG],
        ],
    ]);

    let string = string.clone().replace_range(2..4, replacement).unwrap();
    assert_eq!(string, expected,);
}

#[test]
fn string_replace_with_syl_bound() {
    // start with ['kai.ka]
    // replace segment 1 with [i.ki]
    // end with ['ki.kii.ka]
    // syllable boundary in source is replaced

    let string = PhonoString::new(ud3tree!(
        () => [
            STRESSED => [K_SEG, A_SEG, I_SEG],
            UNSTRESSED => [K_SEG, A_SEG],
        ]
    ));

    let replacement = PhonoString::new(ud3tree!(
        () => [
            STRESSED => [I_SEG],
            UNSTRESSED => [K_SEG, I_SEG],
        ]
    ));

    let expected = PhonoString::new(ud3tree!(
        () => [
            STRESSED => [K_SEG, I_SEG],
            UNSTRESSED => [K_SEG, I_SEG, I_SEG],
            UNSTRESSED => [K_SEG, A_SEG],
        ]
    ));

    let string = string.clone().replace_range(1..2, replacement).unwrap();
    assert_eq!(string, expected,);
}
