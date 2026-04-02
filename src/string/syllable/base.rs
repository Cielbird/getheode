use crate::string::segment::PhonoSegment;
use crate::string::syllable::SyllableFeatures;

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoSyllable {
    features: SyllableFeatures,
    segments: Vec<PhonoSegment>,
}

impl PhonoSyllable {
    /// construct a segement from an array of features
    pub fn new(
        features: impl Into<SyllableFeatures>,
        segments: impl IntoIterator<Item = impl Into<PhonoSegment>>,
    ) -> Self {
        PhonoSyllable {
            features: features.into(),
            segments: segments.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl<F, S> From<(F, S)> for PhonoSyllable
where
    F: Into<SyllableFeatures>,
    S: IntoIterator<Item: Into<PhonoSegment>>,
{
    fn from(tup: (F, S)) -> Self {
        let (features, segments) = tup;

        let features = features.into();
        let segments = segments.into_iter().map(|s| s.into()).collect();

        PhonoSyllable { features, segments }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::string::feature::FeatureState::*;

    const A_SEG: PhonoSegment = PhonoSegment::from_features([
        POS, NEG, NEG, POS, POS, NEG, POS, NEG, NEG, NEG, POS, NEG, NEG, NEG, NEG, NEG, NEG, NA,
        NA, NA, NEG, POS, NEG, POS, NEG, NEG, NA,
    ]);

    #[test]
    fn test_new() {
        let x = PhonoSyllable::new([POS], [A_SEG, A_SEG, A_SEG]);

        assert_eq!(x.features, [POS].into());
        assert_eq!(x.segments, vec![A_SEG, A_SEG, A_SEG]);
    }

    #[test]
    fn test_tuple_into_syllable() {
        let x: PhonoSyllable = ([POS], [A_SEG, A_SEG, A_SEG]).into();

        assert_eq!(x.features, [POS].into());
        assert_eq!(x.segments, vec![A_SEG, A_SEG, A_SEG]);
    }
}
