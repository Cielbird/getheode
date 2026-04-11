use indextree::{Arena, NodeId};

#[derive(Debug, Clone)]
pub struct Pattern<'a> {
    pub(crate) tree: Arena<Node<'a>>,
    pub(crate) root: NodeId,
}

/// Phonological Rule Parse Node
/// Nodes that make the rule parse tree
#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
    Sequence,
    Leaf(&'a str),
    Branch,
    Null,
}


impl<'a> Pattern<'a> {
    /// Single Null node
    pub fn null() -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(Node::Null);
        Self { tree, root }
    }

    /// Single leaf node
    pub fn leaf(elem: &'a str) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(Node::Leaf(elem));
        Self { tree, root }
    }

    /// Ordered sequence of trees under a Sequence root
    pub fn sequence(children: Vec<Self>) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(Node::Sequence);

        for child in children {
            let child_root = Self::transplant(&mut tree, &child.tree, child.root);
            root.append(child_root, &mut tree);
        }

        Self { tree, root }
    }

    /// Unordered alternatives under a Branch root
    pub fn branch(children: Vec<Self>) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(Node::Branch);

        for child in children {
            let child_root = Self::transplant(&mut tree, &child.tree, child.root);
            root.append(child_root, &mut tree);
        }

        Self { tree, root }
    }

    /// optional is just branch with a null
    pub fn optional(child: Self) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(Node::Branch);

        let child_root = Self::transplant(&mut tree, &child.tree, child.root);
        root.append(child_root, &mut tree);

        // alongside, add null child:
        let null_child = Pattern::null();
        let child_root = Self::transplant(&mut tree, &null_child.tree, null_child.root);
        root.append(child_root, &mut tree);

        Self { tree, root }
    }

    fn transplant(
        dst_arena: &mut Arena<Node<'a>>,
        src_arena: &Arena<Node<'a>>,
        src_node: NodeId,
    ) -> NodeId {
        let data = src_arena.get(src_node).unwrap().get().clone();
        let dst_node = dst_arena.new_node(data);

        for child in src_node.children(src_arena).collect::<Vec<_>>() {
            let dst_child = Self::transplant(dst_arena, src_arena, child);
            dst_node.append(dst_child, dst_arena);
        }

        dst_node
    }

    /// lists all possible element sequences, taking each branch combination.
    pub fn enumerate_branches(self) -> Vec<String> {
        let mut results: Vec<String> = vec!["".to_string()]; // start with one empty path

        self.collect_element_sequences(self.root, &mut results);

        results
    }

    /// lists all possible element sequences, taking each branch combination.
    fn collect_element_sequences(&self, node_id: NodeId, paths: &mut Vec<String>) {
        let node = self.tree.get(node_id).unwrap().get();

        match node {
            Node::Leaf(elem) => {
                // Append this element to every current path
                for path in paths.iter_mut() {
                    path.push_str(elem);
                }
            }

            Node::Sequence => {
                // Visit children in order, threading all paths through each
                for child in node_id.children(&self.tree).collect::<Vec<_>>() {
                    self.collect_element_sequences(child, paths);
                }
            }

            Node::Branch => {
                let children: Vec<_> = node_id.children(&self.tree).collect();

                // Fork: each child gets its own copy of the current paths
                let base_paths = paths.clone();
                let mut all_new_paths: Vec<String> = vec![];

                for child in children {
                    let mut forked = base_paths.clone();
                    self.collect_element_sequences(child, &mut forked);
                    all_new_paths.extend(forked);
                }

                *paths = all_new_paths;
            }
            Node::Null => {
                // do nothing
            },
        }
    }
}
