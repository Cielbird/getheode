use crate::{
    d3tree,
    phonology::{
        feature::FeatureState::*,
        rule::{PatternBorder, PhonoRule, PhonoStringPattern, SegmentInfo, SyllableInfo},
        segment::{SEG_FEATURE_COUNT, SegmentFeatures},
        string::PhonoString,
        syllable::SyllableFeatures,
    },
};

const VOWEL_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
]);
const CONS_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
]);
const T_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, POS, NEG,
    NEG, NEG, NEG, NA, NA, NA, NA, NA,
]);
const D_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, POS, POS, NEG,
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
const VOI_SEG: SegmentFeatures = SegmentFeatures::from_features([
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, POS, UNDEF, UNDEF, UNDEF,
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
]);
const UNDEF_SEG: SegmentFeatures =
    SegmentFeatures::from_features([UNDEF; SEG_FEATURE_COUNT as usize]);

const UNSTRESSED: SyllableFeatures = SyllableFeatures::new([NEG]);
const UNDEF_SYL: SyllableFeatures = SyllableFeatures::new([UNDEF]);

#[test]
fn test_rule_simple() {
    // rule follows this rule:
    // VtV => VV / (all in same syllable and word)
    let match_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = rule.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 2..5);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG, I_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_rule_modify_vowel() {
    // rule follows this rule:
    // VCV => VC[+voi]V / (all in same syllable and word)
    // for example, [ati] becomes [adi]
    let match_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: Some(2), features: CONS_SEG},
                SegmentInfo {id: Some(3), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: Some(2), features: VOI_SEG},
                SegmentInfo {id: Some(3), features: VOWEL_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = rule.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 2..5);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG, D_SEG, I_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_rule_new_syllable() {
    // rule follows this rule:
    // VtV => V.V / (in same syllable and word, creates new syllable boundary)
    let match_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
            ],
            SyllableInfo {id: None, features: UNDEF_SYL} => [
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = rule.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 2..5);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG],
            UNDEF_SYL => [I_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_rule_across_syllable() {
    // rule follows this rule:
    // V.V => VtV / (across syllable bound, removes it)
    let match_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: VOWEL_SEG},
            ],
            SyllableInfo {id: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = rule.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 1..3);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG, T_SEG, A_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_rule_new_word() {
    // rule follows this rule:
    // VtV => V.V / (in same syllable and word, creates new syllable boundary)
    let match_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: VOWEL_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
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
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = rule.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 2..5);

    let expected_replace_with = PhonoString::new(d3tree![
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
fn test_rule_across_word() {
    // rule follows this rule:
    // V.V => VtV / (across syllable bound, removes it)
    let match_tree = d3tree![
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
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
        ],
        () => [
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let mut matches = rule.find(hay);

    assert_eq!(matches.len(), 1);
    let pat_match = matches.remove(0);

    assert_eq!(pat_match.range, 1..3);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG, T_SEG, A_SEG],
        ]
    ]);
    assert_eq!(pat_match.replace_with, expected_replace_with)
}

#[test]
fn test_rule_syllable_border_1() {
    // rule follows this rule:
    // V => Vt / _$
    let pattern = PhonoStringPattern {
        tree: d3tree![
            () => [
                SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                    SegmentInfo {id: Some(0), features: VOWEL_SEG},
                ],
            ],
        ],
        left_bound: PatternBorder::StrictSegment,
        right_bound: PatternBorder::SyllableOrWord,
    };
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(pattern, replace_tree);

    // [ta.ati]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [T_SEG, A_SEG],
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let matches = rule.find(hay);

    assert_eq!(matches.len(), 2);

    assert_eq!(matches[0].range, 1..2);
    assert_eq!(matches[1].range, 4..5);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG, T_SEG],
        ]
    ]);
    assert_eq!(matches[0].replace_with, expected_replace_with);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [I_SEG, T_SEG],
        ]
    ]);
    assert_eq!(matches[1].replace_with, expected_replace_with);
}

#[test]
fn test_rule_syllable_border_2() {
    // rule follows this rule:
    // V => Vt / #_
    let pattern = PhonoStringPattern {
        tree: d3tree![
            () => [
                SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                    SegmentInfo {id: Some(0), features: VOWEL_SEG},
                ],
            ],
        ],
        left_bound: PatternBorder::Word,
        right_bound: PatternBorder::Any,
    };
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(pattern, replace_tree);

    // [a#a.ti]
    let hay = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG],
        ],
        () => [
            UNSTRESSED => [A_SEG, T_SEG, I_SEG],
        ]
    ]);

    let matches = rule.find(hay);

    assert_eq!(matches.len(), 2);

    assert_eq!(matches[0].range, 0..1);
    assert_eq!(matches[1].range, 1..2);

    let expected_replace_with = PhonoString::new(d3tree![
        () => [
            UNSTRESSED => [A_SEG, T_SEG],
        ]
    ]);
    assert_eq!(matches[0].replace_with, expected_replace_with);
    assert_eq!(matches[1].replace_with, expected_replace_with);
}

#[test]
fn test_invalid_rule_double_id() {
    // rule follows this rule:
    // V.V => VtV
    let match_tree = d3tree![
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
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(0), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    assert!(!rule.test_invariants());
}

#[test]
fn test_invalid_rule_undef_id() {
    // rule follows this rule:
    // V.V => VtV
    let match_tree = d3tree![
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
    let replace_tree = d3tree![
        () => [
            SyllableInfo {id: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {id: Some(1), features: UNDEF_SEG},
                SegmentInfo {id: None, features: T_SEG},
                SegmentInfo {id: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(PhonoStringPattern::new(match_tree), replace_tree);

    assert!(!rule.test_invariants());
}
