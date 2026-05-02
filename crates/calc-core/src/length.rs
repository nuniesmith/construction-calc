//! Exact dimensional length, stored as rational inches.
//!
//! The canonical internal unit is **inches**, kept as an exact `Rational64`.
//! This guarantees that 1/3 + 1/3 + 1/3 == 1 — there is no floating-point
//! drift. Display formatting (feet-inch-fraction, metric, decimal) is a
//! separate concern handled in [`crate::format`].
//!
//! ## Why inches?
//!
//! Either inches or millimeters work as the canonical unit; inches were
//! chosen because the most common framing dimensions and preset fractions
//! (1/2", 1/4", ..., 1/64") yield small denominators. A cleaner millimeter
//! result like `1mm` is `Rational64::new(1, 254) * 10` in inches — still
//! exact, just slightly larger numerators.

use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Sub};

use num_integer::Integer;
use num_rational::Rational64;
use num_traits::{Signed, Zero};

use crate::error::ParseError;

/// Conversion constants, all exact.
pub mod consts {
    use num_rational::Rational64;

    /// Inches per foot.
    pub const IN_PER_FT: i64 = 12;
    /// Inches per yard.
    pub const IN_PER_YD: i64 = 36;

    /// Inches per millimeter: 1 inch = 25.4 mm exactly,
    /// so 1 mm = 10/254 in = 5/127 in.
    pub fn in_per_mm() -> Rational64 {
        Rational64::new(5, 127)
    }
    pub fn in_per_cm() -> Rational64 {
        Rational64::new(50, 127)
    }
    pub fn in_per_m() -> Rational64 {
        Rational64::new(5000, 127)
    }
}

/// An exact length in inches.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Length {
    /// Inches as a rational. Always reduced (num_rational normalizes).
    inches: Rational64,
}

impl Length {
    pub const ZERO: Length = Length {
        inches: Rational64::new_raw(0, 1),
    };

    pub fn from_inches(inches: Rational64) -> Self {
        Self { inches }
    }

    pub fn from_inches_int(n: i64) -> Self {
        Self {
            inches: Rational64::from_integer(n),
        }
    }

    pub fn from_feet_int(ft: i64) -> Self {
        Self::from_inches_int(ft * consts::IN_PER_FT)
    }

    pub fn from_feet_inches(ft: i64, inches: Rational64) -> Self {
        let sign = if ft < 0 { -1 } else { 1 };
        let abs_ft = ft.abs();
        let total =
            Rational64::from_integer(abs_ft * consts::IN_PER_FT) + inches.abs();
        Self {
            inches: total * Rational64::from_integer(sign),
        }
    }

    pub fn from_mm(mm: Rational64) -> Self {
        Self {
            inches: mm * consts::in_per_mm(),
        }
    }

    pub fn from_cm(cm: Rational64) -> Self {
        Self {
            inches: cm * consts::in_per_cm(),
        }
    }

    pub fn from_m(m: Rational64) -> Self {
        Self {
            inches: m * consts::in_per_m(),
        }
    }

    pub fn inches(&self) -> Rational64 {
        self.inches
    }

    pub fn feet(&self) -> Rational64 {
        self.inches / Rational64::from_integer(consts::IN_PER_FT)
    }

    pub fn yards(&self) -> Rational64 {
        self.inches / Rational64::from_integer(consts::IN_PER_YD)
    }

    pub fn millimeters(&self) -> Rational64 {
        self.inches / consts::in_per_mm()
    }

    pub fn centimeters(&self) -> Rational64 {
        self.inches / consts::in_per_cm()
    }

    pub fn meters(&self) -> Rational64 {
        self.inches / consts::in_per_m()
    }

    pub fn is_zero(&self) -> bool {
        self.inches.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.inches.is_negative()
    }

    pub fn abs(&self) -> Self {
        Self {
            inches: self.inches.abs(),
        }
    }

    /// Decompose into (sign, whole_feet, whole_inches, fractional_inches)
    /// where fractional_inches is in `[0, 1)`. Useful for formatting.
    pub fn decompose_feet_inches(&self) -> (i64, u64, u64, Rational64) {
        let sign = if self.inches.is_negative() { -1 } else { 1 };
        let abs = self.inches.abs();
        let total_in_floor = abs.to_integer();
        let frac = abs - Rational64::from_integer(total_in_floor);
        let feet = total_in_floor / consts::IN_PER_FT;
        let inches = total_in_floor % consts::IN_PER_FT;
        (sign, feet as u64, inches as u64, frac)
    }

    /// Round the fractional inch portion to the nearest `1/denom`.
    /// `denom` must be a power of two between 2 and 64.
    /// Used when displaying to a preset fraction like 1/16".
    pub fn round_to_fraction(&self, denom: u32) -> Self {
        let d = denom as i64;
        // value in units of 1/denom inch:
        let scaled = self.inches * Rational64::from_integer(d);
        let rounded = round_half_to_even(scaled);
        Self {
            inches: Rational64::new(rounded, d),
        }
    }

    /// Parse a length expression like:
    /// `8' 5-3/8"`, `5'`, `3 1/2"`, `12.5"`, `2 ft 6 in`, `1500mm`, `1.5m`.
    ///
    /// Tolerant of extra whitespace and unicode quote characters.
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        crate::length::parse::parse_length(s)
    }
}

fn round_half_to_even(r: Rational64) -> i64 {
    // Banker's rounding to avoid bias in repeated rounding.
    let n = *r.numer();
    let d = *r.denom();
    let q = n.div_euclid(d);
    let rem = n - q * d;
    let twice_rem = rem * 2;
    match twice_rem.cmp(&d) {
        core::cmp::Ordering::Less => q,
        core::cmp::Ordering::Greater => q + 1,
        core::cmp::Ordering::Equal => {
            if q.is_even() {
                q
            } else {
                q + 1
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Arithmetic
// ---------------------------------------------------------------------------

impl Add for Length {
    type Output = Length;
    fn add(self, rhs: Length) -> Length {
        Length {
            inches: self.inches + rhs.inches,
        }
    }
}

impl Sub for Length {
    type Output = Length;
    fn sub(self, rhs: Length) -> Length {
        Length {
            inches: self.inches - rhs.inches,
        }
    }
}

impl Neg for Length {
    type Output = Length;
    fn neg(self) -> Length {
        Length {
            inches: -self.inches,
        }
    }
}

impl Mul<Rational64> for Length {
    type Output = Length;
    fn mul(self, rhs: Rational64) -> Length {
        Length {
            inches: self.inches * rhs,
        }
    }
}

impl Div<Rational64> for Length {
    type Output = Length;
    fn div(self, rhs: Rational64) -> Length {
        Length {
            inches: self.inches / rhs,
        }
    }
}

/// Length / Length = Scalar (a pure ratio).
impl Div for Length {
    type Output = Rational64;
    fn div(self, rhs: Length) -> Rational64 {
        self.inches / rhs.inches
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Default display: feet-inch with 1/16" precision.
        write!(
            f,
            "{}",
            crate::format::format_length(self, crate::format::LengthFormat::FeetInchFraction { denom: 16 })
        )
    }
}

pub(crate) mod parse;
