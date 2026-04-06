#![allow(unused)] // TODO remove once this mod is used

use std::{collections::HashMap, process::id};

use crate::{
    phonology::{
        pattern::PatternMatch, segment::SegmentFeatures, string::PhonoString,
        syllable::{self, SyllableFeatures}, tree::UniformDepth3Tree,
    },
    ud3tree,
};

pub type PatternTree = UniformDepth3Tree<(), SyllableInfo, SegmentInfo>;

/// A pattern to match in phonological strings
pub struct PhonoPattern {
    // use a tree to represent the string, like phonological strings
    pub(crate) match_tree: PatternTree,
    pub(crate) replace_tree: PatternTree,
}

#[derive(Debug)]
pub struct SyllableInfo {
    pub(crate) id: u32,
    pub(crate) features: SyllableFeatures,
}

impl SyllableInfo {
    pub fn new(id: u32, features: SyllableFeatures) -> Self {
        Self { id, features }
    }
}

#[derive(Debug)]
pub struct SegmentInfo {
    pub(crate) id: u32,
    pub(crate) features: SegmentFeatures,
}

impl SegmentInfo {
    pub fn new(id: u32, features: SegmentFeatures) -> Self {
        Self { id, features }
    }
}

impl PhonoPattern {
    pub(crate) fn new(match_tree: PatternTree, replace_tree: PatternTree) -> Self {
        Self {
            match_tree,
            replace_tree,
        }
    }

    pub(crate) fn find(&self, hay: PhonoString) -> Vec<PatternMatch> {
        let mut matches_vec = vec![];

        let hay_seg_n = hay.tree.layer_2.len();
        let match_seg_n = self.match_tree.layer_2.len();
        let match_syl_n = self.match_tree.layer_1.len();
        let match_word_n = self.match_tree.layer_0.len();
        for seg_offset in 0..(hay_seg_n - match_seg_n + 1) {
            println!("offset: {seg_offset}");
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
            let mut words_match = true;
            for word_idx in 0..match_word_n {
                let match_word = &self.match_tree.layer_1[word_idx];
                let hay_word = &hay.tree.layer_1[word_offset + word_idx];

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
                let id = syl_info.id;
                let (syl, _) = &hay.tree.layer_1[syl_offset + idx];
                syl_captures.insert(id, syl.clone());
            }
            for (idx, (seg_info, _)) in self.match_tree.layer_2.iter().enumerate() {
                let id = seg_info.id;
                let (seg, _) = &hay.tree.layer_2[seg_offset + idx];
                seg_captures.insert(id, seg.clone());
            }

            // build replacement
            let mut replace_with = PhonoString { tree: ud3tree![] };
            for word in &self.replace_tree.layer_0 {
                replace_with.tree.layer_0.push(*word);
            }
            for (syllable_info, parent_idx) in &self.replace_tree.layer_1 {
                let id = syllable_info.id;
                let syllable = syl_captures.get(&id).unwrap(); // TODO manage error
                let new_syllable = syllable.clone() + syllable_info.features.clone();
                replace_with.tree.layer_1.push((new_syllable, *parent_idx));
            }
            for (segment_info, parent_idx) in &self.replace_tree.layer_2 {
                let id = segment_info.id;
                let segment = seg_captures.get(&id).unwrap(); // TODO manage error
                let new_segment = segment.clone() + segment_info.features.clone();
                replace_with.tree.layer_2.push((new_segment, *parent_idx));
            }

            matches_vec.push(PatternMatch {
                range: seg_offset..(seg_offset + match_seg_n),
                replace_with,
            });
        }

        matches_vec
    }

    // TODO function to verify invariants of pattern : ids should be unique and have corresponding 
    // ids in replace_with
}
