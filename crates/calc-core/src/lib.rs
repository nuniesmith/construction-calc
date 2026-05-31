//! # calc-core
//!
//! Pure-Rust calculator engine for construction math. No I/O, no UI.
//!
//! ## Design notes
//!
//! All linear dimensions are stored as exact rationals in a canonical unit
//! (inches). This means `1/3 + 1/3 + 1/3 == 1` exactly — critical for a
//! calculator framers actually trust.
//!
//! Trig is the unavoidable f64 island. When trig results feed back into a
//! length, we reify by rounding to the user's current display precision.
//!
//! ## Module layout
//!
//! - [`length`]: exact `Length` type (rational inches) + parse/format
//! - [`angle`]: `Angle` with DMS / decimal-degree / radian modes
//! - [`value`]: tagged union of everything the calculator can hold
//! - [`format`]: display formatting (feet-inch-fraction, decimal, metric)
//! - [`error`]: error types used across the engine
//! - [`calculator`]: the state machine
//! - [`tape`]: paperless-tape entries
//! - [`operations`]: domain math (rafter, stair, polygon, etc.)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod angle;
pub mod calculator;
pub mod error;
pub mod format;
pub mod length;
pub(crate) mod numeric;
pub mod operations;
pub mod tape;
pub mod value;

pub use angle::Angle;
pub use calculator::{Calculator, KeyEvent};
pub use error::{CalcError, ParseError};
pub use format::{AreaFormat, LengthFormat, VolumeFormat};
pub use length::Length;
pub use value::{Dimension, Value};
