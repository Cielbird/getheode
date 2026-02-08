// tests/getheode_test.rs

#[cfg(test)]
mod tests {
    use crate::{
        phonological_rule::{FormatRuleStr, PhonologicalRule},
        segment::{FormatIpa, FormatPhonologicalString, PhonologicalString, Segment},
    };

    #[test]
    fn test_from_rule_string() {
        let rule = PhonologicalRule::parse("a -> e").unwrap();

        let a_seg = Segment::parse_ipa("a").unwrap();
        let a_seg = PhonologicalString::from_segments(vec![a_seg]);
        assert_eq!(rule.input_opts, vec![a_seg.into()]);

        let e_seg = Segment::parse_ipa("e").unwrap();
        let e_seg = PhonologicalString::from_segments(vec![e_seg]);
        assert_eq!(rule.output, e_seg);

        assert_eq!(rule.pre_context_opts, vec![]);
        assert_eq!(rule.post_context_opts, vec![]);
    }

    #[test]
    fn test_rule_single_context() {
        let rule = PhonologicalRule::parse("i -> j /_C").unwrap();

        let i_seg = PhonologicalString::parse("i").unwrap();
        assert_eq!(rule.input_opts, vec![i_seg.into()]);

        let j_seg = PhonologicalString::parse("j").unwrap();
        assert_eq!(rule.output, j_seg);

        assert_eq!(rule.pre_context_opts, vec![]);
        let cons_seg = PhonologicalString::parse("C").unwrap();
        assert_eq!(rule.post_context_opts, vec![cons_seg.into()]);
    }

    #[test]
    fn test_rule_multi_options() {
        let rule = PhonologicalRule::parse("{i, es} -> j /{a, o}_").unwrap();

        let i_seg = PhonologicalString::parse("i").unwrap();
        let es_seg = PhonologicalString::parse("es").unwrap();
        assert_eq!(rule.input_opts, vec![i_seg.into(), es_seg.into()]);

        let j_seg = PhonologicalString::parse("j").unwrap();
        assert_eq!(rule.output, j_seg);

        let a_seg = PhonologicalString::parse("a").unwrap();
        let o_seg = PhonologicalString::parse("o").unwrap();
        assert_eq!(rule.pre_context_opts, vec![a_seg.into(), o_seg.into()]);
        assert_eq!(rule.post_context_opts, vec![]);
    }

    #[test]
    fn test_rule_apply() {
        let rule_str = "{i, es, t͡ʃ} -> j /_{a, o}".to_string();
        let input = "tesotia".to_string();
        let expected_output = "tjotja".to_string();
        let rule = PhonologicalRule::parse(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&PhonologicalString::parse(&input).unwrap())
                .unwrap()
                .to_string(),
            expected_output
        )
    }

    #[test]
    fn test_rule_apply_word_bound() {
        let rule_str = "s -> es /#_".to_string();
        let input = "_strasa_".to_string();
        let expected_output = "_estrasa_".to_string();
        let rule = PhonologicalRule::parse(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&PhonologicalString::parse(&input).unwrap())
                .unwrap()
                .to_string(),
            expected_output
        )
    }
}
