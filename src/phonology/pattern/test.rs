use crate::{
    phonology::{
        feature::FeatureState::*,
        pattern::{PhonoPattern, SegmentInfo, SyllableInfo},
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
fn test_pattern_simple() {
    // pattern follows this rule:
    // VtV => VV / (all in same syllable and word)
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
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

#[test]
fn test_pattern_new_syllable() {
    // pattern follows this rule:
    // VtV => V.V / (in same syllable and word, creates new syllable boundary)
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
            ],
            SyllableInfo {id: None, features: UNDEF_SYL} => [
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
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
            UNSTRESSED => [A_SEG],
            UNDEF_SYL => [I_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_pattern_across_syllable() {
    // pattern follows this rule:
    // V.V => VtV / (across syllable bound, removes it)
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: VOWEL_SEG},
            ],
            SyllableInfo {id: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
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

    assert_eq!(pat_match.range, 1..3);

    let expected_replace_with = PhonoString::new(ud3tree![
        () => [
            UNSTRESSED => [A_SEG, T_SEG, A_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}


#[test]
fn test_pattern_new_word() {
    // pattern follows this rule:
    // VtV => V.V / (in same syllable and word, creates new syllable boundary)
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
            ]
        ],
        () => [
            SyllableInfo {id: None, features: UNDEF_SYL} => [
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
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
            UNSTRESSED => [A_SEG],
        ],
        () => [
            UNDEF_SYL => [I_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_pattern_across_word() {
    // pattern follows this rule:
    // V.V => VtV / (across syllable bound, removes it)
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: VOWEL_SEG},
            ],
        ],
        () => [
            SyllableInfo {id: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
            ]
        ]
    ];
    let pattern = PhonoPattern::new(match_tree, replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(ud3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
        ],
        () => [
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = pattern.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 1..3);

    let expected_replace_with = PhonoString::new(ud3tree![
        () => [
            UNSTRESSED => [A_SEG, T_SEG, A_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_invalid_pattern_double_id() {
    // pattern follows this rule:
    // V.V => VtV
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: VOWEL_SEG},
            ],
        ],
        () => [
            SyllableInfo {id: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
            ]
        ]
    ];
    let pattern = PhonoPattern::new(match_tree, replace_tree);

    assert!(!pattern.test_invariants());
}

#[test]
fn test_invalid_pattern_undef_id() {
    // pattern follows this rule:
    // V.V => VtV
    let match_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: VOWEL_SEG},
            ],
        ],
        () => [
            SyllableInfo {id: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = ud3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let pattern = PhonoPattern::new(match_tree, replace_tree);
    
    assert!(!pattern.test_invariants());
}
