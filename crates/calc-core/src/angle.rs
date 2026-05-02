//! Angles. Stored as exact decimal degrees when possible, with a separate
//! DMS view when the user enters DMS so we don't lose information.
//!
//! For trig we drop to f64 — that's where exactness ends.

use core::fmt;
use num_rational::Rational64;

/// An angle. Internally stored as an exact rational number of degrees,
/// preserving DMS-entered values without rounding.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Angle {
    degrees: Rational64,
}

impl Angle {
    pub const ZERO: Angle = Angle {
        degrees: Rational64::new_raw(0, 1),
    };

    pub fn from_degrees(d: Rational64) -> Self {
        Self { degrees: d }
    }

    pub fn from_dms(d: i64, m: u32, s: Rational64) -> Self {
        let sign = if d < 0 { -1 } else { 1 };
        let abs_d = d.abs();
        let total = Rational64::from_integer(abs_d)
            + Rational64::new(m as i64, 60)
            + s / Rational64::from_integer(3600);
        Self {
            degrees: total * Rational64::from_integer(sign),
        }
    }

    pub fn from_radians(r: f64) -> Self {
        // Lossy by necessity. Pin to an integer milliarcsecond grid so we
        // don't accumulate float fuzz when we round-trip.
        let deg = r.to_degrees();
        let mas = (deg * 3_600_000.0).round() as i64;
        Self {
            degrees: Rational64::new(mas, 3_600_000),
        }
    }

    pub fn degrees(&self) -> Rational64 {
        self.degrees
    }

    pub fn radians(&self) -> f64 {
        let n = *self.degrees.numer() as f64;
        let d = *self.degrees.denom() as f64;
        (n / d).to_radians()
    }

    /// Decompose into (sign, degrees, minutes, seconds_rational).
    pub fn to_dms(&self) -> (i64, u64, u64, Rational64) {
        let sign = if *self.degrees.numer() < 0 { -1 } else { 1 };
        let abs = if sign < 0 { -self.degrees } else { self.degrees };
        let whole_deg = abs.to_integer();
        let rem = abs - Rational64::from_integer(whole_deg);
        let total_min = rem * Rational64::from_integer(60);
        let whole_min = total_min.to_integer();
        let rem_min = total_min - Rational64::from_integer(whole_min);
        let secs = rem_min * Rational64::from_integer(60);
        (sign, whole_deg as u64, whole_min as u64, secs)
    }

    pub fn sin(&self) -> f64 {
        self.radians().sin()
    }
    pub fn cos(&self) -> f64 {
        self.radians().cos()
    }
    pub fn tan(&self) -> f64 {
        self.radians().tan()
    }

    pub fn asin(x: f64) -> Self {
        Self::from_radians(x.asin())
    }
    pub fn acos(x: f64) -> Self {
        Self::from_radians(x.acos())
    }
    pub fn atan(x: f64) -> Self {
        Self::from_radians(x.atan())
    }
    pub fn atan2(y: f64, x: f64) -> Self {
        Self::from_radians(y.atan2(x))
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sign, d, m, s) = self.to_dms();
        let sign_str = if sign < 0 { "-" } else { "" };
        let s_dec = crate::format::rational_to_decimal_string(s, 2);
        write!(f, "{}{}\u{00B0} {}' {}\"", sign_str, d, m, s_dec)
    }
}
