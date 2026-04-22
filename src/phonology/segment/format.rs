use crate::phonology::{
    feature::FeatureState,
    segment::{DIACRITICS, IPA_BASES, NATURAL_CLASSES, SEG_FEATURE_NAMES, SegmentFeatures},
};

// Maximum number of diacritics to stack when searching for an IPA representation.
// ɤ+ʲ has the same features as i; exact base matches are tried first to avoid ɤʲ for i.
const MAX_DIACRITICS: usize = 1;

// Returns the diacritic string to append to a base to reach `target`, or None.
fn match_diacritics(
    seg: SegmentFeatures,
    target: &SegmentFeatures,
    depth: usize,
) -> Option<String> {
    if depth == 0 {
        return None;
    }
    for (d, d_seg) in DIACRITICS {
        let combined = seg.clone() + d_seg.clone();
        if &combined == target {
            return Some(d.to_string());
        }
        if let Some(suffix) = match_diacritics(combined, target, depth - 1) {
            return Some(format!("{}{}", d, suffix));
        }
    }
    None
}

pub fn format_segment(segment: &SegmentFeatures) -> String {
    // Pass 1: exact base match — prioritised to avoid e.g. ɤʲ when i is correct
    for (sym, seg) in IPA_BASES {
        if seg == segment {
            return sym.to_string();
        }
    }
    // Pass 2: base + diacritics (recursive, up to MAX_DIACRITICS deep)
    for (sym, seg) in IPA_BASES {
        if let Some(diacritics) = match_diacritics(seg.clone(), segment, MAX_DIACRITICS) {
            return format!("{}{}", sym, diacritics);
        }
    }
    // natural class fallback
    for (sym, seg) in NATURAL_CLASSES {
        if seg == segment {
            return sym.to_string();
        }
    }
    // feature list fallback
    let mut result = "[".to_string();
    for (i, feature) in SEG_FEATURE_NAMES.iter().enumerate() {
        if segment.features[i] == FeatureState::NA {
            continue;
        } else if segment.features[i] == FeatureState::POS {
            result = result + "+" + feature;
        } else if segment.features[i] == FeatureState::NEG {
            result = result + "-" + feature;
        }
    }

    result + "]"
}

pub fn format_segment_ipa(_seg: &SegmentFeatures) -> String {
    todo!()
}
