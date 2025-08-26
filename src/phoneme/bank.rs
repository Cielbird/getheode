use crate::{
    phoneme::{Phoneme, PhonemeId, PhonemeString},
    segment::{FeatureState, STRESS, Segment, SegmentString},
};
use std::collections::HashMap;

use super::PhonemeStringSylable;

pub struct PhonemeBank {
    /// set of phonemes used and their data
    pub(crate) phonemes: HashMap<PhonemeId, Phoneme>,
}

impl Default for PhonemeBank {
    fn default() -> Self {
        Self::new()
    }
}

impl PhonemeBank {
    pub fn new() -> Self {
        Self {
            phonemes: HashMap::new(),
        }
    }

    /// Adds a new phoneme
    ///
    /// # Returns
    ///
    /// The id of the new phoneme, or `None` if a phoneme with the same symbol is already in the
    /// bank
    pub fn add(&mut self, segment: Segment, symbol: String) -> Option<PhonemeId> {
        // if an equal phoneme is alread in the bank, return false
        if self.phonemes.values().any(|other| symbol == other.symbol) {
            return None;
        }

        let phoneme = Phoneme { segment, symbol };
        let id = self.new_id();
        self.phonemes.insert(id, phoneme);

        Some(id)
    }

    /// Get the underlying representation of a sequence of phonemes
    pub fn underlying_rep(&self, phonemes: PhonemeString) -> SegmentString {
        let mut underlying = SegmentString::new();
        let mut sylables = phonemes.sylables.iter();
        let mut cur_sylable: Option<&PhonemeStringSylable> = None;

        for (idx, id) in phonemes.phonemes.iter().enumerate() {
            // get the segment for the cur phoneme
            let mut segment = self.phonemes.get(&id).unwrap().segment.clone();

            // apply stress from the current sylable
            if cur_sylable.is_none() && let Some(first_syl) = sylables.next() {
                cur_sylable = Some(first_syl);
            } else if let Some(cur) = cur_sylable && cur.start == idx {
                cur_sylable = sylables.next();
            }
            if let Some(cur_sylable) = cur_sylable {
                segment.features[STRESS as usize] = if cur_sylable.stressed {
                    FeatureState::POS
                } else {
                    FeatureState::NEG
                };
            }

            underlying.push(segment);
        }
        underlying
    }

    /// Generate a new id for a phoneme, unique for this bank
    fn new_id(&mut self) -> PhonemeId {
        let mut i: u64 = 0;
        loop {
            let taken = self.phonemes.keys().any(|x| *x == i.into());
            if !taken {
                return i.into();
            }
            i += 1;
        }
    }
}
