use std::collections::HashSet;

use crate::phonology::rule::{
    SegmentInfo, SyllableInfo,
    parse::{parse_elem::parse_rule_elems, pattern::RuleStrings},
};

/// a boundary or a feature set for a segment
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Features(SyllableInfo, SegmentInfo),
    WordBoundary,
    SyllableBoundary,
    Null, // TODO remove this ?
}

pub struct ElementSequence {
    pub elems: Vec<Element>,
}

/// a rule, no branching: input, output and context. unparsed elements.
pub struct RuleElements {
    pub(crate) input: ElementSequence,
    pub(crate) output: ElementSequence,
    pub(crate) pre_context: ElementSequence,
    pub(crate) post_context: ElementSequence,
}

impl RuleElements {
    /// Apply the element parsing algo to each possible input, output and context.
    pub fn parse_elements(strings: RuleStrings) -> Result<Self, ()> {
        // manage the parsing error and remainder
        fn parse(input: String) -> Result<ElementSequence, ()> {
            let (rem, elems) = parse_rule_elems(&input).map_err(|x| ())?;
            if rem != "" {
                return Err(());
            }
            Ok(elems)
        }

        let mut rule = Self {
            input: parse(strings.input)?,
            output: parse(strings.output)?,
            pre_context: parse(strings.pre_context)?,
            post_context: parse(strings.post_context)?,
        };

        // Tag inputs and outputs
        if !rule.tag_all() {
            // tagging failed
            return Err(());
        }

        Ok(rule)
    }

    // TODO Write tests for this
    pub fn tag_all(&mut self) -> bool {
        let mut syl_tags = vec![];
        let mut seg_tags = vec![];

        self.collect_existing_tags(&mut syl_tags, &mut seg_tags);

        tag_paired_syl(&mut self.input.elems, &mut self.output.elems, &mut syl_tags);
        tag_paired_seg(&mut self.input.elems, &mut self.output.elems, &mut seg_tags);

        tag_context(&mut self.pre_context.elems, &mut syl_tags, &mut seg_tags);
        tag_context(&mut self.post_context.elems, &mut syl_tags, &mut seg_tags);

        self.check_invariants()
    }

    fn collect_existing_tags(
        &self,
        syl_tags: &mut Vec<u32>,
        seg_tags: &mut Vec<u32>,
    ) {
        for elem in self
            .input
            .elems
            .iter()
            .chain(self.output.elems.iter())
            .chain(self.pre_context.elems.iter())
            .chain(self.post_context.elems.iter())
        {
            if let Element::Features(syl, seg) = elem {
                if let Some(id) = syl.id {
                    syl_tags.push(id);
                }
                if let Some(id) = seg.id {
                    seg_tags.push(id);
                }
            }
        }
    }
    fn check_invariants(&self) -> bool {
        let input_and_ctx_syl_tags: Vec<u32> = self
            .input
            .elems
            .iter()
            .chain(self.pre_context.elems.iter())
            .chain(self.post_context.elems.iter())
            .filter_map(|e| {
                if let Element::Features(syl, _) = e {
                    syl.id
                } else {
                    None
                }
            })
            .collect();

        let input_and_ctx_seg_tags: Vec<u32> = self
            .input
            .elems
            .iter()
            .chain(self.pre_context.elems.iter())
            .chain(self.post_context.elems.iter())
            .filter_map(|e| {
                if let Element::Features(_, seg) = e {
                    seg.id
                } else {
                    None
                }
            })
            .collect();

        for elem in &self.output.elems {
            if let Element::Features(syl, seg) = elem {
                if let Some(id) = syl.id {
                    if !input_and_ctx_syl_tags.contains(&id) {
                        return false;
                    }
                }
                if let Some(id) = seg.id {
                    if !input_and_ctx_seg_tags.contains(&id) {
                        return false;
                    }
                }
            }
        }

        let input_syl_tags: Vec<u32> = self
            .input
            .elems
            .iter()
            .filter_map(|e| {
                if let Element::Features(syl, _) = e {
                    syl.id
                } else {
                    None
                }
            })
            .collect();
        let input_seg_tags: Vec<u32> = self
            .input
            .elems
            .iter()
            .filter_map(|e| {
                if let Element::Features(_, seg) = e {
                    seg.id
                } else {
                    None
                }
            })
            .collect();

        let syl_unique =
            input_syl_tags.iter().collect::<HashSet<_>>().len() == input_syl_tags.len();
        let seg_unique =
            input_seg_tags.iter().collect::<HashSet<_>>().len() == input_seg_tags.len();

        syl_unique && seg_unique
    }
}

fn next_tag(existing: &mut Vec<u32>) -> u32 {
    let tag = (0u32..).find(|t| !existing.contains(t)).unwrap();
    existing.push(tag);
    tag
}

fn needs_syl_tag(elem: &Element) -> bool {
    if let Element::Features(syl, _) = elem {
        syl.id.is_none() && !syl.features.is_complete()
    } else {
        false
    }
}

fn needs_seg_tag(elem: &Element) -> bool {
    if let Element::Features(_, seg) = elem {
        seg.id.is_none() && !seg.features.is_complete()
    } else {
        false
    }
}

fn tag_paired_syl(input: &mut [Element], output: &mut [Element], existing: &mut Vec<u32>) {
    let mut ii = 0;
    let mut oi = 0;

    while ii < input.len() && oi < output.len() {
        if !needs_syl_tag(&input[ii]) {
            ii += 1;
            continue;
        }
        if !needs_syl_tag(&output[oi]) {
            oi += 1;
            continue;
        }

        let tag = next_tag(existing);
        if let Element::Features(syl, _) = &mut input[ii] {
            syl.id = Some(tag);
        }
        if let Element::Features(syl, _) = &mut output[oi] {
            syl.id = Some(tag);
        }
        ii += 1;
        oi += 1;
    }
}

fn tag_paired_seg(input: &mut [Element], output: &mut [Element], existing: &mut Vec<u32>) {
    let mut ii = 0;
    let mut oi = 0;

    while ii < input.len() && oi < output.len() {
        if !needs_seg_tag(&input[ii]) {
            ii += 1;
            continue;
        }
        if !needs_seg_tag(&output[oi]) {
            oi += 1;
            continue;
        }

        let tag = next_tag(existing);
        if let Element::Features(_, seg) = &mut input[ii] {
            seg.id = Some(tag);
        }
        if let Element::Features(_, seg) = &mut output[oi] {
            seg.id = Some(tag);
        }
        ii += 1;
        oi += 1;
    }
}

fn tag_context(elems: &mut [Element], syl_tags: &mut Vec<u32>, seg_tags: &mut Vec<u32>) {
    for elem in elems {
        if let Element::Features(syl, seg) = elem {
            if syl.id.is_none() {
                syl.id = Some(next_tag(syl_tags));
            }
            if seg.id.is_none() {
                seg.id = Some(next_tag(seg_tags));
            }
        }
    }
}
