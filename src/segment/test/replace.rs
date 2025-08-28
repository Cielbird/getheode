use crate::segment::{Segment, SegmentString};
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
    let mut string = SegmentString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]);
    let replacement = SegmentString::from_segments([I_SEG, A_SEG]);
    let expected = SegmentString::from_segments([I_SEG, A_SEG, K_SEG, A_SEG]);
    string.replace(0, 3, replacement);
    assert_eq!(
        string,
        expected,
    );
}


#[test]
fn segment_string_replace_worded() {
    let mut string = SegmentString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]).worded();
    let replacement = SegmentString::from_segments([I_SEG, A_SEG]);
    let expected = SegmentString::from_segments([I_SEG, A_SEG, K_SEG, A_SEG]).worded();
    string.replace(0, 3, replacement);
    assert_eq!(
        string,
        expected,
    );
}

//  left: SegmentString { segs: [Segment { features: [POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, POS, NEG, POS, NEG, POS] }, Segment { features: [POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA] }, Segment { features: [NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, POS, NEG, NA, NA, NA] }, Segment { features: [POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA] }], words: ChunkSequence { len: 4, boundaries: {4}, chunks: [()] }, sylables: ChunkSequence { len: 4, boundaries: {}, chunks: [Sylable { stressed: false }] } }
//  right: SegmentString { segs: [Segment { features: [POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, POS, NEG, POS, NEG, POS] }, Segment { features: [POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA] }, Segment { features: [NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, POS, NEG, NA, NA, NA] }, Segment { features: [POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA] }], words: ChunkSequence { len: 4, boundaries: {0, 4}, chunks: [()] }, sylables: ChunkSequence { len: 4, boundaries: {}, chunks: [Sylable { stressed: false }] } }