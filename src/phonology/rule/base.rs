use std::collections::{HashMap, HashSet};

use crate::{
    d3tree,
    phonology::{
        rule::{PatternMatch, PhonoStringPattern},
        segment::SegmentFeatures,
        string::PhonoString,
        syllable::SyllableFeatures,
        tree::Depth3Tree,
    },
};

/// A phonological string, where syllable or segment nodes may be tagged
pub struct TaggedPhonoString(Depth3Tree<(), SyllableInfo, SegmentInfo>);
impl TaggedPhonoString {
    pub(crate) fn new(tree: Depth3Tree<(), SyllableInfo, SegmentInfo>) -> Self {
        Self(tree)
    }

    // get slice of words
    pub fn words(&self) -> &[()] {
        &self.0.layer_0
    }

    // iterate on words
    pub fn syls(&self) -> &[(SyllableInfo, usize)] {
        &self.0.layer_1
    }

    // iterate on words
    pub fn segs(&self) -> &[(SegmentInfo, usize)] {
        &self.0.layer_2
    }
}

/// A pattern to match in phonological strings
pub struct PhonoRule {
    // use a tree to represent the string, like phonological strings
    pub pattern: PhonoStringPattern,
    pub replace_tree: TaggedPhonoString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyllableInfo {
    pub tag: Option<u32>,
    pub features: SyllableFeatures,
}

impl SyllableInfo {
    pub fn new(id: Option<u32>, features: SyllableFeatures) -> Self {
        Self { tag: id, features }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentInfo {
    pub tag: Option<u32>,
    pub features: SegmentFeatures,
}

impl SegmentInfo {
    pub fn new(id: Option<u32>, features: SegmentFeatures) -> Self {
        Self { tag: id, features }
    }
}

impl PhonoRule {
    pub fn new(pattern: PhonoStringPattern, replace_tree: TaggedPhonoString) -> Self {
        Self {
            pattern,
            replace_tree,
        }
    }

    pub fn find(&self, hay: PhonoString) -> Vec<PatternMatch> {
        let mut matches_vec = vec![];

        let hay_syllables = hay.tree.layer_1;
        let hay_segments = hay.tree.layer_2;
        let pattern_words = self.pattern.tree.words();
        let pattern_syllables = self.pattern.tree.syls();
        let pattern_segments = self.pattern.tree.segs();

        let hay_seg_n = hay_segments.len();
        let hay_syl_n = hay_syllables.len();
        let match_seg_n = pattern_segments.len();
        let match_syl_n = pattern_syllables.len();
        let match_word_n = pattern_words.len();
        for seg_offset in 0..(hay_seg_n - match_seg_n + 1) {
            // iterate on segments
            let mut segments_match = true;
            let syl_offset = hay_segments[seg_offset].1;
            for (seg_idx, (pattern_seg, syl_idx)) in pattern_segments.iter().enumerate() {
                let (hay_seg, hay_syl_idx) = &hay_segments[seg_offset + seg_idx];

                if *syl_idx != hay_syl_idx - syl_offset {
                    segments_match = false; // segment's parent isn't the same
                    break;
                }

                if !hay_seg.matches(&pattern_seg.features) {
                    segments_match = false;
                    break;
                }
            }
            if !segments_match {
                continue;
            }

            // iterate on syllables
            let mut syllables_match = true;
            let word_offset = hay_syllables[syl_offset].1;
            for syl_idx in 0..match_syl_n {
                let (match_syl, word_idx) = &pattern_syllables[syl_idx];
                let (hay_syl, hay_word_idx) = &hay_syllables[syl_offset + syl_idx];

                if *word_idx != hay_word_idx - word_offset {
                    syllables_match = false; // segment's parent isn't the same
                    break;
                }

                if !hay_syl.matches(&match_syl.features) {
                    syllables_match = false;
                }
            }
            if !syllables_match {
                continue;
            }

            // iterate on words
            let words_match = true;
            for word_idx in 0..match_word_n {
                let _match_word = &pattern_syllables[word_idx];
                let _hay_word = &hay_syllables[word_offset + word_idx];

                // This will be implemented if words have features
                // if !match_word.features.matches(hay_word) {
                //     words_match = false;
                // }
            }
            if !words_match {
                continue;
            }

            // check borders
            let left_segment_on_border = if seg_offset == 0 {
                // syllable border if it's the first segment
                true
            } else {
                // syllable border if segments have different parents
                hay_segments[seg_offset - 1].1 != hay_segments[seg_offset].1
            };
            let left_syllable_on_border = if syl_offset == 0 {
                // syllable border if it's the first segment
                true
            } else {
                // syllable border if segments have different parents
                hay_syllables[syl_offset - 1].1 != hay_syllables[syl_offset].1
            };
            let right_segment_on_border = if seg_offset + match_seg_n == hay_seg_n {
                // syllable border if it's the first segment
                true
            } else {
                // syllable border if segments have different parents
                hay_segments[seg_offset + 1].1 != hay_segments[seg_offset].1
            };
            let right_syllable_on_border = if syl_offset + match_syl_n == hay_syl_n {
                // syllable border if it's the first segment
                true
            } else {
                // syllable border if segments have different parents
                hay_syllables[syl_offset + 1].1 != hay_syllables[syl_offset].1
            };

            let left_bound_respected = self
                .pattern
                .left_bound
                .respects(left_segment_on_border, left_syllable_on_border);
            if !left_bound_respected {
                println!(
                    "left bound not respected: {seg_offset}, {syl_offset}, {right_segment_on_border}{right_syllable_on_border}"
                );
                continue;
            }

            let right_bound_respected = self
                .pattern
                .right_bound
                .respects(right_segment_on_border, right_syllable_on_border);

            if !right_bound_respected {
                println!(
                    "right bound not respected: {seg_offset}, {syl_offset}, {right_segment_on_border}{right_syllable_on_border}"
                );
                continue;
            }

            // Pattern matches from here !

            // Record captured features in hash maps :
            // let word_captures = HashMap::<u32, ()>::new()
            let mut syl_captures = HashMap::<u32, SyllableFeatures>::new();
            let mut seg_captures = HashMap::<u32, SegmentFeatures>::new();
            // for x in self.match_tree.layer_0 {}
            for (idx, (syl_info, _)) in pattern_syllables.iter().enumerate() {
                if let Some(id) = syl_info.tag {
                    let (syl, _) = &hay_syllables[syl_offset + idx];
                    syl_captures.insert(id, syl.clone());
                }
            }
            for (idx, (seg_info, _)) in pattern_segments.iter().enumerate() {
                if let Some(id) = seg_info.tag {
                    let (seg, _) = &hay_segments[seg_offset + idx];
                    seg_captures.insert(id, seg.clone());
                }
            }

            // build replacement
            let mut replace_with = PhonoString { tree: d3tree![] };
            for _word in &self.replace_tree.0.layer_0 {
                replace_with.tree.layer_0.push(());
            }
            for (syllable_info, parent_idx) in &self.replace_tree.0.layer_1 {
                let mut new_syllable = SyllableFeatures::new_undef();
                if let Some(id) = syllable_info.tag {
                    let syllable = syl_captures
                        .get(&id)
                        .expect("Invalid rule : syllable capture id not found");
                    new_syllable = new_syllable + syllable.clone();
                }
                new_syllable = new_syllable + syllable_info.features.clone();
                replace_with.tree.layer_1.push((new_syllable, *parent_idx));
            }

            for (segment_info, parent_idx) in &self.replace_tree.0.layer_2 {
                let mut new_segment = SegmentFeatures::new_undef();
                if let Some(id) = segment_info.tag {
                    let segment = seg_captures
                        .get(&id)
                        .expect("Invalid rule : segment capture id not found");
                    new_segment = new_segment.clone() + segment.clone();
                }
                new_segment = new_segment.clone() + segment_info.features.clone();
                replace_with.tree.layer_2.push((new_segment, *parent_idx));
            }

            matches_vec.push(PatternMatch {
                range: seg_offset..(seg_offset + match_seg_n),
                replace_with,
            });
        }

        matches_vec
    }

    /// Check if invariants are respected.
    /// Returns false if the capture ids are not unique (on the same hierarchical level)
    /// or if the ids in the replacement tree aren't found in the match tree
    pub fn test_invariants(&self) -> bool {
        let mut syl_captures = HashSet::new();
        let mut seg_captures = HashSet::new();
        // verify match tree
        for (info, _) in &self.pattern.tree.0.layer_1 {
            if !syl_captures.insert(info.tag) {
                // syllable capture id already exists !
                return false;
            }
        }
        for (info, _) in &self.pattern.tree.0.layer_2 {
            if !seg_captures.insert(info.tag) {
                // segment capture id already exists !
                return false;
            }
        }

        // verify replace_tree
        for (info, _) in &self.replace_tree.0.layer_1 {
            if !syl_captures.remove(&info.tag) {
                // syllable capture id not defined in match tree !
                return false;
            }
        }
        for (info, _) in &self.replace_tree.0.layer_2 {
            if !seg_captures.remove(&info.tag) {
                // syllable capture id not defined in match tree !
                return false;
            }
        }

        true
    }
}
