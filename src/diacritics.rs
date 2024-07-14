use crate::segment::Segment;
use crate::feature::FeatureState::{UNDEF, POS, NEG};

// syl,stress,long,cons,son,cont,delrel,approx,tap,trill,nasal,voi,spgl,congl,lab,round,labdent,cor,ant,dist,strident,lateral,dor,high,low,front,back,tense

// segments that represent common ipa symbols
pub const DIACRITICS: &[(char, Segment)] = &[
    // combining tilde
	('\u{0303}', Segment::from_features(
		[UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
	('\u{02D0}', Segment::from_features(
		[UNDEF,UNDEF,POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
	),
];
