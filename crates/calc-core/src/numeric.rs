//! Shared numeric helpers for the f64 trig/sqrt island.
//!
//! Several modules need the same handful of conversions: `Rational64 → f64`
//! for handing values to libm, and `f64 → Rational64` (or `Length`) on a
//! coarse grid for reifying the result. They were previously duplicated five
//! times with subtly different overflow behaviour. This module is the single
//! source of truth.

use num_rational::Rational64;

use crate::error::CalcError;
use crate::length::Length;

/// Convert a rational to f64 for handing to libm. Lossy for large
/// numerators/denominators but adequate for trig.
pub(crate) fn rational_to_f64(r: Rational64) -> f64 {
    *r.numer() as f64 / *r.denom() as f64
}

/// Pin an f64 to a rational on a 1/`grid` grid, refusing non-finite or
/// out-of-range values rather than silently saturating to `i64::MAX`.
pub(crate) fn rational_from_f64_on_grid(x: f64, grid: i64) -> Result<Rational64, CalcError> {
    if !x.is_finite() {
        return Err(CalcError::Domain(format!("non-finite result: {x}")));
    }
    let scaled = x * grid as f64;
    if !scaled.is_finite() || scaled.abs() >= i64::MAX as f64 {
        return Err(CalcError::Domain(format!("result out of range: {x}")));
    }
    Ok(Rational64::new(scaled.round() as i64, grid))
}

/// One-millionth-grid version, fine enough for trig results that don't
/// feed back into a length.
pub(crate) fn rational_from_f64(x: f64) -> Result<Rational64, CalcError> {
    rational_from_f64_on_grid(x, 1_000_000)
}

/// Reify an f64-inch value to a `Length` rounded to 1/`denom`. The default
/// for construction work is 1/64".
pub(crate) fn length_from_inches_rounded(inches: f64, denom: i64) -> Result<Length, CalcError> {
    Ok(Length::from_inches(rational_from_f64_on_grid(
        inches, denom,
    )?))
}
