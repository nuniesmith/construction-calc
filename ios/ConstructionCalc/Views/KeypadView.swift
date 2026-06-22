import SwiftUI

/// The button grid. Faithful port of `Keypad.svelte`: a page picker
/// (Rafter / Trig / Miter / Mem / Calc) over a 6-column grid of shared rows.
/// Long-pressing a key surfaces its help entry via the `helpId` binding.
struct KeypadView: View {
    @Binding var helpId: String?
    /// Set when a "Calc" page key is tapped, so the root view can present that
    /// estimator as a sheet.
    @Binding var estimatorRoute: EstimatorRoute?
    @Environment(\.calcTheme) private var theme

    @State private var page: KeypadPage = .rafter
    /// When armed by the "2nd" key, a key with a red secondary label sends that
    /// secondary event instead of its primary one.
    @State private var secondMode = false

    private var rows: [[KeypadButton]] {
        [KeypadModel.functionRow(for: page)] + KeypadModel.sharedRows
    }

    var body: some View {
        VStack(spacing: 7) {
            Picker("Function page", selection: $page) {
                ForEach(KeypadPage.allCases) { p in
                    Text(p.rawValue).tag(p)
                }
            }
            .pickerStyle(.segmented)

            // Tight grid + padding so the pad reads like a physical construction
            // calculator (keys nearly touching) rather than a spaced-out app grid.
            Grid(horizontalSpacing: 4, verticalSpacing: 4) {
                ForEach(Array(rows.enumerated()), id: \.offset) { _, row in
                    GridRow {
                        ForEach(row) { button in
                            KeyButton(button: button, helpId: $helpId,
                                      secondMode: $secondMode, estimatorRoute: $estimatorRoute)
                                .gridCellColumns(button.columns)
                        }
                    }
                }
            }
        }
        .padding(6)
        .background(
            RoundedRectangle(cornerRadius: 12).fill(theme.panel)
        )
    }
}

/// A single key. Tap sends the event through the view model; long-press
/// raises the help id. Filler cells render invisibly so the grid stays
/// aligned. Keys with a `sub` label show it small above the main label (red
/// normally, yellow while the "2nd" shift is armed); tapping such a key while
/// armed fires its secondary event instead of the primary one.
private struct KeyButton: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Environment(\.calcTheme) private var theme
    let button: KeypadButton
    @Binding var helpId: String?
    @Binding var secondMode: Bool
    @Binding var estimatorRoute: EstimatorRoute?

    var body: some View {
        if button.style == .filler {
            // Fixed height (not just a minimum): a bare Color is greedy and would
            // otherwise balloon its grid row to absorb all slack, leaving a gap
            // on pages whose function row has empty cells (Miter, Mem).
            Color.clear.frame(maxWidth: .infinity, minHeight: 48, maxHeight: 48)
        } else {
            Button(action: tap) {
                VStack(spacing: 1) {
                    if let sub = button.sub {
                        Text(sub)
                            .font(.system(size: 10, weight: .semibold))
                            .foregroundStyle(subColor)
                            .lineLimit(1)
                            .minimumScaleFactor(0.7)
                    }
                    Text(button.label)
                        .font(.system(size: 18, weight: .medium))
                        .foregroundStyle(theme.keyText)
                }
                .frame(maxWidth: .infinity, minHeight: 48)
                .background(
                    RoundedRectangle(cornerRadius: 8).fill(fill)
                )
            }
            .buttonStyle(.plain)
            .onLongPressGesture(minimumDuration: 0.4) {
                if let id = button.helpId { helpId = id }
            }
        }
    }

    /// Routes the tap: the "2nd" key toggles shift; an armed shift fires a key's
    /// secondary event (then disarms); a "Calc" key opens its estimator;
    /// otherwise the primary event fires.
    private func tap() {
        if button.shift {
            secondMode.toggle()
        } else if secondMode, let secondary = button.secondary {
            vm.send(secondary)
            secondMode = false
        } else if let route = button.estimator {
            estimatorRoute = route
            if secondMode { secondMode = false }
        } else if let event = button.event {
            vm.send(event)
            if secondMode { secondMode = false }
        }
    }

    /// Secondary caption tint: the accent (yellow / amber) while armed so the
    /// live alternate functions stand out; a muted red otherwise.
    private var subColor: Color {
        secondMode ? theme.accent : theme.subNormal
    }

    /// Key fill. The "2nd" key glows in the accent colour while armed;
    /// everything else uses its category tint from the theme.
    private var fill: Color {
        if button.shift {
            return secondMode ? theme.accent : theme.opKey
        }
        return theme.keyFill(button.style)
    }
}
