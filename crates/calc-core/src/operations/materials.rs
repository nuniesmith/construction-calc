//! Material estimating: drywall sheets, plywood, roofing bundles.
//!
//! These are deliberately simple integer-rounding helpers; real estimates
//! account for waste factors, joist orientation, etc., which we layer on
//! top via the optional `waste_factor` parameter.

use num_rational::Rational64;

use crate::error::CalcError;
use crate::length::Length;

/// A standard sheet size, e.g. 4'×8' drywall.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SheetSize {
    pub width: SheetDim,
    pub height: SheetDim,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SheetDim {
    Feet(u32),
}

impl SheetSize {
    pub fn drywall_4x8() -> Self {
        Self {
            width: SheetDim::Feet(4),
            height: SheetDim::Feet(8),
        }
    }
    pub fn drywall_4x12() -> Self {
        Self {
            width: SheetDim::Feet(4),
            height: SheetDim::Feet(12),
        }
    }
    pub fn area_sq_in(&self) -> Rational64 {
        let w = match self.width {
            SheetDim::Feet(f) => f as i64 * 12,
        };
        let h = match self.height {
            SheetDim::Feet(f) => f as i64 * 12,
        };
        Rational64::from_integer(w * h)
    }
}

/// How many sheets does it take to cover a given area? Rounds up.
pub fn sheets_for_area(
    area_sq_in: Rational64,
    sheet: SheetSize,
    waste_factor: f64,
) -> Result<u32, CalcError> {
    if waste_factor < 0.0 {
        return Err(CalcError::Domain(
            "waste factor must be non-negative".to_string(),
        ));
    }
    let area_per_sheet = sheet.area_sq_in();
    if area_per_sheet <= Rational64::from_integer(0) {
        return Err(CalcError::Domain("invalid sheet size".to_string()));
    }
    let needed = area_sq_in / area_per_sheet;
    let needed_f = *needed.numer() as f64 / *needed.denom() as f64;
    let with_waste = needed_f * (1.0 + waste_factor);
    Ok(with_waste.ceil() as u32)
}

/// Sheets for a wall of the given dimensions, no openings deducted.
pub fn sheets_for_wall(
    wall_width: Length,
    wall_height: Length,
    sheet: SheetSize,
    waste_factor: f64,
) -> Result<u32, CalcError> {
    let area = wall_width.inches() * wall_height.inches();
    sheets_for_area(area, sheet, waste_factor)
}

/// Roofing: how many bundles for a given roof area, where each bundle
/// covers 33.3 sq ft (3 bundles per square = 100 sq ft) for typical
/// 3-tab asphalt shingles. `bundle_coverage_sq_in` lets the caller
/// override for laminates, etc.
pub fn roofing_bundles(
    roof_area_sq_in: Rational64,
    bundle_coverage_sq_ft: f64,
    waste_factor: f64,
) -> Result<u32, CalcError> {
    if bundle_coverage_sq_ft <= 0.0 {
        return Err(CalcError::Domain(
            "bundle coverage must be positive".to_string(),
        ));
    }
    let area_sq_ft = (*roof_area_sq_in.numer() as f64 / *roof_area_sq_in.denom() as f64) / 144.0;
    let bundles = (area_sq_ft / bundle_coverage_sq_ft) * (1.0 + waste_factor);
    Ok(bundles.ceil() as u32)
}

/// Studs needed for a wall at given on-center spacing, plus the standard
/// extras (one for each end, one extra at every corner — caller adds those).
pub fn studs_on_center(wall_length: Length, on_center: Length) -> Result<u32, CalcError> {
    if on_center.is_zero() || on_center.is_negative() {
        return Err(CalcError::Domain(
            "on-center spacing must be positive".to_string(),
        ));
    }
    let ratio = wall_length.inches() / on_center.inches();
    let f = *ratio.numer() as f64 / *ratio.denom() as f64;
    // ceil(length / OC) + 1 to include both ends.
    Ok(f.ceil() as u32 + 1)
}
