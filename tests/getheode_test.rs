// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::segment::Segment;
    use getheode::segment_string::SegmentString;
    use getheode::phonological_rule::PhonologicalRule;
    use getheode::feature::FeatureState::{UNDEF, POS, NEG, NA};

    #[test]
    fn test_segment_from_ipa_string() {
        let a = [POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,NEG,NA];
        assert_eq!(Segment::from_ipa("a").unwrap(), Segment::from_features(a));
        // test multi-char symbols
        let a = [NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA];
        assert_eq!(Segment::from_ipa("d͡ɮ").unwrap(), Segment::from_features(a));
    }

    #[test]
    fn test_segment_from_features_string() {
        // see if the right features are being set
        let a = [UNDEF,UNDEF,UNDEF,UNDEF,POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,NEG,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF];
        assert_eq!(Segment::from_features_string("[+son-nasal]").unwrap(), Segment::from_features(a));
        // edge case
        Segment::from_features_string("[]").unwrap();
    }

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