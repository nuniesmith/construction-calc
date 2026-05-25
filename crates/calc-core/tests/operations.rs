use calc_core::Length;
use calc_core::angle::Angle;
use calc_core::operations::{circle, concrete, materials, polygon, trig};
use num_rational::Rational64;

// -------------------------- Trig --------------------------

#[test]
fn sin_30_degrees_is_one_half() {
    let a = Angle::from_degrees(Rational64::from_integer(30));
    let s = trig::sin(a);
    assert!((s - 0.5).abs() < 1e-10);
}

#[test]
fn cos_60_degrees_is_one_half() {
    let a = Angle::from_degrees(Rational64::from_integer(60));
    let c = trig::cos(a);
    assert!((c - 0.5).abs() < 1e-10);
}

#[test]
fn tan_45_degrees_is_one() {
    let a = Angle::from_degrees(Rational64::from_integer(45));
    let t = trig::tan(a);
    assert!((t - 1.0).abs() < 1e-10);
}

#[test]
fn asin_one_half_is_thirty_degrees() {
    let a = trig::asin(0.5).unwrap();
    let deg = *a.degrees().numer() as f64 / *a.degrees().denom() as f64;
    assert!((deg - 30.0).abs() < 1e-3);
}

#[test]
fn asin_out_of_range_errors() {
    assert!(trig::asin(1.5).is_err());
    assert!(trig::acos(-2.0).is_err());
}

// -------------------------- Polygon --------------------------

#[test]
fn equilateral_triangle_from_side() {
    // 12" sides → perimeter 36", apothem ≈ 3.464", interior 60°
    let s = polygon::solve(
        3,
        polygon::PolygonInput::Side,
        Length::from_inches(Rational64::from_integer(12)),
    )
    .unwrap();
    assert_eq!(s.sides, 3);
    assert_eq!(s.perimeter.inches(), Rational64::from_integer(36));
    let interior_deg =
        *s.interior_angle.degrees().numer() as f64 / *s.interior_angle.degrees().denom() as f64;
    assert!((interior_deg - 60.0).abs() < 1e-3);
}

#[test]
fn square_from_side() {
    let s = polygon::solve(
        4,
        polygon::PolygonInput::Side,
        Length::from_inches(Rational64::from_integer(10)),
    )
    .unwrap();
    let interior_deg =
        *s.interior_angle.degrees().numer() as f64 / *s.interior_angle.degrees().denom() as f64;
    assert!((interior_deg - 90.0).abs() < 1e-3);
    // apothem of a 10" square = 5"
    let apo_in = *s.apothem.inches().numer() as f64 / *s.apothem.inches().denom() as f64;
    assert!((apo_in - 5.0).abs() < 1e-2);
}

#[test]
fn hexagon_from_circumradius() {
    // Regular hexagon: circumradius == side length.
    let s = polygon::solve(
        6,
        polygon::PolygonInput::Circumradius,
        Length::from_inches(Rational64::from_integer(8)),
    )
    .unwrap();
    let side_in = *s.side_length.inches().numer() as f64 / *s.side_length.inches().denom() as f64;
    assert!((side_in - 8.0).abs() < 1e-2);
}

#[test]
fn polygon_rejects_two_sides() {
    let r = polygon::solve(
        2,
        polygon::PolygonInput::Side,
        Length::from_inches(Rational64::from_integer(1)),
    );
    assert!(r.is_err());
}

// -------------------------- Circle --------------------------

#[test]
fn circle_from_radius() {
    let s =
        circle::solve_circle(circle::CircleInput::Radius, Rational64::from_integer(10)).unwrap();
    let circ_in =
        *s.circumference.inches().numer() as f64 / *s.circumference.inches().denom() as f64;
    assert!((circ_in - 2.0 * std::f64::consts::PI * 10.0).abs() < 0.05);
    let area_f = *s.area.numer() as f64 / *s.area.denom() as f64;
    assert!((area_f - std::f64::consts::PI * 100.0).abs() < 0.001);
}

#[test]
fn circle_from_diameter_matches_radius() {
    let from_d =
        circle::solve_circle(circle::CircleInput::Diameter, Rational64::from_integer(20)).unwrap();
    let from_r =
        circle::solve_circle(circle::CircleInput::Radius, Rational64::from_integer(10)).unwrap();
    assert_eq!(from_d.radius, from_r.radius);
}

