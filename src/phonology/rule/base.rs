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

    pub fn words(&self) -> &[()] {
        self.0.layer_0()
    }

    pub fn syls(&self) -> &[(SyllableInfo, usize)] {
        self.0.layer_1()
    }

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
        let hay_seg_n = hay.tree.len_2();
        let hay_syl_n = hay.tree.len_1();
        let hay_word_n = hay.tree.len_0();
        let match_seg_n = self.pattern.tree.segs().len();
        let match_syl_n = self.pattern.tree.syls().len();
        let match_word_n = self.pattern.tree.words().len();

        if hay_seg_n < match_seg_n || hay_syl_n < match_syl_n || hay_word_n < match_word_n {
            return vec![];
        }

        (0..=(hay_seg_n - match_seg_n))
            .filter_map(|seg_offset| self.match_at(&hay, seg_offset))
            .collect()
    }

    fn match_at(&self, hay: &PhonoString, seg_offset: usize) -> Option<PatternMatch> {
        let hay_segs = hay.tree.layer_2();
        let hay_syls = hay.tree.layer_1();
        let pat_segs = self.pattern.tree.segs();
        let pat_syls = self.pattern.tree.syls();

        // syl_offset is the absolute index of the first matched syllable in the hay.
        // the pattern stores relative parent indices (0, 1, 2...), so we subtract
        // syl_offset when comparing to normalize hay indices to the same origin.
        let syl_offset = hay_segs[seg_offset].1;
        for (idx, (pat_seg, pat_syl_idx)) in pat_segs.iter().enumerate() {
            let (hay_seg, hay_syl_idx) = &hay_segs[seg_offset + idx];
            if *pat_syl_idx != hay_syl_idx - syl_offset {
                return None;
            }
            if !hay_seg.matches(&pat_seg.features) {
                return None;
            }
        }

        // same normalization for word indices relative to word_offset
        let word_offset = hay_syls[syl_offset].1;
        for (idx, (pat_syl, pat_word_idx)) in pat_syls.iter().enumerate() {
            let (hay_syl, hay_word_idx) = &hay_syls[syl_offset + idx];
            if *pat_word_idx != hay_word_idx - word_offset {
                return None;
            }
            if !hay_syl.matches(&pat_syl.features) {
                return None;
            }
        }

        let hay_seg_n = hay_segs.len();
        let hay_syl_n = hay_syls.len();
        let match_seg_n = pat_segs.len();
        let match_syl_n = pat_syls.len();

        // a boundary exists when adjacent elements have different parents, or
        // the match starts/ends at the edge of the entire string.
        let left_syl_border = seg_offset == 0 || hay_segs[seg_offset - 1].1 != syl_offset;
        let left_word_border = syl_offset == 0 || hay_syls[syl_offset - 1].1 != word_offset;
        if !self
            .pattern
            .left_bound
            .respects(left_syl_border, left_word_border)
        {
            return None;
        }

        let last_seg = seg_offset + match_seg_n - 1;
        let right_syl_border =
            last_seg + 1 == hay_seg_n || hay_segs[last_seg].1 != hay_segs[last_seg + 1].1;
        let last_syl = syl_offset + match_syl_n - 1;
        let right_word_border =
            last_syl + 1 == hay_syl_n || hay_syls[last_syl].1 != hay_syls[last_syl + 1].1;
        if !self
            .pattern
            .right_bound
            .respects(right_syl_border, right_word_border)
        {
            return None;
        }

        let (syl_captures, seg_captures) =
            self.build_captures(hay_segs, hay_syls, seg_offset, syl_offset)?;

        Some(PatternMatch {
            range: seg_offset..(seg_offset + match_seg_n),
            replace_with: self.build_replacement(&syl_captures, &seg_captures),
        })
    }

    /// collect the hay features referenced by each tag in the pattern.
    /// returns None if two pattern nodes share a tag but map to different hay features
    /// (the twin-tag constraint: V_0...V_0 requires both vowels to be identical).
    fn build_captures(
        &self,
        hay_segs: &[(SegmentFeatures, usize)],
        hay_syls: &[(SyllableFeatures, usize)],
        seg_offset: usize,
        syl_offset: usize,
    ) -> Option<(
        HashMap<u32, SyllableFeatures>,
        HashMap<u32, SegmentFeatures>,
    )> {
        let mut syl_captures: HashMap<u32, SyllableFeatures> = HashMap::new();
        for (idx, (syl_info, _)) in self.pattern.tree.syls().iter().enumerate() {
            if let Some(id) = syl_info.tag {
                let (hay_syl, _) = &hay_syls[syl_offset + idx];
                if let Some(prev) = syl_captures.get(&id) {
                    if prev != hay_syl {
                        return None;
                    }
                } else {
                    syl_captures.insert(id, hay_syl.clone());
                }
            }
        }

        let mut seg_captures: HashMap<u32, SegmentFeatures> = HashMap::new();
        for (idx, (seg_info, _)) in self.pattern.tree.segs().iter().enumerate() {
            if let Some(id) = seg_info.tag {
                let (hay_seg, _) = &hay_segs[seg_offset + idx];
                if let Some(prev) = seg_captures.get(&id) {
                    if prev != hay_seg {
                        return None;
                    }
                } else {
                    seg_captures.insert(id, hay_seg.clone());
                }
            }
        }

        Some((syl_captures, seg_captures))
    }

    fn build_replacement(
        &self,
        syl_captures: &HashMap<u32, SyllableFeatures>,
        seg_captures: &HashMap<u32, SegmentFeatures>,
    ) -> PhonoString {
        let mut tree = d3tree![];
        for (_, syl_iter) in self.replace_tree.0.iter() {
            tree.push_depth_0(());
            for (syl, seg_iter) in syl_iter {
                let mut new_syl = SyllableFeatures::new_undef();
                if let Some(id) = syl.tag {
                    new_syl = new_syl
                        + syl_captures
                            .get(&id)
                            .expect("Invalid rule: syllable capture id not found")
                            .clone();
                }
                // captured features go left, explicit rule features go right so the
                // rule's literal values take precedence over what was captured.
                tree.push_depth_1(new_syl + syl.features.clone());
                for seg in seg_iter {
                    let mut new_seg = SegmentFeatures::new_undef();
                    if let Some(id) = seg.tag {
                        new_seg = new_seg
                            + seg_captures
                                .get(&id)
                                .expect("Invalid rule: segment capture id not found")
                                .clone();
                    }
                    tree.push_depth_2(new_seg + seg.features.clone());
                }
            }
        }
        PhonoString { tree }
    }

    /// returns false if tags in the pattern are not unique per level, or if the
    /// replacement tree references a tag not present in the pattern.
    pub fn test_invariants(&self) -> bool {
        let mut syl_tags = HashSet::new();
        let mut seg_tags = HashSet::new();

        for (_, syllables) in self.pattern.tree.0.iter() {
            for (SyllableInfo { tag, features: _ }, segments) in syllables {
                if let Some(tag) = tag
                    && !syl_tags.insert(tag)
                {
                    return false; // duplicate syl tag in pattern
                }
                for SegmentInfo { tag, features: _ } in segments {
                    if let Some(tag) = tag
                        && !seg_tags.insert(tag)
                    {
                        return false; // duplicate seg tag in pattern
                    }
                }
            }
        }

        // remove each tag referenced by the replacement; if the tag was absent
        // (not in the pattern) remove returns false.
        for (_, syllables) in self.replace_tree.0.iter() {
            for (SyllableInfo { tag, features: _ }, segments) in syllables {
                if let Some(tag) = tag
                    && !syl_tags.remove(tag)
                {
                    return false; // replacement references unknown syl tag
                }
                for SegmentInfo { tag, features: _ } in segments {
                    if let Some(tag) = tag
                        && !seg_tags.remove(tag)
                    {
                        return false; // replacement references unknown seg tag
                    }
                }
            }
        }

        true
    }
}
