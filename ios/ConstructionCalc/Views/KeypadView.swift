import SwiftUI

/// The button grid. Faithful port of `Keypad.svelte`: a page picker
/// (Rafter / Trig / Miter / Mem) over a 6-column grid of shared rows.
/// Long-pressing a key surfaces its help entry via the `helpId` binding.
struct KeypadView: View {
    @Binding var helpId: String?

    @State private var page: KeypadPage = .rafter

    private var rows: [[KeypadButton]] {
        [KeypadModel.functionRow(for: page)] + KeypadModel.sharedRows
    }

    var body: some View {
        VStack(spacing: 10) {
            Picker("Function page", selection: $page) {
                ForEach(KeypadPage.allCases) { p in
                    Text(p.rawValue).tag(p)
                }
            }
            .pickerStyle(.segmented)

            Grid(horizontalSpacing: 6, verticalSpacing: 6) {
                ForEach(Array(rows.enumerated()), id: \.offset) { _, row in
                    GridRow {
                        ForEach(row) { button in
                            KeyButton(button: button, helpId: $helpId)
                                .gridCellColumns(button.columns)
                        }
                    }
                }
            }
        }
        .padding(10)
        .background(
            RoundedRectangle(cornerRadius: 12).fill(Color.white.opacity(0.03))
        )
    }
}

/// A single key. Tap sends the event through the view model; long-press
/// raises the help id. Filler cells (no event) render invisibly so the grid
/// stays aligned.
private struct KeyButton: View {
    @Environment(CalculatorViewModel.self) private var vm
    let button: KeypadButton
    @Binding var helpId: String?

    var body: some View {
        if button.event == nil && button.style == .filler {
            Color.clear.frame(maxWidth: .infinity, minHeight: 52)
        } else {
            Button {
                if let event = button.event { vm.send(event) }
            } label: {
                Text(button.label)
                    .font(.system(size: 19, weight: .medium))
                    .frame(maxWidth: .infinity, minHeight: 52)
                    .background(
                        RoundedRectangle(cornerRadius: 10).fill(background)
                    )
                    .foregroundStyle(.white)
            }
            .buttonStyle(.plain)
            .onLongPressGesture(minimumDuration: 0.4) {
                if let id = button.helpId { helpId = id }
            }
        }
    }

    private var background: Color {
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
