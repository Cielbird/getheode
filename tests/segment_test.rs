// tests/getheode_test.rs

extern crate getheode;


#[cfg(test)]
mod tests {
    use getheode::{segment::Segment, segment_string::SegmentString};
    use getheode::feature::FeatureState::{NEG, POS, UNDEF, NA};

    // TODO: see the todo in phonological_rule_test.rs: these tests are lazy
    #[test]
    fn test_segment_from_ipa() {
        let seg_str = Segment::from_ipa("a").unwrap();
        assert_eq!(
            seg_str, 
            Segment::from_features(
                    [POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,NEG,NA]
            ),
        );
    }

    #[test]
    fn test_segment_from_features() {
        let seg_str = Segment::from_string("[-voi]").unwrap();
        assert_eq!(
            seg_str, 
            Segment::from_features(
                [UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
                 UNDEF,UNDEF,UNDEF,UNDEF,NEG,  UNDEF,UNDEF,
                 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
                 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,])
        );
    }

    #[test]
    fn test_segment_from_ipa_with_diacritic() {
        let seg_str = Segment::from_ipa("ɣ˕").unwrap();
        assert_eq!(
            seg_str, 
            Segment::from_features(
                [NEG,NEG,NEG,POS,NEG,POS,NEG,
                 NEG,NEG,NEG,NEG,POS,NEG,NEG,
                 NEG,NEG,NEG,NEG,NA, NA, NA,
                 NEG,POS,POS,POS,NA, NA, NA]),
        );
    }
}
