//! Domain-specific operations.
//!
//! Each submodule exposes a small, focused API for one family of
//! construction problems. None of them should know about the calculator
//! state machine or the UI — they are pure functions on values.

pub mod board_feet;
pub mod circle;
pub mod compound_miter;
pub mod materials;
pub mod polygon;
pub mod rafter;
pub mod stair;
pub mod trig;
