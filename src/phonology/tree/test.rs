mod uniform_depth3 {
    use crate::phonology::tree::base::{Depth3TreeSpine, UniformDepth3Tree};

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
        // let tree = UniformDepth3Tree::<u8, u8, u8>::new(
        //     vec![0, 1, 2],
        //     vec![(3, 0), (4, 0), (5, 1), (6, 2)],
        //     vec![(7, 0), (8, 1), (9, 1), (10, 2), (11, 3), (12, 3)],
        // );
        let tree = UniformDepth3Tree::<u8, u8, u8>::new(vec![
            (0, vec![(3, vec![7]), (4, vec![8, 9])]),
            (1, vec![(5, vec![10])]),
            (2, vec![(6, vec![11, 12])]),
        ]);

        let left = Depth3TreeSpine::new([0, 1, 1]);
        let right = Depth3TreeSpine::new([1, 2, 3]);

        // replacement looks like:
        //  x
        //  | \
        //  0  1
        //  |  | \
        //  2  3  4
        //  |  |  |
        //  5  6  7
        // let replacement = UniformDepth3Tree::<u8, u8, u8>::new(
        //     vec![0, 1],
        //     vec![(2, 0), (3, 1), (4, 1)],
        //     vec![(5, 0), (6, 1), (7, 2)],
        // );
        let replacement = UniformDepth3Tree::<u8, u8, u8>::new(vec![
            (0, vec![(2, vec![5])]),
            (1, vec![(3, vec![6]), (4, vec![7])]),
        ]);

        // expected result should look like:
        //      x
        //   /  | \
        //  0    1   2
        //  | \  | \   \
        //  3 2  3  4   6
        //  | |  |  |   | \
        //  7 5  6  7   11 12
        let expected = UniformDepth3Tree::<u8, u8, u8>::new(vec![
            (0, vec![(3, vec![7]), (2, vec![5])]),
            (1, vec![(3, vec![6]), (4, vec![7])]),
            (2, vec![(6, vec![11, 12])]),
        ]);

        let res = tree.splice_subtree(left, right, replacement).unwrap();

        assert_eq!(res, expected);
    }
}
