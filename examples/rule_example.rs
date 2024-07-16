// example script to show the usage of segment strings and applying rules to them

// import getheode
extern crate getheode;

use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;

fn main() {
    let mut segstr = SegmentString::new_worded("strictus").unwrap();
    println!("{}", segstr);

    let rule = PhonologicalRule::new("s -> es / #_").unwrap();
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr);

    let rule = PhonologicalRule::new("us -> o").unwrap();
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr);

    let rule = PhonologicalRule::new("i -> e / C_C").unwrap();
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr);

    let rule = PhonologicalRule::new("ct -> t͡ʃ").unwrap();
    segstr = rule.apply(&segstr).unwrap();
    println!("{}", segstr);
}
