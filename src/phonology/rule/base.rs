use std::collections::{HashMap, HashSet};

use crate::{
    d3tree,
    phonology::{
        rule::{PatternMatch, PhonoStringPattern},
        segment::SegmentFeatures,
        string::PhonoString,
        syllable::SyllableFeatures,
        tree::{Depth3Tree, iter::IterDepth0},
    },
};

/// A phonological string, where syllable or segment nodes may be tagged
#[derive(Debug)]
pub struct TaggedPhonoString(Depth3Tree<(), SyllableInfo, SegmentInfo>);
impl TaggedPhonoString {
    pub(crate) fn new(tree: Depth3Tree<(), SyllableInfo, SegmentInfo>) -> Self {
        Self(tree)
    }

    // get slice of words
    pub fn words(&self) -> &[()] {
        self.0.layer_0()
    }

    // iterate on words
    pub fn syls(&self) -> &[(SyllableInfo, usize)] {
        self.0.layer_1()
    }

    // iterate on words
    pub fn segs(&self) -> &[(SegmentInfo, usize)] {
        self.0.layer_2()
    }

    pub fn iter<'a>(&'a self) -> IterDepth0<'a, (), SyllableInfo, SegmentInfo> {
        self.0.iter()
    }

    pub fn pretty_format(&self) -> String {
        self.0.pretty_format()
    }
}

/// A pattern to match in phonological strings
#[derive(Debug)]
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
    pub const fn new(id: Option<u32>, features: SyllableFeatures) -> Self {
        Self { tag: id, features }
    }

    pub const fn new_tagged(id: u32, features: SyllableFeatures) -> Self {
        Self {
            tag: Some(id),
            features,
        }
    }

    pub const fn new_untagged(features: SyllableFeatures) -> Self {
        Self {
            tag: None,
            features,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentInfo {
    pub tag: Option<u32>,
    pub features: SegmentFeatures,
}

impl SegmentInfo {
    pub const fn new(id: Option<u32>, features: SegmentFeatures) -> Self {
        Self { tag: id, features }
    }
    pub const fn new_tagged(id: u32, features: SegmentFeatures) -> Self {
        Self {
            tag: Some(id),
            features,
        }
    }
    pub const fn new_untagged(features: SegmentFeatures) -> Self {
        Self {
            tag: None,
            features,
        }
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

        let hay_words = hay.tree.layer_0();
        let hay_syllables = hay.tree.layer_1();
        let hay_segments = hay.tree.layer_2();
        let pattern_words = self.pattern.tree.words();
        let pattern_syllables = self.pattern.tree.syls();
        let pattern_segments = self.pattern.tree.segs();

        let hay_seg_n = hay_segments.len();
        let hay_syl_n = hay_syllables.len();
        let hay_word_n = hay_words.len();
        let match_seg_n = pattern_segments.len();
        let match_syl_n = pattern_syllables.len();
        let match_word_n = pattern_words.len();
        if hay_seg_n < match_seg_n || hay_syl_n < match_syl_n || hay_word_n < match_word_n {
            return vec![];
        }
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
            let left_side_syl_border = if seg_offset == 0 {
                // syllable border if it's the first segment
                true
            } else {
                // syllable border if segments have different parents
                hay_segments[seg_offset - 1].1 != hay_segments[seg_offset].1
            };
            let left_side_word_border = if syl_offset == 0 {
                // word border if it's the first segment
                true
            } else {
                // word border if segments have different parents
                hay_syllables[syl_offset - 1].1 != hay_syllables[syl_offset].1
            };

            let last_seg_idx = seg_offset + match_seg_n - 1;
            let right_side_syl_border = if last_seg_idx + 1 == hay_seg_n {
                // syllable border if it's the last syllable
                true
            } else {
                // syllable border if syllables have different parents
                hay_segments[last_seg_idx].1 != hay_segments[last_seg_idx + 1].1
            };

            let last_syl_idx = syl_offset + match_syl_n - 1;
            let right_side_word_border = if last_syl_idx + 1 == hay_syl_n {
                // word border if it's the last syllable
                true
            } else {
                // word border if syllables have different parents
                hay_syllables[last_syl_idx].1 != hay_syllables[last_syl_idx + 1].1
            };

            let left_bound_respected = self
                .pattern
                .left_bound
                .respects(left_side_syl_border, left_side_word_border);
            if !left_bound_respected {
                println!(
                    "left bound not respected: {seg_offset}, {syl_offset}, {right_side_syl_border}{right_side_word_border}"
                );
                continue;
            }

            let right_bound_respected = self
                .pattern
                .right_bound
                .respects(right_side_syl_border, right_side_word_border);

            if !right_bound_respected {
                println!(
                    "right bound not respected: {seg_offset}, {syl_offset}, {right_side_syl_border}{right_side_word_border}"
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
            let mut replace_with = d3tree![];
            for (_, syl_iter) in self.replace_tree.0.iter() {
                replace_with.push_depth_0(());

                for (syl, seg_iter) in syl_iter {
                    let mut new_syllable = SyllableFeatures::new_undef();
                    if let Some(id) = syl.tag {
                        let syllable = syl_captures
                            .get(&id)
                            .expect("Invalid rule : syllable capture id not found");
                        new_syllable = new_syllable + syllable.clone();
                    }
                    new_syllable = new_syllable + syl.features.clone();

                    replace_with.push_depth_1(new_syllable);

                    for seg in seg_iter {
                        let mut new_segment = SegmentFeatures::new_undef();
                        if let Some(id) = seg.tag {
                            let segment = seg_captures
                                .get(&id)
                                .expect("Invalid rule : segment capture id not found");
                            new_segment = new_segment.clone() + segment.clone();
                        }
                        new_segment = new_segment.clone() + seg.features.clone();

                        replace_with.push_depth_2(new_segment);
                    }
                }
            }

            let replace_with = PhonoString { tree: replace_with };

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
        // gather tags of match tree
        for (_, syllables) in self.pattern.tree.0.iter() {
            for (SyllableInfo { tag, features: _ }, segments) in syllables {
                if let Some(tag) = tag
                    && !syl_captures.insert(tag)
                {
                    // syllable capture id already exists !
                    return false;
                }

                for SegmentInfo { tag, features: _ } in segments {
                    if let Some(tag) = tag
                        && !seg_captures.insert(tag)
                    {
                        // segment capture id already exists !
                        return false;
                    }
                }
            }
        }

        // verify tags of replacement tree
        for (_, syllables) in self.replace_tree.0.iter() {
            for (SyllableInfo { tag, features: _ }, segments) in syllables {
                if let Some(tag) = tag
                    && !syl_captures.remove(tag)
                {
                    // syllable capture id not defined in match tree !
                    return false;
                }

                for SegmentInfo { tag, features: _ } in segments {
                    if let Some(tag) = tag
                        && !seg_captures.remove(tag)
                    {
                        // syllable capture id not defined in match tree !
                        return false;
                    }
                }
            }
        }

        true
    }
}
