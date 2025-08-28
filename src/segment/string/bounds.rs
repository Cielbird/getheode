//! Defines chunk sequence logic used for sylable and word boundaries
//!

use std::{collections::BTreeSet, fmt::Debug};

/// A sequence of chunks within a space [0, N). Each chunk has a value. Chunks cover a range,
/// which may bo beyond the space [0, N)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkSequence<T> {
    /// Size of the list that is seperated into chunks
    len: usize,
    /// Positions in the vector for boundaries between each chunk. Boundaries may be [0, N].
    boundaries: BTreeSet<usize>,
    /// Items for each chunk in order
    chunks: Vec<T>,
}

impl<T: Debug> ChunkSequence<T> {
    /// Create a new chunk sequence with one unbounded chunk
    pub fn single_unbounded(len: usize) -> Self
    where
        T: Default,
    {
        if len > 0 {
            Self {
                len,
                boundaries: BTreeSet::new(),
                chunks: vec![T::default()],
            }
        } else {
            Self {
                len: 0,
                boundaries: BTreeSet::new(),
                chunks: vec![],
            }
        }
    }

    pub fn new<B, C>(len: usize, boundaries: B, chunks: C) -> Self
    where
        B: Into<BTreeSet<usize>>,
        C: Into<Vec<T>>,
    {
        let boundaries = boundaries.into();
        let chunks = chunks.into();
        let out = Self {
            len,
            boundaries,
            chunks,
        };
        out.assert_valid();

        out
    }

    /// Splits a chunk that covers a given position. The new upper half (rhs) is assigned
    /// a new value, and the lower half (lhs) keeps the original chunk's value.
    /// If the position is already a chunk boundary, the chunk on the right will be assigned the
    /// value `new_rhs`
    fn split_chunk(&mut self, position: usize, new_rhs: T) {
        // insert new chunk
        let chunk_idx = self.get_chunk_idx(position);
        if !self.boundaries.contains(&position) {
            self.chunks.insert(chunk_idx + 1, new_rhs);
        } else {
            self.chunks[chunk_idx] = new_rhs;
        }
        self.boundaries.insert(position);
        self.assert_valid();
    }

    /// Set the value of the chunk that covers the given position
    fn set_chunk(&mut self, position: usize, new_val: T) {
        let chunk_idx = self.get_chunk_idx(position);

        self.chunks[chunk_idx] = new_val;

        self.assert_valid();
    }

    // 0 3
    fn get_chunk_idx(&self, position: usize) -> usize {
        println!("setting chunk {position}, {:?}, len: {}", self.boundaries, self.len);
        let mut chunk_idx = 0;
        for &b in &self.boundaries {
            if b > position {
                break;
            }
            if b != 0 {
                chunk_idx += 1;
            }
        }
        println!("found chunk {chunk_idx}");
        chunk_idx
    }

    /// Merge two chunks with border at a position. The left chunk's value takes over.
    fn merge_chunk(&mut self, position: usize) {
        let mut chunk_idx = 0;
        for &b in &self.boundaries {
            if b > position {
                break;
            }
            if b != 0 {
                chunk_idx += 1;
            }
        }
        self.chunks.remove(chunk_idx);
        self.boundaries.remove(&position);

        self.assert_valid();
        println!("after merge: {:?}", self);
    }

    pub fn replace(&mut self, start: usize, end: usize, replacement: ChunkSequence<T>)
    where
        T: Clone,
    {
        // example (dots are chunk boundaries, letters are chunk data)
        // self:         [a].[b] [b].   [c] [c]
        // replacement:          [d][d].[e]
        // result:       [a].[b] [b][b].[e] [e]

        // shift elements after the replacement zone up if needed
        let do_shift_up = replacement.len > (end - start);
        let shift = if do_shift_up {
            replacement.len - (end - start)
        } else {
            0
        };

        // These are in the replacement zone, remove later
        let mut to_remove = BTreeSet::new();

        let mut new_boundaries = BTreeSet::new();
        for &bound in &self.boundaries {
            if bound < end {
                new_boundaries.insert(bound);
                if bound > start {
                    to_remove.insert(bound);
                }
            } else {
                new_boundaries.insert(bound + shift);
            }
        }
        self.boundaries = new_boundaries;
        self.len += shift;

        // Add new boundaries by splitting existing chunks, using new chunks for new rhs values
        let mut new_chunks = replacement.chunks.into_iter();
        let first_bound = replacement.boundaries.first();
        if first_bound.is_none() || *first_bound.unwrap() != 0 {
            self.set_chunk(start, new_chunks.next().unwrap());
        }
        for new_bound in replacement.boundaries.clone() {
            if let Some(new_chunk) = new_chunks.next() {
                let new_bound = start + new_bound;
                self.split_chunk(new_bound, new_chunk);
                // if we put a new boundary at a position that already had a boundary,
                // don't remove it later.
                to_remove.remove(&new_bound);
            }
        }

        // remove old boundaries in replacement zone by merging
        for old_boundary in to_remove {
            self.merge_chunk(old_boundary);
        }

        // shift elements after the replacement zone down if needed
        if replacement.len < (end - start) {
            let shift = (end - start) - replacement.len;

            // Shift bounderies after end up
            let mut new_boundaries = BTreeSet::new();
            for &bound in &self.boundaries {
                if bound < end {
                    new_boundaries.insert(bound);
                } else if bound >= end {
                    new_boundaries.insert(bound - shift);
                }
            }
            self.boundaries = new_boundaries;
            self.len -= shift;
        }
    }

