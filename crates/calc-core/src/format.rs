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

/// Display format for area results. Area is stored internally as exact
/// square inches; each variant converts to the chosen unit (all conversion
/// factors are exact) and renders to `precision` decimal places.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AreaFormat {
    SquareInches { precision: u8 },
    SquareFeet { precision: u8 },
    SquareYards { precision: u8 },
    SquareMeters { precision: u8 },
    Acres { precision: u8 },
}

impl AreaFormat {
    fn precision(self) -> u8 {
        match self {
            AreaFormat::SquareInches { precision }
            | AreaFormat::SquareFeet { precision }
            | AreaFormat::SquareYards { precision }
            | AreaFormat::SquareMeters { precision }
            | AreaFormat::Acres { precision } => precision,
        }
    }

    pub fn validate(self) -> Result<Self, ParseError> {
        let p = self.precision();
        if p > MAX_PRECISION {
            Err(ParseError::InvalidPrecision(p))
        } else {
            Ok(self)
        }
    }
}

/// Display format for volume results. Volume is stored internally as exact
/// cubic inches; each variant converts to the chosen unit (all conversion
/// factors are exact) and renders to `precision` decimal places.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolumeFormat {
    CubicInches {
        precision: u8,
    },
    CubicFeet {
        precision: u8,
    },
    CubicYards {
        precision: u8,
    },
    CubicMeters {
        precision: u8,
    },
    /// US liquid gallons (231 in³ exactly).
    Gallons {
        precision: u8,
    },
    Liters {
        precision: u8,
    },
}

impl VolumeFormat {
    fn precision(self) -> u8 {
        match self {
            VolumeFormat::CubicInches { precision }
            | VolumeFormat::CubicFeet { precision }
            | VolumeFormat::CubicYards { precision }
            | VolumeFormat::CubicMeters { precision }
            | VolumeFormat::Gallons { precision }
            | VolumeFormat::Liters { precision } => precision,
        }
    }

    pub fn validate(self) -> Result<Self, ParseError> {
        let p = self.precision();
        if p > MAX_PRECISION {
            Err(ParseError::InvalidPrecision(p))
        } else {
            Ok(self)
        }
    }
}

/// Render an area (exact square inches) in the chosen unit.
pub fn format_area(sq_in: Rational64, fmt: AreaFormat) -> String {
    // Square inches per one target unit — all exact. 1 in = 25.4 mm, so
    // 1 m = 5000/127 in and 1 m² = (5000/127)² = 25_000_000/16_129 in².
    // 1 acre = 43_560 ft² = 6_272_640 in².
    let (per_unit, precision, suffix) = match fmt {
        AreaFormat::SquareInches { precision } => (Rational64::from_integer(1), precision, "sq in"),
        AreaFormat::SquareFeet { precision } => (Rational64::from_integer(144), precision, "sq ft"),
        AreaFormat::SquareYards { precision } => {
            (Rational64::from_integer(1296), precision, "sq yd")
        }
        AreaFormat::SquareMeters { precision } => {
            (Rational64::new(25_000_000, 16_129), precision, "sq m")
        }
        AreaFormat::Acres { precision } => (Rational64::from_integer(6_272_640), precision, "ac"),
    };
    format!(
        "{} {}",
        rational_to_decimal_string(sq_in / per_unit, precision),
        suffix
    )
}

/// Render a volume (exact cubic inches) in the chosen unit.
pub fn format_volume(cu_in: Rational64, fmt: VolumeFormat) -> String {
    // Cubic inches per one target unit — all exact. 1 m³ = (5000/127)³ =
    // 125_000_000_000/2_048_383 in³. 1 US gallon = 231 in³. 1 L = 1000 cm³
    // and 1 cm = 50/127 in, so 1 L = 125_000_000/2_048_383 in³.
    let (per_unit, precision, suffix) = match fmt {
        VolumeFormat::CubicInches { precision } => {
            (Rational64::from_integer(1), precision, "cu in")
        }
        VolumeFormat::CubicFeet { precision } => {
            (Rational64::from_integer(1728), precision, "cu ft")
        }
        VolumeFormat::CubicYards { precision } => {
            (Rational64::from_integer(46_656), precision, "cu yd")
        }
        VolumeFormat::CubicMeters { precision } => (
            Rational64::new(125_000_000_000, 2_048_383),
            precision,
            "cu m",
        ),
        VolumeFormat::Gallons { precision } => (Rational64::from_integer(231), precision, "gal"),
        VolumeFormat::Liters { precision } => {
            (Rational64::new(125_000_000, 2_048_383), precision, "L")
        }
    };
    format!(
        "{} {}",
        rational_to_decimal_string(cu_in / per_unit, precision),
        suffix
    )
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
