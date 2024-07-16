// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::phonological_rule::PhonologicalRule;
    use getheode::segment_string::SegmentString;

    #[test]
    fn test_rule_from_string() {
        PhonologicalRule::new("a -> e").unwrap();
        PhonologicalRule::new("i -> j /_C").unwrap();
        PhonologicalRule::new("{i, es} -> j /{a, o}_").unwrap();
        PhonologicalRule::new(" -> e /{a, o}_").unwrap();
    }

    #[test]
    fn test_rule_apply() {
        let rule_str = "{i, es, t͡ʃ} -> j /_{a, o}".to_string();
        let input = "tesotia".to_string();
        let expected_output = "tjotja".to_string();
        let rule = PhonologicalRule::new(&rule_str).unwrap();
        assert_eq!(
            rule.apply(&SegmentString::new(&input).unwrap())
                .unwrap()
                .to_string(),
            expected_output
        )
    }
}
