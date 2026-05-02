//! Paperless tape: an event log of computations the user can review,
//! save, share, or rewind.

use crate::operations::rafter::RafterSolution;
use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TapeEntry {
    /// A committed `=` result.
    Result(Value),
    /// A solved rafter / right-angle problem.
    RafterSolution(Box<RafterSolution>),
    /// A free-form note (e.g. "kitchen wall").
    Note(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tape {
    entries: Vec<TapeEntry>,
}

impl Tape {
    pub fn push(&mut self, e: TapeEntry) {
        self.entries.push(e);
    }
    pub fn entries(&self) -> &[TapeEntry] {
        &self.entries
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
