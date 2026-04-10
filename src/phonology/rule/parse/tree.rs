use crate::phonology::rule::{SegmentInfo, SyllableInfo};
use indextree::{Arena, NodeId};

/// Phonological Rule Parse Node
/// Nodes that make the rule parse tree
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedRuleNode {
    Sequence,
    Leaf(ParsedRuleElem),
    Branch,
}

/// a boundary or a feature set for a segment
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedRuleElem {
    Features(SyllableInfo, SegmentInfo),
    WordBoundary,
    SyllableBoundary,
    Null,
}

#[derive(Debug, Clone)]
pub struct ParsedRulePattern {
    pub(crate) tree: Arena<ParsedRuleNode>,
    pub(crate) root: NodeId,
}
impl ParsedRulePattern {
    pub fn null() -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(ParsedRuleNode::Leaf(ParsedRuleElem::Null));
        Self { tree, root }
    }

    /// Single leaf node
    pub fn leaf(elem: ParsedRuleElem) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(ParsedRuleNode::Leaf(elem));
        Self { tree, root }
    }

    /// Ordered sequence of trees under a Sequence root
    pub fn sequence(children: Vec<Self>) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(ParsedRuleNode::Sequence);

        for child in children {
            let child_root = Self::transplant(&mut tree, &child.tree, child.root);
            root.append(child_root, &mut tree);
        }

        Self { tree, root }
    }

    /// Unordered alternatives under a Branch root
    pub fn branch(children: Vec<Self>) -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(ParsedRuleNode::Branch);

        for child in children {
            let child_root = Self::transplant(&mut tree, &child.tree, child.root);
            root.append(child_root, &mut tree);
        }

        Self { tree, root }
    }

    fn transplant(
        dst_arena: &mut Arena<ParsedRuleNode>,
        src_arena: &Arena<ParsedRuleNode>,
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
}
