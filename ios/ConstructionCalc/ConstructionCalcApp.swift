import SwiftUI

/// App entry point. Owns the single `CalculatorViewModel` for the whole
/// session and hands it down through the environment so the calculator,
/// tape, and preferences screens all drive the same engine instance.
@main
struct ConstructionCalcApp: App {
    @State private var vm = CalculatorViewModel()
    @AppStorage(Preferences.Key.theme) private var themeRaw = ThemePref.midnight.rawValue

    private var theme: CalcTheme {
        (ThemePref(rawValue: themeRaw) ?? .midnight).theme
    }

    var body: some Scene {
        WindowGroup {
            CalculatorView()
                .environment(vm)
                .environment(\.calcTheme, theme)
                .preferredColorScheme(theme.colorScheme)
        }
    }
}
