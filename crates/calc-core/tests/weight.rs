//! Weight dimension: input unit keys (lb / kg / ton / tonne), display
//! conversion, arithmetic, and pricing by weight.

use calc_core::Dimension;
use calc_core::calculator::{BinaryOp, Calculator, KeyEvent, WeightUnitKey};
use calc_core::format::{WeightFormat, format_weight};
use num_rational::Rational64;

fn typed(seq: &[KeyEvent]) -> Calculator {
    let mut c = Calculator::new();
    for ev in seq {
        c.handle(ev.clone()).unwrap();
    }
    c
}

#[test]
fn format_weight_converts_exactly() {
    let lb = |n| Rational64::from_integer(n);
    assert_eq!(
        format_weight(lb(50), WeightFormat::Pounds { precision: 0 }),
        "50 lb"
    );
    assert_eq!(
        format_weight(lb(4000), WeightFormat::Tons { precision: 2 }),
        "2 ton"
    );
    // 1 kg = 2.20462262… lb; rounded to 3 places.
    assert_eq!(
        format_weight(
            Rational64::new(100_000_000, 45_359_237),
            WeightFormat::Pounds { precision: 3 }
        ),
        "2.205 lb"
    );
    // 1 tonne = 1000 kg ≈ 2204.62 lb.
    assert_eq!(
        format_weight(
            Rational64::new(100_000_000_000, 45_359_237),
            WeightFormat::Pounds { precision: 2 }
        ),
        "2204.62 lb"
    );
}

#[test]
fn pounds_key_tags_a_weight() {
    let c = typed(&[
        KeyEvent::Digit(5),
        KeyEvent::Digit(0),
        KeyEvent::WeightUnit(WeightUnitKey::Pounds),
    ]);
    assert_eq!(c.display_string(), "50 lb");
    assert_eq!(c.display_dimension(), Dimension::Weight);
}

#[test]
fn tons_store_as_pounds() {
    // 2 ton = 4000 lb in the default pounds display.
    let c = typed(&[
        KeyEvent::Digit(2),
        KeyEvent::WeightUnit(WeightUnitKey::Tons),
    ]);
    assert_eq!(c.display_string(), "4000 lb");
}

#[test]
fn convert_weight_display_unit() {
    // 1000 lb shown as tons → 0.5 ton.
    let mut c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Digit(0),
        KeyEvent::Digit(0),
        KeyEvent::WeightUnit(WeightUnitKey::Pounds),
    ]);
    c.handle(KeyEvent::ConvertWeight(WeightFormat::Tons { precision: 2 }))
        .unwrap();
    assert_eq!(c.display_string(), "0.5 ton");
}

#[test]
fn weights_add() {
    // 50 lb + 25 lb = 75 lb.
    let c = typed(&[
        KeyEvent::Digit(5),
        KeyEvent::Digit(0),
        KeyEvent::WeightUnit(WeightUnitKey::Pounds),
        KeyEvent::Op(BinaryOp::Add),
        KeyEvent::Digit(2),
        KeyEvent::Digit(5),
        KeyEvent::WeightUnit(WeightUnitKey::Pounds),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "75 lb");
}

#[test]
fn count_times_weight() {
    // 3 × 10 lb = 30 lb (e.g. 3 bags).
    let c = typed(&[
        KeyEvent::Digit(3),
        KeyEvent::Op(BinaryOp::Mul),
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::WeightUnit(WeightUnitKey::Pounds),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "30 lb");
}

#[test]
fn cost_per_pound() {
    // 100 lb @ $0.50/lb = $50.00 — weight integrates with cost-per-unit.
    let c = typed(&[
        KeyEvent::Digit(1),
        KeyEvent::Digit(0),
        KeyEvent::Digit(0),
        KeyEvent::WeightUnit(WeightUnitKey::Pounds),
        KeyEvent::CostPerUnit,
        KeyEvent::Digit(0),
        KeyEvent::Decimal,
        KeyEvent::Digit(5),
        KeyEvent::Equals,
    ]);
    assert_eq!(c.display_string(), "$50.00");
    assert_eq!(c.display_dimension(), Dimension::Money);
}
