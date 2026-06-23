//! Display formatting for lengths.
//!
//! Formatting is intentionally separate from the [`Length`] type itself.
//! A length carries its exact value; the user picks a display mode.

use core::fmt::Write;

use num_integer::Integer;
use num_rational::Rational64;
use num_traits::Zero;

use crate::angle::Angle;
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
    Centimeters {
        precision: u8,
    },
    Millimeters {
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
            | LengthFormat::Meters { precision }
            | LengthFormat::Centimeters { precision }
            | LengthFormat::Millimeters { precision } => {
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
    /// Lumber board feet (144 in³ each, i.e. 1 ft × 1 ft × 1 in).
    BoardFeet {
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
            | VolumeFormat::Liters { precision }
            | VolumeFormat::BoardFeet { precision } => precision,
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

/// Display format for angle results. Angle is stored as exact decimal
/// degrees; this picks between sexagesimal D°M'S" and decimal degrees.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AngleFormat {
    /// Sexagesimal: `30° 15' 30"`.
    DegMinSec,
    /// Decimal degrees: `30.26°`.
    DecimalDegrees { precision: u8 },
}

impl AngleFormat {
    pub fn validate(self) -> Result<Self, ParseError> {
        match self {
            AngleFormat::DegMinSec => Ok(self),
            AngleFormat::DecimalDegrees { precision } => {
                if precision > MAX_PRECISION {
                    Err(ParseError::InvalidPrecision(precision))
                } else {
                    Ok(self)
                }
            }
        }
    }
}

// Square inches per one target unit, plus precision + suffix. All exact:
// 1 in = 25.4 mm, so 1 m = 5000/127 in and 1 m² = 25_000_000/16_129 in².
// 1 acre = 43_560 ft² = 6_272_640 in².
fn area_parts(fmt: AreaFormat) -> (Rational64, u8, &'static str) {
    match fmt {
        AreaFormat::SquareInches { precision } => (Rational64::from_integer(1), precision, "sq in"),
        AreaFormat::SquareFeet { precision } => (Rational64::from_integer(144), precision, "sq ft"),
        AreaFormat::SquareYards { precision } => {
            (Rational64::from_integer(1296), precision, "sq yd")
        }
        AreaFormat::SquareMeters { precision } => {
            (Rational64::new(25_000_000, 16_129), precision, "sq m")
        }
        AreaFormat::Acres { precision } => (Rational64::from_integer(6_272_640), precision, "ac"),
    }
}

/// The numeric magnitude of an area (exact square inches) in the chosen
/// unit, without the suffix. Used for pricing `$/unit × quantity`.
pub fn area_magnitude(sq_in: Rational64, fmt: AreaFormat) -> Rational64 {
    let (per_unit, _, _) = area_parts(fmt);
    sq_in / per_unit
}

/// Render an area (exact square inches) in the chosen unit.
pub fn format_area(sq_in: Rational64, fmt: AreaFormat) -> String {
    let (per_unit, precision, suffix) = area_parts(fmt);
    format!(
        "{} {}",
        rational_to_decimal_string(sq_in / per_unit, precision),
        suffix
    )
}

// Cubic inches per one target unit, plus precision + suffix. All exact:
// 1 m³ = (5000/127)³ = 125_000_000_000/2_048_383 in³. 1 US gallon = 231 in³.
// 1 cm = 50/127 in, so 1 L = 1000 cm³ = 125_000_000/2_048_383 in³.
fn volume_parts(fmt: VolumeFormat) -> (Rational64, u8, &'static str) {
    match fmt {
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
        VolumeFormat::BoardFeet { precision } => {
            (Rational64::from_integer(144), precision, "bd ft")
        }
    }
}

/// The numeric magnitude of a volume (exact cubic inches) in the chosen
/// unit, without the suffix. Used for pricing `$/unit × quantity`.
pub fn volume_magnitude(cu_in: Rational64, fmt: VolumeFormat) -> Rational64 {
    let (per_unit, _, _) = volume_parts(fmt);
    cu_in / per_unit
}

/// Render a volume (exact cubic inches) in the chosen unit.
pub fn format_volume(cu_in: Rational64, fmt: VolumeFormat) -> String {
    let (per_unit, precision, suffix) = volume_parts(fmt);
    format!(
        "{} {}",
        rational_to_decimal_string(cu_in / per_unit, precision),
        suffix
    )
}

/// Render an angle in the chosen format. `DegMinSec` reuses the `Angle`
/// `Display` impl (`30° 15' 30"`); `DecimalDegrees` shows the exact degrees
/// to `precision` places (`30.26°`).
pub fn format_angle(a: &Angle, fmt: AngleFormat) -> String {
    match fmt {
        AngleFormat::DegMinSec => a.to_string(),
        AngleFormat::DecimalDegrees { precision } => {
            format!(
                "{}\u{00B0}",
                rational_to_decimal_string(a.degrees(), precision)
            )
        }
    }
}

/// Display format for weight results. Weight is stored internally as exact
/// pounds; each variant converts to the chosen unit and renders to
/// `precision` decimal places.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WeightFormat {
    Pounds {
        precision: u8,
    },
    Kilograms {
        precision: u8,
    },
    /// US short ton (2000 lb).
    Tons {
        precision: u8,
    },
    /// Metric tonne (1000 kg).
    Tonnes {
        precision: u8,
    },
}

impl WeightFormat {
    fn precision(self) -> u8 {
        match self {
            WeightFormat::Pounds { precision }
            | WeightFormat::Kilograms { precision }
            | WeightFormat::Tons { precision }
            | WeightFormat::Tonnes { precision } => precision,
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

// Pounds per one target unit, plus precision + suffix. All exact:
// 1 lb = 0.453_592_37 kg exactly, so 1 kg = 100_000_000/45_359_237 lb;
// 1 short ton = 2000 lb; 1 tonne = 1000 kg = 100_000_000_000/45_359_237 lb.
fn weight_parts(fmt: WeightFormat) -> (Rational64, u8, &'static str) {
    match fmt {
        WeightFormat::Pounds { precision } => (Rational64::from_integer(1), precision, "lb"),
        WeightFormat::Kilograms { precision } => {
            (Rational64::new(100_000_000, 45_359_237), precision, "kg")
        }
        WeightFormat::Tons { precision } => (Rational64::from_integer(2000), precision, "ton"),
        WeightFormat::Tonnes { precision } => {
            (Rational64::new(100_000_000_000, 45_359_237), precision, "t")
        }
    }
}

/// Pounds in one of the given weight unit — the factor a typed magnitude is
/// multiplied by to reach the canonical pounds store.
pub fn pounds_per(fmt: WeightFormat) -> Rational64 {
    weight_parts(fmt).0
}

/// The numeric magnitude of a weight (exact pounds) in the chosen unit,
/// without the suffix. Used for pricing `$/unit × quantity`.
pub fn weight_magnitude(pounds: Rational64, fmt: WeightFormat) -> Rational64 {
    pounds / weight_parts(fmt).0
}

/// Render a weight (exact pounds) in the chosen unit.
pub fn format_weight(pounds: Rational64, fmt: WeightFormat) -> String {
    let (per_unit, precision, suffix) = weight_parts(fmt);
    format!(
        "{} {}",
        rational_to_decimal_string(pounds / per_unit, precision),
        suffix
    )
}

/// Render a money amount in dollars, always with two decimal places and a
/// leading `$` (sign before the symbol: `-$5.00`).
pub fn format_money(dollars: Rational64) -> String {
    let cents = (dollars * Rational64::from_integer(100))
        .round()
        .to_integer();
    let sign = if cents < 0 { "-" } else { "" };
    let cents = cents.abs();
    format!("{}${}.{:02}", sign, cents / 100, cents % 100)
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
        LengthFormat::Centimeters { precision } => {
            format_rational_with_unit(length.centimeters(), precision, " cm")
        }
        LengthFormat::Millimeters { precision } => {
            format_rational_with_unit(length.millimeters(), precision, " mm")
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
