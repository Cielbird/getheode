mod feature;
mod lect;
mod phonological_rule;
mod metadata;
mod phonology;
mod segment_string;
mod segment;
mod ipa_segments;
mod errors;

use phonological_rule::PhonologicalRule;
use segment::Segment;
use segment_string::SegmentString;

fn main() {
    // sample on how to use segments
    //let my_segment = Segment::from_string("[-voi+front-delrel]").unwrap() + feature::VOI;
    //println!("{}", my_segment.to_string());
    let my_str = SegmentString::from_string("atak").unwrap();
    //println!("{}", my_str.to_string());
    let rule;
    match PhonologicalRule::new("t   ->[+voi]/ a  _ a") {
        Ok(r) => { println!("{}", r); rule = r },
        Err(e) => { println!("{}", e); return }
    }
    match rule.apply_rule(&my_str) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    }
}
