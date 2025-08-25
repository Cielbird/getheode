#[cfg(test)]
mod segment_string_tests {
    use crate::segment::FeatureState::{NA, NEG, POS};
    use crate::segment::FormatSegmentString;
    use crate::{segment::Segment, segment::SegmentString};

    // TODO: see the todo in phonological_rule_test.rs: these tests are lazy
    #[test]
    fn test_parse_segment_string() {
        let seg_str = SegmentString::parse("aski").unwrap();
        assert_eq!(
            seg_str,
            SegmentString::from_segments(vec![
                Segment::from_features([
                    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG,
                    NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
                ]),
                Segment::from_features([
                    NEG, NEG, NEG, POS, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG,
                    NEG, POS, POS, NEG, POS, NEG, NEG, NA, NA, NA, NA, NA
                ]),
                Segment::from_features([
                    NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG,
                    NEG, NEG, NA, NA, NA, NEG, POS, POS, NEG, NA, NA, NA
                ]),
                Segment::from_features([
                    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG,
                    NEG, NEG, NA, NA, NA, NEG, POS, POS, NEG, POS, NEG, POS
                ]),
            ])
        );
    }

    #[test]
    fn test_segment_string_from_string_multi_chars() {
        let seg_str = SegmentString::parse("t͡ʃa").unwrap();
        assert_eq!(
            seg_str,
            SegmentString::from_segments(vec![
                Segment::from_features([
                    NEG, NEG, NEG, POS, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG,
                    NEG, POS, NEG, POS, POS, NEG, NEG, NA, NA, NA, NA, NA
                ]),
                Segment::from_features([
                    POS, NEG, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG,
                    NEG, NEG, NA, NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA
                ]),
            ])
        );
    }

    #[test]
    fn test_segment_string_from_string_word_bound() {
        let seg_str = SegmentString::parse("as#ki").unwrap();

        assert_eq!(seg_str.get_word_boundaries(), vec![2]);
    }

    #[test]
    fn test_segment_string_from_string_syl_bounds() {
        let seg_str = SegmentString::parse("as.ki").unwrap();

        assert_eq!(seg_str.get_syl_boundaries(), vec![2]);
    }

    #[test]
    fn test_segment_string_format() {
        // test simple format
        let s = "aski";
        assert_eq!(SegmentString::parse(s).unwrap().to_string(), s.to_owned());
        assert_eq!(
            SegmentString::parse("as[-voi]i").unwrap().to_string(),
            "as[-voi]i".to_owned()
        );
        let s = "[]";
        assert_eq!(SegmentString::parse(s).unwrap().to_string(), s.to_owned());
        // test format boundaries
        assert_eq!(
            SegmentString::parse("as#ki#").unwrap().to_string(),
            "as#ki#".to_owned()
        );
        assert_eq!(
            SegmentString::parse(".as.ki").unwrap().to_string(),
            ".as.ki".to_owned()
        );
    }
}
