use crate::segment::{Segment, PhonologicalString};
use crate::segment::FeatureState::{NA, NEG, POS};



const A_SEG: Segment = Segment::from_features([
    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
    NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA,
]);
const K_SEG: Segment = Segment::from_features([
    NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA,
    NA, NA, NEG, POS, POS, NEG, NA, NA, NA,
]);
const I_SEG: Segment = Segment::from_features([
    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
    NA, NA, NEG, POS, POS, NEG, POS, NEG, POS,
]);

#[test]
fn segment_string_replace() {
    let mut string = PhonologicalString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]);
    let replacement = PhonologicalString::from_segments([I_SEG, A_SEG]);
    let expected = PhonologicalString::from_segments([I_SEG, A_SEG, K_SEG, A_SEG]);
    string.replace(0, 3, replacement);
    assert_eq!(
        string,
        expected,
    );
}


#[test]
fn segment_string_replace_worded() {
    let mut string = PhonologicalString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]).worded();
    let replacement = PhonologicalString::from_segments([I_SEG, A_SEG]);
    let expected = PhonologicalString::from_segments([I_SEG, A_SEG, K_SEG, A_SEG]).worded();
    string.replace(1, 4, replacement);
    assert_eq!(
        string,
        expected,
    );
}
