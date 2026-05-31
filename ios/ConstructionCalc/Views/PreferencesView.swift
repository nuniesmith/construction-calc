import SwiftUI

/// Settings sheet. Mirrors the web `/preferences` route: default fraction
/// resolution, default length format, and angle mode. Persisted to
/// `UserDefaults` via `@AppStorage`; the display format is re-applied to the
/// live engine immediately on change.
struct PreferencesView: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Environment(\.dismiss) private var dismiss

    @AppStorage(Preferences.Key.denom) private var denom = FractionDenomPref.sixteenth.rawValue
    @AppStorage(Preferences.Key.format) private var format = LengthFormatPref.feetInchFraction.rawValue
    @AppStorage(Preferences.Key.degrees) private var angleInDegrees = true

    var body: some View {
        NavigationStack {
            Form {
                Section("Display") {
                    Picker("Fraction resolution", selection: $denom) {
                        ForEach(FractionDenomPref.allCases) { d in
                            Text(d.label).tag(d.rawValue)
                        }
                    }
                    Picker("Length format", selection: $format) {
                        ForEach(LengthFormatPref.allCases) { f in
                            Text(f.label).tag(f.rawValue)
                        }
                    }
                }

                Section {
                    Toggle("Trig keys use degrees", isOn: $angleInDegrees)
                } header: {
                    Text("Angles")
                } footer: {
                    Text("Off interprets a plain number as radians when you press sin / cos / tan.")
                }

                Section {
                    Button("Reset to defaults", role: .destructive, action: resetDefaults)
                }
            }
            .navigationTitle("Preferences")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
            // Re-apply the display format live whenever the relevant
            // settings change.
            .onChange(of: denom) { _, _ in vm.reapplyDisplayPreference() }
            .onChange(of: format) { _, _ in vm.reapplyDisplayPreference() }
            .onChange(of: angleInDegrees) { _, newValue in vm.setAngleMode(degrees: newValue) }
        }
    }

    private func resetDefaults() {
        denom = Preferences.defaults.fractionDenom.rawValue
        format = Preferences.defaults.lengthFormat.rawValue
        angleInDegrees = Preferences.defaults.angleInDegrees
        vm.reapplyDisplayPreference()
        vm.setAngleMode(degrees: angleInDegrees)
    }
}
