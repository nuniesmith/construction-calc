use calc_core::Length;
use calc_core::format::{LengthFormat, format_length};
use num_rational::Rational64;

#[test]
fn parses_feet_and_inches() {
    let l = Length::parse("8' 5-3/8\"").unwrap();
    // 8*12 + 5 + 3/8 = 101 + 3/8 = 101 3/8 inches
    assert_eq!(l.inches(), Rational64::new(811, 8));
}

#[test]
fn parses_pure_feet() {
    let l = Length::parse("5'").unwrap();
    assert_eq!(l.inches(), Rational64::from_integer(60));
}

#[test]
fn parses_mixed_inches() {
    let l = Length::parse("3 1/2\"").unwrap();
    assert_eq!(l.inches(), Rational64::new(7, 2));
}

#[test]
fn parses_dashed_mixed_inches() {
    let l = Length::parse("3-1/2\"").unwrap();
    assert_eq!(l.inches(), Rational64::new(7, 2));
}

#[test]
fn parses_pure_fraction() {
    let l = Length::parse("3/8\"").unwrap();
    assert_eq!(l.inches(), Rational64::new(3, 8));
}

#[test]
fn parses_decimal_inches() {
    let l = Length::parse("12.5\"").unwrap();
    assert_eq!(l.inches(), Rational64::new(25, 2));
}

#[test]
fn parses_metric_mm() {
    let l = Length::parse("1500mm").unwrap();
    // 1500 * 5/127 = 7500/127 inches
    assert_eq!(l.inches(), Rational64::new(7500, 127));
}

#[test]
fn parses_metric_meters() {
    let l = Length::parse("1.5m").unwrap();
    // 1.5 m * 5000/127 in/m = 7500/127
    assert_eq!(l.inches(), Rational64::new(7500, 127));
}

#[test]
fn parses_yards() {
    let l = Length::parse("2 yd").unwrap();
    assert_eq!(l.inches(), Rational64::from_integer(72));
}

#[test]
fn parses_bare_number_as_inches() {
    let l = Length::parse("12").unwrap();
    assert_eq!(l.inches(), Rational64::from_integer(12));
}

#[test]
fn parses_unicode_quotes() {
    let l = Length::parse("8\u{2032} 5\u{2033}").unwrap();
    assert_eq!(l.inches(), Rational64::from_integer(101));
}

#[test]
fn formats_feet_inch_fraction_basic() {
    let l = Length::from_feet_inches(8, Rational64::new(43, 8));
    let s = format_length(&l, LengthFormat::FeetInchFraction { denom: 16 });
    assert_eq!(s, "8' 5-3/8\"");
}

#[test]
fn formats_feet_only() {
    let l = Length::from_feet_int(5);
    let s = format_length(&l, LengthFormat::FeetInchFraction { denom: 16 });
    assert_eq!(s, "5'");
}

#[test]
fn formats_inches_only() {
    let l = Length::from_inches(Rational64::from_integer(7));
    let s = format_length(&l, LengthFormat::FeetInchFraction { denom: 16 });
    assert_eq!(s, "7\"");
}

#[test]
fn formats_inches_with_fraction() {
    let l = Length::from_inches(Rational64::new(15, 2));
    let s = format_length(&l, LengthFormat::FeetInchFraction { denom: 16 });
    assert_eq!(s, "7-1/2\"");
}

#[test]
fn formats_negative_length() {
    let l = Length::from_inches(Rational64::new(-15, 2));
    let s = format_length(&l, LengthFormat::FeetInchFraction { denom: 16 });
    assert_eq!(s, "-7-1/2\"");
}

#[test]
fn formats_decimal_feet() {
    let l = Length::from_inches(Rational64::new(15, 2));
    let s = format_length(&l, LengthFormat::DecimalFeet { precision: 4 });
    assert_eq!(s, "0.625'");
}

#[test]
fn formats_metric() {
    let l = Length::from_mm(Rational64::from_integer(1500));
    let s = format_length(&l, LengthFormat::Millimeters { precision: 0 });
    assert_eq!(s, "1500 mm");
}

#[test]
fn round_trip_through_feet_inch_fraction() {
    // Anything with a denom that's a power of two ≤ 64 should round-trip.
    let cases = ["8' 5-3/8\"", "12-1/16\"", "5'", "0\""];
    for c in &cases {
        let parsed = Length::parse(c).unwrap();
        let formatted = format_length(&parsed, LengthFormat::FeetInchFraction { denom: 64 });
        let reparsed = Length::parse(&formatted).unwrap();
        assert_eq!(parsed, reparsed, "round-trip failed for {}", c);
    }
}

#[test]
fn one_third_plus_one_third_plus_one_third_is_one_inch() {
    // The reason we use rationals.
    let third = Length::from_inches(Rational64::new(1, 3));
    let total = third + third + third;
    assert_eq!(total.inches(), Rational64::from_integer(1));
}

#[test]
fn rounds_to_preset_fraction() {
    // 0.4" rounded to nearest 1/16 = 6/16 = 3/8
    let l = Length::from_inches(Rational64::new(2, 5));
    let r = l.round_to_fraction(16);
    assert_eq!(r.inches(), Rational64::new(3, 8));
}

#[test]
fn negative_feet_inches_construction() {
    let l = Length::from_feet_inches(-3, Rational64::new(6, 1));
    assert_eq!(l.inches(), Rational64::from_integer(-42));
}
