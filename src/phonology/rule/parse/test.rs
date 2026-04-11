// TODO make parser for these rule sets
use paste::paste;

use crate::phonology::rule::{parse::pattern::Node, parse_rule_elem_branch, parse_rule_pattern};

/// Macro for generating tests for phonlogical rule syntax parsing
macro_rules! test_phono_rule_syntax {
    ($name:ident, $rule:expr) => {
        paste! {
            #[test]
            fn [<test_rule_ $name>]() {
                // println!("Testing phonological rule : {}", stringify!($name));
                let opts = $crate::phonology::rule::parse::PhonoRuleParseOpts::default();
                let (rem, _pat) = $crate::phonology::rule::parse::parse_rule_patterns($rule, opts).unwrap();
                assert_eq!("", rem);
            }
        }
    };
}

#[test]
fn test_rule_simple_multi_pattern() {
    let opts = crate::phonology::rule::parse::PhonoRuleParseOpts::default();
    let (rem, rule) =
        crate::phonology::rule::parse::parse_rule_patterns("z ʃ tʃ -> ʒ s s", opts).unwrap();
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

test_phono_rule_syntax!(branching_post_ctx_1, "q -> i / #_V{z,C[+ant+dist+cor]}");

// ⟨#⟩ : A word boundary
test_phono_rule_syntax!(branching_post_ctx_2, "n -> l / #_(V){s,ʃ,h}V{m,b}#");

// apostrophe here could be written as '
test_phono_rule_syntax!(weird_apostrophe, "tlʼ -> ɬ / _C[+cons+cor+strident]");

// F: fricative, S: stop (in this context ?)
test_phono_rule_syntax!(odd_natural_classes, "S -> F / _S");

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
    assert_eq!(*branch, Node::Branch);

    let mut children = pat.root.children(&pat.tree);
    let a = children.next().unwrap();
    let b = children.next().unwrap();
    let c = children.next().unwrap();
    assert_eq!(children.next(), None);

    let a = pat.tree.get(a).unwrap().get();
    assert!(matches!(*a, Node::Leaf(_)));
    let b = pat.tree.get(b).unwrap().get();
    assert!(matches!(*b, Node::Leaf(_)));
    let c = pat.tree.get(c).unwrap().get();
    assert!(matches!(*c, Node::Leaf(_)));
}

#[test]
fn test_parse_pattern() {
    let (remaining, pat) = parse_rule_pattern("{V[+ant-dist+cor], a}S ").unwrap();
    let pretty = pat.root.debug_pretty_print(&pat.tree);
    println!("{pretty:?}");

    assert_eq!(remaining, " ");

    let branch = pat.tree.get(pat.root).unwrap().get();
    assert_eq!(*branch, Node::Sequence);

    let mut children = pat.root.children(&pat.tree);
    let a = children.next().unwrap();
    let b = children.next().unwrap();
    assert_eq!(children.next(), None);

    let a_node = pat.tree.get(a).unwrap().get();
    assert!(matches!(*a_node, Node::Branch));
    let b_node = pat.tree.get(b).unwrap().get();
    assert!(matches!(*b_node, Node::Leaf(_)));

    let mut children = a.children(&pat.tree);
    let a = children.next().unwrap();
    let b = children.next().unwrap();
    assert_eq!(children.next(), None);

    let a_node = pat.tree.get(a).unwrap().get();
    assert!(matches!(*a_node, Node::Leaf(_)));
    let b_node = pat.tree.get(b).unwrap().get();
    assert!(matches!(*b_node, Node::Leaf(_)));
}
