

/// Phonological Rule Parse Node
/// Nodes that make the rule parse tree
#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
    Sequence(Vec<Node<'a>>),
    Leaf(&'a str),
    Branch(Vec<Node<'a>>),
    Null,
}

impl Node<'_> {
    /// lists all possible element sequences, taking each branch combination.
    pub fn collect_element_sequences(&self, paths: &mut Vec<String>) {
        match self {
            Node::Sequence(nodes) => {
                for child in nodes {
                    child.collect_element_sequences(paths);
                }
            },
            Node::Leaf(s) => {
                for path in paths {
                    path.push_str(s);
                }
            },
            Node::Branch(nodes) => {
                let mut sub_paths = vec![];
                for choice in nodes {
                    let mut sub = vec!["".to_string()];
                    choice.collect_element_sequences(&mut sub);
                    sub_paths.append(&mut sub);
                }
                
                let mut new_paths = vec![];
                for path in paths.drain(..) {
                    for sub in &sub_paths {
                        let mut path = path.clone();
                        path.push_str(sub);
                        new_paths.push(path);
                    }
                }
                *paths = new_paths;
            },
            Node::Null => {
                // do nothing
            },
        }
    }
}

