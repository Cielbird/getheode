use crate::phonology::rule::parse::node::Node;

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
pub struct RuleStrings {
    pub(crate) input: Vec<Vec<String>>,
    pub(crate) output: Vec<String>,
    pub(crate) pre_context: Vec<String>,
    pub(crate) post_context: Vec<String>,
}

impl RulePatterns<'_> {
    pub fn enumerate(self) -> RuleStrings {
        let mut strings = RuleStrings {
            input: vec![],
            output: vec![],
            pre_context: vec![],
            post_context: vec![],
        };
        for input in self.input {
            strings.input.push(input.enumerate_branches());
        }
        for output in self.output {
            strings.output.push(output.to_string());
        }
        if let Some(pre) = self.pre_context {
            strings.pre_context = pre.enumerate_branches();
        }
        if let Some(post) = self.post_context {
            strings.post_context = post.enumerate_branches();
        }
        
        strings
    }
}


