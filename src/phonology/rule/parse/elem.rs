use std::{collections::HashSet, iter::zip};

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
}

#[derive(Debug, Clone)]
pub struct ElementSequence {
    pub elems: Vec<Element>,
}

/// Represents a rule with a sequence of elements : either segments or boundaries
/// no branching: input, output and context.
pub struct RuleElements {
    pub(crate) input: ElementSequence,
    pub(crate) output: ElementSequence,
    pub(crate) pre_context: ElementSequence,
    pub(crate) post_context: ElementSequence,
}

impl RuleElements {
    fn new(
        input: ElementSequence,
        output: ElementSequence,
        pre_context: ElementSequence,
        post_context: ElementSequence,
    ) -> Result<Self, ()> {
        let mut rule = Self {
            input,
            output,
            pre_context,
            post_context,
        };

        if !rule.check_invariants() {
            return Err(());
        }

        // Tag inputs and outputs
        if !rule.tag_all() {
            // tagging failed
            return Err(());
        }

        Ok(rule)
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
                    syl.tag
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
                    seg.tag
                } else {
                    None
                }
            })
            .collect();

        for elem in &self.output.elems {
            if let Element::Features(syl, seg) = elem {
                if let Some(id) = syl.tag {
                    if !input_and_ctx_syl_tags.contains(&id) {
                        return false;
                    }
                }
                if let Some(id) = seg.tag {
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
                    syl.tag
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
                    seg.tag
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

    /// Apply the element parsing algo to each possible input, output and context.
    pub fn from_strings(strings: RuleStrings) -> Result<Vec<Self>, ()> {
        // manage the parsing error and remainder
        fn parse(input: String) -> Result<ElementSequence, ()> {
            let (rem, elems) = parse_rule_elems(&input).map_err(|x| ())?;
            if rem != "" {
                return Err(());
            }
            Ok(elems)
        }

        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut pre_context = vec![];
        let mut post_context = vec![];
        for input_opts in strings.input {
            let mut parsed_input_opts = vec![];
            for opt in input_opts {
                parsed_input_opts.push(parse(opt)?);
            }
            inputs.push(parsed_input_opts);
        }
        for output in strings.output {
            outputs.push(parse(output)?);
        }
        for pre_context_opt in strings.pre_context {
            pre_context.push(parse(pre_context_opt)?);
        }
        for post_context_opt in strings.post_context {
            post_context.push(parse(post_context_opt)?);
        }

        let mut rules = vec![];
        for (input, output) in zip(inputs, outputs) {
            for input_opt in &input {
                for pre_context_opt in &pre_context {
                    for post_context_opt in &post_context {
                        rules.push(RuleElements::new(
                            input_opt.clone(),
                            output.clone(),
                            pre_context_opt.clone(),
                            post_context_opt.clone(),
                        )?);
                    }
                }
            }
        }

        Ok(rules)
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

        // todo!("Tag untagged border elements in input and output");

        self.check_invariants()
    }

    fn collect_existing_tags(&self, syl_tags: &mut Vec<u32>, seg_tags: &mut Vec<u32>) {
        for elem in self
            .input
            .elems
            .iter()
            .chain(self.output.elems.iter())
            .chain(self.pre_context.elems.iter())
            .chain(self.post_context.elems.iter())
        {
            if let Element::Features(syl, seg) = elem {
                if let Some(id) = syl.tag {
                    syl_tags.push(id);
                }
                if let Some(id) = seg.tag {
                    seg_tags.push(id);
                }
            }
        }
    }
}

fn next_tag(existing: &mut Vec<u32>) -> u32 {
    let tag = (0u32..).find(|t| !existing.contains(t)).unwrap();
    existing.push(tag);
    tag
}

fn needs_syl_tag(elem: &Element) -> bool {
    if let Element::Features(syl, _) = elem {
        syl.tag.is_none() && !syl.features.is_complete()
    } else {
        false
    }
}

fn needs_seg_tag(elem: &Element) -> bool {
    if let Element::Features(_, seg) = elem {
        seg.tag.is_none() && !seg.features.is_complete()
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
            syl.tag = Some(tag);
        }
        if let Element::Features(syl, _) = &mut output[oi] {
            syl.tag = Some(tag);
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
            seg.tag = Some(tag);
        }
        if let Element::Features(_, seg) = &mut output[oi] {
            seg.tag = Some(tag);
        }
        ii += 1;
        oi += 1;
    }
}

fn tag_context(elems: &mut [Element], syl_tags: &mut Vec<u32>, seg_tags: &mut Vec<u32>) {
    for elem in elems {
        if let Element::Features(syl, seg) = elem {
            if syl.tag.is_none() && !syl.features.is_complete() {
                syl.tag = Some(next_tag(syl_tags));
            }
            if seg.tag.is_none() && !seg.features.is_complete() {
                seg.tag = Some(next_tag(seg_tags));
            }
        }
    }
}
