use crate::{d3tree, phonology::tree::Depth3Tree};

mod depth3 {
    use crate::d3tree;

    #[test]
    fn test_leaves_depth_3() {
        let tree = d3tree![
            0 => [
                3 => [7, 8],
                4 => [9],
            ],
            1 => [
                5 => [10, 11],
            ],
            2 => [
                6 => [12],
            ],
        ];
        assert!(tree.are_leaves_depth_3());
    }

    #[test]
    fn test_leaves_not_depth_3_1() {
        let tree = d3tree![
            0 => [2 => [5, 6],3 => [7],],
            1 => [4 => [],],
        ];
        assert!(!tree.are_leaves_depth_3());
    }

    #[test]
    fn test_leaves_not_depth_3_2() {
        let tree = d3tree![
            0 => [2 => [4, 5], 3 => [6]],
            1 => [],
        ];
        assert!(!tree.are_leaves_depth_3());
    }

    #[test]
    fn test_replace_range_1() {
        // tree looks like:
        //      x
        //   /  | \
        //  0   1   2
        //  | \   \   \
        //  3  4   5   6
        //  |  |\  |   | \
        //  7  8 9 10  11 12
        let tree = d3tree![
            0 => [
                3 => [7],
                4 => [8, 9],
            ],
            1 => [
                5 => [10],
            ],
            2 => [
                6 => [11, 12],
            ],
        ];

        // replacement looks like:
        //  x
        //  | \
        //  0  1
        //  |  | \
        //  2  3  4
        //  |  |  |
        //  5  6  7
        let replacement = d3tree![
            0 => [
                2 => [5],
            ],
            1 => [
                3 => [6],
                4 => [7],
            ],
        ];

        // expected result should look like:
        //      x
        //   /  | \
        //  0    1   2
        //  | \  | \   \
        //  3 2  3  4   6
        //  | |  |  |   | \
        //  7 5  6  7   11 12
        let expected = d3tree![
            0 => [
                3 => [7],
                2 => [5],
            ],
            1 => [
                3 => [6],
                4 => [7],
            ],
            2 => [
                6 => [11, 12],
            ],
        ];

        let res = tree.replace_range(1..4, replacement).unwrap();

        assert_eq!(res, expected);
    }

    #[test]
    fn test_replace_range_2() {
        let tree = d3tree![
            0 => [
                4 => [11, 12, 13],
                5 => [14, 15],
                6 => [16, 17, 18],
            ],
            1 => [
                7 => [19],
            ],
            2 => [
                8 => [20, 21, 22],
            ],
            3 => [
                9 => [23, 24],
                10 => [25, 26],
            ],
        ];

        let replacement = d3tree![
            0 => [1 => [2]]
        ];

        let expected = d3tree![
            0 => [
                4 => [11, 12, 13],
                5 => [14, 15],
                1 => [16, 2, 24],
                10 => [25, 26],
            ],
        ];

        let res = tree.replace_range(6..13, replacement).unwrap();

        assert_eq!(res, expected);
    }
}

#[test]
fn test_macro() {
    let tree = d3tree![
        0 => [
            3 => [7, 8],
            4 => [9],
        ],
        1 => [
            5 => [10, 11],
        ],
        2 => [
            6 => [12],
        ],
    ];

    assert_eq!(tree.layer_0, vec![0, 1, 2]);
    assert_eq!(tree.layer_1, vec![(3, 0), (4, 0), (5, 1), (6, 2)]);
    assert_eq!(
        tree.layer_2,
        vec![(7, 0), (8, 0), (9, 1), (10, 2), (11, 2), (12, 3)]
    );
}

#[test]
fn test_nonuniform_macro() {
    let tree = d3tree![
        0 => [
            3 => [7, 8],
            4 => [9],
        ],
        1 => [],
        2 => [
            6 => [12],
        ],
    ];

    assert_eq!(tree.layer_0, vec![0, 1, 2]);
    assert_eq!(tree.layer_1, vec![(3, 0), (4, 0), (6, 2)]);
    assert_eq!(tree.layer_2, vec![(7, 0), (8, 0), (9, 1), (12, 2)]);
}

#[test]
fn test_invalid_invariants_1() {
    let tree = Depth3Tree {
        layer_0: vec![0, 1],
        layer_1: vec![(0, 1), (1, 0), (2, 1)], // out of order ! invalid
        layer_2: vec![(0, 0), (1, 1), (2, 2)],
    };
    assert!(!tree.test_invariants());
}

#[test]
fn test_invalid_invariants_2() {
    let tree = Depth3Tree {
        layer_0: vec![0, 1],
        layer_1: vec![(0, 0), (1, 1)],
        layer_2: vec![(0, 1), (1, 0), (2, 1)], // out of order ! invalid
    };
    assert!(!tree.test_invariants());
}
