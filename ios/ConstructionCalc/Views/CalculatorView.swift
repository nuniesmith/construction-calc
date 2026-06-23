import SwiftUI

/// Root screen. Composes the display, format strip, and keypad, and presents
/// the tape / preferences / help as sheets. Mirrors the web `+page.svelte`
/// plus its header links.
struct CalculatorView: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Environment(\.calcTheme) private var theme

    @State private var showTape = false
    @State private var showPreferences = false
    @State private var showCalcs = false
    @State private var showHelp = false
    /// Non-nil while a long-press help entry is being shown.
    @State private var helpId: String?
    /// Set when a keypad "Calc" key launches a guided estimator.
    @State private var estimatorRoute: EstimatorRoute?

    var body: some View {
        VStack(spacing: 8) {
            header
            DisplayView(display: vm.display, error: vm.errorMessage)
            FormatStripView()
            KeypadView(helpId: $helpId, estimatorRoute: $estimatorRoute)
        }
        .padding(.horizontal, 12)
        .padding(.top, 8)
        .padding(.bottom, 8)
        .frame(maxWidth: 560)
        .frame(maxWidth: .infinity)
        .background(theme.appBackground.ignoresSafeArea())
        .sheet(isPresented: $showTape) { TapeView() }
        .sheet(isPresented: $showPreferences) { PreferencesView() }
        .sheet(isPresented: $showCalcs) { CalcsMenuView() }
        .sheet(isPresented: $showHelp) { HelpReferenceView() }
        .sheet(item: $estimatorRoute) { route in
            EstimatorSheet(route: route)
        }
        .sheet(item: helpBinding) { item in
            HelpOverlayView(entry: item.entry)
        }
    }

    private var header: some View {
        HStack(spacing: 16) {
            Text("Construction Calc")
                .font(.headline)
            Spacer()
            Button { showCalcs = true } label: {
                Image(systemName: "square.grid.2x2")
            }
            .accessibilityLabel("Construction calculators")
            Button { showHelp = true } label: {
                Image(systemName: "questionmark.circle")
            }
            .accessibilityLabel("Function guide")
            Button { showTape = true } label: {
                Image(systemName: "list.bullet.rectangle")
            }
            Button { showPreferences = true } label: {
                Image(systemName: "gearshape")
            }
        }
        .tint(theme.accent)
    }

    /// Bridges the `String?` help id to the `sheet(item:)` API, which needs an
    /// `Identifiable`. Resolves the id into its entry, or nil if unknown.
    private var helpBinding: Binding<HelpItem?> {
        Binding(
            get: {
                guard let id = helpId, let entry = HelpText.entry(for: id) else { return nil }
                return HelpItem(id: id, entry: entry)
            },
            set: { newValue in helpId = newValue?.id }
        )
    }
}

/// Wrapper so a help entry can drive `sheet(item:)`.
private struct HelpItem: Identifiable {
    let id: String
    let entry: HelpEntry
}
