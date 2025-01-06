// tests/getheode_test.rs

extern crate getheode;

#[cfg(test)]
mod tests {
    use getheode::{gbnf::{parser::*, term::Term}, segment_string::SegmentString};

    #[test]
    fn test_parse_two_expression_production() {
        let bnf = "<rule> ::= <expression> | <another_expression>";
        let grammar = parse_bnf(bnf).expect("Failed to parse BNF");

        assert_eq!(grammar.productions.len(), 1);
        assert_eq!(grammar.productions[0].lhs, "<rule>");
        assert_eq!(grammar.productions[0].rhs.len(), 2);
    }

    #[test]
    fn test_parse_terminal() {
        let bnf = "<expression> ::= [terminal]";
        let grammar = parse_bnf(bnf).expect("Failed to parse BNF");

        assert_eq!(grammar.productions.len(), 1);
        let terms = &grammar.productions[0].rhs[0].terms;
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
        let bnf = "<expression> ::= <non_terminal>";
        let grammar = parse_bnf(bnf).expect("Failed to parse BNF");

        let terms = &grammar.productions[0].rhs[0].terms;
        assert_eq!(terms.len(), 1);
        match &terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "non_terminal");
            }
            _ => panic!("Expected NonTerminal"),
        }
    }

    #[test]
    fn test_parse_expression() {
        let bnf = "<expression> ::= <one><two>";
        let grammar = parse_bnf(bnf).expect("Failed to parse BNF");

        let terms = &grammar.productions[0].rhs[0].terms;
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
        let bnf = "<expression> ::= <one>[two]";
        let grammar = parse_bnf(bnf).expect("Failed to parse BNF");

        let terms = &grammar.productions[0].rhs[0].terms;
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
    fn test_invalid_non_terminal_bnf() {
        let bnf = "<rule> ::= <ex<another_expression>pression>";
        let result = parse_bnf(bnf);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_terminal_bnf() {
        let bnf = "<rule> ::= [a]]";
        let result = parse_bnf(bnf);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_two_productions() {
        let bnf = "<expression> ::= <one>\n<one> ::= <alpha> <beta>";
        let grammar = parse_bnf(bnf).expect("Failed to parse BNF");

        let first_terms = &grammar.productions[0].rhs[0].terms;
        assert_eq!(first_terms.len(), 1);
        match &first_terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "one");
            }
            _ => panic!("Expected NonTerminal"),
        }
        let second_terms = &grammar.productions[1].rhs[0].terms;
        assert_eq!(second_terms.len(), 2);
        match &second_terms[0] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "alpha");
            }
            _ => panic!("Expected NonTerminal"),
        }
        match &second_terms[1] {
            Term::NonTerminal(name) => {
                assert_eq!(name, "beta");
            }
            _ => panic!("Expected NonTerminal"),
        }
    }
}
