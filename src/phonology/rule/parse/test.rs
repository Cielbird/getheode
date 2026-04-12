// TODO make parser for these rule sets
use paste::paste;

use crate::phonology::rule::{
    SegmentInfo, SyllableInfo,
    parse::{
        elem::{Element, ElementSequence, RuleElements},
        node::Node,
        parse_elem::parse_rule_elems,
        parse_patterns::{parse_rule_elem_branch, parse_rule_pattern, parse_rule_patterns},
        pattern::{Pattern, RulePatterns, RuleStrings},
    },
};

/// Macro for generating tests for phonlogical rule syntax parsing
macro_rules! test_phono_rule_syntax {
    ($name:ident, $rule:expr) => {
        paste! {
            #[test]
            fn [<test_rule_ $name>]() {
                // println!("Testing phonological rule : {}", stringify!($name));
                let opts = $crate::phonology::rule::parse::PhonoRuleParseOpts::default();
                let (rem, _pat) = $crate::phonology::rule::parse::parse_patterns::parse_rule_patterns($rule, opts).unwrap();
                assert_eq!("", rem);
            }
        }
    };
}

#[test]
fn test_rule_simple_multi_pattern() {
    let opts = crate::phonology::rule::parse::PhonoRuleParseOpts::default();
    let (rem, rule) = parse_rule_patterns("z ʃ tʃ -> ʒ s s", opts).unwrap();
    assert_eq!(rem, "");
    assert_eq!(
        rule.input,
        vec![
            Pattern {
                root: Node::Leaf("z")
            },
            Pattern {
                root: Node::Leaf("ʃ"),
            },
            Pattern {
                root: Node::Leaf("tʃ")
            }
        ]
    );
    assert_eq!(rule.output, vec!["ʒ", "s", "s"]);
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
    assert_eq!(
        pat.root,
        Node::Branch(vec![Node::Leaf("C"), Node::Leaf("V"), Node::Leaf("C")])
    );
}

#[test]
fn test_parse_pattern() {
    let (remaining, pat) = parse_rule_pattern("{V[+ant-dist+cor], a}S ").unwrap();
    println!("{:?}", pat);
    assert_eq!(remaining, " ");
    assert_eq!(
        pat.root,
        Node::Sequence(vec![
            Node::Branch(vec![Node::Leaf("V[+ant-dist+cor]"), Node::Leaf("a")]),
            Node::Leaf("S")
        ])
    );
}

#[test]
fn test_enumerate_branches() {
    let rule = "{θ,t}a{i(t),Ø}";
    let (remainder, rule) = parse_rule_pattern(rule).unwrap();
    assert_eq!(remainder, "");

    let possibilities = rule.enumerate_branches();
    let mut expected = vec!["θait", "θai", "θa", "tait", "tai", "ta"];
    // test ignoring order
    for expected in expected.drain(..) {
        let match_n = possibilities.iter().filter(|x| *x == expected).count();
        assert_eq!(match_n, 1);
    }
}

#[test]
fn test_enumerate_rule() {
    let rule = "ʃ {θ,t} m k -> s ts tʲ bʲ / _{i(ː),j,#}";
    let (remainder, rule) = parse_rule_patterns(rule, Default::default()).unwrap();
    assert_eq!(remainder, "");

    let rule = rule.enumerate();
    assert_eq!(
        rule.input,
        vec![vec!["ʃ"], vec!["θ", "t"], vec!["m"], vec!["k"]]
    );
    assert_eq!(rule.output, vec!["s", "ts", "tʲ", "bʲ"]);
    assert_eq!(rule.pre_context, vec![""]);
    assert_eq!(rule.post_context, vec!["iː", "i", "j", "#"],);
}

// test conversion from RuleStrings to RuleElements
fn test_parse_rule_strings() {
    let rule = RuleStrings {
        input: vec![vec!["Vʃ".to_string()]],
        output: vec!["Vbʲ".to_string()],
        pre_context: vec!["C$".to_string()],
        post_context: vec!["iː".to_string()],
    };

    let rules = RuleElements::from_strings(rule).unwrap();
    assert_eq!(rules.len(), 1);

    let rule = &rules[0];
    let input = &rule.input;
    let output = &rule.output;
    let pre_ctx = &rule.pre_context;
    let post_ctx = &rule.post_context;
    assert_eq!(input.elems.len(), 2);
    assert_eq!(output.elems.len(), 2);
    assert_eq!(pre_ctx.elems.len(), 2);
    assert_eq!(post_ctx.elems.len(), 1);
    
    let input_0 = &input.elems[0];
    let input_1 = &input.elems[1];
    let output_0 = &output.elems[0];
    let output_1 = &output.elems[1];
    let pre_0 = &pre_ctx.elems[0];
    let pre_1 = &pre_ctx.elems[1];
    let post_0 = &post_ctx.elems[0];

    // check input and output V have same tag, and that precontext's C tag is different
    if let (
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl_in),
                features: _,
            },
            SegmentInfo {
                tag: Some(tag_seg_in),
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl_out),
                features: _,
            },
            SegmentInfo {
                tag: Some(tag_seg_out),
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl_ctx),
                features: _,
            },
            SegmentInfo {
                tag: Some(tag_seg_ctx),
                features: _,
            },
        ),
    ) = (input_0, output_0, pre_0)
    {
        assert_eq!(tag_syl_in, tag_syl_out);
        assert_eq!(tag_seg_in, tag_seg_out);

        assert_ne!(tag_syl_ctx, tag_syl_in);
        assert_ne!(tag_seg_ctx, tag_seg_in);
    } else {
        panic!("rule doesn't have the right input !");
    }

    // assert no tags
    // ʃ
    assert!(matches!(input_1, Element::Features(SyllableInfo { tag: None, features: _ }, _)));
    // bʲ
    assert!(matches!(output_1, Element::Features(SyllableInfo { tag: None, features: _ }, _)));
    // iː
    assert!(matches!(post_0, Element::Features(SyllableInfo { tag: None, features: _ }, _)));

    // assert boundary
    // $
    assert!(matches!(pre_1, Element::SyllableBoundary));
}
