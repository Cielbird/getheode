use cfg::{Cfg, RuleContainer};

use crate::{
    phoneme::PhonemeBank,
    phonotactics::{FormatGbnf, Phonotactics, Term, parse::parse_gbnf_production},
    segment::{FormatIpa, Segment},
};

#[test]
fn test_parse_gbnf_production() {
    let prod = "<one> ::= <two>te |<three>a";
    let mut bank = PhonemeBank::new();
    let phoneme_t = bank
        .add(Segment::parse_ipa("t").unwrap(), "t".to_string())
        .unwrap();
    let phoneme_e = bank
        .add(Segment::parse_ipa("e").unwrap(), "e".to_string())
        .unwrap();
    let phoneme_a = bank
        .add(Segment::parse_ipa("a").unwrap(), "a".to_string())
        .unwrap();

    let res = parse_gbnf_production(&bank, prod);

    let (lhs, rhs) = res.expect("Parse GBNF failed");
    assert_eq!(lhs, "one".to_string());

    let expected_rhs = vec![
        vec![
            Term::NonTerminal("two".to_string()),
            Term::Terminal(phoneme_t),
            Term::Terminal(phoneme_e),
        ],
        vec![
            Term::NonTerminal("three".to_string()),
            Term::Terminal(phoneme_a),
        ],
    ];
    assert_eq!(rhs, expected_rhs);
}

#[test]
fn test_from_gbnf() {
    let mut bank = PhonemeBank::new();
    let phoneme_t = bank
        .add(Segment::parse_ipa("t").unwrap(), "t".to_string())
        .unwrap();
    let phoneme_e = bank
        .add(Segment::parse_ipa("e").unwrap(), "e".to_string())
        .unwrap();
    let phoneme_a = bank
        .add(Segment::parse_ipa("a").unwrap(), "a".to_string())
        .unwrap();

    let contents = "
        <one> ::= <two>te |<three>a\n
        <two> ::= <three>ee\n
        <four> ::= <four> |<one>t\n
    "
    .to_string();

    let phonotactics = Phonotactics::parse_gbnf(&bank, &contents).expect("GBNF Parsing failed");

    // since our assert_cfg_eq is naive, order matters in symbol generation.
    let mut expected_cfg = Cfg::new();
    let [sym_one, sym_two, sym_t, sym_e, sym_three, sym_a] = expected_cfg.sym();
    let rule = expected_cfg.rule(sym_one);
    rule.rhs(&[sym_two, sym_t, sym_e]).rhs(&[sym_three, sym_a]);
    let rule = expected_cfg.rule(sym_two);
    rule.rhs(&[sym_three, sym_e, sym_e]);
    let [sym_four] = expected_cfg.sym();
    let rule = expected_cfg.rule(sym_four);
    rule.rhs(&[sym_four]).rhs(&[sym_one, sym_t]);

    assert_cfg_eq(&phonotactics.grammar, &expected_cfg);

    if let Term::Terminal(actual_a) = phonotactics.terms.get(&sym_a).unwrap() {
        assert_eq!(*actual_a, phoneme_a);
    } else {
        panic!("Term a was the wrong type!");
    };
    if let Term::Terminal(actual_e) = phonotactics.terms.get(&sym_e).unwrap() {
        assert_eq!(*actual_e, phoneme_e);
    } else {
        panic!("Term e was the wrong type!");
    };
    if let Term::Terminal(actual_t) = phonotactics.terms.get(&sym_t).unwrap() {
        assert_eq!(*actual_t, phoneme_t);
    } else {
        panic!("Term t was the wrong type!");
    };
    if let Term::NonTerminal(one_str) = phonotactics.terms.get(&sym_one).unwrap() {
        assert_eq!(one_str, "one");
    } else {
        panic!("Term one was the wrong type!");
    };
    if let Term::NonTerminal(two_str) = phonotactics.terms.get(&sym_two).unwrap() {
        assert_eq!(two_str, "two");
    } else {
        panic!("Term two was the wrong type!");
    };
    if let Term::NonTerminal(three_str) = phonotactics.terms.get(&sym_three).unwrap() {
        assert_eq!(three_str, "three");
    } else {
        panic!("Term three was the wrong type!");
    };
    if let Term::NonTerminal(four_str) = phonotactics.terms.get(&sym_four).unwrap() {
        assert_eq!(four_str, "four");
    } else {
        panic!("Term four was the wrong type!");
    };
}

/// Assert the grammars are equal. The generation of the symbols and rules must have been done
/// in the same order.
pub fn assert_cfg_eq(lhs: &Cfg, rhs: &Cfg) {
    let rules_lhs = lhs
        .rules()
        .map(|rule| (rule.lhs, rule.rhs.to_vec()))
        .collect::<Vec<_>>();

    let rules_rhs = rhs
        .rules()
        .map(|rule| (rule.lhs, rule.rhs.to_vec()))
        .collect::<Vec<_>>();

    assert_eq!(rules_lhs, rules_rhs);
}
