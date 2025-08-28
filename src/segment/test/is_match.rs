use crate::segment::{FormatSegmentString, Segment, SegmentString};
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
fn segment_string_is_match() {
    let pattern = SegmentString::from_segments([K_SEG, A_SEG]);
    let haystack = SegmentString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]);
    let is_match = haystack.is_match(&pattern, 0) && haystack.is_match(&pattern, 3);
    assert_eq!(
        is_match,
        true,
    );
}
