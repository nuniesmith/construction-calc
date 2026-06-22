import SwiftUI

/// Centralised colour palette so the whole calculator can switch between the
/// dark "Midnight" look and a lighter physical-calculator "Daylight" look from
/// one place. Every hard-coded colour in the view tree reads from here via the
/// `\.calcTheme` environment value, so adding a theme is a matter of filling in
/// one more `CalcTheme` instance rather than hunting colours across views.
struct CalcTheme {
    /// Page background behind everything.
    let appBackground: Color
    /// Translucent backplate behind the keypad and the format strip.
    let panel: Color
    /// The display capsule fill (the "LCD").
    let display: Color
    /// Text colour inside the display.
    let displayText: Color
    /// Header tint, active format chip, and the armed-shift glow.
    let accent: Color
    /// Inactive format-chip fill.
    let chipInactive: Color
    /// Inactive format-chip text.
    let chipText: Color
    /// Label colour on every key.
    let keyText: Color
    /// Secondary ("2nd") caption colour when the shift is not armed.
    let subNormal: Color
    // Key fills by category.
    let numKey: Color
    let unitKey: Color
    let funcKey: Color
    let opKey: Color
    let controlKey: Color
    /// Drives `preferredColorScheme` so sheets (estimators, preferences) match.
    let colorScheme: ColorScheme

    /// The fill for a given key category.
    func keyFill(_ style: KeyStyle) -> Color {
        switch style {
        case .function: return funcKey
        case .unit:     return unitKey
        case .op:       return opKey
        case .control:  return controlKey
        case .num:      return numKey
        case .filler:   return .clear
        }
    }

    /// The original dark scheme — these are the exact values the app shipped
    /// with, so Midnight is pixel-identical to before the theme system landed.
    static let midnight = CalcTheme(
        appBackground: Color(red: 0.04, green: 0.05, blue: 0.10),
        panel: Color.white.opacity(0.03),
        display: Color(red: 0.05, green: 0.09, blue: 0.16),
        displayText: .white,
        accent: Color(red: 0.99, green: 0.83, blue: 0.30),
        chipInactive: Color.white.opacity(0.04),
        chipText: Color(white: 0.8),
        keyText: .white,
        subNormal: Color(red: 0.95, green: 0.45, blue: 0.40),
        numKey: Color(red: 0.16, green: 0.20, blue: 0.25),
        unitKey: Color(red: 0.28, green: 0.35, blue: 0.46),
        funcKey: Color(red: 0.23, green: 0.23, blue: 0.54),
        opKey: Color(red: 0.85, green: 0.47, blue: 0.02),
        controlKey: Color(red: 0.48, green: 0.12, blue: 0.12),
        colorScheme: .dark
    )

    /// A light chassis with colourful keys — closer to a physical ProjectCalc /
    /// CMPro: warm-grey body, pale-green LCD, white key text on saturated keys.
    static let daylight = CalcTheme(
        appBackground: Color(red: 0.88, green: 0.88, blue: 0.86),
        panel: Color.black.opacity(0.05),
        display: Color(red: 0.80, green: 0.84, blue: 0.78),
        displayText: Color(red: 0.10, green: 0.14, blue: 0.10),
        accent: Color(red: 0.90, green: 0.52, blue: 0.05),
        chipInactive: Color.black.opacity(0.06),
        chipText: Color(white: 0.25),
        keyText: .white,
        subNormal: Color(red: 0.74, green: 0.20, blue: 0.15),
        numKey: Color(red: 0.42, green: 0.45, blue: 0.48),
        unitKey: Color(red: 0.30, green: 0.42, blue: 0.55),
        funcKey: Color(red: 0.34, green: 0.34, blue: 0.58),
        opKey: Color(red: 0.88, green: 0.52, blue: 0.10),
        controlKey: Color(red: 0.70, green: 0.22, blue: 0.20),
        colorScheme: .light
    )
}

/// User-selectable theme, persisted in `@AppStorage`.
enum ThemePref: String, CaseIterable, Identifiable {
    case midnight
    case daylight

    var id: String { rawValue }

    var label: String {
        switch self {
        case .midnight: return "Midnight (dark)"
        case .daylight: return "Daylight (light)"
        }
    }

    var theme: CalcTheme {
        switch self {
        case .midnight: return .midnight
        case .daylight: return .daylight
        }
    }
}

// MARK: - Environment plumbing

private struct CalcThemeKey: EnvironmentKey {
    static let defaultValue = CalcTheme.midnight
}

extension EnvironmentValues {
    /// The active calculator palette. Injected once at the app root and read by
    /// every view that paints a themed surface.
    var calcTheme: CalcTheme {
        get { self[CalcThemeKey.self] }
        set { self[CalcThemeKey.self] = newValue }
    }
}
