// tests/getheode_test.rs


extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::phonological_rule::PhonologicalRule;
    use getheode::representation::Representation;
    use getheode::segment_string::SegmentString;
    use std::fs;

    #[test]
    fn test_from_rep() {
        let x: String = fs::read_to_string("tests/representation_test.txt").unwrap();
        let rep = Representation::from_str(&x).unwrap();
        let actual = rep.from_rep("cu̅l").unwrap();
        let expected = SegmentString::new("kuːl").unwrap();
        assert_eq!(
            actual,
            expected
        )

    }
}
