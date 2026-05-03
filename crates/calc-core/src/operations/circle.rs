//! Circle and arc geometry.
//!
//! A circle is fully determined by any one of {radius, diameter,
//! circumference, area}. An arc adds a central angle and gives you
//! arc length and chord length.
//!
//! Formulas:
//!   diameter      = 2r
//!   circumference = 2πr
//!   area          = πr²
//!   arc_length    = r * θ           (θ in radians)
//!   chord         = 2r * sin(θ/2)
//!   segment_area  = ½ r² (θ - sin θ)

use num_rational::Rational64;
use std::f64::consts::PI;

use crate::angle::Angle;
use crate::error::CalcError;
use crate::length::Length;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CircleInput {
    Radius,
    Diameter,
    Circumference,
    Area,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CircleSolution {
    pub radius: Length,
    pub diameter: Length,
    pub circumference: Length,
    /// Square inches.
    pub area: Rational64,
}

pub fn solve_circle(kind: CircleInput, value: Rational64) -> Result<CircleSolution, CalcError> {
    let v = rational_to_f64(value);
    if v <= 0.0 {
        return Err(CalcError::Domain(
            "circle dimension must be positive".to_string(),
        ));
    }

    let radius_in = match kind {
        CircleInput::Radius => v,
        CircleInput::Diameter => v / 2.0,
        CircleInput::Circumference => v / (2.0 * PI),
        CircleInput::Area => (v / PI).sqrt(),
    };

    let radius = length_from_inches_rounded(radius_in);
    let diameter = radius * Rational64::from_integer(2);
    let circ_in = 2.0 * PI * radius_in;
    let area_in2 = PI * radius_in * radius_in;

    Ok(CircleSolution {
        radius,
        diameter,
        circumference: length_from_inches_rounded(circ_in),
        area: rational_from_decimal_with_grid(area_in2, 1_000_000),
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArcSolution {
    pub radius: Length,
    pub central_angle: Angle,
    pub arc_length: Length,
    pub chord_length: Length,
    /// Sagitta — perpendicular distance from chord midpoint to arc.
    pub sagitta: Length,
    /// Area of the circular segment (between chord and arc), sq inches.
    pub segment_area: Rational64,
}

pub fn solve_arc(radius: Length, central_angle: Angle) -> Result<ArcSolution, CalcError> {
    if radius.is_zero() || radius.is_negative() {
        return Err(CalcError::Domain("arc radius must be positive".to_string()));
    }
    let r = rational_to_f64(radius.inches());
    let theta = central_angle.radians();
    if theta <= 0.0 {
        return Err(CalcError::Domain("arc angle must be positive".to_string()));
    }

    let arc_in = r * theta;
    let chord_in = 2.0 * r * (theta / 2.0).sin();
    let sagitta_in = r * (1.0 - (theta / 2.0).cos());
    let segment_in2 = 0.5 * r * r * (theta - theta.sin());

    Ok(ArcSolution {
        radius,
        central_angle,
        arc_length: length_from_inches_rounded(arc_in),
        chord_length: length_from_inches_rounded(chord_in),
        sagitta: length_from_inches_rounded(sagitta_in),
        segment_area: rational_from_decimal_with_grid(segment_in2, 1_000_000),
    })
}

fn rational_to_f64(r: Rational64) -> f64 {
    *r.numer() as f64 / *r.denom() as f64
}

fn rational_from_decimal_with_grid(x: f64, grid: i64) -> Rational64 {
    Rational64::new((x * grid as f64).round() as i64, grid)
}

fn length_from_inches_rounded(inches: f64) -> Length {
    Length::from_inches(Rational64::new((inches * 64.0).round() as i64, 64))
}
