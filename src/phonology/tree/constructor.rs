//! Constructor macro for UniformDepth3Tree

/// macro for easy construction. example :
/// ```json
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
/// use getheode::d3tree;
/// let x = d3tree![
///     'a' => [
///         'c' => ['f'],
///         'd' => ['g'],
///     ],
///     'b' => [
///         'e' => ['h'],
///     ],
/// ];
/// ```
///
#[macro_export]
macro_rules! d3tree {
    (
        $( $l0:expr => [
            $( $l1:expr => [
                $( $l2:expr ),* $(,)?
            ] ),* $(,)?
        ] ),* $(,)?
    ) => {{
        #[allow(unused_mut)]
        let mut tree = $crate::phonology::tree::Depth3Tree::new();
        $(
            tree.push_depth_0($l0);
            #[allow(unused_mut)]
            $(
                tree.push_depth_1($l1);
                $(
                    tree.push_depth_2($l2);
                )*
            )*
        )*

        tree
    }};
}
