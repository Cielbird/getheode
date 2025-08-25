use crate::segment::FeatureState::{NEG, POS, UNDEF};
use crate::segment::Segment;

// syl,stress,long,cons,son,cont,delrel,approx,tap,trill,nasal,voi,spgl,congl,lab,round,labdent,cor,ant,dist,strident,lateral,dor,high,low,front,back,tense

// segments that represent common ipa symbols
#[rustfmt::skip]
pub const NATURAL_CLASSES: &[(&str, Segment)] = &[
    // consonants
  ("C", Segment::from_features(
    [NEG,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
  ),
    // vowels
  ("V", Segment::from_features(
    [POS,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF])
  ),
];
