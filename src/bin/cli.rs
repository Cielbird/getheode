/// this is the cli tool to interact with the getheode library most directly

use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;

fn main() {
    // sample on how to use the lib
    let mut segstr = SegmentString::from_string("atak").unwrap();
    println!("{}", segstr); 

    let rule = PhonologicalRule::new("t ->[+voi]/ a_a").unwrap();
    println!("{}", rule); 

    segstr = rule.apply_rule(&segstr).unwrap();
    println!("{}", segstr); 
}
