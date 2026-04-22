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
    // exact base match — prioritised to avoid e.g. ɤʲ when i is correct
    for (sym, seg) in IPA_BASES {
        if seg == segment {
            return sym.to_string();
        }
    }
    // base + diacritics (recursive, up to MAX_DIACRITICS deep)
    for (sym, seg) in IPA_BASES {
        if let Some(diacritics) = match_diacritics(seg.clone(), segment, MAX_DIACRITICS) {
            return format!("{}{}", sym, diacritics);
        }
    }
    // natural class exact match
    for (sym, seg) in NATURAL_CLASSES {
        if seg == segment {
            return sym.to_string();
        }
    }
    // ipa base + minimal feature list: pick the base that minimises appended features
    if let Some((sym, base)) = IPA_BASES
        .iter()
        .min_by_key(|(_, seg)| SegmentFeatures::diff_count(seg, segment))
    {
        return format!("{}[{}]", sym, diff_feature_list(base, segment));
    }

    // natural class + minimal feature list: pick the base that minimises appended features
    if let Some((sym, base)) = NATURAL_CLASSES
        .iter()
        .min_by_key(|(_, seg)| SegmentFeatures::diff_count(seg, segment))
    {
        return format!("{}[{}]", sym, diff_feature_list(base, segment));
    }

    // bare feature list (unreachable while IPA_BASES and NATURAL_CLASSES is non-empty)
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

// Feature list string for features where `target` is POS/NEG and differs from `base`.
fn diff_feature_list(base: &SegmentFeatures, target: &SegmentFeatures) -> String {
    let mut result = String::new();
    for (i, (b, t)) in base.features.iter().zip(target.features.iter()).enumerate() {
        if t == b {
            continue;
        }
        match t {
            FeatureState::POS => {
                result.push('+');
                result.push_str(SEG_FEATURE_NAMES[i]);
            }
            FeatureState::NEG => {
                result.push('-');
                result.push_str(SEG_FEATURE_NAMES[i]);
            }
            _ => {}
        }
    }
    result
}
