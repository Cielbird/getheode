mod uniform_depth3 {
    use crate::ud3tree;

    #[test]
    fn test_splice() {
        // tree looks like:
        //      x
        //   /  | \
        //  0   1   2
        //  | \   \   \
        //  3  4   5   6
        //  |  |\  |   | \
        //  7  8 9 10  11 12
        let tree = ud3tree![
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
        let replacement = ud3tree![
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
        let expected = ud3tree![
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
}
