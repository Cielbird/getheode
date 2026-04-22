use crate::phonology::tree::Depth3Tree;

pub struct IterDepth0<'a, T0, T1, T2> {
    tree: &'a Depth3Tree<T0, T1, T2>,
    idx: usize,
}

pub struct IterDepth1<'a, T0, T1, T2> {
    tree: &'a Depth3Tree<T0, T1, T2>,
    parent_idx: usize,
    idx: Option<usize>, // if null, there's nothing to iterate on
}

pub struct IterDepth2<'a, T0, T1, T2> {
    tree: &'a Depth3Tree<T0, T1, T2>,
    parent_idx: usize,
    idx: Option<usize>, // if null, there's nothing to iterate on
}

impl<'a, T0, T1, T2> Iterator for IterDepth0<'a, T0, T1, T2> {
    type Item = (&'a T0, IterDepth1<'a, T0, T1, T2>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.tree.layer_0.len() {
            let sub_iter = IterDepth1::new(self.tree, self.idx);
            let ret = &self.tree.layer_0[self.idx];

            self.idx += 1;
            Some((ret, sub_iter))
        } else {
            None
        }
    }
}

impl<'a, T0, T1, T2> Iterator for IterDepth1<'a, T0, T1, T2> {
    type Item = (&'a T1, IterDepth2<'a, T0, T1, T2>);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = &mut self.idx else {
            // nothing to iterate on
            return None;
        };

        if *idx < self.tree.layer_1.len() {
            let (item, parent) = &self.tree.layer_1[*idx];
            if *parent == self.parent_idx {
                let sub_iter = IterDepth2::new(self.tree, *idx);

                *idx += 1;
                return Some((item, sub_iter));
            }
        }
        None
    }
}

impl<'a, T0, T1, T2> Iterator for IterDepth2<'a, T0, T1, T2> {
    type Item = &'a T2;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = &mut self.idx else {
            // nothing to iterate on
            return None;
        };

        if *idx < self.tree.layer_2.len() {
            let (item, parent) = &self.tree.layer_2[*idx];
            if *parent == self.parent_idx {
                *idx += 1;
                return Some(item);
            }
        }
        None
    }
}

impl<'a, T0, T1, T2> IterDepth0<'a, T0, T1, T2> {
    pub fn new(tree: &'a Depth3Tree<T0, T1, T2>) -> Self {
        let idx = 0;

        Self { tree, idx }
    }
}

impl<'a, T0, T1, T2> IterDepth1<'a, T0, T1, T2> {
    fn new(tree: &'a Depth3Tree<T0, T1, T2>, parent_idx: usize) -> Self {
        // TODO eliminate this; redundant
        let idx = tree
            .layer_1
            .iter()
            .enumerate()
            .find_map(|(idx, (_, parent))| {
                if *parent == parent_idx {
                    Some(idx)
                } else {
                    None
                }
            });

        Self {
            tree,
            parent_idx,
            idx,
        }
    }
}

impl<'a, T0, T1, T2> IterDepth2<'a, T0, T1, T2> {
    fn new(tree: &'a Depth3Tree<T0, T1, T2>, parent_idx: usize) -> Self {
        let idx = tree
            .layer_2
            .iter()
            .enumerate()
            .find_map(|(idx, (_, parent))| {
                if *parent == parent_idx {
                    Some(idx)
                } else {
                    None
                }
            });

        Self {
            tree,
            parent_idx,
            idx,
        }
    }
}
