use crate::phoneme::{PhonemeBank, PhonemeString, PhonemeStringSylable};
use crate::segment::FormatIpa;
use crate::segment::Segment;

#[test]
fn test_phonemes_simple() {
    let mut bank = PhonemeBank::new();
    let t = bank
        .add(Segment::parse_ipa("t").unwrap(), "t".to_string())
        .unwrap();
    let i = bank
        .add(Segment::parse_ipa("i").unwrap(), "i".to_string())
        .unwrap();
    let a = bank
        .add(Segment::parse_ipa("a").unwrap(), "a".to_string())
        .unwrap();

    let string =
        PhonemeString::parse_phonemes("/tia/", &bank).expect("Phoneme string parsing failed");

    let expected = PhonemeString {
        phonemes: vec![t, i, a],
        sylables: vec![PhonemeStringSylable {
            start: 0,
            end: 3,
            stressed: false,
        }],
    };
    assert_eq!(string, expected);
}
#[test]
fn test_phonemes_sylables() {
    let mut bank = PhonemeBank::new();
    let t = bank
        .add(Segment::parse_ipa("t").unwrap(), "t".to_string())
        .unwrap();
    let i = bank
        .add(Segment::parse_ipa("i").unwrap(), "i".to_string())
        .unwrap();
    let a = bank
        .add(Segment::parse_ipa("a").unwrap(), "a".to_string())
        .unwrap();

    let string = PhonemeString::parse_phonemes("/ti.ta'at.ai/", &bank)
        .expect("Phoneme string parsing failed");

    let expected = PhonemeString {
        phonemes: vec![t, i, t, a, a, t, a, i],
        sylables: vec![
            PhonemeStringSylable {
                start: 0,
                end: 2,
                stressed: false,
            },
            PhonemeStringSylable {
                start: 2,
                end: 4,
                stressed: false,
            },
            PhonemeStringSylable {
                start: 4,
                end: 6,
                stressed: true,
            },
            PhonemeStringSylable {
                start: 6,
                end: 8,
                stressed: false,
            },
        ],
    };
    assert_eq!(string, expected);
}
