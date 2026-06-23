import SwiftUI

/// Shared building blocks for the guided estimator sheets (Concrete, Stair,
/// Circle, Framing, Rebar, Roofing). Each estimator is a self-contained Swift
/// form that computes its formula directly — fast, offline, and independent of
/// the calculator engine (the exact-rational engine can back these later).

/// A linear-dimension unit with its conversion to meters (the common base).
enum LenUnit: String, CaseIterable, Identifiable {
    case feet = "ft"
    case inch = "in"
    case meters = "m"
    case cm = "cm"
    var id: String { rawValue }
    var toMeters: Double {
        switch self {
        case .feet: return 0.3048
        case .inch: return 0.0254
        case .meters: return 1.0
        case .cm: return 0.01
        }
    }
}

/// A labelled numeric field + unit picker, returning meters via `LenUnit`.
struct DimField: View {
    let label: String
    @Binding var text: String
    @Binding var unit: LenUnit

    var body: some View {
        HStack {
            Text(label).frame(width: 104, alignment: .leading)
            TextField("0", text: $text)
                .keyboardType(.decimalPad)
                .multilineTextAlignment(.trailing)
            Picker("", selection: $unit) {
                ForEach(LenUnit.allCases) { u in Text(u.rawValue).tag(u) }
            }
            .labelsHidden()
            .pickerStyle(.menu)
        }
    }
}

/// A right-aligned label → value result line.
struct ResultRow: View {
    let label: String
    let value: String
    init(_ label: String, _ value: String) {
        self.label = label
        self.value = value
    }
    var body: some View {
        HStack {
            Text(label)
            Spacer()
            Text(value).fontWeight(.semibold).monospacedDigit()
        }
    }
}

/// Parse a field in its unit into meters (nil if blank/invalid/≤0).
func meters(_ text: String, _ unit: LenUnit) -> Double? {
    guard let v = Double(text), v > 0 else { return nil }
    return v * unit.toMeters
}

/// Format a number to at most 3 decimals, trimming trailing zeros (and a
/// dangling decimal point) so a seeded field reads cleanly: 8 → "8", not
/// "8.000".
func trimmedNumber(_ v: Double) -> String {
    var s = String(format: "%.3f", v)
    if s.contains(".") {
        while s.hasSuffix("0") { s.removeLast() }
        if s.hasSuffix(".") { s.removeLast() }
    }
    return s
}

/// A share/copy row for an estimator's results — shown only once there's a
/// result. The iOS share sheet includes Copy, so this covers both.
struct ShareResultsRow: View {
    let summary: String?
    var body: some View {
        if let summary {
            ShareLink(item: summary) {
                Label("Share results", systemImage: "square.and.arrow.up")
            }
            .font(.footnote)
        }
    }
}

/// One-tap row that seeds a dimension field from the calculator's current
/// display, shown only when the display holds a length. Bridges the keypad and
/// the guided estimators so a measured value flows straight into a takeoff.
struct SeedRow: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Binding var text: String
    @Binding var unit: LenUnit

    var body: some View {
        if let feet = vm.displayLengthFeet, feet > 0 {
            let value = trimmedNumber(feet)
            Button {
                text = value
                unit = .feet
                Haptics.shared.tap()
            } label: {
                Label("Use calculator value — \(value) ft", systemImage: "arrow.down.to.line")
            }
            .font(.footnote)
        }
    }
}
