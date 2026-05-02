//! Equal-sided polygon geometry.
//!
//! Given a number of sides `n` (≥ 3) and one of {side length, apothem,
//! circumradius}, derive the rest plus interior angle, area, and the
//! diagonal between opposite/adjacent vertices.
//!
//! Formulas (regular n-gon):
//!   interior_angle = (n - 2) * 180° / n
//!   apothem        = side / (2 * tan(π/n))
//!   circumradius   = side / (2 * sin(π/n))
//!   area           = (1/2) * n * side * apothem
//!
//! Lengths are reified to 1/64".

use num_rational::Rational64;
use std::f64::consts::PI;

use crate::angle::Angle;
use crate::error::CalcError;
use crate::length::Length;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PolygonInput {
    /// Length of one side.
    Side,
    /// Distance from center to midpoint of a side.
    Apothem,
    /// Distance from center to a vertex.
    Circumradius,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PolygonSolution {
    pub sides: u32,
    pub side_length: Length,
    pub apothem: Length,
    pub circumradius: Length,
    pub interior_angle: Angle,
    pub central_angle: Angle,
    /// Square inches.
    pub area: Rational64,
    pub perimeter: Length,
}

pub fn solve(
    sides: u32,
    input_kind: PolygonInput,
    input_value: Length,
) -> Result<PolygonSolution, CalcError> {
    if sides < 3 {
        return Err(CalcError::Domain(
            "polygon must have at least 3 sides".to_string(),
        ));
    }
    if input_value.is_zero() || input_value.is_negative() {
        return Err(CalcError::Domain(
            "polygon dimension must be positive".to_string(),
        ));
    }

    let n = sides as f64;
    let half_angle = PI / n;
    let in_val = rational_to_f64(input_value.inches());

    let (side_in, apothem_in, circumr_in) = match input_kind {
        PolygonInput::Side => {
            let apo = in_val / (2.0 * half_angle.tan());
            let circ = in_val / (2.0 * half_angle.sin());
            (in_val, apo, circ)
        }
        PolygonInput::Apothem => {
            let side = 2.0 * in_val * half_angle.tan();
            let circ = in_val / half_angle.cos();
            (side, in_val, circ)
        }
        PolygonInput::Circumradius => {
            let side = 2.0 * in_val * half_angle.sin();
            let apo = in_val * half_angle.cos();
            (side, apo, in_val)
        }
    };

    let side_length = length_from_inches_rounded(side_in);
    let apothem = length_from_inches_rounded(apothem_in);
    let circumradius = length_from_inches_rounded(circumr_in);

    let perimeter = side_length * Rational64::from_integer(sides as i64);

    // Area via exact formula on reified side+apothem.
    let area_in2 = side_length.inches()
        * apothem.inches()
        * Rational64::new(sides as i64, 2);

    let interior_deg = ((n - 2.0) * 180.0) / n;
    let central_deg = 360.0 / n;

    Ok(PolygonSolution {
        sides,
        side_length,
        apothem,
        circumradius,
        interior_angle: Angle::from_degrees(rational_from_decimal(interior_deg)),
        central_angle: Angle::from_degrees(rational_from_decimal(central_deg)),
        area: area_in2,
        perimeter,
    })
}

fn rational_to_f64(r: Rational64) -> f64 {
    *r.numer() as f64 / *r.denom() as f64
}

fn rational_from_decimal(x: f64) -> Rational64 {
    let n = (x * 1_000_000.0).round() as i64;
    Rational64::new(n, 1_000_000)
}

fn length_from_inches_rounded(inches: f64) -> Length {
    Length::from_inches(Rational64::new((inches * 64.0).round() as i64, 64))
}
