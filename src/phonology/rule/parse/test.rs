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

test_phono_rule_syntax!(simple_multi_pattern, "z ʃ tʃ -> ʒ s s");

test_phono_rule_syntax!(simple_context_and_alt_g, "ɡ(w) -> dʒ / #_Vd");

test_phono_rule_syntax!(branching_post_ctx_1, "q -> i / #_V{Z,C[+dental]} ");

// ⟨#⟩ : A word boundary
test_phono_rule_syntax!(branching_post_ctx_2, "n -> l / #_(V){s,ʃ,h}V{m,b}# ");

// apostrophe here could be written as '
test_phono_rule_syntax!(weird_apostrophe, "tlʼ → ɬ / _C[+sibilant]");

// F: fricative, S: stop (in this context ?)
test_phono_rule_syntax!(odd_natural_classes, "S → F / _S ");

// ⟨$⟩ : Either a phonological word boundary or syllable boundary
// note : syntactic word != phonological word
test_phono_rule_syntax!(simple_removal, "j → ∅ / Ck_$");
