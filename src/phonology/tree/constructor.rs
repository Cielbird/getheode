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
        let mut layer_0 = Vec::new();
        #[allow(unused_mut)]
        let mut layer_1 = Vec::new();
        #[allow(unused_mut)]
        let mut layer_2 = Vec::new();
        #[allow(unused_mut)]
        let mut l0_idx = 0usize;
        $(
            layer_0.push($l0);
            #[allow(unused_mut)]
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

        $crate::phonology::tree::Depth3Tree { layer_0, layer_1, layer_2 }
    }};
}
