use crate::{
    d3tree,
    phonology::{
        rule::{
            ElementSequence, PhonoRule, SyllableInfo, TaggedPhonoString,
            parse::{Element, RuleElements},
        },
        string::PhonoString,
        syllable::SyllableFeatures,
    },
};

use super::tree::compile_tree;

pub fn compile_rule(rule_elements: RuleElements) -> PhonoRule {
    let mut input_elems = rule_elements.pre_context_clone().elems;
    input_elems.extend(rule_elements.input_clone().elems);
    input_elems.extend(rule_elements.post_context_clone().elems);

    let mut output_elems = rule_elements.pre_context_clone().elems;
    output_elems.extend(rule_elements.output_clone().elems);
    output_elems.extend(rule_elements.post_context_clone().elems);

    let pattern = compile_tree(&input_elems, |old_tag, new_tag| {
        for output_elem in &mut output_elems {
            if let Element::Features(output_syllable, _) = output_elem
                && output_syllable.tag.is_some()
                && output_syllable.tag == Some(old_tag)
            {
                output_syllable.tag = Some(new_tag);
            }
        }
    })
    .unwrap();

    let replace_tree = if output_elems.is_empty() {
        // output is deletion edge case :
        let first_syl = SyllableInfo::new(None, SyllableFeatures::new_undef());
        TaggedPhonoString::new(d3tree![() => [first_syl => []]])
    } else {
        compile_tree(&output_elems, |_, _| {}).unwrap().tree
    };

    PhonoRule {
        pattern,
        replace_tree,
    }
}

/// Compile elements without tags into a phonological string
pub fn compile_untagged_elements(elements: ElementSequence) -> Result<PhonoString, String> {
    let tree = compile_tree(&elements.elems, |_, _| {})?.tree;
    let mut untagged = d3tree![];
    for (_, syls) in tree.iter() {
        untagged.push_depth_0(());
        for (syl, segs) in syls {
            untagged.push_depth_1(syl.features.clone());
            for seg in segs {
                untagged.push_depth_2(seg.features.clone());
            }
        }
    }

    Ok(PhonoString { tree: untagged })
}
