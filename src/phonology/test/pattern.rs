// tests/getheode_test.rs
#[cfg(test)]
mod pattern_tests {
    use crate::phonology::feature::FeatureState::{self, *};
    use crate::phonology::pattern::PhonoPattern;
    use crate::phonology::syllable::{PhonoSyllable, SyllableFeatures};
    use crate::phonology::{segment::SegmentFeatures, string::PhonoString};

    const A_SEG: SegmentFeatures = SegmentFeatures::from_features([
        POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA,
    ]);
    const K_SEG: SegmentFeatures = SegmentFeatures::from_features([
        NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, POS, NEG, NA, NA, NA,
    ]);
    const I_SEG: SegmentFeatures = SegmentFeatures::from_features([
        POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, POS, NEG, POS, NEG, POS,
    ]);

    const UNSTRESSED: SyllableFeatures = SyllableFeatures::new([NEG]);
    const STRESSED: SyllableFeatures = SyllableFeatures::new([POS]);

    #[test]
    fn string_replace_in_syl() {
        // start with ['kai.ka]
        // replace first 3 segments with [ia]
        // end with ['ia.ka]

        let string = PhonoString::new(vec![vec![
            (STRESSED, vec![K_SEG, A_SEG, I_SEG]), // stressed syllable, segments are [kai]
            (UNSTRESSED, vec![K_SEG, A_SEG]),      // unstressed syllable, segments are [ka]
        ]]);

        // let replacement = PhonoPattern::new([([UNDEF], I_SEG), ([UNDEF], A_SEG)]);
        // let expected = PhonoString::new([
        //     PhonoSyllable::new(STRESSED, [I_SEG, A_SEG]), // stressed syllable, segments are [ia]
        //     PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        // ]);
        // string.clone().replace(0, 3, replacement);
        // assert_eq!(string, expected,);
    }

    #[test]
    fn string_replace_across_syl_bound() {
        // start with ['kai.ka]
        // replace segments [2,4) with [ia]
        // end with ['kaiaa]
        // syllable boundary in source is replaced

        // let string = PhonoString::new([
        //     PhonoSyllable::new(STRESSED, [K_SEG, A_SEG, I_SEG]), // stressed syllable, segments are [kai]
        //     PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        // ]);

        // let replacement = PhonoPattern::new([([UNDEF], I_SEG), ([UNDEF], A_SEG)]);
        // TODO
        // let expected = PhonoString::new([
        //     PhonoSyllable::new(STRESSED, [I_SEG, A_SEG]), // stressed syllable, segments are [ia]
        //     PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        // // ]);
        // string.clone().replace(0, 3, replacement);
        // assert_eq!(string, expected,);
    }

    #[test]
    fn string_replace_with_syl_bound() {
        // start with ['kai.ka]
        // replace segment 1 with [i.ki]
        // end with ['ki.kii.a]
        // syllable boundary in source is replaced

        // TODO
        // let string = PhonoString::new([
        //     PhonoSyllable::new(STRESSED, [K_SEG, A_SEG, I_SEG]), // stressed syllable, segments are [kai]
        //     PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        // ]);

        // let replacement = PhonoPattern::new([([UNDEF], I_SEG), ([UNDEF], A_SEG)]);
        // let expected = PhonoString::new([
        //     PhonoSyllable::new(STRESSED, [I_SEG, A_SEG]), // stressed syllable, segments are [ia]
        //     PhonoSyllable::new(UNSTRESSED, [K_SEG, A_SEG]), // unstressed syllable, segments are [ka]
        // ]);
        // string.clone().replace(0, 3, replacement);
        // assert_eq!(string, expected,);
    }

    #[test]
    fn string_is_match() {
        // let _replacement = PhonoPattern::new([([UNDEF], I_SEG), ([UNDEF], A_SEG)]);
        // let haystack = PhonologicalString::from_segments([K_SEG, A_SEG, I_SEG, K_SEG, A_SEG]);
        // let is_match = pattern.is_match(&haystack, 0) && pattern.is_match(&haystack, 3);
        // assert_eq!(
        //     is_match,
        //     true,
        // );
    }
}
