// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::segment_string::SegmentString;
    use getheode::phonological_rule::PhonologicalRule;

    #[test]
    fn test_segment_string_from_string() {
        SegmentString::from_string("asabaki").unwrap();
        SegmentString::from_string("asabak [-voi]").unwrap();
        SegmentString::from_string("[]").unwrap();
        SegmentString::from_string("a[-voi+delrel-son]a").unwrap();
        assert_eq!(&SegmentString::from_string("t͡ʃ").unwrap().to_string(),"t͡ʃ");
    }

    #[test]
    fn test_segment_string_format() {
        let s = "asabaki";
        assert_eq!(SegmentString::from_string(s).unwrap().to_string(), s.to_owned());
        assert_eq!(SegmentString::from_string("asabak [-voi]").unwrap().to_string(),  "asabak[-voi]".to_owned());
        let s = "[]";
        assert_eq!(SegmentString::from_string(s).unwrap().to_string(), s.to_owned());
        assert_eq!(SegmentString::from_string("a[-voi+delrel-son]a").unwrap().to_string(), "a[-son+delrel-voi]a".to_owned());
    }

    #[test]
    fn test_rule_from_string() {
        PhonologicalRule::new("{i, es} -> j /_{a, o}").unwrap();
    }

    #[test]
    fn test_rule_apply() {
        let rule_str = "{i, es, t͡ʃ} -> j /_{a, o}".to_string();
        let input = "esotia".to_string();
        let expected_output = "jotja".to_string();
        let rule = PhonologicalRule::new(&rule_str).unwrap();
        assert_eq!(rule.apply_rule(&SegmentString::from_string(&input).unwrap()).unwrap().to_string(), expected_output)
    }
}
