// example script to show the usage of segment strings and applying rules to them

// import getheode
extern crate getheode;

use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;

fn main() {
    let mut segstr = SegmentString::from_string("estrictus").unwrap();
    println!("{}", segstr); 

    let rule = PhonologicalRule::new("us -> o").unwrap();
    println!("{}", rule);
    segstr = rule.apply_rule(&segstr).unwrap();
    println!("{}", segstr); 

    let rule = PhonologicalRule::new("ict -> et͡ʃ").unwrap();
    println!("{}", rule);
    segstr = rule.apply_rule(&segstr).unwrap();
    println!("{}", segstr);
}

