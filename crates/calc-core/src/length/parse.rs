//! Length parser. Tolerant of whitespace and quote variants.
//!
//! Accepts:
//! - `8' 5-3/8"`, `8'5"`, `5'`, `8 ft 5 in`
//! - `3 1/2"`, `3-1/2"`, `1/2 in`
//! - `12.5"`, `12.5 in`
//! - `1500mm`, `150 cm`, `1.5 m`, `1.5m`
//! - `2 yd`, `2yd`
//! - bare numbers are interpreted as inches.

use num_rational::Rational64;
use num_traits::Zero;

use super::{Length, consts};
use crate::error::ParseError;

pub fn parse_length(input: &str) -> Result<Length, ParseError> {
    let s = input.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    // Normalize unicode quotes/dashes to ASCII so the rest is mechanical.
    let normalized: String = s
        .chars()
        .map(|c| match c {
            '\u{2019}' | '\u{2032}' => '\'', // ’ ′ -> '
            '\u{201D}' | '\u{2033}' => '"',  // ” ″ -> "
            '\u{2013}' | '\u{2014}' => '-',  // – —
            _ => c,
        })
        .collect();

    // Pull out an explicit metric/yard suffix first; those modes are
    // unambiguous and easier to handle separately.
    let lower = normalized.to_ascii_lowercase();
    if let Some(stripped) = strip_suffix_word(&lower, &["mm", "millimeters", "millimetres"]) {
        let n = parse_decimal_or_fraction(stripped.trim())?;
        return Ok(Length::from_mm(n));
    }
    if let Some(stripped) = strip_suffix_word(&lower, &["cm", "centimeters", "centimetres"]) {
        let n = parse_decimal_or_fraction(stripped.trim())?;
        return Ok(Length::from_cm(n));
    }
    if let Some(stripped) = strip_suffix_word(&lower, &["m", "meters", "metres"]) {
        let n = parse_decimal_or_fraction(stripped.trim())?;
        return Ok(Length::from_m(n));
    }
    if let Some(stripped) = strip_suffix_word(&lower, &["yd", "yds", "yard", "yards"]) {
        let n = parse_decimal_or_fraction(stripped.trim())?;
        return Ok(Length::from_inches(
            n * Rational64::from_integer(consts::IN_PER_YD),
        ));
    }

    // Imperial mixed: feet (' or ft) and inches (" or in), either may be absent.
    parse_feet_inches(&normalized)
}

/// If `s` ends in any of `words` (as a trailing token), return the prefix.
/// Words are matched against ASCII-lowercased input but trimmed length is
/// used to slice the original.
fn strip_suffix_word<'a>(lower: &'a str, words: &[&str]) -> Option<&'a str> {
    for w in words {
        if let Some(prefix) = lower.strip_suffix(w) {
            // Avoid 'm' matching 'cm'/'mm' tail: require prefix to end with
            // digit or whitespace (not another letter).
            let last = prefix.chars().last();
            match last {
                Some(c) if c.is_alphabetic() => continue,
                _ => return Some(prefix),
            }
        }
    }
    None
}

/// Parse "[feet] [inches][fraction]" with flexible markers.
fn parse_feet_inches(s: &str) -> Result<Length, ParseError> {
    // Tokenize by extracting feet portion (number followed by ' or "ft") if
    // present, then the remainder is inches.
    let lower = s.to_ascii_lowercase();

    // Try to split on ' or " ft" boundary.
    let (feet_part, inches_part) = split_feet_inches(&lower);

    let feet_in = match feet_part {
        Some(fp) => {
            let v = parse_decimal_or_fraction(fp.trim())?;
            v * Rational64::from_integer(consts::IN_PER_FT)
        }
        None => Rational64::zero(),
    };

    let inches = match inches_part {
        Some(ip) => {
            let cleaned = strip_inch_marker(ip.trim());
            if cleaned.is_empty() {
                Rational64::zero()
            } else {
                parse_decimal_or_fraction(cleaned)?
            }
        }
        None => Rational64::zero(),
    };

    // If no feet marker was found AND no inch marker either, bare number is inches.
    Ok(Length::from_inches(feet_in + inches))
}

