use std::marker::PhantomData;

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
pub struct UniformDepth3Tree<T0, T1, T2> {
    pub layer_0: Vec<T0>,          // parent is always root for these nodes
    pub layer_1: Vec<(T1, usize)>, // data with index of parent in `layer0`
    pub layer_2: Vec<(T2, usize)>, // data with index of parent in `layer1`
}

impl<T0, T1, T2> UniformDepth3Tree<T0, T1, T2> {
    pub fn new(tree: Vec<(T0, Vec<(T1, Vec<T2>)>)>) -> Self {
        let mut layer_0 = vec![];
        let mut layer_1 = vec![];
        let mut layer_2 = vec![];
        for (data, subtree) in tree {
            for (data, subtree_2) in subtree {
                for data in subtree_2 {
                    let parent_idx = layer_1.len();
                    layer_2.push((data, parent_idx));
                }

                let parent_idx = layer_0.len();
                layer_1.push((data, parent_idx));
            }

            layer_0.push(data);
        }
        Self {
            layer_0,
            layer_1,
            layer_2,
        }
    }

    pub fn test_invariants(&self) -> bool {
        // all nodes of layer 0 need to be parent of at least one node in layer 1
        let n0 = self.layer_0.len();
        let mut is_parent = vec![false; n0];
        for (node, parent_idx) in &self.layer_1 {
            if *parent_idx >= n0 {
                // node on layer 1 has invalid parent index !
                return false;
            }
            is_parent[*parent_idx] = true;
        }
        if !is_parent.iter().all(|x| *x) {
            return false;
        }

        // all nodes of layer 1 need to be parent of at least one node in layer 2
        let n1 = self.layer_1.len();
        let mut is_parent = vec![false; n1];
        for (node, parent_idx) in &self.layer_2 {
            if *parent_idx >= n1 {
                // node on layer 2 has invalid parent index !
                return false;
            }
            is_parent[*parent_idx] = true;
        }
        if !is_parent.iter().all(|x| *x) {
            return false;
        }

        true
    }

    /// A graft operation. Using two "spines", cut out a subtree, from the root, and replace it
    /// with another UniformDepth3Tree. nodes on the spines are replaced by the corresponding
    /// nodes on the edge of the inserted subtree.
    pub fn splice_subtree(
        mut self,
        l_spine: Depth3TreeSpine,
        r_spine: Depth3TreeSpine,
        mut replacement: UniformDepth3Tree<T0, T1, T2>,
    ) -> Result<Self, ()> {
        // assert validity of spines
        l_spine.assert_fits_tree(&self);
        r_spine.assert_fits_tree(&self);

        // assert left spine is on the left of the right spine
        l_spine.assert_dont_cross(&r_spine);

        // adjust replacement's indices
        replacement
            .layer_1
            .iter_mut()
            .for_each(|(data, parent)| *parent += l_spine.idx[0]);
        replacement
            .layer_2
            .iter_mut()
            .for_each(|(data, parent)| *parent += l_spine.idx[1]);

        // layer 1: adjust parent indices after replacement zone
        let adjustment =
            replacement.layer_0.len() as isize - (r_spine.idx[0] + 1 - l_spine.idx[0]) as isize;
        self.layer_1
            .iter_mut()
            .skip(r_spine.idx[1] + 1)
            .for_each(|(_, idx)| *idx = ((*idx) as isize + adjustment) as usize);

        // layer 2: adjust parent indices after replacement zone
        let adjustment =
            replacement.layer_1.len() as isize - (r_spine.idx[1] + 1 - l_spine.idx[1]) as isize;
        self.layer_2
            .iter_mut()
            .skip(r_spine.idx[2] + 1)
            .for_each(|(_, idx)| *idx = ((*idx) as isize + adjustment) as usize);

        // replace layers, from left to right spine (inclusive !)
        let range_0 = l_spine.idx[0]..=r_spine.idx[0];
        self.layer_0.splice(range_0, replacement.layer_0);

        let range_1 = l_spine.idx[1]..=r_spine.idx[1];
        self.layer_1.splice(range_1, replacement.layer_1);

        let range_2 = l_spine.idx[2]..=r_spine.idx[2];
        self.layer_2.splice(range_2, replacement.layer_2);

        Ok(self)
    }
}

/// A path from the root to a leaf of a depth-3 uniform tree
pub struct Depth3TreeSpine {
    // within-layer (same depth) index for each node of the spine
    idx: [usize; 3],
}

impl Depth3TreeSpine {
    pub fn new(idx: [usize; 3]) -> Self {
        Self { idx }
    }

    pub fn assert_fits_tree<T0, T1, T2>(&self, tree: &UniformDepth3Tree<T0, T1, T2>) {
        // assert first index of spine is valid
        assert!(tree.layer_0.len() > self.idx[0]);

        // make sure spine nodes are parent/child
        assert_eq!(tree.layer_1[self.idx[1]].1, self.idx[0]);
        assert_eq!(tree.layer_2[self.idx[2]].1, self.idx[1]);
    }

    /// Assert `self` is on the left or on top of (equal to) the `other` spine
    pub fn assert_dont_cross(&self, other: &Self) {
        assert!(self.idx[0] <= other.idx[0]);
        assert!(self.idx[1] <= other.idx[1]);
        assert!(self.idx[2] <= other.idx[2]);
    }
}
