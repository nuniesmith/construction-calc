import Foundation
import CalcEngine

/// Which length format the calculator opens in. Raw values match the
/// `DefaultLengthFormat` string union in `frontend/src/lib/preferences.ts`
/// so a tape / preference exported from the web reads identically here.
enum LengthFormatPref: String, CaseIterable, Identifiable {
    case feetInchFraction = "feet_inch_fraction"
    case decimalFeet = "decimal_feet"
    case decimalInches = "decimal_inches"
    case meters
    case yards

    var id: String { rawValue }

    var label: String {
        switch self {
        case .feetInchFraction: return "Feet + inch + fraction"
        case .decimalFeet: return "Decimal feet"
        case .decimalInches: return "Decimal inches"
        case .meters: return "Meters"
        case .yards: return "Yards"
        }
    }
}

/// Allowed rounding resolutions for the feet-inch-fraction display. Matches
/// `FractionDenom = 4 | 8 | 16` on the web and the engine's
/// `denom ∈ {2,4,8,16}` validation.
enum FractionDenomPref: Int, CaseIterable, Identifiable {
    case quarter = 4
    case eighth = 8
    case sixteenth = 16

    var id: Int { rawValue }
    var label: String { "1/\(rawValue)\"" }
}

/// User preferences, persisted to `UserDefaults`. The web equivalent is
/// `localStorage` (`preferences.ts`); the shape is intentionally identical
/// so the two stay in lockstep.
struct Preferences {
    var fractionDenom: FractionDenomPref
    var lengthFormat: LengthFormatPref
    var angleInDegrees: Bool

    static let defaults = Preferences(
        fractionDenom: .sixteenth,
        lengthFormat: .feetInchFraction,
        angleInDegrees: true
    )

    /// UserDefaults keys. Kept here so the view model and the preferences
    /// screen read/write the same storage.
    enum Key {
        static let denom = "cc.pref.fractionDenom"
        static let format = "cc.pref.lengthFormat"
        static let degrees = "cc.pref.angleInDegrees"
    }

    static func load(from defaults: UserDefaults = .standard) -> Preferences {
        var prefs = Preferences.defaults

        if let raw = defaults.object(forKey: Key.denom) as? Int,
           let denom = FractionDenomPref(rawValue: raw) {
            prefs.fractionDenom = denom
        }
        if let raw = defaults.string(forKey: Key.format),
           let format = LengthFormatPref(rawValue: raw) {
            prefs.lengthFormat = format
        }
        if defaults.object(forKey: Key.degrees) != nil {
            prefs.angleInDegrees = defaults.bool(forKey: Key.degrees)
        }
        return prefs
    }

    func save(to defaults: UserDefaults = .standard) {
        defaults.set(fractionDenom.rawValue, forKey: Key.denom)
        defaults.set(lengthFormat.rawValue, forKey: Key.format)
        defaults.set(angleInDegrees, forKey: Key.degrees)
    }

    /// Translate the chosen format into the engine `Convert` event. Mirrors
    /// `preferencesToConvertKey()` in `preferences.ts`.
    func toConvertKey() -> KeyEvent {
        switch lengthFormat {
        case .feetInchFraction:
            return .convert(format: .feetInchFraction(denom: UInt32(fractionDenom.rawValue)))
        case .decimalFeet:
            return .convert(format: .decimalFeet(precision: 4))
        case .decimalInches:
            return .convert(format: .decimalInches(precision: 4))
        case .meters:
            return .convert(format: .meters(precision: 4))
        case .yards:
            return .convert(format: .yards(precision: 4))
        }
    }

    /// Translate the angle preference into the engine `SetAngleMode` event.
    /// Mirrors `preferencesToAngleModeKey()` in `preferences.ts`.
    func toAngleModeKey() -> KeyEvent {
        .setAngleMode(degrees: angleInDegrees)
    }
}
