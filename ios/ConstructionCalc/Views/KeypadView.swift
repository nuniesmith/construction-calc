import SwiftUI

/// The button grid. Faithful port of `Keypad.svelte`: a page picker
/// (Rafter / Trig / Miter / Mem) over a 6-column grid of shared rows.
/// Long-pressing a key surfaces its help entry via the `helpId` binding.
struct KeypadView: View {
    @Binding var helpId: String?

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
                            KeyButton(button: button, helpId: $helpId, secondMode: $secondMode)
                                .gridCellColumns(button.columns)
                        }
                    }
                }
            }
        }
        .padding(6)
        .background(
            RoundedRectangle(cornerRadius: 12).fill(Color.white.opacity(0.03))
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
    let button: KeypadButton
    @Binding var helpId: String?
    @Binding var secondMode: Bool

    var body: some View {
        if button.style == .filler {
            Color.clear.frame(maxWidth: .infinity, minHeight: 48)
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
                        .foregroundStyle(.white)
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
    /// secondary event (then disarms); otherwise the primary event fires.
    private func tap() {
        if button.shift {
            secondMode.toggle()
        } else if secondMode, let secondary = button.secondary {
            vm.send(secondary)
            secondMode = false
        } else if let event = button.event {
            vm.send(event)
            if secondMode { secondMode = false }
        }
    }

    /// Secondary caption tint: brighter yellow while armed so the live alternate
    /// functions stand out; a muted red otherwise.
    private var subColor: Color {
        secondMode
            ? Color(red: 0.99, green: 0.83, blue: 0.30)
            : Color(red: 0.95, green: 0.45, blue: 0.40)
    }

    /// Key fill. The "2nd" key glows yellow while armed; everything else uses
    /// its category tint.
    private var fill: Color {
        if button.shift {
            return secondMode
                ? Color(red: 0.99, green: 0.83, blue: 0.30)
                : Color(red: 0.85, green: 0.47, blue: 0.02)
        }
        switch button.style {
        case .function: return Color(red: 0.23, green: 0.23, blue: 0.54)
        case .unit:     return Color(red: 0.28, green: 0.35, blue: 0.46)
        case .op:       return Color(red: 0.85, green: 0.47, blue: 0.02)
        case .control:  return Color(red: 0.48, green: 0.12, blue: 0.12)
        case .num:      return Color(red: 0.16, green: 0.20, blue: 0.25)
        case .filler:   return .clear
        }
    }
}
