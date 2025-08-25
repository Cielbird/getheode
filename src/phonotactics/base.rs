use std::collections::HashMap;

use cfg::{self, Cfg, RuleContainer};

use crate::phoneme::PhonemeId;

pub struct Phonotactics {
    pub(crate) grammar: Cfg,
    pub(crate) terms: HashMap<cfg::Symbol, Term>,
}

impl Phonotactics {
    pub fn new() -> Self {
        Self {
            grammar: Cfg::new(),
            terms: HashMap::new(),
        }
    }

    pub fn add_production(&mut self, lhs: String, rhs: Vec<Vec<Term>>) {
        let lhs = Term::NonTerminal(lhs);
        let lhs_sym = self.get_term_symbol(lhs);
        let mut rhs_syms = vec![];
        for terms in rhs {
            let mut syms = vec![];
            for term in terms {
                syms.push(self.get_term_symbol(term));
            }
            rhs_syms.push(syms);
        }

        let mut builder = self.grammar.rule(lhs_sym);
        for syms in rhs_syms {
            builder = builder.rhs(syms);
        }
    }

    /// Find the cfg symbol for a term with a given name. Otherwise,
    /// create a new symbol and add the term
    fn get_term_symbol(&mut self, term: Term) -> cfg::Symbol {
        if let Some((sym, _)) = self.terms.iter().filter(|(_, t)| *t == &term).next() {
            // term was already defined.
            *sym
        } else {
            // not yet defined, create it here
            let [sym] = self.grammar.sym();
            self.terms.insert(sym, term);

            sym
        }
    }
}

/// a Term can represent a Terminal or NonTerminal node
/// a Terminal node is a segment string used in the syntax.
/// a NonTerminal node is used to represent an intermediate symbol, used as lhs of a production
#[derive(Clone, Debug, PartialEq)] //Deserialize, Serialize,
pub enum Term {
    /// A term which cannot be expanded further via productions
    Terminal(PhonemeId),
    /// A term which may be be expanded further via productions
    NonTerminal(String),
}
