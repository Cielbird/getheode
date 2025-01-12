// tests/getheode_test.rs

extern crate getheode;


#[cfg(test)]
mod tests {
    use getheode::{segment::Segment, segment_string::SegmentString};
    use getheode::feature::FeatureState::{NEG, POS, UNDEF, NA};

    // TODO: see the todo in phonological_rule_test.rs: these tests are lazy
    #[test]
    fn test_segment_string_from_string() {
        let seg_str = SegmentString::new("aski").unwrap();
        assert_eq!(seg_str, 
            SegmentString::from_segments(vec![
                Segment::from_features(
                    [POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,NEG,NA]),
                Segment::from_features(
                    [NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA]),
                Segment::from_features(
                    [NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA]),
                Segment::from_features(
                    [POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,POS]),
            ])
        );
    }

    #[test]
    fn test_segment_string_from_string_multi_chars() {
        let seg_str = SegmentString::new("t͡ʃa").unwrap();
        assert_eq!(seg_str, 
            SegmentString::from_segments(vec![
                Segment::from_features(
                    [NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,NEG,NA,NA,NA,NA,NA]),
                Segment::from_features(
                    [POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,NEG,NA]),
            ])
        );
    }

    #[test]
    fn test_segment_string_from_string_word_bound() {
        let seg_str = SegmentString::new("as#ki").unwrap();
        
        assert_eq!(
            seg_str.get_word_boundaries(), 
            vec![2]
        );
    }

    #[test]
    fn test_segment_string_from_string_syl_bounds() {
        let seg_str = SegmentString::new("as.ki").unwrap();

        assert_eq!(
            seg_str.get_syl_boundaries(), 
            vec![2]
        );
    }

    #[test]
    fn test_segment_string_format() {
        // test simple format
        let s = "aski";
        assert_eq!(SegmentString::new(s).unwrap().to_string(), s.to_owned());
        assert_eq!(
            SegmentString::new("as[-voi]i").unwrap().to_string(),
            "as[-voi]i".to_owned()
        );
        let s = "[]";
        assert_eq!(SegmentString::new(s).unwrap().to_string(), s.to_owned());
        // test format boundaries
        assert_eq!(
            SegmentString::new("as#ki#").unwrap().to_string(),
            "as#ki#".to_owned()
        );
        assert_eq!(
            SegmentString::new(".as.ki").unwrap().to_string(),
            ".as.ki".to_owned()
        );
    }
}
