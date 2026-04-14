use crate::{
    d3tree,
    phonology::{
        rule::{
            PhonoRule, SegmentInfo, SyllableInfo, TaggedPhonoString, parse::elem::RuleElements,
        },
        tree::Depth3Tree,
    },
};

fn build_rule(mut rule_elements: RuleElements) -> PhonoRule {
    let input_elems = &rule_elements.input.elems;
    let output_elems = &mut rule_elements.output.elems;

    let mut input_tree = TaggedPhonoString::new(d3tree![]);
    for input_elem in rule_elements.input.elems {
        match input_elem {
            super::elem::Element::Features(syllable_info, segment_info) => {
                // the syllable info of this segment is getting lost. go find matching syllable
                // tag in output and contexts and set them to this.
                // ALSO they should combine, they could have mutually exclussive feature definitions
                // TODO
                // go find output tags matching

                let rhs = TaggedPhonoString::new(d3tree![
                    () => [syllable_info => [segment_info]]
                ]);
                input_tree = merge_at_syllable(input_tree, rhs);
            }
            super::elem::Element::WordBoundary => todo!(),
            super::elem::Element::SyllableBoundary => todo!(),
        }
    }

    todo!()
}

/// Merge two tagged phonological strings at the syllable level.
/// The last syllable of lhs and first syllable of rhs are merged, resulting on one final tree.
/// The syllable tag of rhs is dropped, and features are merged. lhs overrides rhs in cases of
/// conflicting features.
fn merge_at_syllable(lhs: TaggedPhonoString, rhs: TaggedPhonoString) -> TaggedPhonoString {
    let _ = lhs;
    todo!()
}
