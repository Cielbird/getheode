// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::{gbnf::{grammar::Grammar, production::Production, term::Term}, segment_string::SegmentString};

    #[test]
    fn test_parse_two_expression_production() {
        let prod_str = "<rule> ::= <expression> | <another_expression>";
        let production = Production::from_string(prod_str).expect("Failed to parse BNF");

        assert_eq!(production.lhs, "rule");
        assert_eq!(production.rhs.len(), 2);
    }

    #[test]
    fn test_parse_terminal() {
        let prod_str = "<expression> ::= [terminal]";
        let production = Production::from_string(prod_str).expect("Failed to parse BNF");

        assert_eq!(production.lhs, "expression");
        assert_eq!(production.rhs.len(), 1);
        let terms = &production.rhs[0].terms;
        assert_eq!(terms.len(), 1);
        match &terms[0] {
            Term::Terminal(segment_string) => {
                assert_eq!(*segment_string, SegmentString::new("terminal").unwrap());
            }
            _ => panic!("Expected Terminal"),
        }
    }

    #[test]
    fn test_parse_non_terminal() {
        let prod_str = "<expression> ::= <non_terminal>";
        let production = Production::from_string(prod_str).expect("Failed to parse BNF");

        let terms = &production.rhs[0].terms;
        assert_eq!(terms.len(), 1);
        match &terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "non_terminal");
            }
            _ => panic!("Expected NonTerminal"),
        }
    }

    #[test]
    fn test_parse_two_term_expression() {
        let prod_str = "<expression> ::= <one><two>";
        let production = Production::from_string(prod_str).expect("Failed to parse BNF");

        let terms = &production.rhs[0].terms;
        assert_eq!(terms.len(), 2);
        match &terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "one");
            }
            _ => panic!("Expected NonTerminal"),
        }
        match &terms[1] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "two");
            }
            _ => panic!("Expected NonTerminal"),
        }
    }

    #[test]
    fn test_parse_mixed_expression() {
        let prod_str = "<expression> ::= <one>[two]";
        let production = Production::from_string(prod_str).expect("Failed to parse BNF");

        let terms = &production.rhs[0].terms;
        assert_eq!(terms.len(), 2);
        match &terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "one");
            }
            _ => panic!("Expected NonTerminal"),
        }
        match &terms[1] {
            Term::Terminal(segment_string) => {
                assert_eq!(*segment_string, SegmentString::new("two").unwrap());
            }
            _ => panic!("Expected Terminal"),
        }
    }

    #[test]
    fn test_invalid_format_bnf() {
        let prod_str = "<rule> = <term>";
        let result = Production::from_string(prod_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_non_terminal_bnf() {
        let prod_str = "<rule> ::= <ex<another_expression>pression>";
        let result = Production::from_string(prod_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_terminal_bnf() {
        let prod_str = "<rule> ::= [a]]";
        let result = Production::from_string(prod_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_expression() {
        let bnf = "<expression> ::= <one> | []";
        let production = Production::from_string(bnf).expect("Failed to parse BNF");

        let rhs = &production.rhs;
        assert_eq!(rhs.len(), 2);
        match &rhs[0].terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "one");
            }
            _ => panic!("Expected NonTerminal"),
        }
        match &rhs[1].terms[0] {
            Term::Terminal(segment_string) => {
                assert_eq!(*segment_string, SegmentString::new("").unwrap());
            }
            _ => panic!("Expected Terminal"),
        }
    }
}
