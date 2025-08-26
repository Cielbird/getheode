// tests/getheode_test.rs

#[cfg(test)]
mod tests {
    use crate::{
        phonological_rule::{FormatRuleStr, PhonologicalRule},
        segment::{FormatIpa, FormatSegmentString, Segment, SegmentString},
    };

    #[test]
    fn test_from_rule_string() {
        let rule = PhonologicalRule::parse("a -> e").unwrap();

        let a_seg = Segment::parse_ipa("a").unwrap();
        let a_seg = SegmentString::from_segments(vec![a_seg]);
        assert_eq!(rule.input_opts, vec![a_seg]);

        let e_seg = Segment::parse_ipa("e").unwrap();
        let e_seg = SegmentString::from_segments(vec![e_seg]);
        assert_eq!(rule.output, e_seg);

        assert_eq!(rule.pre_context_opts, vec![]);
        assert_eq!(rule.post_context_opts, vec![]);
    }

    #[test]
    fn test_rule_single_context() {
        let rule = PhonologicalRule::parse("i -> j /_C").unwrap();

        let i_seg = SegmentString::parse("i").unwrap();
        assert_eq!(rule.input_opts, vec![i_seg]);

        let j_seg = SegmentString::parse("j").unwrap();
        assert_eq!(rule.output, j_seg);

        assert_eq!(rule.pre_context_opts, vec![]);
        let cons_seg = SegmentString::parse("C").unwrap();
        assert_eq!(rule.post_context_opts, vec![cons_seg]);
    }

    #[test]
    fn test_rule_multi_options() {
        let rule = PhonologicalRule::parse("{i, es} -> j /{a, o}_").unwrap();

        let i_seg = SegmentString::parse("i").unwrap();
        let es_seg = SegmentString::parse("es").unwrap();
        assert_eq!(rule.input_opts, vec![i_seg, es_seg]);

        let j_seg = SegmentString::parse("j").unwrap();
        assert_eq!(rule.output, j_seg);

        let a_seg = SegmentString::parse("a").unwrap();
        let o_seg = SegmentString::parse("o").unwrap();
        assert_eq!(rule.pre_context_opts, vec![a_seg, o_seg]);
        assert_eq!(rule.post_context_opts, vec![]);
    }

    #[test]
    fn test_rule_apply() {
        let rule_str = "{i, es, t͡ʃ} -> j /_{a, o}".to_string();
        let input = "tesotia".to_string();
        let expected_output = "tjotja".to_string();
        let rule = PhonologicalRule::parse(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&SegmentString::parse(&input).unwrap())
                .unwrap()
                .to_string(),
            expected_output
        )
    }

    #[test]
    fn test_rule_apply_word_bound() {
        let rule_str = "s -> es /#_".to_string();
        let input = "#strasa#".to_string();
        let expected_output = "#estrasa#".to_string();
        let rule = PhonologicalRule::parse(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&SegmentString::parse(&input).unwrap())
                .unwrap()
                .to_string(),
            expected_output
        )
    }
}
