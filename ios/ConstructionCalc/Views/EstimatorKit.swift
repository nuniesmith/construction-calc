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
