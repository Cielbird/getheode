// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::phonological_rule::PhonologicalRule;
    use getheode::segment_string::SegmentString;

    // TODO improve: this should be 4 different tests and there should be a verification of the segments
    #[test]
    fn test_rule_from_string() {
        PhonologicalRule::from_string("a -> e").unwrap();
        PhonologicalRule::from_string("i -> j /_C").unwrap();
        PhonologicalRule::from_string("{i, es} -> j /{a, o}_").unwrap();
        PhonologicalRule::from_string(" -> e /{a, o}_").unwrap();
    }

    #[test]
    fn test_rule_apply() {
        let rule_str = "{i, es, t͡ʃ} -> j /_{a, o}".to_string();
        let input = "tesotia".to_string();
        let expected_output = "tjotja".to_string();
        let rule = PhonologicalRule::from_string(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&SegmentString::new(&input).unwrap())
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
        let rule = PhonologicalRule::from_string(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&SegmentString::new(&input).unwrap())
                .unwrap()
                .to_string(),
            expected_output
        )
    }
}
