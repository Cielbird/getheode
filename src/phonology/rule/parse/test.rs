// TODO make parser for these rule sets
use paste::paste;

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
    let (rem, rule) =
        crate::phonology::rule::parse::parse_rule("z ʃ tʃ -> ʒ s s", opts).unwrap();
    assert_eq!(rem, "");

    let x = &rule.input[0];
    println!("{:?}", x.root.debug_pretty_print(&x.tree));

    assert_eq!(rule.input.len(), 3);
    assert_eq!(rule.output.len(), 3);

    // TODO
}

test_phono_rule_syntax!(simple_context_and_alt_g, "ɡ(w) -> dʒ / #_Vd");

paste!{
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
test_phono_rule_syntax!(optional_diacritic_and_nested_branch, "ʃ {θ,t} m k -> s ts tʲ bʲ / _{i(ː),j,#}");
