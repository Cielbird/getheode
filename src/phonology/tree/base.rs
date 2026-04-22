use std::{
    fmt::{Debug, Write as _},
    ops::Range,
};

use crate::error::*;

use crate::phonology::tree::iter::IterDepth0;

/// A uniform 3-depth tree, where each "layer" has it's own node data type. Node order relative to
/// other nodes at the same depth is important.
///
///  x      root
/// | \
/// o  o    layer 0 (data is type T0)
/// |   \
/// o    o  layer 1 (data is type T1)
/// | \  |
/// o  o o  layer 2 (data is type T2)
///
/// Invariants :
/// - all leaf nodes are depth 3 (uniform)
///
/// Use :
/// This could be used to represent phonological sequences. Each layer contains feature data about
/// that layer of the hierarchy.
#[derive(Debug, PartialEq, Clone)]
pub struct Depth3Tree<T0, T1, T2> {
    pub(super) layer_0: Vec<T0>, // parent is always root for these nodes
    pub(super) layer_1: Vec<(T1, usize)>, // data with index of parent in `layer0`
    pub(super) layer_2: Vec<(T2, usize)>, // data with index of parent in `layer1`
}

impl<T0, T1, T2> Depth3Tree<T0, T1, T2> {
    pub fn new() -> Self {
        Self {
            layer_0: vec![],
            layer_1: vec![],
            layer_2: vec![],
        }
    }

    pub fn push_depth_0(&mut self, element: T0) {
        self.layer_0.push(element);
    }

    pub fn push_depth_1(&mut self, element: T1) {
        let last_idx = self.layer_0.len() - 1;
        self.layer_1.push((element, last_idx));
    }

    pub fn push_depth_2(&mut self, element: T2) {
        let last_idx = self.layer_1.len() - 1;
        self.layer_2.push((element, last_idx));
    }

    /// Insert a node at layer 0 at `index` relative to other nodes at layer 0
    pub fn insert_depth_0(&mut self, index: usize, element: T0) {
        self.layer_0.insert(index, element);
        for (_, parent_idx) in &mut self.layer_2 {
            if *parent_idx >= index {
                *parent_idx += 1;
            }
        }
    }

    /// Insert a node at layer 1 at `index` relative to other nodes at layer 1
    pub fn insert_depth_1(&mut self, idx: usize, parent_idx: usize, element: T1) {
        self.layer_1.insert(idx, (element, parent_idx));
        for (_, parent_idx) in &mut self.layer_2 {
            if *parent_idx >= idx {
                *parent_idx += 1;
            }
        }
    }

    /// Insert a node at layer 2 at `index` relative to other nodes at layer 2
    pub fn insert_depth_2(&mut self, idx: usize, parent_idx: usize, element: T2) {
        self.layer_2.insert(idx, (element, parent_idx));
    }

