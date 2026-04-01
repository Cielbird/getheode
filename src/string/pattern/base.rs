
struct PhonoPattern {
    elems : Vec<PhonoPatternElement>,
}

// A pattern for matching phonological strings
enum PhonoPatternElement {
    Boundary,
    SyllableBoundary,
    Element(PhonoPatternSegment)
}

// A segment element of a pattern for phonological strings
struct PhonoPatternSegment {
    syllable_features : [FeatureState; SYL_FEATURE_COUNT as usize],
    segment_features : [FeatureState; SEG_FEATURE_COUNT as usize],
}

