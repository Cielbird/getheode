// tests/getheode_test.rs
#[cfg(test)]
mod segment_tests {
    use crate::segment::FeatureState::{NA, NEG, POS, UNDEF};
    use crate::segment::{FormatFeatureSet, Segment};
    use crate::segment::FormatIpa;

    // TODO: see the todo in phonological_rule_test.rs: these tests are lazy
    #[test]
    fn test_segment_from_ipa() {
        let seg = Segment::parse_ipa("a").unwrap();
        assert_eq!(
            seg,
            Segment::from_features([
                POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG,
                NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
            ]),
        );
    }

    #[test]
    fn test_segment_from_features() {
        let seg = Segment::parse_feature_set("[-voi]").unwrap();
        assert_eq!(
            seg,
            Segment::from_features([
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, NEG,
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
                UNDEF, UNDEF, UNDEF, UNDEF,
            ])
        );
    }

    #[test]
    fn test_segment_from_ipa_with_diacritic() {
        let seg = Segment::parse_ipa("ɣ˕").unwrap();
        assert_eq!(
            seg,
            Segment::from_features([
                NEG, NEG, NEG, POS, NEG, POS, NEG, NEG, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG,
                NEG, NEG, NA, NA, NA, NEG, POS, POS, POS, NA, NA, NA
            ]),
        );
    }
}
