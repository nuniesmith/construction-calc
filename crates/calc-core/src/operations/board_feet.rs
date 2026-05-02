//! Board feet.
//!
//! 1 board foot = 144 cubic inches = 1" × 12" × 12".
//! Formula: BF = (thickness_in × width_in × length_in) / 144.
//!
//! Lumber industry uses nominal sizes (a "2x4" is actually 1.5 × 3.5),
//! but the calculator works with whatever dimensions you put in.

use num_rational::Rational64;

use crate::error::CalcError;
use crate::length::Length;

pub fn board_feet(
    thickness: Length,
    width: Length,
    length: Length,
    quantity: u32,
) -> Result<Rational64, CalcError> {
    if quantity == 0 {
        return Ok(Rational64::from_integer(0));
    }
    let cubic_in = thickness.inches() * width.inches() * length.inches();
    let bf_each = cubic_in / Rational64::from_integer(144);
    Ok(bf_each * Rational64::from_integer(quantity as i64))
}
