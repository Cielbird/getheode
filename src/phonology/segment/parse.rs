use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0};
use nom::combinator::{map, map_res, opt};
use nom::error::{Error, ErrorKind};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::{Err, IResult, Parser as _};
use unicode_normalization::UnicodeNormalization;

use crate::phonology::feature::{Feature, FeatureState};
use crate::phonology::segment::{
    DIACRITICS, IPA_BASES, NATURAL_CLASSES, SEG_FEATURE_NAMES, SegmentFeatures,
};

/// return a segment from either an ipa character, or a phonological class. this may be
/// followed by a feature set within brackets.
/// no whitespace allowed
pub fn parse_segment(input: &str) -> IResult<&str, SegmentFeatures> {
    let mut parser = map(
        (
            alt((parse_segment_ipa, parse_natural_class)),
            opt(delimited(tag("["), parse_segment_feature_set, tag("]"))),
        ),
        // apply feature set in brackets to the ipa symbol or class symbol
        |(mut base, features)| {
            if let Some(features) = features {
                base = base + features;
            }
            base
        },
    );

    parser.parse(input)
}

/// Parse an ipa symbol, no diacritics, no extra features
/// ex: "b"
/// should parse a bilabial voiced plosive
fn parse_ipa_base(input: &str) -> IResult<&str, SegmentFeatures> {
    let index = IPA_BASES.iter().position(|(symbol, _)| {
        // normalize unicode to NFD form !
        let input_norm = input.nfd().to_string();
        let symbol_norm = symbol.nfd().to_string();
        input_norm.starts_with(&symbol_norm)
    });
    match index {
        Some(i) => {
            let end = IPA_BASES[i].0.len();
            let ipa_base = IPA_BASES[i].1.clone();
            Ok((&input[end..], ipa_base))
        }
        None => {
            // unknown ipa base
            Err(Err::Error(Error::new(input, ErrorKind::Verify)))
        }
    }
}

/// Parse a diacritic at the beginning of `input`,
/// returning the diacritic's features with remaining input
fn parse_ipa_diacritic(input: &str) -> IResult<&str, SegmentFeatures> {
    let index = DIACRITICS.iter().position(|(symbol, _)| {
        // normalize unicode to NFD form !
        let input_norm = input.nfd().to_string();
        let symbol_norm = symbol.nfd().to_string();
        input_norm.starts_with(&symbol_norm)
    });
    match index {
        Some(i) => {
            let end = DIACRITICS[i].0.len_utf8();
            let ipa_base = DIACRITICS[i].1.clone();
            Ok((&input[end..], ipa_base))
        }
        None => {
            // unknown ipa diacritic
            Err(Err::Error(Error::new(input, ErrorKind::Verify)))
        }
    }
}

/// parse an IPA symbol, with diacritics
/// ex : "t̪"
/// see https://www.unicode.org/reports/tr15/#Canon_Compat_Equivalence
pub(crate) fn parse_segment_ipa(input: &str) -> IResult<&str, SegmentFeatures> {
    let mut parser = map(
        (parse_ipa_base, many0(parse_ipa_diacritic)),
        |(mut base, diacritics)| {
            for d in diacritics {
                base = base + d;
            }
            base
        },
    );

    parser.parse(input)
}

/// parse a natural class
pub fn parse_natural_class(class_symbol: &str) -> IResult<&str, SegmentFeatures> {
    let index = NATURAL_CLASSES.iter().position(|(symbol, _)| {
        // normalizing to NFD not necessary here...
        class_symbol.starts_with(symbol)
    });
    match index {
        Some(i) => {
            let end = NATURAL_CLASSES[i].0.len();
            let ipa_base = NATURAL_CLASSES[i].1.clone();
            Ok((&class_symbol[end..], ipa_base))
        }
        None => {
            // unknown ipa diacritic
            Err(Err::Error(Error::new(class_symbol, ErrorKind::Verify)))
        }
    }
}

/// construct a segement from a list of features in brackets
/// ex. "+voi-delrel"
/// may be preceded by whitespace
pub fn parse_segment_feature_set(s: &str) -> IResult<&str, SegmentFeatures> {
    let mut parser = separated_list1(space0, parse_segment_feature);

    let features = parser.parse(s);

    features.map(|(remaining, features)| {
        let mut combined = SegmentFeatures::new_undef();
        for f in features {
            combined = combined + f;
        }

        (remaining, combined)
    })
}

/// parse a feature name with plus or minus sign before
/// ex: "+delrel"
fn parse_segment_feature(s: &str) -> IResult<&str, SegmentFeatures> {
    let mut parser = (alt((tag("+"), tag("-"))), parse_feature_tag);

    let (remainder, (sign, feature)) = parser.parse(s)?;

    let mut seg = SegmentFeatures::new_undef();
    // set feature
    if sign == "+" {
        seg.features[feature as usize] = FeatureState::POS;
    } else if sign == "-" {
        seg.features[feature as usize] = FeatureState::NEG;
    }

    Ok((remainder, seg))
}

/// converts a feature name string to the corresponding u8 index
pub fn parse_feature_tag(string: &str) -> IResult<&str, Feature> {
    let index = SEG_FEATURE_NAMES.iter().position(|s| string.starts_with(s));
    match index {
        Some(i) => {
            let end = SEG_FEATURE_NAMES[i].len();
            Ok((&string[end..], i as u8))
        }
        None => todo!("Handle error better"), //Err(Error::UnknownFeatureName(string.to_string())),
    }
}
