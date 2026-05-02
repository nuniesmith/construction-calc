use calc_core::calculator::{BinaryOp, Calculator, FunctionKey, KeyEvent, LengthUnitKey};
use calc_core::Value;
use num_rational::Rational64;

fn typed(seq: &[KeyEvent]) -> Calculator {
    let mut c = Calculator::new();
    for ev in seq {
        c.handle(ev.clone()).unwrap();
    }
    c
}

#[test]
fn simple_scalar_addition() {
    let c = typed(&[
        KeyEvent::Digit(2),
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(3),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(5)));
}

#[test]
fn add_two_lengths() {
    let c = typed(&[
        KeyEvent::Digit(5),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(3),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "8'");
}

#[test]
fn add_feet_inches_compound() {
    // 5'6" + 2'7" = 8'1"
    let c = typed(&[
        KeyEvent::Digit(5),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Digit(6),
        KeyEvent::Unit(LengthUnitKey::Inch),
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(2),
        KeyEvent::Unit(LengthUnitKey::Feet),
        KeyEvent::Digit(7),
        KeyEvent::Unit(LengthUnitKey::Inch),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "8' 1\"");
}

#[test]
fn add_inches_with_fractions_exactly() {
    // 5-3/8" + 2-5/8" = 8" (exact, no float drift)
    let c = typed(&[
        KeyEvent::Digit(5),
        KeyEvent::Slash, // start fraction even without whole
        // Better: enter as mixed via the fraction: use whole 5, then "3/8"
        // Reset and use a clearer keystroke flow:
    ]);
    drop(c);
    let mut c = Calculator::new();
    // 5-3/8 inches: type 5 then push fraction 3/8 by typing 3, /, 8
    // Our buffer: whole="5", numerator=Some("3"), denominator=Some("8") via Slash twice
    c.handle(KeyEvent::Digit(5)).unwrap();
    c.handle(KeyEvent::Slash).unwrap();
    c.handle(KeyEvent::Digit(3)).unwrap();
    c.handle(KeyEvent::Slash).unwrap();
    c.handle(KeyEvent::Digit(8)).unwrap();
    c.handle(KeyEvent::Unit(LengthUnitKey::Inch)).unwrap();
    c.handle(KeyEvent::Op(BinaryOp::Add)).unwrap();
    c.handle(KeyEvent::Digit(2)).unwrap();
    c.handle(KeyEvent::Slash).unwrap();
    c.handle(KeyEvent::Digit(5)).unwrap();
    c.handle(KeyEvent::Slash).unwrap();
    c.handle(KeyEvent::Digit(8)).unwrap();
    c.handle(KeyEvent::Unit(LengthUnitKey::Inch)).unwrap();
    c.handle(KeyEvent::Equals).unwrap();
    assert_eq!(c.display_string(), "8\"");
}

#[test]
fn convert_feet_to_meters() {
    use calc_core::format::LengthFormat;
    let mut c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Unit(LengthUnitKey::Feet),
    ]);
    c.handle(KeyEvent::Convert(LengthFormat::Meters { precision: 4 })).unwrap();
    // 10 ft = 120 in = 120 / (5000/127) m = 120 * 127/5000 = 15240/5000 = 3.048 m
    assert_eq!(c.display_string(), "3.048 m");
}

#[test]
fn rafter_pitch_and_run_solve() {
    // Pitch 6/12, run 10' → rise 5', diagonal ≈ 11' 1-7/8"... approx
    let mut c = Calculator::new();
    c.handle(KeyEvent::Digit(6)).unwrap();
    c.handle(KeyEvent::Function(FunctionKey::Pitch)).unwrap();
    c.handle(KeyEvent::Digit(1)).unwrap();
    c.handle(KeyEvent::Digit(0)).unwrap();
    c.handle(KeyEvent::Unit(LengthUnitKey::Feet)).unwrap();
    c.handle(KeyEvent::Function(FunctionKey::Run)).unwrap();
    // After Run: display should show the run length.
    assert_eq!(c.display_string(), "10'");

    // Asking for Rise should produce rise (5')
    c.handle(KeyEvent::Function(FunctionKey::Rise)).unwrap();
    assert_eq!(c.display_string(), "5'");

    // Diagonal
    c.handle(KeyEvent::Function(FunctionKey::Diagonal)).unwrap();
    let d = c.display_string();
    // 5' rise, 10' run -> hypotenuse = sqrt(25+100)= sqrt(125) ft = 11.180' ≈ 11' 2-3/16"
    // We rounded to 1/64", then display at 1/16" default.
    assert!(d.starts_with("11' "), "diag should start with 11' but got {}", d);
}

#[test]
fn clear_and_clear_all_reset_state() {
    let mut c = typed(&[
        KeyEvent::Digit(7),
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(3),
    ]);
    c.handle(KeyEvent::Clear).unwrap();
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(0)));
    c.handle(KeyEvent::ClearAll).unwrap();
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(0)));
}

#[test]
fn backspace_removes_last_digit() {
    let mut c = typed(&[KeyEvent::Digit(1), KeyEvent::Digit(2), KeyEvent::Digit(3)]);
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(123)));
    c.handle(KeyEvent::Backspace).unwrap();
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(12)));
}

#[test]
fn memory_store_recall() {
    use calc_core::calculator::MemoryOp;
    let mut c = typed(&[KeyEvent::Digit(4), KeyEvent::Digit(2)]);
    c.handle(KeyEvent::Memory(MemoryOp::Store(0))).unwrap();
    c.handle(KeyEvent::Clear).unwrap();
    c.handle(KeyEvent::Memory(MemoryOp::Recall(0))).unwrap();
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(42)));
}

#[test]
fn sin_30_degrees() {
    // Bare scalar 30 with degree mode → sin = 0.5
    let mut c = typed(&[KeyEvent::Digit(3), KeyEvent::Digit(0)]);
    c.handle(KeyEvent::Function(FunctionKey::Sin)).unwrap();
    if let Value::Scalar(r) = c.display {
        let f = *r.numer() as f64 / *r.denom() as f64;
        assert!((f - 0.5).abs() < 1e-3);
    } else {
        panic!("expected scalar, got {:?}", c.display);
    }
}

#[test]
fn sqrt_of_25() {
    let mut c = typed(&[KeyEvent::Digit(2), KeyEvent::Digit(5)]);
    c.handle(KeyEvent::Function(FunctionKey::Sqrt)).unwrap();
    if let Value::Scalar(r) = c.display {
        let f = *r.numer() as f64 / *r.denom() as f64;
        assert!((f - 5.0).abs() < 1e-6);
    } else {
        panic!("expected scalar");
    }
}

#[test]
fn square_of_seven() {
    let mut c = typed(&[KeyEvent::Digit(7)]);
    c.handle(KeyEvent::Function(FunctionKey::Square)).unwrap();
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(49)));
}

#[test]
fn percent_as_rhs() {
    // 200 + 5% should yield 210 (200 + 200*0.05)
    let mut c = typed(&[
        KeyEvent::Digit(2),
        KeyEvent::Digit(0),
        KeyEvent::Digit(0),
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(5),
    ]);
    c.handle(KeyEvent::Function(FunctionKey::Percent)).unwrap();
    c.handle(KeyEvent::Equals).unwrap();
    assert_eq!(c.display, Value::Scalar(Rational64::from_integer(210)));
}
