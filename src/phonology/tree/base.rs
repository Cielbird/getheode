use std::ops::Range;

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

/// macro for easy construction. example : 
/// ```
///   x
///  |  \
///  a    b 
///  | \  |
///  c d  e
///  | |  |
///  f g  h
/// ```
/// would be :
/// 
/// ```
/// let x = ud3tree![
///     'a' => [
///         'c' => ['f'],
///         'd' => ['g'],
///     ],
///     'b' => [
///         'e' => ['h'],
///     ],
/// ];
/// ```
#[macro_export]
macro_rules! ud3tree {
    (
        $( $l0:expr => [
            $( $l1:expr => [
                $( $l2:expr ),* $(,)?
            ] ),* $(,)?
        ] ),* $(,)?
    ) => {{
        let mut layer_0 = Vec::new();
        let mut layer_1 = Vec::new();
        let mut layer_2 = Vec::new();

        let mut l0_idx = 0usize;
        $(
            layer_0.push($l0);
            let mut l1_idx = layer_1.len();
            $(
                layer_1.push(($l1, l0_idx));
                $(
                    layer_2.push(($l2, l1_idx));
                )*
                l1_idx += 1;
            )*
            let _ = l1_idx;
            l0_idx += 1;
        )*
        let _ = l0_idx;

        UniformDepth3Tree { layer_0, layer_1, layer_2 }
    }};
}

impl<T0, T1, T2> UniformDepth3Tree<T0, T1, T2> {
    pub fn test_invariants(&self) -> bool {
        // all nodes of layer 0 need to be parent of at least one node in layer 1
        let n0 = self.layer_0.len();
        let mut is_parent = vec![false; n0];
        for (_, parent_idx) in &self.layer_1 {
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
        for (_, parent_idx) in &self.layer_2 {
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

    /// Replace a section of the tree delimited by a range on leaf nodes, up to the root.
    /// The leaf node range creates two "spines", which cut out a subtree, from the root.
    /// This zone is replaced with another UniformDepth3Tree.
    /// nodes on the spines are replaced by the corresponding nodes on the edge of the inserted
    /// subtree.
    pub fn replace_range(
        mut self,
        leaf_range: Range<usize>,
        mut replace_with: UniformDepth3Tree<T0, T1, T2>,
    ) -> Result<Self, String> {
        if leaf_range.start >= self.layer_2.len() {
            return Err("Invalid range".to_string());
        }
        if leaf_range.end >= self.layer_2.len() {
            return Err("Invalid range".to_string());
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
}
