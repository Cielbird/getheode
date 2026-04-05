use std::ops::Range;

use crate::phonology::string::PhonoString;

pub struct PatternMatch {
    pub range: Range<usize>,
    pub replace_with: PhonoString
}
