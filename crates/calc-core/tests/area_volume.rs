//! Area / volume display formats and the convert events that select them.

use calc_core::Dimension;
use calc_core::calculator::{BinaryOp, Calculator, KeyEvent, LengthUnitKey};
use calc_core::format::{AreaFormat, VolumeFormat, format_area, format_volume};
use num_rational::Rational64;

fn typed(seq: &[KeyEvent]) -> Calculator {
    let mut c = Calculator::new();
    for ev in seq {
        c.handle(ev.clone()).unwrap();
    }
    c
}

fn int(n: i64) -> Rational64 {
    Rational64::from_integer(n)
}

// --- direct formatter conversions (all exact) -----------------------------

#[test]
fn area_unit_conversions_are_exact() {
    // 17_280 in² = 120 ft² = 13.33 yd².
    assert_eq!(
        format_area(int(17_280), AreaFormat::SquareFeet { precision: 2 }),
        "120 sq ft"
    );
    assert_eq!(
        format_area(int(17_280), AreaFormat::SquareYards { precision: 2 }),
        "13.33 sq yd"
    );
    assert_eq!(
        format_area(int(17_280), AreaFormat::SquareInches { precision: 0 }),
        "17280 sq in"
    );
    // 1 acre = 6_272_640 in²; 1 m² = 25_000_000/16_129 in².
    assert_eq!(
        format_area(int(6_272_640), AreaFormat::Acres { precision: 4 }),
        "1 ac"
    );
    assert_eq!(
        format_area(
            Rational64::new(25_000_000, 16_129),
            AreaFormat::SquareMeters { precision: 4 }
        ),
        "1 sq m"
    );
}

#[test]
fn volume_unit_conversions_are_exact() {
    // 1 cu yd = 46_656 in³ = 27 cu ft.
    assert_eq!(
        format_volume(int(46_656), VolumeFormat::CubicYards { precision: 2 }),
        "1 cu yd"
    );
    assert_eq!(
        format_volume(int(46_656), VolumeFormat::CubicFeet { precision: 2 }),
        "27 cu ft"
    );
    // 1 US gallon = 231 in³ exactly.
    assert_eq!(
        format_volume(int(231), VolumeFormat::Gallons { precision: 2 }),
        "1 gal"
    );
    // 1 L = 125_000_000/2_048_383 in³; 1 m³ = 125_000_000_000/2_048_383 in³.
    assert_eq!(
        format_volume(
            Rational64::new(125_000_000, 2_048_383),
            VolumeFormat::Liters { precision: 4 }
        ),
        "1 L"
    );
    assert_eq!(
        format_volume(
            Rational64::new(125_000_000_000, 2_048_383),
            VolumeFormat::CubicMeters { precision: 4 }
        ),
        "1 cu m"
    );
}

#[test]
fn board_feet_conversion_is_exact() {
    // 1 board foot = 144 in³ (1 ft × 1 ft × 1 in); 1 cu ft = 1728 in³ = 12 bd ft.
    assert_eq!(
        format_volume(int(144), VolumeFormat::BoardFeet { precision: 2 }),
        "1 bd ft"
    );
    assert_eq!(
        format_volume(int(1728), VolumeFormat::BoardFeet { precision: 0 }),
        "12 bd ft"
    );
}

// --- through the calculator -----------------------------------------------

#[test]
fn length_times_length_displays_square_feet_by_default() {
    // 10' × 12' = 120 sq ft, and the display reports an Area dimension.
    let c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Op(BinaryOp::Mul),
        KeyEvent::Digit(1),
        KeyEvent::Digit(2),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "120 sq ft");
    assert_eq!(c.display_dimension(), Dimension::Area);
}

#[test]
fn convert_area_switches_the_live_display() {
    // Same 120 sq ft, then convert to square yards and to acres.
    let mut c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Op(BinaryOp::Mul),
        KeyEvent::Digit(1),
        KeyEvent::Digit(2),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Equals,
    ]);
    c.handle(KeyEvent::ConvertArea(AreaFormat::SquareYards {
        precision: 2,
    }))
    .unwrap();
    assert_eq!(c.display_string(), "13.33 sq yd");
    c.handle(KeyEvent::ConvertArea(AreaFormat::SquareInches {
        precision: 0,
    }))
    .unwrap();
    assert_eq!(c.display_string(), "17280 sq in");
}

#[test]
fn length_cubed_displays_cubic_feet_then_converts_to_yards() {
    // 10' × 12' × 3' = 360 cu ft = 13.33 cu yd.
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
    assert_eq!(c.display_string(), "360 cu ft");
    assert_eq!(c.display_dimension(), Dimension::Volume);

    c.handle(KeyEvent::ConvertVolume(VolumeFormat::CubicYards {
        precision: 2,
    }))
    .unwrap();
    assert_eq!(c.display_string(), "13.33 cu yd");
}

#[test]
fn convert_format_rejects_excessive_precision() {
    let mut c = Calculator::new();
    let err = c.handle(KeyEvent::ConvertArea(AreaFormat::SquareFeet {
        precision: 99,
    }));
    assert!(err.is_err(), "precision 99 should be rejected");
}
