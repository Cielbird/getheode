use crate::{phoneme::{Phoneme, PhonemeId}, segment::{Segment}};
use std::collections::HashMap;

pub struct PhonemeBank {
    /// set of phonemes used and their data
    pub(crate) phonemes: HashMap<PhonemeId, Phoneme>,
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
