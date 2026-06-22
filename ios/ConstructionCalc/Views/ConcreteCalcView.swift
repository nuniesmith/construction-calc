import SwiftUI

/// Quick concrete-volume estimator: enter length × width × depth (each with its
/// own unit) and get the pour in m³ — plus yd³ / ft³ and a bag estimate. Concrete
/// is ordered to the quarter-yard, so a Double estimate is plenty precise here;
/// this stays self-contained (no engine round-trip needed).
struct ConcreteCalcView: View {
    @Environment(\.dismiss) private var dismiss

    /// Length unit for one dimension.
    enum DimUnit: String, CaseIterable, Identifiable {
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

    @State private var length = ""
    @State private var lengthUnit: DimUnit = .feet
    @State private var width = ""
    @State private var widthUnit: DimUnit = .feet
    @State private var depth = ""
    @State private var depthUnit: DimUnit = .inch

    /// Volume in cubic meters, or nil until all three dimensions are valid + positive.
    private var cubicMeters: Double? {
        guard let l = Double(length), let w = Double(width), let d = Double(depth),
              l > 0, w > 0, d > 0 else { return nil }
        return (l * lengthUnit.toMeters) * (w * widthUnit.toMeters) * (d * depthUnit.toMeters)
    }

    var body: some View {
        NavigationStack {
            Form {
                Section("Dimensions") {
                    dimensionRow("Length", text: $length, unit: $lengthUnit)
                    dimensionRow("Width", text: $width, unit: $widthUnit)
                    dimensionRow("Depth", text: $depth, unit: $depthUnit)
                }

                Section("Volume") {
                    if let m3 = cubicMeters {
                        result("Cubic meters", String(format: "%.3f m³", m3))
                        result("Cubic yards", String(format: "%.2f yd³", m3 * 1.307950619))
                        result("Cubic feet", String(format: "%.1f ft³", m3 * 35.3146667))
                        result("60 lb bags", "≈ \(Int((m3 * 35.3146667 / 0.45).rounded(.up)))")
                        result("80 lb bags", "≈ \(Int((m3 * 35.3146667 / 0.60).rounded(.up)))")
                    } else {
                        Text("Enter length, width, and depth.")
                            .foregroundStyle(.secondary)
                    }
                }

                Section {
                    Text("A slab is usually feet × feet × inches. Concrete is sold by the cubic yard — round up, and add ~10% for waste.")
                        .font(.footnote)
                        .foregroundStyle(.secondary)
                }
            }
            .navigationTitle("Concrete")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }

    private func dimensionRow(_ label: String, text: Binding<String>, unit: Binding<DimUnit>) -> some View {
        HStack {
            Text(label).frame(width: 70, alignment: .leading)
            TextField("0", text: text)
                .keyboardType(.decimalPad)
                .multilineTextAlignment(.trailing)
            Picker("", selection: unit) {
                ForEach(DimUnit.allCases) { u in Text(u.rawValue).tag(u) }
            }
            .labelsHidden()
            .pickerStyle(.menu)
        }
    }

    private func result(_ label: String, _ value: String) -> some View {
        HStack {
            Text(label)
            Spacer()
            Text(value).fontWeight(.semibold).monospacedDigit()
        }
    }
}