    pub fn is_match(&self, words: &ChunkSequence<()>, offset: isize) -> bool {
        todo!()
    }

    /// Asserts the validity of the bounds sequence
    fn assert_valid(&self) {
        let mut chunk_count = 1;
        for &b in &self.boundaries {
            assert!(
                b <= self.len,
                "boundary ({b}) must be <= len ({})",
                self.len
            );
            if b != 0 && b != self.len {
                chunk_count += 1;
            }
        }
        assert_eq!(
            chunk_count,
            self.chunks.len(),
            "The number of chunks doesn't match the length and number of boundaries"
        );
    }

    /// Set boundaries at the start and end of the chunk sequence
    pub fn bounded(mut self) -> Self {
        if !self.boundaries.contains(&0) {
            self.boundaries.insert(0);
        }
        if !self.boundaries.contains(&self.len) {
            self.boundaries.insert(self.len);
        }

        self
    }

    /// Extend the last chunk's size
    pub fn extend_last(&mut self, len: usize) {
        let new_len = self.len + len;
        if let Some(&last) = self.boundaries.last()
            && last == self.len
        {
            self.boundaries.pop_last();
            self.boundaries.insert(new_len);
        }
        self.len = new_len;
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use crate::segment::bounds::ChunkSequence;

    // 0 1 2 3   4 5
    // a.b b.c   c
    //     d d.e
    // a.d d d.e e
    #[test]
    fn test_replace_simple() {
        let mut original = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([1, 3]),
            chunks: vec!['a', 'b', 'c'],
        };
        let replacement = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([2]),
            chunks: vec!['d', 'e'],
        };
        let expected = ChunkSequence::<char> {
            len: 6,
            boundaries: BTreeSet::from([1, 4]),
            chunks: vec!['a', 'd', 'e'],
        };

        original.replace(2, 4, replacement);

        assert_eq!(original, expected);
    }

    #[test]
    fn test_replace_whole_1() {
        let mut original = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([0, 5]),
            chunks: vec!['a'],
        };
        let replacement = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([0, 3]),
            chunks: vec!['b'],
        };
        let expected = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([0, 3]),
            chunks: vec!['b'],
        };

        original.replace(0, 5, replacement);

        assert_eq!(original, expected);
    }

    #[test]
    fn test_replace_whole_one_border() {
        let mut original = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([0, 5]),
            chunks: vec!['a'],
        };
        let replacement = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([3]),
            chunks: vec!['b'],
        };
        let expected = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([0, 3]),
            chunks: vec!['b'],
        };

        original.replace(0, 5, replacement);

        assert_eq!(original, expected);
    }

    #[test]
    fn test_replace_whole_inner_borders() {
        let mut original = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([0, 1, 4, 5]),
            chunks: vec!['a', 'b', 'c'],
        };
        let replacement = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([0, 1]),
            chunks: vec!['d', 'e'],
        };
        let expected = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([0, 1, 2, 4, 5]),
            chunks: vec!['a', 'd', 'e', 'c'],
        };

        original.replace(1, 4, replacement);

        assert_eq!(original, expected);
    }

    #[test]
    fn test_replace_whole_inner_borders_2() {
        let mut original = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([0, 1, 4, 5]),
            chunks: vec!['a', 'b', 'c'],
        };
        let replacement = ChunkSequence::<char> {
            len: 3,
            boundaries: BTreeSet::from([2, 3]),
            chunks: vec!['d', 'e'],
        };
        let expected = ChunkSequence::<char> {
            len: 5,
            boundaries: BTreeSet::from([0, 1, 3, 4, 5]),
            chunks: vec!['a', 'd', 'e', 'c'],
        };

        original.replace(1, 4, replacement);

        assert_eq!(original, expected);
    }

    // 0123456
    // a.b b.c c c
    // d
    // d.b b.c c c
    // d.c c c

    #[test]
    fn test_replace_shift_conflict() {
        let mut original = ChunkSequence::<char> {
            len: 6,
            boundaries: BTreeSet::from([1, 3]),
            chunks: vec!['a', 'b', 'c'],
        };
        let replacement = ChunkSequence::<char> {
            len: 1,
            boundaries: BTreeSet::from([]),
            chunks: vec!['d'],
        };
        let expected = ChunkSequence::<char> {
            len: 4,
            boundaries: BTreeSet::from([1]),
            chunks: vec!['d', 'c'],
        };

        original.replace(0, 3, replacement);

        assert_eq!(original, expected);
    }
}
