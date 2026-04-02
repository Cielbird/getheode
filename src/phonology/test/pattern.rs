// tests/getheode_test.rs
#[cfg(test)]
mod pattern_tests {
    use crate::phonology::feature::FeatureState::{self, *};
    use crate::phonology::pattern::PhonoPattern;
    use crate::phonology::syllable::PhonoSyllable;
    use crate::phonology::{segment::PhonoSegment, string::PhonoString};

    const A_SEG: PhonoSegment = PhonoSegment::from_features([
        POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA,
    ]);
    const K_SEG: PhonoSegment = PhonoSegment::from_features([
        NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, POS, NEG, NA, NA, NA,
    ]);
    const I_SEG: PhonoSegment = PhonoSegment::from_features([
        POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, POS, NEG, POS, NEG, POS,
    ]);

    const UNSTRESSED: [FeatureState; 1] = [NEG];
    const STRESSED: [FeatureState; 1] = [POS];

    #[test]
    fn segment_string_replace() {
        // start with ['kai.ka]
        // replace first 3 segments with [ia]
        // end with ['ia.ka]

        let string = PhonoString::new([
            PhonoSyllable::new(STRESSED, [K_SEG, A_SEG, I_SEG]), // stressed syllable, segments are [kai]
            PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        ]);

        let replacement = PhonoPattern::new([([UNDEF], I_SEG), ([UNDEF], A_SEG)]);
        let expected = PhonoString::new([
            PhonoSyllable::new(STRESSED, [I_SEG, A_SEG]), // stressed syllable, segments are [ia]
            PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        ]);
        string.clone().replace(0, 3, replacement);
        assert_eq!(string, expected,);
    }

    #[test]
    fn segment_string_is_match() {
        let _replacement = PhonoPattern::new([([UNDEF], I_SEG), ([UNDEF], A_SEG)]);
        // let haystack = PhonologicalString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]);
        // let is_match = pattern.is_match(&haystack, 0) && pattern.is_match(&haystack, 3);
        // assert_eq!(
        //     is_match,
        //     true,
        // );
    }
}
