// tests/getheode_test.rs
#[cfg(test)]
mod segment_tests {

    use crate::phonology::feature::FeatureState::*;
    use crate::phonology::segment::{
        SegmentFeatures, parse_segment_feature_set, parse_segment_ipa,
    };

    #[test]
    fn test_segment_from_ipa() {
        let seg = parse_segment_ipa("a").unwrap();
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG,
                NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
            ]),
        );
    }

    #[test]
    fn test_segment_from_features() {
        let seg = parse_segment_feature_set("[-voi]").unwrap();
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, NEG, UNDEF,
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
                UNDEF, UNDEF, UNDEF,
            ])
        );
    }

    #[test]
    fn test_segment_from_ipa_with_diacritic() {
        let seg = parse_segment_ipa("ɣ˕").unwrap();
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                NEG, NEG, POS, NEG, POS, NEG, NEG, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG,
                NEG, NA, NA, NA, NEG, POS, POS, POS, NA, NA, NA
            ]),
        );
    }

    #[test]
    fn test_segment_ipa_with_feature() {
        let seg = parse_segment_ipa("a(+cons)").unwrap();
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                POS, NEG, POS, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG,
                NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
            ]),
        );
    }
}
