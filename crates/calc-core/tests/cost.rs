//! Cost-per-unit: price × quantity-in-display-unit = a money total.

use calc_core::Dimension;
use calc_core::calculator::{BinaryOp, Calculator, KeyEvent, LengthUnitKey};
use calc_core::format::{VolumeFormat, format_money};
use num_rational::Rational64;

fn typed(seq: &[KeyEvent]) -> Calculator {
    let mut c = Calculator::new();
    for ev in seq {
        c.handle(ev.clone()).unwrap();
    }
    c
}

#[test]
fn money_formats_with_two_decimals() {
    assert_eq!(format_money(Rational64::from_integer(20)), "$20.00");
    assert_eq!(format_money(Rational64::new(7, 2)), "$3.50");
    assert_eq!(format_money(Rational64::from_integer(420)), "$420.00");
    assert_eq!(format_money(Rational64::from_integer(-5)), "-$5.00");
    // 1/3 dollar rounds to 33 cents.
    assert_eq!(format_money(Rational64::new(1, 3)), "$0.33");
}

#[test]
fn cost_per_linear_foot() {
    // 10 ft @ $2/ft = $20.00, tagged as Money.
    let c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(2),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "$20.00");
    assert_eq!(c.display_dimension(), Dimension::Money);
}

#[test]
fn cost_per_square_foot() {
    // 10' × 12' = 120 sq ft @ $3.50/sq ft = $420.00.
    let c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Op(BinaryOp::Mul),
        KeyEvent::Digit(1),
        KeyEvent::Digit(2),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Equals,
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(3),
        KeyEvent::Decimal,
        KeyEvent::Digit(5),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "$420.00");
}

#[test]
fn cost_per_cubic_yard_follows_the_display_unit() {
    // 10' × 12' × 3' = 360 cu ft; shown as cu yd (13.33); @ $150/cu yd.
    // 360/27 yd³ × $150 = $2000.00 exactly.
    let mut c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Op(BinaryOp::Mul),
        KeyEvent::Digit(1),
        KeyEvent::Digit(2),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Op(BinaryOp::Mul),
        KeyEvent::Digit(3),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Equals,
    ]);
    c.handle(KeyEvent::ConvertVolume(VolumeFormat::CubicYards {
        precision: 2,
    }))
    .unwrap();
    for ev in [
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(1),
        KeyEvent::Digit(5),
        KeyEvent::Digit(0),
        KeyEvent::Equals,
    ] {
        c.handle(ev).unwrap();
    }
    assert_eq!(c.display_string(), "$2000.00");
}

#[test]
fn cost_per_each_on_a_scalar_count() {
    // 5 fixtures @ $3 each = $15.00.
    let c = typed(&[
        KeyEvent::Digit(5),
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(3),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "$15.00");
}

#[test]
fn money_subtotals_fold_through_a_pending_add() {
    // $20 (10 ft @ $2) + $15 (5 @ $3) = $35.00 — cost binds tighter than +.
    let c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(2),
        KeyEvent::Equals,
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(5),
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(3),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "$35.00");
}

#[test]
fn price_must_be_a_plain_number() {
    let mut c = Calculator::new();
    for ev in [
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(2),
        KeyEvent::Unit(LengthUnitKey::Feet),
    ] {
        c.handle(ev).unwrap();
    }
    assert!(
        c.handle(KeyEvent::Equals).is_err(),
        "pricing by a length should fail"
    );
}
