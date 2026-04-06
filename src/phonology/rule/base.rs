use std::collections::{HashMap, HashSet};

use crate::{
    phonology::{
        rule::RuleMatch, segment::SegmentFeatures, string::PhonoString, syllable::SyllableFeatures,
        tree::UniformDepth3Tree,
    },
    ud3tree,
};

pub type RuleTree = UniformDepth3Tree<(), SyllableInfo, SegmentInfo>;

/// A pattern to match in phonological strings
pub struct PhonoRule {
    // use a tree to represent the string, like phonological strings
    pub match_tree: RuleTree,
    pub replace_tree: RuleTree,
}

#[derive(Debug)]
pub struct SyllableInfo {
    pub id: Option<u32>,
    pub features: SyllableFeatures,
}

impl SyllableInfo {
    pub fn new(id: Option<u32>, features: SyllableFeatures) -> Self {
        Self { id, features }
    }
}

#[derive(Debug)]
pub struct SegmentInfo {
    pub id: Option<u32>,
    pub features: SegmentFeatures,
}

impl SegmentInfo {
    pub fn new(id: Option<u32>, features: SegmentFeatures) -> Self {
        Self { id, features }
    }
}

impl PhonoRule {
    pub fn new(match_tree: RuleTree, replace_tree: RuleTree) -> Self {
        Self {
            match_tree,
            replace_tree,
        }
    }

    pub fn find(&self, hay: PhonoString) -> Vec<RuleMatch> {
        let mut matches_vec = vec![];

        let hay_seg_n = hay.tree.layer_2.len();
        let match_seg_n = self.match_tree.layer_2.len();
        let match_syl_n = self.match_tree.layer_1.len();
        let match_word_n = self.match_tree.layer_0.len();
        for seg_offset in 0..(hay_seg_n - match_seg_n + 1) {
            // iterate on segments
            let mut segments_match = true;
            let syl_offset = hay.tree.layer_2[seg_offset].1;
            for seg_idx in 0..match_seg_n {
                let (match_seg, syl_idx) = &self.match_tree.layer_2[seg_idx];
                let (hay_seg, hay_syl_idx) = &hay.tree.layer_2[seg_offset + seg_idx];

                if *syl_idx != hay_syl_idx - syl_offset {
                    segments_match = false; // segment's parent isn't the same
                    break;
                }

                if !hay_seg.matches(&match_seg.features) {
                    segments_match = false;
                    break;
                }
            }
            if !segments_match {
                continue;
            }

            // iterate on syllables
            let mut syllables_match = true;
            let word_offset = hay.tree.layer_1[syl_offset].1;
            for syl_idx in 0..match_syl_n {
                let (match_syl, word_idx) = &self.match_tree.layer_1[syl_idx];
                let (hay_syl, hay_word_idx) = &hay.tree.layer_1[syl_offset + syl_idx];

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
                let _match_word = &self.match_tree.layer_1[word_idx];
                let _hay_word = &hay.tree.layer_1[word_offset + word_idx];

                // This will be implemented if words have features
                // if !match_word.features.matches(hay_word) {
                //     words_match = false;
                // }
            }
            if !words_match {
                continue;
            }

            // Pattern matches from here !

            // Record captured features in hash maps :
            // let word_captures = HashMap::<u32, ()>::new()
            let mut syl_captures = HashMap::<u32, SyllableFeatures>::new();
            let mut seg_captures = HashMap::<u32, SegmentFeatures>::new();
            // for x in self.match_tree.layer_0 {}
            for (idx, (syl_info, _)) in self.match_tree.layer_1.iter().enumerate() {
                if let Some(id) = syl_info.id {
                    let (syl, _) = &hay.tree.layer_1[syl_offset + idx];
                    syl_captures.insert(id, syl.clone());
                }
            }
            for (idx, (seg_info, _)) in self.match_tree.layer_2.iter().enumerate() {
                if let Some(id) = seg_info.id {
                    let (seg, _) = &hay.tree.layer_2[seg_offset + idx];
                    seg_captures.insert(id, seg.clone());
                }
            }

            // build replacement
            let mut replace_with = PhonoString { tree: ud3tree![] };
            for _word in &self.replace_tree.layer_0 {
                replace_with.tree.layer_0.push(());
            }
            for (syllable_info, parent_idx) in &self.replace_tree.layer_1 {
                let mut new_syllable = SyllableFeatures::new_undef();
                if let Some(id) = syllable_info.id {
                    let syllable = syl_captures
                        .get(&id)
                        .expect("Invalid rule : syllable capture id not found");
                    new_syllable = new_syllable + syllable.clone();
                }
                new_syllable = new_syllable + syllable_info.features.clone();
                replace_with.tree.layer_1.push((new_syllable, *parent_idx));
            }

            for (segment_info, parent_idx) in &self.replace_tree.layer_2 {
                let mut new_segment = SegmentFeatures::new_undef();
                if let Some(id) = segment_info.id {
                    let segment = seg_captures
                        .get(&id)
                        .expect("Invalid rule : segment capture id not found");
                    new_segment = new_segment.clone() + segment.clone();
                }
                new_segment = new_segment.clone() + segment_info.features.clone();
                replace_with.tree.layer_2.push((new_segment, *parent_idx));
            }

            matches_vec.push(RuleMatch {
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
        for (info, _) in &self.match_tree.layer_1 {
            if !syl_captures.insert(info.id) {
                // syllable capture id already exists !
                return false;
            }
        }
        for (info, _) in &self.match_tree.layer_2 {
            if !seg_captures.insert(info.id) {
                // segment capture id already exists !
                return false;
            }
        }

        // verify replace_tree
        for (info, _) in &self.replace_tree.layer_1 {
            if !syl_captures.remove(&info.id) {
                // syllable capture id not defined in match tree !
                return false;
            }
        }
        for (info, _) in &self.replace_tree.layer_2 {
            if !seg_captures.remove(&info.id) {
                // syllable capture id not defined in match tree !
                return false;
            }
        }

        true
    }
}
