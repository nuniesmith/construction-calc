//! Stair layout.
//!
//! Given total rise (floor-to-floor), compute a riser/tread plan that
//! satisfies common building-code constraints. The CM Pro behaviour is:
//!
//! 1. User enters total rise (and optionally total run, riser height
//!    target, or stairwell opening).
//! 2. Calculator picks the riser count that brings the per-riser height
//!    closest to the user's preferred target (default 7-1/2").
//! 3. From that, derive tread depth, stair length, headroom check.

use crate::error::CalcError;
use crate::length::Length;
use crate::numeric::{length_from_inches_rounded, rational_to_f64};
use num_rational::Rational64;

#[derive(Clone, Debug, PartialEq)]
pub struct StairInputs {
    pub total_rise: Length,
    /// Preferred per-riser height (default 7-1/2").
    pub target_riser: Length,
    /// Preferred tread depth (default 10").
    pub target_tread: Length,
    /// Optional riser-height upper limit (code: typically 7-3/4" residential).
    pub max_riser: Option<Length>,
    /// Optional tread-depth lower limit (code: typically 10" residential).
    pub min_tread: Option<Length>,
}

impl Default for StairInputs {
    fn default() -> Self {
        Self {
            total_rise: Length::ZERO,
            target_riser: Length::from_inches(Rational64::new(15, 2)), // 7.5"
            target_tread: Length::from_inches(Rational64::from_integer(10)),
            max_riser: Some(Length::from_inches(Rational64::new(31, 4))), // 7.75"
            min_tread: Some(Length::from_inches(Rational64::from_integer(10))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StairSolution {
    pub riser_count: u32,
    pub riser_height: Length,
    pub tread_count: u32,
    pub tread_depth: Length,
    pub total_run: Length,
    /// Stringer (diagonal) length, rounded to 1/64".
    pub stringer_length: Length,
}

pub fn solve(inputs: &StairInputs) -> Result<StairSolution, CalcError> {
    if inputs.total_rise.is_zero() || inputs.total_rise.is_negative() {
        return Err(CalcError::Domain("total rise must be positive".to_string()));
    }

    // Pick riser count that minimizes |rise/n - target_riser|.
    let total_rise_in = rational_to_f64(inputs.total_rise.inches());
    let target_in = rational_to_f64(inputs.target_riser.inches());
    let approx_n = (total_rise_in / target_in).round() as i32;

    // Candidate range around approx_n; pick the legal one closest to target.
    let mut best: Option<(u32, f64)> = None;
    for delta in -2..=2 {
        let n = approx_n + delta;
        if n < 2 {
            continue;
        }
        let n_u = n as u32;
        let per = total_rise_in / n as f64;
        if let Some(max_r) = inputs.max_riser {
            if per > rational_to_f64(max_r.inches()) {
                continue;
            }
        }
        let cost = (per - target_in).abs();
        if best.is_none_or(|(_, c)| cost < c) {
            best = Some((n_u, cost));
        }
    }
    let (riser_count, _) =
        best.ok_or_else(|| CalcError::Domain("no legal riser configuration found".to_string()))?;

    // Riser height = total_rise / riser_count, exact.
    let riser_height = inputs.total_rise / Rational64::from_integer(riser_count as i64);

    // Tread count = riser_count - 1 (top landing is not a tread).
    let tread_count = riser_count - 1;

    let tread_depth = inputs.target_tread;
    let total_run = tread_depth * Rational64::from_integer(tread_count as i64);

    // Stringer = sqrt(total_rise² + total_run²), reified to 1/64".
    let r_in = rational_to_f64(inputs.total_rise.inches());
    let run_in = rational_to_f64(total_run.inches());
    let stringer_in = (r_in * r_in + run_in * run_in).sqrt();
    let stringer_length = length_from_inches_rounded(stringer_in, 64)?;

    Ok(StairSolution {
        riser_count,
        riser_height,
        tread_count,
        tread_depth,
        total_run,
        stringer_length,
    })
}
