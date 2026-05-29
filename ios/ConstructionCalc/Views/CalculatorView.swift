import SwiftUI

/// Root screen. Composes the display, format strip, and keypad, and presents
/// the tape / preferences / help as sheets. Mirrors the web `+page.svelte`
/// plus its header links.
struct CalculatorView: View {
    @Environment(CalculatorViewModel.self) private var vm

    @State private var showTape = false
    @State private var showPreferences = false
    /// Non-nil while a long-press help entry is being shown.
    @State private var helpId: String?

    var body: some View {
        VStack(spacing: 12) {
            header
            DisplayView(display: vm.display, error: vm.errorMessage)
            FormatStripView()
            KeypadView(helpId: $helpId)
            Spacer(minLength: 0)
        }
        .padding(.horizontal, 12)
        .padding(.top, 8)
        .frame(maxWidth: 560)
        .frame(maxWidth: .infinity)
        .background(Color(red: 0.04, green: 0.05, blue: 0.10).ignoresSafeArea())
        .sheet(isPresented: $showTape) { TapeView() }
        .sheet(isPresented: $showPreferences) { PreferencesView() }
        .sheet(item: helpBinding) { item in
            HelpOverlayView(entry: item.entry)
        }
    }

    private var header: some View {
        HStack {
            Text("Construction Calc")
                .font(.headline)
            Spacer()
            Button { showTape = true } label: {
                Image(systemName: "list.bullet.rectangle")
            }
            Button { showPreferences = true } label: {
                Image(systemName: "gearshape")
            }
        }
        .tint(Color(red: 0.99, green: 0.83, blue: 0.30))
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
