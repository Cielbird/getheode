use crate::feature::FeatureState::{NA, NEG, POS};
use crate::segment::Segment;

// syl,stress,long,cons,son,cont,delrel,approx,tap,trill,nasal,voi,spgl,congl,lab,round,labdent,cor,ant,dist,strident,lateral,dor,high,low,front,back,tense

// segments that represent common ipa symbols
#[rustfmt::skip]
pub const IPA_BASES: &[(&str, Segment)] = &[
	("ɒ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,POS,NA])
	),
	("ɑ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,POS,NA])
	),
	("ɶ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,POS,NEG,NA])
	),
	("a", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,NEG,NA])
	),
	("æ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,POS,NEG,NA])
	),
	("ʌ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NEG])
	),
	("ɔ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NEG])
	),
	("o", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,POS])
	),
	("ɤ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,POS])
	),
	("ɘ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,NEG,POS])
	),
	("œ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,POS,NEG,NEG])
	),
	("ə", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,NEG,NEG])
	),
	("e", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,POS,NEG,POS])
	),
	("ɞ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,NEG,NEG])
	),
	("ø", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,POS,NEG,POS])
	),
	("ɛ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,POS,NEG,NEG])
	),
	("ɵ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,NEG,POS])
	),
	("ɯ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,POS])
	),
	("u", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,POS])
	),
	("ʊ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,NEG])
	),
	("ɨ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,NEG,POS])
	),
	("ʉ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,NEG,POS])
	),
	("y", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,POS])
	),
	("i", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,POS])
	),
	("ʏ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,NEG])
	),
	("ɪ", Segment::from_features(
		[POS,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,NEG])
	),
	("ɫ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,POS,NEG,NEG,NEG,POS,NA])
	),
	("ɴ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("ʀ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("ɲ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ʎ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,POS,POS,POS,NEG,POS,NEG,NA])
	),
	("ŋ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("ŋ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,NA])
	),
	("ʟ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,POS,POS,POS,NEG,NA,NA,NA])
	),
	("ʟ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,POS,POS,POS,NEG,NEG,POS,NA])
	),
	("ɳ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʙ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,POS,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɭ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("ɺ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("ɻ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɽ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("r", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("n", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("m", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("l", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("ɾ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,POS,NEG,POS,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɱ", Segment::from_features(
		[NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,POS,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʔ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ħ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,POS,NA])
	),
	("ʕ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,POS,NEG,POS,NA])
	),
	("ʁ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("q", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("χ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("ɢ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("ɕ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,POS,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ɟ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ʝ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("c", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ç", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("d͡ʑ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,POS,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("t͡ɕ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,POS,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ɣ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("ɣ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,NA])
	),
	("x", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("k", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("ɡ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("g", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("ɡ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,NA])
	),
	("ʑ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,POS,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ʈ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɖ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɬ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("ʐ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɸ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʂ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʒ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("z", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("v", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,POS,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("t", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʃ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("s", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("p", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("f", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("d", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("b", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("θ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɮ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("ð", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("β", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("d͡ʒ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("d͡z", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("d͡ɮ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("d̠͡ɮ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("t͡ʃ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("t̠͡ɬ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("t͡s", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("t͡ɬ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("t̪͡s̪", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("t̪͡ɬ̪", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("d̪͡z̪", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("d̪͡ɮ̪", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,NEG,POS,NEG,NA,NA,NA,NA,NA])
	),
	("ʈ͡ʂ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɖ͡ʐ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("p͡f", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("b͡v", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,POS,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("p͡ɸ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("b͡β", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("t̪͡θ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,POS,POS,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("c͡ç", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("ɟ͡ʝ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NA])
	),
	("k͡x", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("k̠͡x̠", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,NA])
	),
	("ɡ͡ɣ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("ɡ̠͡ɣ̠", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,NA])
	),
	("q͡χ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("ɢ͡ʁ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,NEG,NEG,NEG,POS,NA])
	),
	("ɧ", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("k͡p", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("ɡ͡b", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,NA])
	),
	("p͡t", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("b͡d", Segment::from_features(
		[NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,POS,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ɰ", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NA,NA,POS])
	),
	("ɰ̠", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,POS])
	),
	("w", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,POS])
	),
	("ɥ", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,POS])
	),
	("j", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,POS,NEG,POS])
	),
	("ɹ", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,NEG,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʋ", Segment::from_features(
		[NEG,NEG,NEG,NEG,POS,POS,NA,POS,NEG,NEG,NEG,POS,NEG,NEG,POS,NEG,POS,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("ʍ", Segment::from_features(
		[NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,POS,POS,NEG,NEG,NA,NA,NA,NEG,POS,POS,NEG,NEG,POS,POS])
	),
	("ɦ", Segment::from_features(
		[NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,POS,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	),
	("h", Segment::from_features(
		[NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NEG,POS,NEG,NEG,NEG,NEG,NEG,NA,NA,NA,NEG,NEG,NA,NA,NA,NA,NA])
	)
];