fn split_feet_inches(s: &str) -> (Option<&str>, Option<&str>) {
    // Look for `'` first.
    if let Some(idx) = s.find('\'') {
        let feet = &s[..idx];
        let rest = &s[idx + 1..];
        return (Some(feet), if rest.is_empty() { None } else { Some(rest) });
    }
    // Then `ft`.
    if let Some(idx) = s.find("ft") {
        let feet = &s[..idx];
        let rest = &s[idx + 2..];
        return (Some(feet), if rest.is_empty() { None } else { Some(rest) });
    }
    (None, Some(s))
}

fn strip_inch_marker(s: &str) -> &str {
    let s = s.trim_end_matches('"');
    let s = s.trim();
    if let Some(stripped) = s.strip_suffix("in") {
        return stripped.trim_end();
    }
    if let Some(stripped) = s.strip_suffix("inches") {
        return stripped.trim_end();
    }
    if let Some(stripped) = s.strip_suffix("inch") {
        return stripped.trim_end();
    }
    s
}

/// Parse a number that may be an integer, decimal, fraction, or mixed number.
/// Examples: `5`, `5.25`, `1/2`, `5 1/2`, `5-1/2`.
pub(crate) fn parse_decimal_or_fraction(s: &str) -> Result<Rational64, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    // Mixed number: "5 1/2" or "5-1/2"
    let separator = s.find([' ', '-']);
    if let Some(idx) = separator {
        let whole = &s[..idx];
        let rest = s[idx + 1..].trim();
        if rest.contains('/') && !whole.is_empty() {
            let w: i64 = whole
                .parse()
                .map_err(|_| ParseError::InvalidNumber(whole.to_string()))?;
            let f = parse_simple_fraction(rest)?;
            let sign = if w < 0 { -1 } else { 1 };
            let total = Rational64::from_integer(w.abs()) + f;
            return Ok(total * Rational64::from_integer(sign));
        }
    }

    // Pure fraction
    if s.contains('/') {
        return parse_simple_fraction(s);
    }

    // Decimal or integer
    parse_decimal(s)
}

fn parse_simple_fraction(s: &str) -> Result<Rational64, ParseError> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidFraction(s.to_string()));
    }
    let num: i64 = parts[0]
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidFraction(s.to_string()))?;
    let den: i64 = parts[1]
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidFraction(s.to_string()))?;
    if den == 0 {
        return Err(ParseError::InvalidFraction(s.to_string()));
    }
    Ok(Rational64::new(num, den))
}

fn parse_decimal(s: &str) -> Result<Rational64, ParseError> {
    if let Ok(n) = s.parse::<i64>() {
        return Ok(Rational64::from_integer(n));
    }
    // Decimal like "12.5" or "-0.125": convert exactly via the digit string,
    // never f64.
    let (sign, body) = if let Some(rest) = s.strip_prefix('-') {
        (-1i64, rest)
    } else if let Some(rest) = s.strip_prefix('+') {
        (1i64, rest)
    } else {
        (1i64, s)
    };

    let mut parts = body.split('.');
    let int_part = parts.next().unwrap_or("0");
    let frac_part = parts.next().unwrap_or("");
    if parts.next().is_some() {
        return Err(ParseError::InvalidNumber(s.to_string()));
    }

    let int_val: i64 = if int_part.is_empty() {
        0
    } else {
        int_part
            .parse()
            .map_err(|_| ParseError::InvalidNumber(s.to_string()))?
    };

    if frac_part.is_empty() {
        return Ok(Rational64::from_integer(sign * int_val));
    }

    if !frac_part.chars().all(|c| c.is_ascii_digit()) {
        return Err(ParseError::InvalidNumber(s.to_string()));
    }

    let frac_num: i64 = frac_part
        .parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
    let denom: i64 = 10i64
        .checked_pow(frac_part.len() as u32)
        .ok_or_else(|| ParseError::InvalidNumber(s.to_string()))?;

    let total = Rational64::from_integer(int_val) + Rational64::new(frac_num, denom);
    Ok(total * Rational64::from_integer(sign))
}
