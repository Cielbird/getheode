// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::segment_string::SegmentString;
    use getheode::phonological_rule::PhonologicalRule;

    #[test]
    fn test_rule_from_string() {
        PhonologicalRule::new("{i, es} -> j /_{a, o}").unwrap();
    }

    #[test]
    fn test_rule_apply() {
        let rule_str = "{i, es, t͡ʃ} -> j /_{a, o}".to_string();
        let input = "tesotia".to_string();
        let expected_output = "tjotja".to_string();
        let rule = PhonologicalRule::new(&rule_str).unwrap();
        assert_eq!(rule.apply(&SegmentString::new(&input).unwrap()).unwrap().to_string(), expected_output)
    }
}
