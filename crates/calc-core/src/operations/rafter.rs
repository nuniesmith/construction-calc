//! Right-angle / rafter geometry.
//!
//! Pitch is "rise per 12 inches of run", expressed as a rational. The
//! relationship is the standard right triangle:
//!
//!   diagonal² = rise² + run²
//!   pitch     = rise / run        (× 12 to display as "/12")
//!
//! Hip/Valley rafters: the hip rafter on a regular (square) hip roof has
//! a run that is √2 × the common run; for an irregular hip you supply
//! the secondary pitch. We compute `hip_valley` as the rafter length
//! over the hip's diagonal run.
//!
//! Jack rafter difference: the change in length from one common rafter
//! to the next when they are spaced at on-center distance `OC`. The
//! formula is `jack_diff = OC × (diag / run)`.

use num_rational::Rational64;
use num_traits::Zero;

use crate::error::CalcError;
use crate::length::Length;
use crate::numeric::{length_from_inches_rounded, rational_from_f64, rational_to_f64};
use crate::value::Value;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RafterField {
    Pitch,
    Rise,
    Run,
    Diagonal,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartialRafter {
    /// Pitch as `rise/run` ratio (the value, not the "per-12" form).
    pub pitch_ratio: Option<Rational64>,
    pub rise: Option<Length>,
    pub run: Option<Length>,
    pub diagonal: Option<Length>,
    /// On-center spacing for jack rafter calculations.
    pub on_center: Option<Length>,
}

impl PartialRafter {
    pub fn set_field(&mut self, field: RafterField, v: Value) -> Result<(), CalcError> {
        match (field, v) {
            (RafterField::Pitch, Value::Scalar(r)) => {
                // CM Pro convention: pitch entered as "X" means X/12 inches/foot.
                // If r is a small integer like 4, treat it as 4/12.
                let twelve = Rational64::from_integer(12);
                let ratio = if r.numer().abs() <= 24 && *r.denom() == 1 {
                    r / twelve
                } else {
                    r
                };
                self.pitch_ratio = Some(ratio);
            }
            (RafterField::Pitch, Value::Length(l)) => {
                // Less common, but if user enters pitch with units, treat as "rise per foot"
                let ratio = l.inches() / Rational64::from_integer(12);
                self.pitch_ratio = Some(ratio);
            }
            (RafterField::Rise, Value::Length(l)) => self.rise = Some(l),
            (RafterField::Run, Value::Length(l)) => self.run = Some(l),
            (RafterField::Diagonal, Value::Length(l)) => self.diagonal = Some(l),
            _ => return Err(CalcError::TypeMismatch),
        }
        Ok(())
    }

    /// Solve as much as we can given the inputs. Needs at least 2 of
    /// {pitch, rise, run, diagonal}. Returns `Ok(None)` when underdetermined.
    pub fn try_solve(&self) -> Result<Option<RafterSolution>, CalcError> {
        // Determine pitch and one length (run preferred), then compute the rest.
        let (pitch, rise, run, diag) = match self.solve_quad()? {
            Some(t) => t,
            None => return Ok(None),
        };

        let diagonal = diag;
        let hip_valley = compute_regular_hip_valley(rise, run)?;
        let jack_difference = match self.on_center {
            Some(oc) => Length::from_inches(oc.inches() * (diagonal.inches() / run.inches())),
            None => Length::ZERO,
        };

        Ok(Some(RafterSolution {
            pitch_ratio: pitch,
            rise,
            run,
            diagonal,
            hip_valley,
            jack_difference,
        }))
    }

    fn solve_quad(&self) -> Result<Option<(Rational64, Length, Length, Length)>, CalcError> {
        // Strategy: derive whichever pair we don't have from the pair we do.
        // Pitch + run: rise = pitch * run; diag = sqrt(rise²+run²) (irrational → f64 reify)
        // Pitch + rise: run = rise / pitch
        // Pitch + diag: rise = diag * sin(theta), run = diag * cos(theta)
        // Rise + run: pitch = rise/run; diag = sqrt
        // Rise + diag: run = sqrt(diag² - rise²)
        // Run + diag: rise = sqrt(diag² - run²)
        //
        // Where a square-root is needed, we reify to a length rounded to 1/64".

        let p = self.pitch_ratio;
        let rise = self.rise;
        let run = self.run;
        let diag = self.diagonal;

        let count =
            p.is_some() as u8 + rise.is_some() as u8 + run.is_some() as u8 + diag.is_some() as u8;
        if count < 2 {
            return Ok(None);
        }

        let solved = match (p, rise, run, diag) {
            (Some(p), _, Some(run), _) => {
                let rise_l = Length::from_inches(p * run.inches());
                let diag_l = pythag_diagonal(rise_l, run)?;
                (p, rise_l, run, diag_l)
            }
            (Some(p), Some(rise), _, _) => {
                if p.is_zero() {
                    return Ok(None);
                }
                let run_l = Length::from_inches(rise.inches() / p);
                let diag_l = pythag_diagonal(rise, run_l)?;
                (p, rise, run_l, diag_l)
            }
            (Some(p), _, _, Some(diag)) => {
                // tan(theta) = p, so cos = 1/sqrt(1+p²), sin = p/sqrt(1+p²)
                let p_f = rational_to_f64(p);
                let theta = p_f.atan();
                let rise_l = Length::from_inches(diag.inches() * rational_from_f64(theta.sin())?);
                let run_l = Length::from_inches(diag.inches() * rational_from_f64(theta.cos())?);
                (p, rise_l, run_l, diag)
            }
            (_, Some(rise), Some(run), _) => {
                if run.is_zero() {
                    return Ok(None);
                }
                let pitch = rise.inches() / run.inches();
                let diag_l = pythag_diagonal(rise, run)?;
                (pitch, rise, run, diag_l)
            }
            (_, Some(rise), _, Some(diag)) => {
                let run_l = match pythag_leg(diag, rise)? {
                    Some(l) => l,
                    None => return Ok(None),
                };
                let pitch = if run_l.is_zero() {
                    Rational64::zero()
                } else {
                    rise.inches() / run_l.inches()
                };
                (pitch, rise, run_l, diag)
            }
            (_, _, Some(run), Some(diag)) => {
                let rise_l = match pythag_leg(diag, run)? {
                    Some(l) => l,
                    None => return Ok(None),
                };
                let pitch = if run.is_zero() {
                    Rational64::zero()
                } else {
                    rise_l.inches() / run.inches()
                };
                (pitch, rise_l, run, diag)
            }
            _ => return Ok(None),
        };
        Ok(Some(solved))
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RafterSolution {
    /// rise / run ratio (multiply by 12 for "/12" form).
    pub pitch_ratio: Rational64,
    pub rise: Length,
    pub run: Length,
    pub diagonal: Length,
    pub hip_valley: Length,
    pub jack_difference: Length,
}

/// Compute hypotenuse for a right triangle, rounding to 1/64" inch.
fn pythag_diagonal(a: Length, b: Length) -> Result<Length, CalcError> {
    let a2 = rational_to_f64(a.inches()).powi(2);
    let b2 = rational_to_f64(b.inches()).powi(2);
    length_from_inches_rounded((a2 + b2).sqrt(), 64)
}

/// Compute the missing leg given hypotenuse `h` and the other leg `a`.
/// Returns `Ok(None)` when the triangle is impossible (h < a).
fn pythag_leg(h: Length, a: Length) -> Result<Option<Length>, CalcError> {
    let h2 = rational_to_f64(h.inches()).powi(2);
    let a2 = rational_to_f64(a.inches()).powi(2);
    let diff = h2 - a2;
    if diff < 0.0 {
        return Ok(None);
    }
    Ok(Some(length_from_inches_rounded(diff.sqrt(), 64)?))
}

fn compute_regular_hip_valley(rise: Length, run: Length) -> Result<Length, CalcError> {
    // Hip rafter run = run × √2. Hip rafter length = √(rise² + (run√2)²).
    let run_f = rational_to_f64(run.inches());
    let rise_f = rational_to_f64(rise.inches());
    let hip_run = run_f * 2.0_f64.sqrt();
    length_from_inches_rounded((rise_f.powi(2) + hip_run.powi(2)).sqrt(), 64)
}
