// tests/getheode_test.rs
#[cfg(test)]
mod segment_tests {

    use crate::phonology::feature::FeatureState::*;
    use crate::phonology::segment::{
        SegmentFeatures, parse_segment, parse_segment_feature_set, parse_segment_ipa,
    };

    #[test]
    fn test_segment_from_ipa() {
        let (remaining, seg) = parse_segment_ipa("a").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG,
                NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
            ]),
        );
    }

    #[test]
    fn test_parse_feature_set() {
        let (remaining, seg) = parse_segment_feature_set("-voi+back -tense").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, NEG, UNDEF,
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
                UNDEF, POS, NEG,
            ])
        );
    }

    #[test]
    fn test_parse_ipa_with_diacritic() {
        let (remaining, seg) = parse_segment_ipa("ɣ˕").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                NEG, NEG, POS, NEG, POS, NEG, NEG, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG,
                NEG, NA, NA, NA, NEG, POS, POS, POS, NA, NA, NA
            ]),
        );
    }

    #[test]
    fn test_parse_ipa_with_feature() {
        // can't use parse_segment_ipa, because it wouldn't catch the non-ipa feature set
        let (remaining, seg) = parse_segment("a[+cons]").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                POS, NEG, POS, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG,
                NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
            ]),
        );
    }

    #[test]
    fn test_parse_nat_class_with_feature() {
        // can't use parse_natural_class, because it wouldn't catch the feature set.
        // intended behevior.
        let (remaining, seg) = parse_segment("V[+front]").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                POS, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
                UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF, UNDEF,
                POS, UNDEF, UNDEF,
            ]),
        );
    }

    #[test]
    fn test_parse_ipa_with_diacritics_and_feature() {
        let (remaining, seg) = parse_segment("t̪[+delrel]").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            seg,
            SegmentFeatures::from_features([
                NEG, NEG, POS, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG,
                POS, POS, POS, NEG, NEG, NEG, NA, NA, NA, NA, NA
            ]),
        );
    }
}
