use crate::phonology::segment::SegmentFeatures;

use crate::phonology::feature::FeatureState::*;

// syl,   	long,	cons,	son,	cont,	delrel,
// approx,	tap,	trill,	nasal,	voi,	spgl,	congl,
// lab,		round,	labdent,cor,	ant,	dist,	strident,
// lateral,	dor,	high,	low,	front,	back,	tense

// segments that represent common ipa symbols
#[rustfmt::skip]
pub const DIACRITICS: &[(char, SegmentFeatures)] = &[
    // combining tilde: nasalization
	('\u{0303}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	// triangular colon: long
	('\u{02D0}', SegmentFeatures::from_features(
		[UNDEF,POS,  UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	// combining down tack below: lowered
	('\u{031E}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF])
	),
	// modifier down tack: lowered
	('\u{02D5}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF])
	),
	// labialization
	('\u{02B7}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 POS,  POS  ,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	// dental : COMBINING BRIDGE BELOW
	// dental diacritic (t̪) indicates [+anterior, +distributed] within the coronal node [+cor]
	('\u{032A}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS  ,POS  ,POS  ,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	// ejective: ʼ
	// features: -spgl, -son, -voi, +cons
	('\u{02BC}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,POS  ,NEG  ,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,NEG  ,NEG  ,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF]
	)),
	// palatalization : ʲ (U+02B2. MODIFIER LETTER SMALL J)
	// features: [+high+front-low-back]
	('\u{02B2}', SegmentFeatures::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,POS  ,NEG  ,POS  ,NEG  ,UNDEF]
	)),
	// length : ː (U+02D0, MODIFIER LETTER TRIANGULAR COLON)
	// features: [+long]
	('\u{02D0}', SegmentFeatures::from_features(
		[UNDEF,POS  ,UNDEF,UNDEF,UNDEF,UNDEF,
		UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF]
	)),
];
