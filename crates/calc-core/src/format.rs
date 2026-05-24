//! Display formatting for lengths.
//!
//! Formatting is intentionally separate from the [`Length`] type itself.
//! A length carries its exact value; the user picks a display mode.

use core::fmt::Write;

use num_integer::Integer;
use num_rational::Rational64;
use num_traits::Zero;

use crate::error::ParseError;
use crate::length::Length;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LengthFormat {
    /// e.g. `8' 5-3/8"`. `denom` is the rounding resolution and must be a
    /// power of two from 2 to 16 (1/2", 1/4", 1/8", 1/16").
    FeetInchFraction {
        denom: u32,
    },
    /// e.g. `8.4479'`
    DecimalFeet {
        precision: u8,
    },
    /// e.g. `101.375"`
    DecimalInches {
        precision: u8,
    },
    Yards {
        precision: u8,
    },
    Meters {
        precision: u8,
    },
}

/// Caller can ask for arbitrary precision in the conversion API; this is the
/// generous upper bound we'll honour. Anything more is almost certainly a
/// caller bug — `f64` only has ~15 significant digits anyway.
const MAX_PRECISION: u8 = 12;

impl LengthFormat {
    pub fn validate(self) -> Result<Self, ParseError> {
        let check_precision = |p: u8| -> Result<(), ParseError> {
            if p > MAX_PRECISION {
                Err(ParseError::InvalidPrecision(p))
            } else {
                Ok(())
            }
        };
        match self {
            LengthFormat::FeetInchFraction { denom } => {
                if !matches!(denom, 2 | 4 | 8 | 16) {
                    return Err(ParseError::InvalidDenominator(denom));
                }
                Ok(self)
            }
            LengthFormat::DecimalFeet { precision }
            | LengthFormat::DecimalInches { precision }
            | LengthFormat::Yards { precision }
            | LengthFormat::Meters { precision } => {
                check_precision(precision)?;
                Ok(self)
            }
        }
    }
}

pub fn format_length(length: &Length, fmt: LengthFormat) -> String {
    match fmt {
        LengthFormat::FeetInchFraction { denom } => format_feet_inch_fraction(length, denom),
        LengthFormat::DecimalFeet { precision } => {
            format_rational_with_unit(length.feet(), precision, "'")
        }
        LengthFormat::DecimalInches { precision } => {
            format_rational_with_unit(length.inches(), precision, "\"")
        }
        LengthFormat::Yards { precision } => {
            format_rational_with_unit(length.yards(), precision, " yd")
        }
        LengthFormat::Meters { precision } => {
            format_rational_with_unit(length.meters(), precision, " m")
        }
    }
}

fn format_feet_inch_fraction(length: &Length, denom: u32) -> String {
    let rounded = length.round_to_fraction(denom);
    let (sign, feet, inches, frac) = rounded.decompose_feet_inches();

    let mut out = String::new();
    if sign < 0 {
        out.push('-');
    }

    let frac_str = format_proper_fraction(frac);

    match (feet, inches, frac_str.as_str()) {
        (0, 0, "") => out.push_str("0\""),
        (f, 0, "") => write!(out, "{}'", f).unwrap(),
        (0, i, "") => write!(out, "{}\"", i).unwrap(),
        (0, 0, fr) => write!(out, "{}\"", fr).unwrap(),
        (f, i, "") => write!(out, "{}' {}\"", f, i).unwrap(),
        (f, 0, fr) => write!(out, "{}' {}\"", f, fr).unwrap(),
        (0, i, fr) => write!(out, "{}-{}\"", i, fr).unwrap(),
        (f, i, fr) => write!(out, "{}' {}-{}\"", f, i, fr).unwrap(),
    }
    out
}

/// Format the fractional inch part as a proper reduced fraction,
/// or an empty string if it rounds to zero.
fn format_proper_fraction(frac: Rational64) -> String {
    if frac.is_zero() {
        return String::new();
    }
    // num_rational normalizes; just present numerator/denominator.
    let n = *frac.numer();
    let d = *frac.denom();
    let g = n.gcd(&d);
    format!("{}/{}", n / g, d / g)
}

fn format_rational_with_unit(r: Rational64, precision: u8, unit: &str) -> String {
    format!("{}{}", rational_to_decimal_string(r, precision), unit)
}

/// Convert a rational to a decimal string with at most `precision` digits
/// after the decimal point, trimming trailing zeros.
pub fn rational_to_decimal_string(r: Rational64, precision: u8) -> String {
    let n = *r.numer();
    let d = *r.denom();
    let sign = if (n < 0) ^ (d < 0) { "-" } else { "" };
    let n = n.abs();
    let d = d.abs();

    let int_part = n / d;
    let mut rem = n % d;

    let mut frac = String::new();
    let mut emitted = 0u8;
    while rem != 0 && emitted < precision {
        rem *= 10;
        let digit = rem / d;
        rem %= d;
        frac.push(char::from_digit(digit as u32, 10).unwrap());
        emitted += 1;
    }
    // Round the next digit if we cut off
    if rem != 0 {
        let next = (rem * 10) / d;
        if next >= 5 {
            // Round up the trailing digits
            frac = round_up_decimal(frac);
        }
    }
    // Trim trailing zeros and trailing dot
    while frac.ends_with('0') {
        frac.pop();
    }

    if frac.is_empty() {
        format!("{}{}", sign, int_part)
    } else {
        format!("{}{}.{}", sign, int_part, frac)
    }
}

fn round_up_decimal(s: String) -> String {
    let mut bytes: Vec<u8> = s.into_bytes();
    let mut i = bytes.len();
    while i > 0 {
        i -= 1;
        if bytes[i] == b'9' {
            bytes[i] = b'0';
            if i == 0 {
                bytes.insert(0, b'1');
                return String::from_utf8(bytes).unwrap();
            }
        } else {
            bytes[i] += 1;
            return String::from_utf8(bytes).unwrap();
        }
    }
    String::from_utf8(bytes).unwrap()
}
