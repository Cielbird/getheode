use crate::phonology::{
    feature::FeatureState,
    segment::{DIACRITICS, IPA_BASES, NATURAL_CLASSES, SEG_FEATURE_NAMES, SegmentFeatures},
};

// Maximum number of diacritics to stack when searching for an IPA representation.
// ɤ+ʲ has the same features as i; exact base matches are tried first to avoid ɤʲ for i.
const MAX_DIACRITICS: usize = 1;

pub fn format_segment(segment: &SegmentFeatures) -> String {
    format_ipa_exact(segment)
        .or_else(|| format_ipa_diacritics(segment))
        .or_else(|| format_natural_class_exact(segment))
        .or_else(|| format_ipa_feature_list(segment))
        .or_else(|| format_natural_class_feature_list(segment))
        .unwrap_or_else(|| format_bare_feature_list(segment))
}

fn format_ipa_exact(segment: &SegmentFeatures) -> Option<String> {
    IPA_BASES
        .iter()
        .find(|(_, seg)| seg == segment)
        .map(|(sym, _)| sym.to_string())
}

fn format_ipa_diacritics(segment: &SegmentFeatures) -> Option<String> {
    IPA_BASES.iter().find_map(|(sym, seg)| {
        match_diacritics(seg.clone(), segment, MAX_DIACRITICS)
            .map(|diacritics| format!("{}{}", sym, diacritics))
    })
}

fn format_natural_class_exact(segment: &SegmentFeatures) -> Option<String> {
    NATURAL_CLASSES
        .iter()
        .find(|(_, seg)| seg == segment)
        .map(|(sym, _)| sym.to_string())
}

fn format_ipa_feature_list(segment: &SegmentFeatures) -> Option<String> {
    IPA_BASES
        .iter()
        .min_by_key(|(_, seg)| SegmentFeatures::diff_count(seg, segment))
        .map(|(sym, base)| format!("{}[{}]", sym, diff_feature_list(base, segment)))
}

fn format_natural_class_feature_list(segment: &SegmentFeatures) -> Option<String> {
    NATURAL_CLASSES
        .iter()
        .min_by_key(|(_, seg)| SegmentFeatures::diff_count(seg, segment))
        .map(|(sym, base)| format!("{}[{}]", sym, diff_feature_list(base, segment)))
}

fn format_bare_feature_list(segment: &SegmentFeatures) -> String {
    let mut result = "[".to_string();
    for (i, feature) in SEG_FEATURE_NAMES.iter().enumerate() {
        match segment.features[i] {
            FeatureState::POS => result = result + "+" + feature,
            FeatureState::NEG => result = result + "-" + feature,
            _ => {}
        }
    }
    result + "]"
}

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
