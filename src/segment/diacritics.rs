use crate::segment::{
    FeatureState::{POS, UNDEF},
    Segment,
};

// syl,   	stress,	long,	cons,	son,	cont,	delrel,
// approx,	tap,	trill,	nasal,	voi,	spgl,	congl,
// lab,		round,	labdent,cor,	ant,	dist,	strident,
// lateral,	dor,	high,	low,	front,	back,	tense

// segments that represent common ipa symbols
#[rustfmt::skip]
pub const DIACRITICS: &[(char, Segment)] = &[
    // combining tilde: nasalization
	('\u{0303}', Segment::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	// triangular colon: long
	('\u{02D0}', Segment::from_features(
		[UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	// combining down tack below: lowered
	('\u{031E}', Segment::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF])
	),
	// modifier down tack: lowered
	('\u{02D5}', Segment::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,POS,  UNDEF,UNDEF,UNDEF])
	),
	// labialization
	('\u{02B7}', Segment::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 POS,  POS  ,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,
		 UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
];
