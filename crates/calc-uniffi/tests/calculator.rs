//! End-to-end test of the UniFFI bindings, exercised through the same
//! Rust API that Swift will hit at runtime. If this passes, the iOS
//! side can drive the engine identically.

use calc_uniffi::{Calculator, KeyEvent, LengthFormat, Op, Unit};

#[test]
fn add_two_lengths_and_format_at_quarter_inch() {
    let calc = Calculator::new();

    // 5' 6-3/8" + 2' 7-1/4"
    calc.handle(KeyEvent::Digit { value: 5 });
    calc.handle(KeyEvent::Unit { unit: Unit::Feet });
    calc.handle(KeyEvent::Digit { value: 6 });
    calc.handle(KeyEvent::Slash);
    calc.handle(KeyEvent::Digit { value: 3 });
    calc.handle(KeyEvent::Slash);
    calc.handle(KeyEvent::Digit { value: 8 });
    calc.handle(KeyEvent::Unit { unit: Unit::Inch });

    calc.handle(KeyEvent::Op { op: Op::Add });

    calc.handle(KeyEvent::Digit { value: 2 });
    calc.handle(KeyEvent::Unit { unit: Unit::Feet });
    calc.handle(KeyEvent::Digit { value: 7 });
    calc.handle(KeyEvent::Slash);
    calc.handle(KeyEvent::Digit { value: 1 });
    calc.handle(KeyEvent::Slash);
    calc.handle(KeyEvent::Digit { value: 4 });
    calc.handle(KeyEvent::Unit { unit: Unit::Inch });

    let snap = calc.handle(KeyEvent::Equals);
    assert!(snap.error.is_none(), "unexpected error: {:?}", snap.error);
    // 5'6-3/8" + 2'7-1/4" = 8'1-5/8" at default 1/16" display.
    assert_eq!(snap.display, "8' 1-5/8\"");
}

#[test]
fn convert_changes_display_resolution() {
    let calc = Calculator::new();

    // 7-3/16" — at default 1/16" shows as itself, at 1/4" rounds to 7-1/4"
    calc.handle(KeyEvent::Digit { value: 7 });
    calc.handle(KeyEvent::Slash);
    calc.handle(KeyEvent::Digit { value: 3 });
    calc.handle(KeyEvent::Slash);
    calc.handle(KeyEvent::Digit { value: 1 });
    calc.handle(KeyEvent::Digit { value: 6 });
    calc.handle(KeyEvent::Unit { unit: Unit::Inch });

    let snap = calc.handle(KeyEvent::Convert {
        format: LengthFormat::FeetInchFraction { denom: 4 },
    });
    assert!(snap.error.is_none());
    assert_eq!(snap.display, "7-1/4\"");
}

#[test]
fn divide_by_zero_surfaces_in_snapshot() {
    let calc = Calculator::new();
    calc.handle(KeyEvent::Digit { value: 5 });
    calc.handle(KeyEvent::Op { op: Op::Div });
    calc.handle(KeyEvent::Digit { value: 0 });
    let snap = calc.handle(KeyEvent::Equals);
    assert!(snap.error.is_some(), "expected a div-by-zero error");
}

#[test]
fn export_and_reload_json_tape_round_trips() {
    let calc = Calculator::new();
    calc.handle(KeyEvent::Digit { value: 5 });
    calc.handle(KeyEvent::Unit { unit: Unit::Feet });
    calc.handle(KeyEvent::Equals);

    let json = calc.export_json().expect("export should succeed");
    assert!(json.contains("entries"), "json should hold tape entries");

    let calc2 = Calculator::new();
    calc2
        .load_json_tape(json.clone())
        .expect("reload should succeed");
    let md = calc2.export_markdown();
    assert!(!md.is_empty(), "reloaded tape should have content");
}

#[test]
fn set_angle_mode_switches_trig_through_the_bindings() {
    use calc_uniffi::FunctionKey;

    // Degrees (default): sin(30°) = 0.5
    let calc = Calculator::new();
    calc.handle(KeyEvent::Digit { value: 3 });
    calc.handle(KeyEvent::Digit { value: 0 });
    let snap = calc.handle(KeyEvent::Function {
        function: FunctionKey::Sin,
    });
    assert_eq!(snap.display, "0.5");

    // Radians: sin(30 rad) ≈ -0.988
    let calc = Calculator::new();
    calc.handle(KeyEvent::SetAngleMode { degrees: false });
    calc.handle(KeyEvent::Digit { value: 3 });
    calc.handle(KeyEvent::Digit { value: 0 });
    let snap = calc.handle(KeyEvent::Function {
        function: FunctionKey::Sin,
    });
    assert!(
        snap.display.starts_with("-0.988"),
        "expected sin(30 rad) ≈ -0.988, got {}",
        snap.display
    );
}

#[test]
fn area_result_carries_dimension_and_converts_through_the_bindings() {
    use calc_uniffi::{AreaFormat, Dimension};

    // 10' × 12' = 120 sq ft by default, tagged as an Area dimension.
    let calc = Calculator::new();
    calc.handle(KeyEvent::Digit { value: 1 });
    calc.handle(KeyEvent::Digit { value: 0 });
    calc.handle(KeyEvent::Unit { unit: Unit::Feet });
    calc.handle(KeyEvent::Op { op: Op::Mul });
    calc.handle(KeyEvent::Digit { value: 1 });
    calc.handle(KeyEvent::Digit { value: 2 });
    calc.handle(KeyEvent::Unit { unit: Unit::Feet });
    let snap = calc.handle(KeyEvent::Equals);
    assert_eq!(snap.display, "120 sq ft");
    assert_eq!(snap.dimension, Dimension::Area);

    // Convert the live display to square yards.
    let snap = calc.handle(KeyEvent::ConvertArea {
        format: AreaFormat::SquareYards { precision: 2 },
    });
    assert_eq!(snap.display, "13.33 sq yd");
}

#[test]
fn cost_per_unit_through_the_bindings() {
    use calc_uniffi::Dimension;

    // 10 ft @ $2/ft = $20.00, tagged as Money.
    let calc = Calculator::new();
    calc.handle(KeyEvent::Digit { value: 1 });
    calc.handle(KeyEvent::Digit { value: 0 });
    calc.handle(KeyEvent::Unit { unit: Unit::Feet });
    calc.handle(KeyEvent::CostPerUnit);
    calc.handle(KeyEvent::Digit { value: 2 });
    let snap = calc.handle(KeyEvent::Equals);
    assert_eq!(snap.display, "$20.00");
    assert_eq!(snap.dimension, Dimension::Money);
}
