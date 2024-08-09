use std::string;

use unicode_normalization::UnicodeNormalization;


fn main() {
    let string_a = "\u{00E9}";
    let string_b = "e\u{0301}";
    let string_c = "u̅";

    println!("{:?}", string_c.chars().collect::<String>().bytes());

    let modded_a = string_a.nfd().collect::<String>();
    let modded_b = string_b.nfd().collect::<String>();

    println!("{:?}", modded_a.chars());
    assert_eq!(modded_a, modded_b);
}
