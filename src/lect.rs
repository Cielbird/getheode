use crate::phonology::Phonology;
use crate::metadata::Metadata;

pub struct Lect {
    metadata: Metadata,
    phonology: Phonology,
    lexicon: Vec<String>
}
