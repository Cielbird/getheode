// example script to show the usage of segment strings and applying rules to them

// import getheode
extern crate getheode;

use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;

fn main() {
    // make a segment string
    let mut segstr = SegmentString::from_string("vreitamanon").unwrap();
    println!("{}", segstr); 

    // define a rule
    let rule = PhonologicalRule::new("t -> t͡ʃ / [+high]_").unwrap();
    println!("{}", rule); 

    // apply the rule
    segstr = rule.apply_rule(&segstr).unwrap();
    println!("{}", segstr); 

    // another rule
    let rule = PhonologicalRule::new("V -> [+nasal] / [+nasal]_").unwrap();
    println!("{}", rule);

    // apply the rule
    segstr = rule.apply_rule(&segstr).unwrap();
    println!("{}", segstr); 
}

