use crate::string::{pattern::PhonoPattern, syllable::PhonoSyllable};

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoString {
    pub(crate) syllables: Vec<PhonoSyllable>,
}

impl PhonoString {
    /// construct a string from syllables
    pub fn new(syllables: impl IntoIterator<Item = impl Into<PhonoSyllable>>) -> Self {
        PhonoString {
            syllables: syllables.into_iter().map(|s| s.into()).collect(),
        }
    }

    pub(crate) fn replace(self, start: usize, end: usize, replacement: PhonoPattern) -> Self {
        // TODO
        self
    }
}
