use std::{fmt, ops::Range};

use nom::{IResult, Parser};

use crate::error::*;
use crate::phonology::{
    feature::FeatureState,
    rule::{compile_untagged_elements, parse_rule_elems},
    segment::{SegmentFeatures, format_segment},
    syllable::SyllableFeatures,
    tree::Depth3Tree,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoString {
    pub tree: Depth3Tree<(), SyllableFeatures, SegmentFeatures>,
}

impl PhonoString {
    pub fn new(tree: Depth3Tree<(), SyllableFeatures, SegmentFeatures>) -> Self {
        Self { tree }
    }

    pub fn replace_range(mut self, range: Range<usize>, replace_with: PhonoString) -> Result<Self> {
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
        // TODO add format config options
        let mut output = "".to_string();
        let mut is_first_word = true;
        for (_, syls) in self.tree.iter() {
            if is_first_word {
                is_first_word = false;
            } else {
                output.push('#');
            }

            let mut is_first_syl = true;
            for (syl, segs) in syls {
                if is_first_syl {
                    is_first_syl = false;
                } else {
                    if syl.features[0] == FeatureState::POS {
                        output.push('\'');
                    } else {
                        output.push('.');
                    }
                }

                for seg in segs {
                    output.push_str(&format_segment(seg));
                }
            }
        }

        output
    }
}

impl fmt::Display for PhonoString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}
