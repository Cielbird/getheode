/// module file for getheode's Backus–Naur form: gbnf
/// gbnf is a syntax for defining the phonotactic struction of a lect

pub mod expression;
pub mod grammar;
pub mod production;
pub mod term;
pub mod parser;

// Example public function in `bnf` module
pub fn describe_bnf() {
    println!("This is the BNF sub-library of getheode!");
}
