use crate::{
    phonology::{
        feature::FeatureState::*,
        pattern::{PatternMatch, PhonoPattern, SegmentInfo, SyllableInfo},
        segment::{SEG_FEATURE_COUNT, SegmentFeatures},
        string::PhonoString,
        syllable::SyllableFeatures,
    },
    ud3tree,
};

const VOWEL_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
]);
const T_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, POS, NEG,
    NEG, NEG, NEG, NA, NA, NA, NA, NA,
]);
const A_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA,
    NA, NEG, POS, NEG, POS, NEG, NEG, NA,
]);
const I_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA,
    NA, NEG, POS, POS, NEG, POS, NEG, POS,
]);
const UNDEF_SEG: SegmentFeatures =
    SegmentFeatures::from_features([UNDEF; SEG_FEATURE_COUNT as usize]);

const UNSTRESSED: SyllableFeatures = SyllableFeatures::new([NEG]);
const UNDEF_SYL: SyllableFeatures = SyllableFeatures::new([UNDEF]);

#[test]
fn test_pattern() {
    // pattern follows this rule:
    // VtV => VV / (all in same syllable and word)
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: 0, features: UNDEF_SYL} => [
                SegmentInfo {id: 1, features: VOWEL_SEG},
                SegmentInfo {id: 2, features: T_SEG},
                SegmentInfo {id: 3, features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: 0, features: UNDEF_SYL} => [
                SegmentInfo {id: 1, features: UNDEF_SEG},
                SegmentInfo {id: 3, features: UNDEF_SEG},
            ]
        ]
    ];
    let pattern = PhonoPattern::new(match_tree, replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(ud3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = pattern.find(hay);


    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 2..5);
    
    let expected_replace_with = PhonoString::new(ud3tree![
        () => [
            UNSTRESSED => [A_SEG, I_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}
