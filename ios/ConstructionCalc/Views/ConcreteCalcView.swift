import SwiftUI

/// Concrete-volume estimator: length × width × depth → m³ (primary), plus yd³,
/// ft³ and a 60/80 lb bag count. Concrete is ordered to the quarter-yard, so a
/// Double estimate is plenty precise here.
struct ConcreteCalcView: View {
    @State private var length = ""
    @State private var lengthUnit: LenUnit = .feet
    @State private var width = ""
    @State private var widthUnit: LenUnit = .feet
    @State private var depth = ""
    @State private var depthUnit: LenUnit = .inch

    private var cubicMeters: Double? {
        guard let l = meters(length, lengthUnit),
              let w = meters(width, widthUnit),
              let d = meters(depth, depthUnit) else { return nil }
        return l * w * d
    }

    var body: some View {
        Form {
            Section("Dimensions") {
                DimField(label: "Length", text: $length, unit: $lengthUnit)
                DimField(label: "Width", text: $width, unit: $widthUnit)
                DimField(label: "Depth", text: $depth, unit: $depthUnit)
            }
            Section("Volume") {
                if let m3 = cubicMeters {
                    ResultRow("Cubic meters", String(format: "%.3f m³", m3))
                    ResultRow("Cubic yards", String(format: "%.2f yd³", m3 * 1.307950619))
                    ResultRow("Cubic feet", String(format: "%.1f ft³", m3 * 35.3146667))
                    ResultRow("60 lb bags", "≈ \(Int((m3 * 35.3146667 / 0.45).rounded(.up)))")
                    ResultRow("80 lb bags", "≈ \(Int((m3 * 35.3146667 / 0.60).rounded(.up)))")
                } else {
                    Text("Enter length, width, and depth.").foregroundStyle(.secondary)
                }
            }
            Section {
                Text("A slab is usually feet × feet × inches. Concrete is sold by the cubic yard — round up, and add ~10% for waste.")
                    .font(.footnote).foregroundStyle(.secondary)
            }
        }
        .navigationTitle("Concrete")
        .navigationBarTitleDisplayMode(.inline)
    }
}