#[test]
fn arc_quarter_circle() {
    // r=10", 90° arc → arc_length = 10 * π/2 ≈ 15.708"
    let r = Length::from_inches(Rational64::from_integer(10));
    let a = Angle::from_degrees(Rational64::from_integer(90));
    let s = circle::solve_arc(r, a).unwrap();
    let arc_in = *s.arc_length.inches().numer() as f64 / *s.arc_length.inches().denom() as f64;
    assert!((arc_in - std::f64::consts::FRAC_PI_2 * 10.0).abs() < 0.05);
    // chord for 90° arc r=10 = 10√2 ≈ 14.142"
    let chord_in =
        *s.chord_length.inches().numer() as f64 / *s.chord_length.inches().denom() as f64;
    assert!((chord_in - 10.0 * 2f64.sqrt()).abs() < 0.05);
}

#[test]
fn circle_zero_input_errors() {
    assert!(
        circle::solve_circle(circle::CircleInput::Radius, Rational64::from_integer(0)).is_err()
    );
}

// -------------------------- Materials --------------------------

#[test]
fn drywall_for_8_by_10_wall_no_waste() {
    // 8' × 10' wall = 80 sq ft = 11520 sq in.
    // 4×8 sheet = 32 sq ft = 4608 sq in.
    // 80/32 = 2.5 → ceil = 3 sheets.
    let area = Rational64::from_integer(11520);
    let n = materials::sheets_for_area(area, materials::SheetSize::drywall_4x8(), 0.0).unwrap();
    assert_eq!(n, 3);
}

#[test]
fn drywall_with_ten_percent_waste() {
    // 80/32 * 1.1 = 2.75 → ceil = 3
    let area = Rational64::from_integer(11520);
    let n = materials::sheets_for_area(area, materials::SheetSize::drywall_4x8(), 0.10).unwrap();
    assert_eq!(n, 3);

    // Bigger wall: 200 sq ft → 200/32 = 6.25 → with 10% = 6.875 → ceil = 7
    let area_big = Rational64::from_integer(200 * 144);
    let n2 =
        materials::sheets_for_area(area_big, materials::SheetSize::drywall_4x8(), 0.10).unwrap();
    assert_eq!(n2, 7);
}

#[test]
fn studs_for_twelve_foot_wall_at_sixteen_oc() {
    // 144" / 16" = 9 stud spaces → 10 studs.
    let n = materials::studs_on_center(
        Length::from_feet_int(12),
        Length::from_inches(Rational64::from_integer(16)),
    )
    .unwrap();
    assert_eq!(n, 10);
}

#[test]
fn studs_for_uneven_wall_rounds_up() {
    // 145" / 16" = 9.0625 → ceil 10 spaces → 11 studs.
    let n = materials::studs_on_center(
        Length::from_inches(Rational64::from_integer(145)),
        Length::from_inches(Rational64::from_integer(16)),
    )
    .unwrap();
    assert_eq!(n, 11);
}

// -------------------------- Concrete --------------------------

#[test]
fn slab_volume_is_exact_rational() {
    // 10' × 10' × 4" slab = 10*12 × 10*12 × 4 = 57_600 cu in = 1.234... cy
    let v = concrete::slab_volume(
        Length::from_feet_int(10),
        Length::from_feet_int(10),
        Length::from_inches(Rational64::from_integer(4)),
    )
    .unwrap();
    assert_eq!(v, Rational64::from_integer(57_600));
    // 57_600 / 46_656 ≈ 1.2345679
    let cy = concrete::cubic_inches_to_cubic_yards(v);
    assert!((cy - 1.234_567_9).abs() < 1e-6);
}

#[test]
fn slab_volume_rejects_zero_thickness() {
    let r = concrete::slab_volume(
        Length::from_feet_int(10),
        Length::from_feet_int(10),
        Length::ZERO,
    );
    assert!(r.is_err());
}

#[test]
fn column_volume_matches_cylinder_formula() {
    // 12" diameter × 8' tall = π × 6² × 96 = 10_857.34 cu in
    let v = concrete::column_volume(
        Length::from_inches(Rational64::from_integer(12)),
        Length::from_feet_int(8),
    )
    .unwrap();
    let f = *v.numer() as f64 / *v.denom() as f64;
    let expected = std::f64::consts::PI * 36.0 * 96.0;
    assert!((f - expected).abs() < 0.01, "got {f}, expected {expected}");
}

#[test]
fn cone_volume_is_one_third_of_cylinder() {
    let cyl = concrete::column_volume(
        Length::from_inches(Rational64::from_integer(12)),
        Length::from_feet_int(6),
    )
    .unwrap();
    let cone = concrete::cone_volume(
        Length::from_inches(Rational64::from_integer(12)),
        Length::from_feet_int(6),
    )
    .unwrap();
    let ratio =
        (*cyl.numer() as f64 / *cyl.denom() as f64) / (*cone.numer() as f64 / *cone.denom() as f64);
    assert!((ratio - 3.0).abs() < 1e-3);
}
