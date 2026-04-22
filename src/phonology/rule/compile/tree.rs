use crate::error::*;
use crate::{
    d3tree,
    phonology::{
        rule::{PatternBorder, PhonoStringPattern, SyllableInfo, parse::Element},
        syllable::SyllableFeatures,
    },
};

/// Compiles a sequence of `elements` into a tagged phonological string.
/// `syl_tag_squash_callback` is called when two syllable nodes are merged and a tag is lost.
pub(super) fn compile_tree<F>(
    mut elements: &[Element],
    mut syl_tag_squash_callback: F,
) -> Result<PhonoStringPattern>
where
    F: FnMut(u32, u32),
{
    // parse possible initial boundary
    let left_bound = match elements.first() {
        Some(Element::Features(_, _)) => PatternBorder::Any,
        Some(bound) => {
            // remove initial boundary
            elements = &elements[1..];
            match bound {
                Element::WordBoundary => PatternBorder::Word,
                Element::SyllableBoundary => PatternBorder::StrictSyllable,
                _ => unreachable!(),
            }
        }
        None => PatternBorder::Any, // edge case: output deletion with no context
    };

    // parse possible final boundary
    let right_bound = match elements.last() {
        Some(Element::Features(_, _)) => PatternBorder::Any,
        Some(bound) => {
            // remove final boundary
            elements = &elements[..(elements.len() - 1)];
            match bound {
                Element::WordBoundary => PatternBorder::Word,
                Element::SyllableBoundary => PatternBorder::StrictSyllable,
                _ => unreachable!(),
            }
        }
        None => PatternBorder::Any, // edge case: output deletion with no context
    };

    // parse elements that make up the tree
    let first_syllable = SyllableInfo::new(None, SyllableFeatures::new_undef());
    // in case there are no elements in the tree, populate it with a placeholder word and syllable
    let mut tree = d3tree![() => [first_syllable => []]];
    let mut is_new_syllable = false;
    let mut is_new_word = false;
    for element in elements {
        match element {
            Element::Features(syllable, segment) => {
                if is_new_word {
                    tree.push_depth_0(());
                    is_new_word = false;
                }

                if is_new_syllable {
                    tree.push_depth_1(syllable.clone());
                    is_new_syllable = false;
                } else {
                    // merge syllable with last existing syllable
                    let last_syl = tree.get_depth_1_mut(tree.len_1() - 1);
                    // existing last syllable takes precedence, goes on rhs of addition
                    last_syl.features = syllable.features.clone() + last_syl.features.clone();

                    if let Some(old_tag) = syllable.tag {
                        if let Some(new_tag) = last_syl.tag {
                            syl_tag_squash_callback(old_tag, new_tag);
                        } else {
                            last_syl.tag = Some(old_tag)
                        }
                    }
                }

                tree.push_depth_2(segment.clone());
            }
            Element::WordBoundary => is_new_word = true,
            Element::SyllableBoundary => is_new_syllable = true,
        }
    }

    Ok(PhonoStringPattern::new(tree, left_bound, right_bound))
}
