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

    /// Pretty-print to Markdown, suitable for sharing in Slack/iMessage/etc.
    /// Format keeps lengths in feet-inch-fraction at 1/16" — same default
    /// as the calculator display. Caller can post-process if needed.
    pub fn to_markdown(&self) -> String {
        use core::fmt::Write;
        let mut out = String::new();
        let _ = writeln!(out, "# Calculator tape");
        let _ = writeln!(out);
        if self.entries.is_empty() {
            let _ = writeln!(out, "_(empty)_");
            return out;
        }
        for (i, e) in self.entries.iter().enumerate() {
            match e {
                TapeEntry::Result(v) => {
                    let _ = writeln!(out, "{}. **=** {}", i + 1, fmt_value(v));
                }
                TapeEntry::RafterSolution(r) => {
                    let _ = writeln!(out, "{}. **Rafter solution**", i + 1);
                    let pitch_per_12 = r.pitch_ratio * num_rational::Rational64::from_integer(12);
                    let _ = writeln!(
                        out,
                        "    - pitch: {}/12",
                        crate::format::rational_to_decimal_string(pitch_per_12, 3)
                    );
                    let _ = writeln!(out, "    - rise: {}", fmt_length(&r.rise));
                    let _ = writeln!(out, "    - run: {}", fmt_length(&r.run));
                    let _ = writeln!(out, "    - diagonal: {}", fmt_length(&r.diagonal));
                    let _ = writeln!(out, "    - hip/valley: {}", fmt_length(&r.hip_valley));
                    if !r.jack_difference.is_zero() {
                        let _ = writeln!(
                            out,
                            "    - jack difference: {}",
                            fmt_length(&r.jack_difference)
                        );
                    }
                }
                TapeEntry::Note(s) => {
                    let _ = writeln!(out, "{}. _note:_ {}", i + 1, s);
                }
            }
        }
        out
    }
}

fn fmt_length(l: &crate::length::Length) -> String {
    crate::format::format_length(
        l,
        crate::format::LengthFormat::FeetInchFraction { denom: 16 },
    )
}

fn fmt_value(v: &Value) -> String {
    match v {
        Value::Scalar(r) => crate::format::rational_to_decimal_string(*r, 6),
        Value::Length(l) => fmt_length(l),
        Value::Area(r) => format!("{} sq in", crate::format::rational_to_decimal_string(*r, 4)),
        Value::Volume(r) => format!("{} cu in", crate::format::rational_to_decimal_string(*r, 4)),
        Value::Angle(a) => a.to_string(),
        Value::Money(r) => crate::format::format_money(*r),
    }
}
