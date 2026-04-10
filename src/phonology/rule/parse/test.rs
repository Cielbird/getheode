// TODO make parser for these rule sets
use paste::paste;

use crate::phonology::{
    rule::{
        SegmentInfo, SyllableInfo,
        parse::tree::{
            ParsedRuleElem,
            ParsedRuleNode::{self, Leaf},
        },
        parse_rule_elem, parse_rule_elem_branch,
    },
    segment::{SegmentFeatures, parse_segment},
    syllable::SyllableFeatures,
};

/// Macro for generating tests for phonlogical rule syntax parsing
macro_rules! test_phono_rule_syntax {
    ($name:ident, $rule:expr) => {
        paste! {
            #[test]
            fn [<test_rule_ $name>]() {
                // println!("Testing phonological rule : {}", stringify!($name));
                let opts = $crate::phonology::rule::parse::PhonoRuleParseOpts::default();
                $crate::phonology::rule::parse::parse_rule($rule, opts).unwrap();
            }
        }
    };
}

#[test]
fn test_rule_simple_multi_pattern() {
    let opts = crate::phonology::rule::parse::PhonoRuleParseOpts::default();
    let (rem, rule) = crate::phonology::rule::parse::parse_rule("z ʃ tʃ -> ʒ s s", opts).unwrap();
    assert_eq!(rem, "");

    // let pat = &rule.input[0];
    // let a = pat.tree.get(pat.root).unwrap().first_child().unwrap();
    // let node = pat.tree.get(a).unwrap().get();
    // if let Leaf(ParsedRuleElem::Features(_, seg)) = node {
    //     println!("First Segment is {}", seg.features);
    // }
    // let b = pat.tree.get(a).unwrap().next_sibling().unwrap();
    // let node = pat.tree.get(b).unwrap().get();
    // if let Leaf(ParsedRuleElem::Features(_, seg)) = node {
    //     println!("2nd Segment is {}", seg.features);
    // }
    // let c = pat.tree.get(b).unwrap().next_sibling().unwrap();
    // let node = pat.tree.get(c).unwrap().get();
    // if let Leaf(ParsedRuleElem::Features(_, seg)) = node {
    //     println!("3rd Segment is {}", seg.features);
    // }
    // let d = pat.tree.get(c).unwrap().next_sibling().unwrap();
    // let node = pat.tree.get(d).unwrap().get();
    // if let Leaf(ParsedRuleElem::Features(_, seg)) = node {
    //     println!("4th Segment is {}", seg.features);
    // }

    // assert_eq!(rule.input.len(), 3);
    // assert_eq!(rule.output.len(), 3);

    // TODO
}

test_phono_rule_syntax!(simple_context_and_alt_g, "ɡ(w) -> dʒ / #_Vd");

paste! {
    #[test]fn[<test_rule_ branching_post_ctx_1>](){
        let opts = crate::phonology::rule::parse::PhonoRuleParseOpts::default();
        crate::phonology::rule::parse::parse_rule("q -> i / #_V{Z,C[+dental]} ",opts).unwrap();
    }
}

// ⟨#⟩ : A word boundary
test_phono_rule_syntax!(branching_post_ctx_2, "n -> l / #_(V){s,ʃ,h}V{m,b}# ");

// apostrophe here could be written as '
test_phono_rule_syntax!(weird_apostrophe, "tlʼ -> ɬ / _C[+sibilant]");

// F: fricative, S: stop (in this context ?)
test_phono_rule_syntax!(odd_natural_classes, "S -> F / _S ");

// ⟨$⟩ : Either a phonological word boundary or syllable boundary
// note : syntactic word != phonological word
test_phono_rule_syntax!(simple_removal, "j -> ∅ / Ck_$");

// this complicated rule has a complicated post-context.
test_phono_rule_syntax!(
    optional_diacritic_and_nested_branch,
    "ʃ {θ,t} m k -> s ts tʲ bʲ / _{i(ː),j,#}"
);

#[test]
fn test_parse_branch() {
    let (remaining, pat) = parse_rule_elem_branch("{C, V,C } ").unwrap();
    assert_eq!(remaining, " ");

    let branch = pat.tree.get(pat.root).unwrap().get();
    assert_eq!(*branch, ParsedRuleNode::Branch);

    let mut children = pat.root.children(&pat.tree);
    let a = children.next().unwrap();
    let b = children.next().unwrap();
    let c = children.next().unwrap();
    assert_eq!(children.next(), None);

    let a = pat.tree.get(a).unwrap().get();
    assert!(matches!(*a, ParsedRuleNode::Leaf(_)));
    let b = pat.tree.get(b).unwrap().get();
    assert!(matches!(*a, ParsedRuleNode::Leaf(_)));
    let c = pat.tree.get(c).unwrap().get();
    assert!(matches!(*a, ParsedRuleNode::Leaf(_)));
}

#[test]
fn test_parse_elem() {
    let (remaining, pat) = parse_rule_elem("s ts tʲ bʲ").unwrap();
    assert_eq!(remaining, " ts tʲ bʲ");

    assert_eq!(
        pat,
        ParsedRuleElem::Features(
            SyllableInfo::new(None, SyllableFeatures::new_undef()),
            SegmentInfo::new(None, parse_segment("s").unwrap().1)
        )
    );
}