    pub fn iter<'a>(&'a self) -> IterDepth0<'a, T0, T1, T2> {
        IterDepth0::new(self)
    }

    /// Tests if all leaf nodes of the tree are depth 3, in other words, is the tree "uniform"
    pub fn test_invariants(&self) -> bool {
        let mut last_idx = 0;
        for (_, parent_idx) in &self.layer_1 {
            if last_idx > *parent_idx {
                // parent index should never descend : tree would be unordered
                return false;
            }
            last_idx = *parent_idx;
            if *parent_idx >= self.layer_0.len() {
                // node on layer 1 has invalid parent index !
                return false;
            }
        }

        let mut last_idx = 0;
        for (_, parent_idx) in &self.layer_2 {
            if last_idx > *parent_idx {
                // parent index should never descend : tree would be unordered
                return false;
            }
            last_idx = *parent_idx;
            if *parent_idx >= self.layer_1.len() {
                // node on layer 2 has invalid parent index !
                return false;
            }
        }

        true
    }

    /// Tests if all leaf nodes of the tree are depth 3, in other words, is the tree "uniform"
    pub fn are_leaves_depth_3(&self) -> bool {
        // all nodes of layer 0 need to be parent of at least one node in layer 1
        let n0 = self.layer_0.len();
        let mut is_parent = vec![false; n0];
        for (_, parent_idx) in &self.layer_1 {
            is_parent[*parent_idx] = true;
        }
        if !is_parent.iter().all(|x| *x) {
            // a node on layer 0 is a leaf !
            return false;
        }

        // all nodes of layer 1 need to be parent of at least one node in layer 2
        let n1 = self.layer_1.len();
        let mut is_parent = vec![false; n1];
        for (_, parent_idx) in &self.layer_2 {
            is_parent[*parent_idx] = true;
        }
        if !is_parent.iter().all(|x| *x) {
            // a node on layer 1 is a leaf !
            return false;
        }

        true
    }

    /// Replace a section of the tree delimited by a range on leaf nodes, up to the root.
    /// The leaf node range creates two "spines", which cut out a subtree, from the root.
    /// This zone is replaced with another UniformDepth3Tree.
    /// nodes on the spines are replaced by the corresponding nodes on the edge of the inserted
    /// subtree.
    pub fn replace_range(
        mut self,
        leaf_range: Range<usize>,
        mut replace_with: Depth3Tree<T0, T1, T2>,
    ) -> Result<Self> {
        if leaf_range.start >= self.layer_2.len() {
            return Err(Error::other("Invalid range"));
        }
        if leaf_range.end > self.layer_2.len() {
            return Err(Error::other("Invalid range"));
        }

        // construct left and right spines
        let l_spine_2 = leaf_range.start;
        let l_spine_1 = self.layer_2[l_spine_2].1;
        let l_spine_0 = self.layer_1[l_spine_1].1;
        // (range.end is not inclusive, we want the spine to include the replacement zone)
        let r_spine_2 = leaf_range.end - 1;
        let r_spine_1 = self.layer_2[r_spine_2].1;
        let r_spine_0 = self.layer_1[r_spine_1].1;

        // adjust replacement's indices
        replace_with
            .layer_1
            .iter_mut()
            .for_each(|(_, parent)| *parent += l_spine_0);
        replace_with
            .layer_2
            .iter_mut()
            .for_each(|(_, parent)| *parent += l_spine_1);

        // layer 1: adjust parent indices after replacement zone
        let adjustment = replace_with.layer_0.len() as isize - (r_spine_0 + 1 - l_spine_0) as isize;
        self.layer_1
            .iter_mut()
            .skip(r_spine_1 + 1)
            .for_each(|(_, idx)| *idx = ((*idx) as isize + adjustment) as usize);

        // layer 2: adjust parent indices after replacement zone
        let adjustment = replace_with.layer_1.len() as isize - (r_spine_1 + 1 - l_spine_1) as isize;
        self.layer_2
            .iter_mut()
            .skip(r_spine_2 + 1)
            .for_each(|(_, idx)| *idx = ((*idx) as isize + adjustment) as usize);

        // replace layers, from left to right spine (inclusive !)
        let range_0 = l_spine_0..=r_spine_0;
        self.layer_0.splice(range_0, replace_with.layer_0);

        let range_1 = l_spine_1..=r_spine_1;
        self.layer_1.splice(range_1, replace_with.layer_1);

        let range_2 = l_spine_2..=r_spine_2;
        self.layer_2.splice(range_2, replace_with.layer_2);

        Ok(self)
    }

    pub fn layer_0(&self) -> &[T0] {
        self.layer_0.as_slice()
    }

    pub fn layer_1(&self) -> &[(T1, usize)] {
        self.layer_1.as_slice()
    }

    pub fn layer_2(&self) -> &[(T2, usize)] {
        self.layer_2.as_slice()
    }

    pub fn get_depth_0_mut(&mut self, idx: usize) -> &mut T0 {
        &mut self.layer_0[idx]
    }

    pub fn get_depth_1_mut(&mut self, idx: usize) -> &mut T1 {
        &mut self.layer_1[idx].0
    }

    pub fn get_depth_2_mut(&mut self, idx: usize) -> &mut T2 {
        &mut self.layer_2[idx].0
    }

    /// Get number of nodes with depth 0
    pub fn len_0(&self) -> usize {
        self.layer_0.len()
    }

    /// Get number of nodes with depth 1
    pub fn len_1(&self) -> usize {
        self.layer_1.len()
    }

    /// Get number of nodes with depth 2
    pub fn len_2(&self) -> usize {
        self.layer_2.len()
    }

    pub fn pretty_format(&self) -> String
    where
        T0: Debug,
        T1: Debug,
        T2: Debug,
    {
        let mut result = String::new();

        for (l0, sub) in self.iter() {
            write!(&mut result, "{:#?}", l0).unwrap();

            for (l1, sub) in sub {
                write!(&mut result, "+ {:#?}", l1).unwrap();

                for l2 in sub {
                    write!(&mut result, "+-- {:#?}", l2).unwrap();
                }
            }
        }

        result
    }
}

impl<T0, T1, T2> Default for Depth3Tree<T0, T1, T2> {
    fn default() -> Self {
        Self::new()
    }
}
