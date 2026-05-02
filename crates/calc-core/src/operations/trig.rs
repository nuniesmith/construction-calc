//! Trigonometric operations.
//!
//! Trig is the f64 island. We accept the precision loss but pin results
//! to a milliarcsecond grid (via `Angle::from_radians`) so that repeated
//! conversions don't accumulate drift.
//!
//! The forward operations (sin/cos/tan) take an `Angle` and return a
//! plain `f64` ratio — most useful when the caller is going to multiply
//! that ratio by a length anyway and reify it.

use crate::angle::Angle;
use crate::error::CalcError;

pub fn sin(a: Angle) -> f64 {
    a.sin()
}
pub fn cos(a: Angle) -> f64 {
    a.cos()
}
pub fn tan(a: Angle) -> f64 {
    a.tan()
}

pub fn asin(x: f64) -> Result<Angle, CalcError> {
    if !(-1.0..=1.0).contains(&x) {
        return Err(CalcError::Domain(format!(
            "asin domain: {} not in [-1, 1]",
            x
        )));
    }
    Ok(Angle::asin(x))
}

pub fn acos(x: f64) -> Result<Angle, CalcError> {
    if !(-1.0..=1.0).contains(&x) {
        return Err(CalcError::Domain(format!(
            "acos domain: {} not in [-1, 1]",
            x
        )));
    }
    Ok(Angle::acos(x))
}

pub fn atan(x: f64) -> Angle {
    Angle::atan(x)
}

pub fn atan2(y: f64, x: f64) -> Angle {
    Angle::atan2(y, x)
}
