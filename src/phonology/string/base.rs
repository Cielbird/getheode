use std::{fmt, ops::Range};

use nom::{IResult, Parser};

use crate::phonology::{
    rule::{compile_untagged_elements, parse_rule_elems}, segment::{SegmentFeatures, format_segment}, syllable::SyllableFeatures, tree::Depth3Tree
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoString {
    pub tree: Depth3Tree<(), SyllableFeatures, SegmentFeatures>,
}

impl PhonoString {
    pub fn new(tree: Depth3Tree<(), SyllableFeatures, SegmentFeatures>) -> Self {
        Self { tree }
    }

    pub fn replace_range(
        mut self,
        range: Range<usize>,
        replace_with: PhonoString,
    ) -> Result<Self, String> {
        self.tree = self.tree.replace_range(range, replace_with.tree)?;

        Ok(self)
    }

    /// Parse a phonological string
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut parser = parse_rule_elems;

        let (remainder, elements) = parser.parse(input)?;

        // TODO manage this error better
        let string = compile_untagged_elements(elements).unwrap();

        Ok((remainder, string))
    }

    pub fn format(&self) -> String {
        self.tree
            .layer_2()
            .iter()
            .map(|(s, _)| format_segment(s))
            .collect()
    }
}

impl fmt::Display for PhonoString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (seg, _) in self.tree.layer_2() {
            write!(f, "{}", format_segment(seg))?;
        }
        Ok(())
    }
}
