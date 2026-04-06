use std::ops::Range;

use crate::phonology::string::PhonoString;

pub struct RuleMatch {
    pub range: Range<usize>,
    pub replace_with: PhonoString,
}
