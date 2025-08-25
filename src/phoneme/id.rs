pub type PhonemeIdRepr = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhonemeId(u64);

impl From<u64> for PhonemeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
