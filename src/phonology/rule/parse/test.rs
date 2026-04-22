// TODO make parser for these rule sets
use paste::paste;

use crate::phonology::feature::FeatureState::*;
use crate::phonology::{
    rule::{
        SegmentInfo, SyllableInfo,
        compile::compile_rule,
        parse::{
            elem::{Element, ElementSequence, RuleElements},
            node::Node,
            parse_patterns::{parse_rule_elem_branch, parse_rule_pattern, parse_rule_patterns},
            pattern::{Pattern, RuleStrings},
        },
    },
    segment::SegmentFeatures,
    syllable::SyllableFeatures,
};

/// Macro for generating tests for phonlogical rule syntax parsing
macro_rules! test_phono_rule_syntax {
    ($name:ident, $rule:expr) => {
        paste! {
            #[test]
            fn [<test_rule_ $name>]() {
                let opts = $crate::phonology::rule::parse::PhonoRuleParseOpts::default();
                let (rem, _pat) = $crate::phonology::rule::parse::parse_patterns::parse_rule_patterns($rule, opts).unwrap();
                assert_eq!("", rem);
            }
        }
    };
}

const VOWEL_SEG: SegmentFeatures = SegmentFeatures::from_features([
    POS, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
]);
const CONS_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
    UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
]);
const T_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, POS, NEG,
    NEG, NEG, NEG, NA, NA, NA, NA, NA,
]);
const D_SEG: SegmentFeatures = SegmentFeatures::from_features([
    NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, POS, POS, NEG,
    NEG, NEG, NEG, NA, NA, NA, NA, NA,
]);

const UNDEF_SYL: SyllableFeatures = SyllableFeatures::from_features([UNDEF]);

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
#[test]
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
    let input = rule.input();
    let output = rule.output();
    let pre_ctx = rule.pre_context();
    let post_ctx = rule.post_context();
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
                tag: Some(tag_syl0_in),
                features: _,
            },
            SegmentInfo {
                tag: Some(tag_seg_in),
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl1_in),
                features: _,
            },
            SegmentInfo {
                tag: None,
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl0_out),
                features: _,
            },
            SegmentInfo {
                tag: Some(tag_seg_out),
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl1_out),
                features: _,
            },
            SegmentInfo {
                tag: None,
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl_prectx),
                features: _,
            },
            SegmentInfo {
                tag: Some(tag_seg_prectx),
                features: _,
            },
        ),
        Element::Features(
            SyllableInfo {
                tag: Some(tag_syl_postctx),
                features: _,
            },
            SegmentInfo {
                tag: None,
                features: _,
            },
        ),
    ) = (input_0, input_1, output_0, output_1, pre_0, post_0)
    {
        // in/out tags should match
        assert_eq!(tag_syl0_in, tag_syl0_out);
        assert_eq!(tag_syl1_in, tag_syl1_out);
        assert_eq!(tag_seg_in, tag_seg_out);

        // tags for pre-context should be different from in/out tags
        assert_ne!(tag_syl_prectx, tag_syl0_in);
        assert_ne!(tag_syl_prectx, tag_syl1_in);
        assert_ne!(tag_seg_prectx, tag_seg_in);

        // tags for post-context should be different from in/out tags
        assert_ne!(tag_syl_postctx, tag_syl0_in);
        assert_ne!(tag_syl_postctx, tag_syl1_in);
    } else {
        panic!("rule doesn't have the right input !");
    }

    // assert no tags
    // ʃ
    assert!(matches!(
        input_1,
        Element::Features(
            SyllableInfo {
                tag: Some(_),
                features: _
            },
            SegmentInfo {
                tag: None,
                features: _
            }
        )
    ));
    // bʲ
    assert!(matches!(
        output_1,
        Element::Features(
            SyllableInfo {
                tag: Some(_),
                features: _
            },
            SegmentInfo {
                tag: None,
                features: _
            }
        )
    ));
    // iː
    assert!(matches!(
        post_0,
        Element::Features(
            SyllableInfo {
                tag: Some(_),
                features: _
            },
            SegmentInfo {
                tag: None,
                features: _
            }
        )
    ));

    // assert boundary
    // $
    assert!(matches!(pre_1, Element::SyllableBoundary));
}

#[test]
fn test_compile_rule() {
    // VtV_1 -> VdV_1 / $C_$d
    let input_elems = vec![
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_untagged(VOWEL_SEG),
        ),
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_untagged(T_SEG),
        ),
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_tagged(1, VOWEL_SEG),
        ),
    ];
    let output_elems = vec![
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_untagged(VOWEL_SEG),
        ),
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_untagged(D_SEG),
        ),
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_tagged(1, VOWEL_SEG),
        ),
    ];
    let pre_context_elems = vec![
        Element::SyllableBoundary,
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_untagged(CONS_SEG),
        ),
    ];
    let post_context_elems = vec![
        Element::SyllableBoundary,
        Element::Features(
            SyllableInfo::new_untagged(UNDEF_SYL),
            SegmentInfo::new_untagged(D_SEG),
        ),
    ];

    let rule = RuleElements::new(
        ElementSequence::new(input_elems),
        ElementSequence::new(output_elems),
        ElementSequence::new(pre_context_elems),
        ElementSequence::new(post_context_elems),
    )
    .unwrap();
    let result = compile_rule(rule);

    let pat_segs = result.pattern.tree.segs();
    let rep_segs = result.replace_tree.segs();

    // context C (non-complete): tagged in pattern and replace, same tag
    assert!(
        pat_segs[0].0.tag.is_some(),
        "pattern context C should have a seg tag"
    );
    assert!(
        rep_segs[0].0.tag.is_some(),
        "replace context C should have a seg tag"
    );
    assert_eq!(
        pat_segs[0].0.tag, rep_segs[0].0.tag,
        "context C should share tag across pattern and replace"
    );

    // input/output V (non-complete): tagged, and pattern/replace share the same tag
    assert!(
        pat_segs[1].0.tag.is_some(),
        "pattern V should have a seg tag"
    );
    assert_eq!(
        pat_segs[1].0.tag, rep_segs[1].0.tag,
        "input and output V should share seg tag"
    );

    // t in pattern and d in replace are complete: no seg tag
    assert!(
        pat_segs[2].0.tag.is_none(),
        "complete t should not be tagged"
    );
    assert!(
        rep_segs[2].0.tag.is_none(),
        "complete d should not be tagged"
    );

    // V pre-assigned tag 1: preserved in both
    assert_eq!(pat_segs[3].0.tag, Some(1), "input V_1 should keep tag 1");
    assert_eq!(rep_segs[3].0.tag, Some(1), "output V_1 should keep tag 1");

    // post-context d (complete): no tag
    assert!(
        pat_segs[4].0.tag.is_none(),
        "post-context d in pattern should not be tagged"
    );
    assert!(
        rep_segs[4].0.tag.is_none(),
        "post-context d in replace should not be tagged"
    );

    assert!(result.test_invariants());
}
