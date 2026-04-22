use crate::{
    d3tree,
    phonology::rule::{PatternBorder, PhonoStringPattern, parse::Element},
};

/// Compiles a sequence of `elements` into a tagged phonological string.
/// `syl_tag_squash_callback` is called when two syllable nodes are merged and a tag is lost.
pub(super) fn compile_tree<F>(
    elements: &[Element],
    mut syl_tag_squash_callback: F,
) -> Result<PhonoStringPattern, String>
where
    F: FnMut(u32, u32),
{
    if elements.is_empty() {
        return Err("Can't construct tree from 0 elements".to_string());
    }

    let mut tree = d3tree![];
    let mut is_new_syllable = false;
    let mut is_new_word = false;
    for element in elements {
        match element {
            Element::Features(syllable, segment) => {
                let num_words = tree.len_0();
                if is_new_word || num_words == 0 {
                    tree.push_depth_0(());
                    is_new_word = false;
                }

                let num_syllables = tree.len_1();
                if is_new_syllable || num_syllables == 0 {
                    tree.push_depth_1(syllable.clone());
                    is_new_syllable = false;
                } else {
                    // merge syllable with last existing syllable
                    let last_syl = tree.get_depth_1_mut(num_syllables - 1);
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

    let left_bound = match elements.first().unwrap() {
        Element::WordBoundary => PatternBorder::Word,
        Element::SyllableBoundary => PatternBorder::StrictSyllable,
        Element::Features(_, _) => PatternBorder::Any,
    };

    let right_bound = match elements.last().unwrap() {
        Element::WordBoundary => PatternBorder::Word,
        Element::SyllableBoundary => PatternBorder::StrictSyllable,
        Element::Features(_, _) => PatternBorder::Any,
    };

    Ok(PhonoStringPattern::new(tree, left_bound, right_bound))
}
