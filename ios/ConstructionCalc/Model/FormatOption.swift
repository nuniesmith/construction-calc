import Foundation

/// One chip in the display-format strip. Mirrors the `choices` array in
/// `frontend/src/lib/FormatStrip.svelte`: 1/4", 1/8", 1/16", dec ft,
/// dec in, m, yd. The fraction chips double as rounding controls.
struct FormatOption: Identifiable {
    let id = UUID()
    let label: String
    let event: KeyEvent

    static let all: [FormatOption] = [
        FormatOption(label: "1/4\"", event: .convert(format: .feetInchFraction(denom: 4))),
        FormatOption(label: "1/8\"", event: .convert(format: .feetInchFraction(denom: 8))),
        FormatOption(label: "1/16\"", event: .convert(format: .feetInchFraction(denom: 16))),
        FormatOption(label: "dec ft", event: .convert(format: .decimalFeet(precision: 4))),
        FormatOption(label: "dec in", event: .convert(format: .decimalInches(precision: 4))),
        FormatOption(label: "m", event: .convert(format: .meters(precision: 4))),
        FormatOption(label: "yd", event: .convert(format: .yards(precision: 4)))
    ]

    /// Default highlight matches the engine's default mode (1/16").
    static let defaultIndex = 2

    /// Which chip corresponds to a stored preference, so the strip opens
    /// highlighting the right one.
    static func index(matching prefs: Preferences) -> Int {
        switch prefs.lengthFormat {
        case .feetInchFraction:
            switch prefs.fractionDenom {
            case .quarter: return 0
            case .eighth: return 1
            case .sixteenth: return 2
            }
        case .decimalFeet: return 3
        case .decimalInches: return 4
        case .meters: return 5
        case .yards: return 6
        }
    }
}
