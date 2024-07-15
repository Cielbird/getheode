// example script to show the usage of segment strings and applying rules to them

// import getheode
extern crate getheode;

use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;

fn main() {
    let mut segstr = SegmentString::new_worded("strictus").unwrap();
    println!("{}", segstr); 

    let rule = PhonologicalRule::new("s -> es / #_").unwrap();
    println!("{}", rule);
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr); 

    let rule = PhonologicalRule::new("us -> o / _").unwrap();
    println!("{}", rule);
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr); 

    let rule = PhonologicalRule::new("ict -> et͡ʃ / _").unwrap();
    println!("{}", rule);
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr);

    let rule = PhonologicalRule::new("o -> / _#").unwrap();
    println!("{}", rule);
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr);
}

