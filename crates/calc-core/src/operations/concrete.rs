//! Concrete volume estimating.
//!
//! Concrete is priced and delivered by the cubic yard:
//!   1 cubic yard = 27 cubic feet = 46_656 cubic inches.
//!
//! All public functions return volume in **cubic inches as an exact
//! rational**, matching how the engine already represents `Value::Volume`.
//! The frontend / EZ Calc layer divides by 46_656 to display cubic yards.
//!
//! For pours large enough to matter, suppliers typically tack on a 5–10%
//! waste factor (spillage, formwork variance, sub-grade variance). That's
//! a multiplier the caller applies — keeping it out of the engine means
//! the math stays exact.

use num_rational::Rational64;
use std::f64::consts::PI;

use crate::error::CalcError;
use crate::length::Length;
use crate::numeric::{rational_from_f64_on_grid, rational_to_f64};

/// Cubic inches per cubic yard. Useful when converting for display.
pub const CU_IN_PER_CU_YD: i64 = 46_656;

/// Rectangular slab / pad / footing.
///
/// Returns volume in cubic inches (exact rational).
pub fn slab_volume(
    length: Length,
    width: Length,
    thickness: Length,
) -> Result<Rational64, CalcError> {
    if length.is_zero() || length.is_negative() {
        return Err(CalcError::Domain("slab length must be positive".into()));
    }
    if width.is_zero() || width.is_negative() {
        return Err(CalcError::Domain("slab width must be positive".into()));
    }
    if thickness.is_zero() || thickness.is_negative() {
        return Err(CalcError::Domain("slab thickness must be positive".into()));
    }
    Ok(length.inches() * width.inches() * thickness.inches())
}

/// Cylindrical column / sonotube.
///
/// Volume = π × r² × h. The exact-rational engine breaks down at π, so
/// the answer is reified onto a 1/1_000_000 cubic-inch grid — plenty for
/// volume estimates.
pub fn column_volume(diameter: Length, height: Length) -> Result<Rational64, CalcError> {
    if diameter.is_zero() || diameter.is_negative() {
        return Err(CalcError::Domain("column diameter must be positive".into()));
    }
    if height.is_zero() || height.is_negative() {
        return Err(CalcError::Domain("column height must be positive".into()));
    }
    let r = rational_to_f64(diameter.inches()) / 2.0;
    let h = rational_to_f64(height.inches());
    rational_from_f64_on_grid(PI * r * r * h, 1_000_000)
}

/// Conical pile / pier (e.g. a bell-bottom footing tail).
///
/// Volume = (1/3) × π × r² × h.
pub fn cone_volume(diameter: Length, height: Length) -> Result<Rational64, CalcError> {
    if diameter.is_zero() || diameter.is_negative() {
        return Err(CalcError::Domain("cone diameter must be positive".into()));
    }
    if height.is_zero() || height.is_negative() {
        return Err(CalcError::Domain("cone height must be positive".into()));
    }
    let r = rational_to_f64(diameter.inches()) / 2.0;
    let h = rational_to_f64(height.inches());
    rational_from_f64_on_grid(PI * r * r * h / 3.0, 1_000_000)
}

/// Convenience: convert a cubic-inch volume to a `f64` cubic-yard count.
/// Cheap and lossy by design — concrete is measured to 0.25 cy
/// resolution at most.
pub fn cubic_inches_to_cubic_yards(cu_in: Rational64) -> f64 {
    rational_to_f64(cu_in) / CU_IN_PER_CU_YD as f64
}

/// Lateral (side) surface area of a cylindrical column — the area of form
/// material / wrap, *excluding* the two end caps.
///
/// Lateral area = π × d × h. Returned in square inches reified onto a
/// 1/1_000_000 grid, matching the volume helpers.
pub fn column_lateral_area(diameter: Length, height: Length) -> Result<Rational64, CalcError> {
    if diameter.is_zero() || diameter.is_negative() {
        return Err(CalcError::Domain("column diameter must be positive".into()));
    }
    if height.is_zero() || height.is_negative() {
        return Err(CalcError::Domain("column height must be positive".into()));
    }
    let d = rational_to_f64(diameter.inches());
    let h = rational_to_f64(height.inches());
    rational_from_f64_on_grid(PI * d * h, 1_000_000)
}

/// Lateral (side) surface area of a cone — the curved area along the slant,
/// *excluding* the circular base.
///
/// Lateral area = π × r × ℓ, where the slant height ℓ = √(r² + h²).
/// Returned in square inches reified onto a 1/1_000_000 grid.
pub fn cone_lateral_area(diameter: Length, height: Length) -> Result<Rational64, CalcError> {
    if diameter.is_zero() || diameter.is_negative() {
        return Err(CalcError::Domain("cone diameter must be positive".into()));
    }
    if height.is_zero() || height.is_negative() {
        return Err(CalcError::Domain("cone height must be positive".into()));
    }
    let r = rational_to_f64(diameter.inches()) / 2.0;
    let h = rational_to_f64(height.inches());
    let slant = (r * r + h * h).sqrt();
    rational_from_f64_on_grid(PI * r * slant, 1_000_000)
}
