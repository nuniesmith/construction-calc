use calc_core::calculator::{BinaryOp, Calculator, FunctionKey, KeyEvent, LengthUnitKey};

#[test]
fn empty_tape_renders_markdown() {
    let c = Calculator::new();
    let md = c.tape.to_markdown();
    assert!(md.contains("# Calculator tape"));
    assert!(md.contains("(empty)"));
}

#[test]
fn results_render_in_markdown() {
    let mut c = Calculator::new();
    c.handle(KeyEvent::Digit(5)).unwrap();
    c.handle(KeyEvent::Unit(LengthUnitKey::Feet)).unwrap();
    c.handle(KeyEvent::Op(BinaryOp::Add)).unwrap();
    c.handle(KeyEvent::Digit(6)).unwrap();
    c.handle(KeyEvent::Unit(LengthUnitKey::Inch)).unwrap();
    c.handle(KeyEvent::Equals).unwrap();
    let md = c.tape.to_markdown();
    assert!(md.contains("5' 6\""), "got: {}", md);
}

#[test]
fn rafter_solution_renders_with_all_fields() {
    let mut c = Calculator::new();
    c.handle(KeyEvent::Digit(6)).unwrap();
    c.handle(KeyEvent::Function(FunctionKey::Pitch)).unwrap();
    c.handle(KeyEvent::Digit(1)).unwrap();
    c.handle(KeyEvent::Digit(0)).unwrap();
    c.handle(KeyEvent::Unit(LengthUnitKey::Feet)).unwrap();
    c.handle(KeyEvent::Function(FunctionKey::Run)).unwrap();
    let md = c.tape.to_markdown();
    assert!(md.contains("Rafter solution"), "{}", md);
    assert!(md.contains("pitch:"), "{}", md);
    assert!(md.contains("rise:"), "{}", md);
    assert!(md.contains("diagonal:"), "{}", md);
}

#[test]
fn json_round_trip_preserves_tape() {
    let mut c = Calculator::new();
    c.handle(KeyEvent::Digit(2)).unwrap();
    c.handle(KeyEvent::Op(BinaryOp::Add)).unwrap();
    c.handle(KeyEvent::Digit(3)).unwrap();
    c.handle(KeyEvent::Equals).unwrap();
    let json = serde_json::to_string(&c.tape).unwrap();
    let restored: calc_core::tape::Tape = serde_json::from_str(&json).unwrap();
    assert_eq!(c.tape, restored);
}
