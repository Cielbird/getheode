use std::{iter::zip, vec};

use crate::phonology::rule::parse::{
    elem::{ElementSequence, RuleElements},
    node::Node,
    parse_elem::parse_rule_elems,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern<'a> {
    pub(crate) root: Node<'a>,
}

impl<'a> Pattern<'a> {
    /// Single Null node
    pub fn null() -> Self {
        Self { root: Node::Null }
    }

    /// Single leaf node
    pub fn leaf(elem: &'a str) -> Self {
        Self {
            root: Node::Leaf(elem),
        }
    }

    /// Ordered sequence of trees under a Sequence root
    pub fn sequence(children: Vec<Self>) -> Self {
        let children = children.into_iter().map(|x| x.root).collect();
        Self {
            root: Node::Sequence(children),
        }
    }

    /// Unordered alternatives under a Branch root
    pub fn branch(children: Vec<Self>) -> Self {
        let children = children.into_iter().map(|x| x.root).collect();
        Self {
            root: Node::Branch(children),
        }
    }

    /// optional is just branch with a null
    pub fn optional(child: Self) -> Self {
        Self {
            root: Node::Branch(vec![child.root, Node::Null]),
        }
    }

    /// lists all possible element sequences, taking each branch combination.
    pub fn enumerate_branches(self) -> Vec<String> {
        let mut results: Vec<String> = vec!["".to_string()]; // start with one empty path

        self.root.collect_element_sequences(&mut results);

        results
    }
}

/// rule, with branching parsed
pub struct RulePatterns<'a> {
    pub(crate) input: Vec<Pattern<'a>>,
    pub(crate) output: Vec<&'a str>, // no branching in the output : deterministic
    pub(crate) pre_context: Option<Pattern<'a>>,
    pub(crate) post_context: Option<Pattern<'a>>,
}

/// a rule, no branching: input, output and context. unparsed elements.
#[derive(Debug, PartialEq)]
pub struct RuleStrings {
    pub(crate) input: String,
    pub(crate) output: String,
    pub(crate) pre_context: String,
    pub(crate) post_context: String,
}

impl RulePatterns<'_> {
    pub fn enumerate(self) -> Vec<RuleStrings> {
        let mut input_vec = vec![];
        let mut output_vec = vec![];
        let mut pre_context_opts = vec!["".to_string()];
        let mut post_context_opts = vec!["".to_string()];

        for input in self.input {
            input_vec.push(input.enumerate_branches());
        }
        for output in self.output {
            output_vec.push(output.to_string());
        }
        if let Some(pre) = self.pre_context {
            pre_context_opts = pre.enumerate_branches();
        }
        if let Some(post) = self.post_context {
            post_context_opts = post.enumerate_branches();
        }

        let mut strings = vec![];
        for (input_ops, output) in zip(input_vec, output_vec) {
            for input in input_ops {
                for pre_context in &pre_context_opts {
                    for post_context in &post_context_opts {
                        strings.push(RuleStrings {
                            input: input.clone(),
                            output: output.clone(),
                            pre_context: pre_context.clone(),
                            post_context: post_context.clone(),
                        });
                    }
                }
            }
        }

        strings
    }
}
