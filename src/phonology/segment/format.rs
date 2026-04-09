use crate::phonology::{feature::FeatureState, segment::{DIACRITICS, IPA_BASES, NATURAL_CLASSES, SEG_FEATURE_NAMES, SegmentFeatures}};


pub fn format_segment(segment: &SegmentFeatures) -> String {
    // see if there is a matching ipa symbol
    for (sym, seg) in IPA_BASES {
        if seg == segment {
            return sym.to_string();
        }
        // WARNING this tries all possible ipa symbols with all possible diacritics.
        // not only is it limited to only one diacritic, but it is extremely slow, in theory.
        // for now, there are only a handfull of diacritics. the algorithm to do this well and
        // fast is too much for me to think of right now; a fun puzzle for later.
        // TODO tackle this when performance becomes important, or when i need multiple
        // diacritics
        // TODO this can be done recursively
        for (d, d_seg) in DIACRITICS {
            // TODO figure out if cloning these is really what i'm supposed to do
            if (seg.clone() + d_seg.clone()) == *segment {
                let mut s = sym.to_string();
                s.push(*d);
                return s.to_string();
            }
        }
    }

    // see if there is a matching class
    for (sym, seg) in NATURAL_CLASSES {
        if seg == segment {
            return sym.to_string();
        }
    }

    // otherwise spit out a list of the features
    let mut result: String = "[".to_string();
    for (i, feature) in SEG_FEATURE_NAMES.iter().enumerate() {
        if segment.features[i] == FeatureState::NA {
            continue;
        } else if segment.features[i] == FeatureState::POS {
            result = result + "+" + feature;
        } else if segment.features[i] == FeatureState::NEG {
            result = result + "-" + feature;
        }
    }

    (result + "]").to_string()
}

pub fn format_segment_ipa(_seg: &SegmentFeatures) -> String {
    todo!()
}

