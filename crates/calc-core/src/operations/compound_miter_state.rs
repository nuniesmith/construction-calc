//! State accumulator for the compound miter family of keys, mirroring
//! `PartialRafter`. The user enters a corner angle and a spring angle in
//! either order; pressing the result keys returns miter or bevel.

use crate::angle::Angle;
use crate::error::CalcError;
use crate::operations::compound_miter::{self, CompoundMiterSolution, SpringMode};
use crate::value::Value;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MiterField {
    Corner,
    Spring,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartialCompoundMiter {
    pub corner: Option<Angle>,
    pub spring: Option<Angle>,
    pub spring_mode: SpringMode,
}

impl PartialCompoundMiter {
    pub fn set_field(&mut self, field: MiterField, v: Value) -> Result<(), CalcError> {
        let angle = match v {
            Value::Angle(a) => a,
            // Bare scalars are interpreted as degrees — same convention as
            // the trig keys.
            Value::Scalar(r) => Angle::from_degrees(r),
            _ => return Err(CalcError::TypeMismatch),
        };
        match field {
            MiterField::Corner => self.corner = Some(angle),
            MiterField::Spring => self.spring = Some(angle),
        }
        Ok(())
    }

    pub fn try_solve(&self) -> Result<Option<CompoundMiterSolution>, CalcError> {
        match (self.corner, self.spring) {
            (Some(c), Some(s)) => Ok(Some(compound_miter::solve(c, s, self.spring_mode)?)),
            _ => Ok(None),
        }
    }
}
