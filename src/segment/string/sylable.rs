#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sylable {
    pub(crate) stressed: bool, // could have more modifiers
}

impl Default for Sylable {
    fn default() -> Self {
        Self { stressed: false }
    }
}
