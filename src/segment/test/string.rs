use crate::segment::FeatureState::{NA, NEG, POS};
use crate::segment::{
    FormatPhonologicalString, PhonologicalElement, PhonologicalString, Segment,
};

const A_SEG: Segment = Segment::from_features([
    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
    NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA,
]);
const S_SEG: Segment = Segment::from_features([
    NEG, NEG, NEG, POS, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, POS,
    NEG, POS, NEG, NEG, NA, NA, NA, NA, NA,
]);
const K_SEG: Segment = Segment::from_features([
    NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA,
    NA, NA, NEG, POS, POS, NEG, NA, NA, NA,
]);
const I_SEG: Segment = Segment::from_features([
    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
    NA, NA, NEG, POS, POS, NEG, POS, NEG, POS,
]);
const CH_SEG: Segment = Segment::from_features([
    NEG, NEG, NEG, POS, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, NEG,
    POS, POS, NEG, NEG, NA, NA, NA, NA, NA,
]);

// TODO: see the todo in phonological_rule_test.rs: these tests are lazy
#[test]
fn parse_segment_string() {
    let seg_str = PhonologicalString::parse("aski").unwrap();
    assert_eq!(
        seg_str,
        PhonologicalString::from_segments(vec![
            A_SEG.clone(),
            S_SEG.clone(),
            K_SEG.clone(),
            I_SEG.clone(),
        ]),
    );
}

#[test]
fn segment_string_parse_multi_chars() {
    let seg_str = PhonologicalString::parse("t͡ʃa").unwrap();
    assert_eq!(
        seg_str,
        PhonologicalString::from_segments(vec![CH_SEG.clone(), A_SEG.clone(),]),
    );
}

#[test]
fn segment_string_parse_word_bound() {
    let str = PhonologicalString::parse("as#ki").unwrap();

    let expected = vec![
        PhonologicalElement::SegmentElement(A_SEG),
        PhonologicalElement::SegmentElement(S_SEG),
        PhonologicalElement::WordBoundary,
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
    ];
    assert_eq!(str.elements, expected);
}

#[test]
fn segment_string_parse_syl_bounds() {
    let str = PhonologicalString::parse("as.ki'ki").unwrap();

    let expected = vec![
        PhonologicalElement::SegmentElement(A_SEG),
        PhonologicalElement::SegmentElement(S_SEG),
        PhonologicalElement::SyllableBoundary { stressed: false },
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
        PhonologicalElement::SyllableBoundary { stressed: true },
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
    ];
    assert_eq!(str.elements, expected);
}

#[test]
fn segment_string_parse_syl_and_word_bounds() {
    let str = PhonologicalString::parse("_a.si_ki'ka").unwrap();

    let expected = vec![
        PhonologicalElement::WordBoundary,
        PhonologicalElement::SegmentElement(A_SEG),
        PhonologicalElement::SyllableBoundary { stressed: false },
        PhonologicalElement::SegmentElement(S_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
        PhonologicalElement::WordBoundary,
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
        PhonologicalElement::SyllableBoundary { stressed: true },
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(A_SEG),
    ];
    assert_eq!(str.elements, expected);
}

#[test]
fn segment_string_format() {
    // test simple format
    let string = PhonologicalString::from_segments(vec![
        A_SEG.clone(),
        S_SEG.clone(),
        K_SEG.clone(),
        I_SEG.clone(),
    ]);
    let actual = string.format();
    let expected = "aski".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_features() {
    let string = PhonologicalString::from_segments(vec![
        A_SEG.clone(),
        S_SEG.clone(),
        Segment::parse_feature_set("[-voi]").unwrap(),
        I_SEG.clone(),
    ]);
    let actual = string.format();
    let expected = "as[-voi]i".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_empty() {
    let string = PhonologicalString::from_segments(vec![]);
    let actual = string.format();
    let expected = "[]".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_word_bounds() {
    let string = PhonologicalString::from_elements(vec![
        PhonologicalElement::SegmentElement(A_SEG),
        PhonologicalElement::SegmentElement(S_SEG),
        PhonologicalElement::WordBoundary,
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
        PhonologicalElement::WordBoundary,
    ]);
    let actual = string.format();
    let expected = "as_ki_".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_syl_bounds() {
    let string = PhonologicalString::from_elements(vec![
        PhonologicalElement::SyllableBoundary { stressed: false },
        PhonologicalElement::SegmentElement(A_SEG),
        PhonologicalElement::SegmentElement(S_SEG),
        PhonologicalElement::SyllableBoundary { stressed: true },
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
    ]);
    let actual = string.format();
    let expected = ".as'ki".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_syl_and_word_bounds() {
    let string = PhonologicalString::from_elements(vec![
        PhonologicalElement::WordBoundary,
        PhonologicalElement::SegmentElement(A_SEG),
        PhonologicalElement::SyllableBoundary { stressed: false },
        PhonologicalElement::SegmentElement(S_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
        PhonologicalElement::WordBoundary,
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(I_SEG),
        PhonologicalElement::SyllableBoundary { stressed: true },
        PhonologicalElement::SegmentElement(K_SEG),
        PhonologicalElement::SegmentElement(A_SEG),
    ]);
    let actual = string.format();
    let expected = "_a.si_ki'ka".to_string();
    assert_eq!(actual, expected);
}
