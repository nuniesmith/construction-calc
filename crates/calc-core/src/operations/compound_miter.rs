//! Compound miter and crown angle.
//!
//! When you cut crown molding (or any sloped trim) at a corner, a single
//! miter angle isn't enough — the saw blade also has to tilt. The
//! compound-miter problem solves for both:
//!
//!   - `miter_angle`: the angle on the horizontal scale (saw rotation)
//!   - `bevel_angle`: the angle the blade tilts off vertical
//!
//! given:
//!
//!   - `corner_angle`: the angle of the wall corner, viewed from above.
//!     90° for a standard inside or outside corner.
//!   - `spring_angle`: how the molding springs out from the wall — i.e.
//!     the angle between the back of the molding and the wall. 38° and
//!     45° are the common values printed on crown.
//!
//! Derivation: the standard compound-miter formula operates on the
//! angle the molding face makes with the **ceiling**, not the wall.
//! Spring-from-wall and slope-from-ceiling are complementary (sum to
//! 90°), so we convert internally:
//!
//! ```text
//! slope = 90° − spring_from_wall
//! half  = corner / 2
//! tan(miter) = cos(slope) * tan(half)
//! sin(bevel) = sin(slope) * sin(half)
//! ```
//!
//! For the canonical 90° corner / 38° spring case this yields
//! 31.62°/33.86°, matching the printed crown-cut tables.

use crate::angle::Angle;
use crate::error::CalcError;

/// Two ways the user can describe their molding's spring angle.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum SpringMode {
    /// Angle between molding back and wall (38° and 45° are standard).
    #[default]
    SpringFromWall,
    /// Angle between molding back and ceiling. Equal to 90° − spring-from-wall.
    SlopeFromCeiling,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompoundMiterSolution {
    /// Saw rotation angle (degrees off the perpendicular).
    pub miter_angle: Angle,
    /// Blade tilt angle (degrees off vertical).
    pub bevel_angle: Angle,
    /// The corner angle that was used (echoed back for clarity).
    pub corner_angle: Angle,
    /// The spring angle, normalised to spring-from-wall.
    pub spring_from_wall: Angle,
}

pub fn solve(
    corner: Angle,
    spring: Angle,
    spring_mode: SpringMode,
) -> Result<CompoundMiterSolution, CalcError> {
    let corner_deg = angle_deg(corner);
    if corner_deg <= 0.0 || corner_deg >= 360.0 {
        return Err(CalcError::Domain(format!(
            "corner angle must be in (0°, 360°), got {corner_deg}°"
        )));
    }
    let spring_from_wall_deg = match spring_mode {
        SpringMode::SpringFromWall => angle_deg(spring),
        SpringMode::SlopeFromCeiling => 90.0 - angle_deg(spring),
    };
    if !(0.0..90.0).contains(&spring_from_wall_deg) {
        return Err(CalcError::Domain(format!(
            "spring-from-wall must be in [0°, 90°), got {spring_from_wall_deg}°"
        )));
    }

    // The published formula operates on slope-from-ceiling.
    let slope_rad = (90.0 - spring_from_wall_deg).to_radians();
    let half_corner_rad = (corner_deg / 2.0).to_radians();

    let miter_rad = (slope_rad.cos() * half_corner_rad.tan()).atan();
    // Clamp before asin to absorb any drift just outside [-1, 1].
    let bevel_input = (slope_rad.sin() * half_corner_rad.sin()).clamp(-1.0, 1.0);
    let bevel_rad = bevel_input.asin();

    Ok(CompoundMiterSolution {
        miter_angle: Angle::from_radians(miter_rad),
        bevel_angle: Angle::from_radians(bevel_rad),
        corner_angle: corner,
        spring_from_wall: Angle::from_radians(spring_from_wall_deg.to_radians()),
    })
}

fn angle_deg(a: Angle) -> f64 {
    let d = a.degrees();
    *d.numer() as f64 / *d.denom() as f64
}
