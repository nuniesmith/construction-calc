use calc_core::operations::stair::{solve, StairInputs};
use calc_core::Length;
use num_rational::Rational64;

#[test]
fn standard_residential_stair() {
    // 9' total rise (108"), defaults: target riser 7-1/2", target tread 10".
    // Candidates (target distance):
    //   14 risers → 7.714" each (0.214 from target, legal under 7.75" cap)
    //   15 risers → 7.2"   each (0.3 from target)
    // 14 wins on closeness to target.
    let inputs = StairInputs {
        total_rise: Length::from_feet_int(9),
        ..StairInputs::default()
    };
    let s = solve(&inputs).unwrap();
    assert_eq!(s.riser_count, 14);
    assert_eq!(s.riser_height.inches(), Rational64::new(108, 14));
    assert_eq!(s.tread_count, 13);
}

#[test]
fn tighter_riser_target_pushes_count_up() {
    // If we target 7" risers, 108/7 ≈ 15.43 → 15 risers @ 7.2".
    let inputs = StairInputs {
        total_rise: Length::from_feet_int(9),
        target_riser: Length::from_inches(Rational64::from_integer(7)),
        ..StairInputs::default()
    };
    let s = solve(&inputs).unwrap();
    assert_eq!(s.riser_count, 15);
}

#[test]
fn rejects_zero_rise() {
    let inputs = StairInputs {
        total_rise: Length::ZERO,
        ..StairInputs::default()
    };
    assert!(solve(&inputs).is_err());
}

#[test]
fn deck_stair_three_feet() {
    let inputs = StairInputs {
        total_rise: Length::from_feet_int(3),
        ..StairInputs::default()
    };
    let s = solve(&inputs).unwrap();
    // 36" / 7.5" = 4.8 → 5 risers @ 7.2" each
    assert_eq!(s.riser_count, 5);
    assert_eq!(s.tread_count, 4);
}
