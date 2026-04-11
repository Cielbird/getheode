// TODO make parser for these rule sets
use paste::paste;

use crate::phonology::rule::parse::{
    node::Node,
    parse_patterns::{parse_rule_elem_branch, parse_rule_pattern, parse_rule_patterns},
    pattern::{Pattern, RulePatterns},
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
    assert_eq!(rule.pre_context.len(), 0);
    assert_eq!(rule.post_context, vec!["iː", "i", "j", "#"],);
}
