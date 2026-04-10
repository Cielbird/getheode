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
    let (rem, rule_set) =
        crate::phonology::rule::parse::parse_rule("z ʃ tʃ -> ʒ s s", opts).unwrap();
    assert_eq!(rem, "");

    assert_eq!(rule_set.rules.len(), 3);

    // for each rule in the set, assert ids match
    for i in 0..3 {
        let rule = &rule_set.rules[i];

        todo!()
        // assert syllable id
        // let (_, match_syl_id) = rule.match_tree.layer_1[0];
        // let (_, replace_syl_id) = rule.replace_tree.layer_1[0];
        // assert_eq!(match_syl_id, replace_syl_id);

        // // assert segment id
        // let (_, match_seg_id) = rule.match_tree.layer_2[0];
        // let (_, replace_seg_id) = rule.replace_tree.layer_2[0];
        // assert_eq!(match_seg_id, replace_seg_id);
    }
}

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

// this complicated rule has a complicated post-context.
test_phono_rule_syntax!(optional_diacritic_and_nested_branch, "ʃ {θ,t} m k -> s ts tʲ bʲ / _{i(ː),j,#}");
