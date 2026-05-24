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
