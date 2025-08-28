use crate::segment::FeatureState::{NA, NEG, POS};
use crate::segment::bounds::ChunkSequence;
use crate::segment::{FormatSegmentString, Segment, SegmentString, Sylable};

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
    let seg_str = SegmentString::parse("aski").unwrap();
    assert_eq!(
        seg_str,
        SegmentString::from_segments(vec![
            A_SEG.clone(),
            S_SEG.clone(),
            K_SEG.clone(),
            I_SEG.clone(),
        ]),
    );
}

#[test]
fn segment_string_parse_multi_chars() {
    let seg_str = SegmentString::parse("t͡ʃa").unwrap();
    assert_eq!(
        seg_str,
        SegmentString::from_segments(vec![CH_SEG.clone(), A_SEG.clone(),]),
    );
}

#[test]
fn segment_string_parse_word_bound() {
    let seg_str = SegmentString::parse("as#ki").unwrap();

    assert_eq!(seg_str.words, ChunkSequence::new(4, [2], [(), ()]));
}

#[test]
fn segment_string_parse_syl_bounds() {
    let seg_str = SegmentString::parse("as.ki'ki").unwrap();

    assert_eq!(
        seg_str.sylables,
        ChunkSequence::new(
            6,
            [2, 4],
            [
                Sylable { stressed: false },
                Sylable { stressed: false },
                Sylable { stressed: true }
            ]
        ),
    );
}

#[test]
fn segment_string_parse_syl_and_word_bounds() {
    let expected = SegmentString {
        segs: vec![
            A_SEG.clone(),
            S_SEG.clone(),
            I_SEG.clone(),
            K_SEG.clone(),
            I_SEG.clone(),
            K_SEG.clone(),
            A_SEG.clone(),
        ],
        words: ChunkSequence::new(7, [0, 3], [(), ()]),
        sylables: ChunkSequence::new(
            7,
            [0, 1, 3, 5],
            [
                Sylable { stressed: false },
                Sylable { stressed: false },
                Sylable { stressed: false },
                Sylable { stressed: true },
            ],
        ),
    };
    let actual = SegmentString::parse("_a.si_ki'ka").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format() {
    // test simple format
    let string = SegmentString::from_segments(vec![
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
    let string = SegmentString::from_segments(vec![
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
    let string = SegmentString::from_segments(vec![]);
    let actual = string.format();
    let expected = "[]".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_word_bounds() {
    let string = SegmentString {
        segs: vec![A_SEG.clone(), S_SEG.clone(), K_SEG.clone(), I_SEG.clone()],
        words: ChunkSequence::new(4, [2, 4], [(), ()]),
        sylables: ChunkSequence::new(4, [2, 4], [Sylable::default(), Sylable::default()]),
    };
    let actual = string.format();
    let expected = "as_ki_".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_syl_bounds() {
    let string = SegmentString {
        segs: vec![A_SEG.clone(), S_SEG.clone(), K_SEG.clone(), I_SEG.clone()],
        words: ChunkSequence::single_unbounded(4),
        sylables: ChunkSequence::new(4, [0, 2], [Sylable::default(), Sylable::default()]),
    };
    let actual = string.format();
    let expected = ".as'ki".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn segment_string_format_syl_and_word_bounds() {
    let string = SegmentString {
        segs: vec![
            A_SEG.clone(),
            S_SEG.clone(),
            I_SEG.clone(),
            K_SEG.clone(),
            I_SEG.clone(),
            K_SEG.clone(),
            A_SEG.clone(),
        ],
        words: ChunkSequence::new(7, [0, 3], [(), ()]),
        sylables: ChunkSequence::new(
            7,
            [0, 1, 3, 5],
            [
                Sylable { stressed: false },
                Sylable { stressed: false },
                Sylable { stressed: false },
                Sylable { stressed: true },
            ],
        ),
    };
    let actual = string.format();
    let expected = "_a.si_ki'ka".to_string();
    assert_eq!(actual, expected);
}
