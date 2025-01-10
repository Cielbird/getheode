// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::segment_string::SegmentString;

    // TODO: see the todo in phonological_rule_test.rs: these tests are lazy
    #[test]
    fn test_segment_string_from_string() {
        // test segment parsing
        SegmentString::new("aski").unwrap();
        SegmentString::new("as[-voi]i").unwrap();
        SegmentString::new("asɣ˕i").unwrap();
        SegmentString::new("[]").unwrap();

        // test parsing of multi-char symbols
        assert_eq!(&SegmentString::new("t͡ʃa").unwrap().to_string(), "t͡ʃa");

        // test word/syl bounds
        SegmentString::new("as ki").unwrap();
        SegmentString::new("as.ki").unwrap();
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
