use crate::phonology::feature::FeatureState::{NEG, POS, UNDEF};
use crate::phonology::segment::SegmentFeatures;

// syl,long,cons,son,cont,delrel,approx,tap,trill,nasal,voi,spgl,congl,lab,round,labdent,cor,ant,dist,strident,lateral,dor,high,low,front,back,tense

// segments that represent common ipa symbols
#[rustfmt::skip]
pub const NATURAL_CLASSES: &[(&str, SegmentFeatures)] = &[
    // consonants
  ("C", SegmentFeatures::from_features(
    [NEG,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
  ),
    // vowels
  ("V", SegmentFeatures::from_features(
    [POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
  ),
    // syllabant : [+cor+strident]
  ("S", SegmentFeatures::from_features(
    [UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,POS,UNDEF,UNDEF,POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
  ),
    // fricative : [-son+cont]
  ("F", SegmentFeatures::from_features(
    [UNDEF,UNDEF,UNDEF,NEG,POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
  ),
];
