//! Tagged value carried by the calculator's display register.

use num_rational::Rational64;
use num_traits::Zero;

use crate::angle::Angle;
use crate::error::CalcError;
use crate::length::Length;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    /// A pure number (no units). Used for counts, ratios, board feet, etc.
    Scalar(Rational64),
    /// 1-dimensional: stored as inches.
    Length(Length),
    /// 2-dimensional: stored as square inches.
    Area(Rational64),
    /// 3-dimensional: stored as cubic inches.
    Volume(Rational64),
    Angle(Angle),
}

/// The dimensional category of a [`Value`]. Lets a UI pick the right set of
/// unit / format controls (length vs. area vs. volume) without parsing the
/// formatted display string.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Dimension {
    Scalar,
    Length,
    Area,
    Volume,
    Angle,
}

impl Dimension {
    /// Lowercase tag, matching the serde representation. Handy for the WASM
    /// snapshot and the CLI.
    pub fn as_str(self) -> &'static str {
        match self {
            Dimension::Scalar => "scalar",
            Dimension::Length => "length",
            Dimension::Area => "area",
            Dimension::Volume => "volume",
            Dimension::Angle => "angle",
        }
    }
}

// add/sub/mul/div take a `Result` and use dimension-aware promotion (e.g.
// Length × Length = Area), so they can't be exact `std::ops::*` impls.
// Allow the `should_implement_trait` lint that otherwise fires on the names.
#[allow(clippy::should_implement_trait)]
impl Value {
    pub fn zero_scalar() -> Self {
        Value::Scalar(Rational64::zero())
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Value::Scalar(r) => r.is_zero(),
            Value::Length(l) => l.is_zero(),
            Value::Area(r) => r.is_zero(),
            Value::Volume(r) => r.is_zero(),
            Value::Angle(a) => a.degrees().is_zero(),
        }
    }

    /// The dimensional category of this value.
    pub fn dimension(&self) -> Dimension {
        match self {
            Value::Scalar(_) => Dimension::Scalar,
            Value::Length(_) => Dimension::Length,
            Value::Area(_) => Dimension::Area,
            Value::Volume(_) => Dimension::Volume,
            Value::Angle(_) => Dimension::Angle,
        }
    }

    pub fn add(self, rhs: Value) -> Result<Value, CalcError> {
        match (self, rhs) {
            (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a + b)),
            (Value::Length(a), Value::Length(b)) => Ok(Value::Length(a + b)),
            (Value::Area(a), Value::Area(b)) => Ok(Value::Area(a + b)),
            (Value::Volume(a), Value::Volume(b)) => Ok(Value::Volume(a + b)),
            (Value::Angle(a), Value::Angle(b)) => {
                Ok(Value::Angle(Angle::from_degrees(a.degrees() + b.degrees())))
            }
            _ => Err(CalcError::TypeMismatch),
        }
    }

    pub fn sub(self, rhs: Value) -> Result<Value, CalcError> {
        match (self, rhs) {
            (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a - b)),
            (Value::Length(a), Value::Length(b)) => Ok(Value::Length(a - b)),
            (Value::Area(a), Value::Area(b)) => Ok(Value::Area(a - b)),
            (Value::Volume(a), Value::Volume(b)) => Ok(Value::Volume(a - b)),
            (Value::Angle(a), Value::Angle(b)) => {
                Ok(Value::Angle(Angle::from_degrees(a.degrees() - b.degrees())))
            }
            _ => Err(CalcError::TypeMismatch),
        }
    }

    /// Multiplication promotes dimensions:
    ///   Length * Length = Area
    ///   Area * Length = Volume
    ///   Length * Length * Length = Volume
    ///   Anything * Scalar = same dimension
    pub fn mul(self, rhs: Value) -> Result<Value, CalcError> {
        match (self, rhs) {
            (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
            (Value::Scalar(s), Value::Length(l)) | (Value::Length(l), Value::Scalar(s)) => {
                Ok(Value::Length(l * s))
            }
            (Value::Scalar(s), Value::Area(a)) | (Value::Area(a), Value::Scalar(s)) => {
                Ok(Value::Area(a * s))
            }
            (Value::Scalar(s), Value::Volume(v)) | (Value::Volume(v), Value::Scalar(s)) => {
                Ok(Value::Volume(v * s))
            }
            (Value::Length(a), Value::Length(b)) => Ok(Value::Area(a.inches() * b.inches())),
            (Value::Length(l), Value::Area(a)) | (Value::Area(a), Value::Length(l)) => {
                Ok(Value::Volume(l.inches() * a))
            }
            _ => Err(CalcError::TypeMismatch),
        }
    }

    pub fn div(self, rhs: Value) -> Result<Value, CalcError> {
        if rhs.is_zero() {
            return Err(CalcError::DivByZero);
        }
        match (self, rhs) {
            (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a / b)),
            (Value::Length(l), Value::Scalar(s)) => Ok(Value::Length(l / s)),
            (Value::Length(a), Value::Length(b)) => Ok(Value::Scalar(a / b)),
            (Value::Area(a), Value::Scalar(s)) => Ok(Value::Area(a / s)),
            (Value::Area(a), Value::Length(l)) => {
                Ok(Value::Length(Length::from_inches(a / l.inches())))
            }
            (Value::Volume(v), Value::Scalar(s)) => Ok(Value::Volume(v / s)),
            (Value::Volume(v), Value::Length(l)) => Ok(Value::Area(v / l.inches())),
            (Value::Volume(v), Value::Area(a)) => Ok(Value::Length(Length::from_inches(v / a))),
            _ => Err(CalcError::TypeMismatch),
        }
    }
}
