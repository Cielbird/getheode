use crate::{
    d3tree,
    phonology::{
        feature::FeatureState::*,
        rule::{
            PatternBorder, PhonoRule, PhonoRuleParseOpts, PhonoRuleSet, PhonoStringPattern,
            SegmentInfo, SyllableInfo, TaggedPhonoString,
        },
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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: UNDEF_SEG},
                SegmentInfo {tag: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
                SegmentInfo {tag: Some(2), features: CONS_SEG},
                SegmentInfo {tag: Some(3), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
                SegmentInfo {tag: Some(2), features: VOI_SEG},
                SegmentInfo {tag: Some(3), features: VOWEL_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: UNDEF_SEG},
            ],
            SyllableInfo {tag: None, features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: VOWEL_SEG},
            ],
            SyllableInfo {tag: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: UNDEF_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(1), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(2), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: UNDEF_SEG},
            ]
        ],
        () => [
            SyllableInfo {tag: None, features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: VOWEL_SEG},
            ],
        ],
        () => [
            SyllableInfo {tag: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: UNDEF_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(1), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

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
    let pattern = PhonoStringPattern::new(
        d3tree![
            () => [
                SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                    SegmentInfo {tag: Some(0), features: VOWEL_SEG},
                ],
            ],
        ],
        PatternBorder::StrictSegment,
        PatternBorder::SyllableOrWord,
    );
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: UNDEF_SEG},
                SegmentInfo {tag: None, features: T_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(pattern, TaggedPhonoString::new(replace_tree));

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
    let pattern = PhonoStringPattern::new(
        d3tree![
            () => [
                SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                    SegmentInfo {tag: Some(0), features: VOWEL_SEG},
                ],
            ],
        ],
        PatternBorder::Word,
        PatternBorder::Any,
    );
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: UNDEF_SEG},
                SegmentInfo {tag: None, features: T_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(pattern, TaggedPhonoString::new(replace_tree));

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
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: VOWEL_SEG},
            ],
        ],
        () => [
            SyllableInfo {tag: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: UNDEF_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(0), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

    assert!(!rule.test_invariants());
}

#[test]
fn test_invalid_rule_undef_id() {
    // rule follows this rule:
    // V.V => VtV
    let match_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(0), features: VOWEL_SEG},
            ],
        ],
        () => [
            SyllableInfo {tag: Some(1), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: VOWEL_SEG},
            ]
        ]
    ];
    let replace_tree = d3tree![
        () => [
            SyllableInfo {tag: Some(0), features: UNDEF_SYL} => [
                SegmentInfo {tag: Some(1), features: UNDEF_SEG},
                SegmentInfo {tag: None, features: T_SEG},
                SegmentInfo {tag: Some(2), features: UNDEF_SEG},
            ]
        ]
    ];
    let rule = PhonoRule::new(
        PhonoStringPattern::new(match_tree, PatternBorder::Any, PatternBorder::Any),
        TaggedPhonoString::new(replace_tree),
    );

    assert!(!rule.test_invariants());
}

/// Macro for generating tests for phonlogical rule syntax parsing
use paste::paste;
macro_rules! gen_test_rule_apply {
    ($name:ident, $rule:expr, $input:expr, $expected:expr) => {
        paste! {
            #[test]
            fn [<test_rule_apply_ $name>]() {
                let opts = PhonoRuleParseOpts::default();
                let rule_set = PhonoRuleSet::parse($rule, opts).unwrap();

                let (_, string) = PhonoString::parse($input).unwrap();
                let (_, expected) = PhonoString::parse($expected).unwrap();
                let actual = rule_set.apply(string);
                if actual != expected {
                    panic!("expected=[{expected}] != actual=[{actual}]");
                }
            }
        }
    };
}

gen_test_rule_apply!(intervocalic, "t -> d / V_V", "ata", "ada");
gen_test_rule_apply!(word_bounded_matches, "t -> d / #V_V#", "ata", "ada");
gen_test_rule_apply!(word_bounded_no_match, "t -> d / #V_V#", "tata", "tata");
gen_test_rule_apply!(
    across_syllables_matches,
    "t -> d / V$_V",
    "mi$tan",
    "mi$dan"
);
gen_test_rule_apply!(
    across_syllables_no_match,
    "t -> d / V$_V",
    "nat$ip",
    "nat$ip"
);
gen_test_rule_apply!(
    many_options_niham,
    "n -> l / #_(V){s,ʃ,h}V{m,b}#",
    "niham",
    "liham"
);
gen_test_rule_apply!(
    many_options_nhab,
    "n -> l / #_(V){s,ʃ,h}V{m,b}#",
    "nhab",
    "lhab"
);
gen_test_rule_apply!(
    many_options_nosim,
    "n -> l / #_(V){s,ʃ,h}V{m,b}#",
    "nosim",
    "losim"
);
gen_test_rule_apply!(right_border, "j -> ∅ / Ck_$", "eskj.mo", "esk.mo");
gen_test_rule_apply!(
    right_optional,
    "{n,q,h} -> Ø / _(s)",
    "oqs.in.ihso",
    "os.i.iso"
);
gen_test_rule_apply!(across_word, "t -> d / V#_V", "a#ta", "a#da");
gen_test_rule_apply!(shwa_removal_and_arrow, "ə → ∅ / _#", "taɪmə", "taɪm");
gen_test_rule_apply!(multiple, "θ ð → t d", "ði.θo", "di.to");
gen_test_rule_apply!(twin_tags, "V_0ʔV_0 -> Vː_0", "aʔaʔi", "aːʔi");
