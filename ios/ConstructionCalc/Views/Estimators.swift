import SwiftUI

// Guided construction estimators, each a self-contained Swift form (math in a
// computed property, views in the body). Reached from the Calcs menu.

/// Launcher listing every estimator.
struct CalcsMenuView: View {
    @Environment(\.dismiss) private var dismiss
    var body: some View {
        NavigationStack {
            List {
                Section("Estimators") {
                    NavigationLink { ConcreteCalcView() } label: { Label("Concrete", systemImage: "cube") }
                    NavigationLink { StairCalcView() } label: { Label("Stairs", systemImage: "figure.stairs") }
                    NavigationLink { CircleCalcView() } label: { Label("Circle / Column", systemImage: "circle") }
                    NavigationLink { FramingCalcView() } label: { Label("Framing", systemImage: "rectangle.split.3x1") }
                    NavigationLink { RebarCalcView() } label: { Label("Rebar", systemImage: "square.grid.3x3") }
                    NavigationLink { RoofingCalcView() } label: { Label("Roofing", systemImage: "house") }
                }
                Section {
                    Text("Each tool fills in a few dimensions and gives you quantities — concrete volume, stair layout, sheet/bundle counts, and so on.")
                        .font(.footnote).foregroundStyle(.secondary)
                }
            }
            .navigationTitle("Calcs")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) { Button("Done") { dismiss() } }
            }
        }
    }
}

/// Stair layout: total rise (+ optional run) → riser/tread counts and sizes.
struct StairCalcView: View {
    @State private var rise = ""
    @State private var riseUnit: LenUnit = .feet
    @State private var run = ""
    @State private var runUnit: LenUnit = .feet

    private var result: (risers: Int, riserH: Double, treads: Int, treadD: Double, runIn: Double)? {
        guard let rm = meters(rise, riseUnit) else { return nil }
        let riseIn = rm / 0.0254
        let risers = max(1, Int((riseIn / 7.0).rounded()))
        let riserH = riseIn / Double(risers)
        let treads = max(0, risers - 1)
        let runIn: Double
        let treadD: Double
        if let runM = meters(run, runUnit), treads > 0 {
            runIn = runM / 0.0254
            treadD = runIn / Double(treads)
        } else {
            treadD = 10.0
            runIn = Double(treads) * treadD
        }
        return (risers, riserH, treads, treadD, runIn)
    }

    var body: some View {
        Form {
            Section("Stair opening") {
                DimField(label: "Total rise", text: $rise, unit: $riseUnit)
                DimField(label: "Total run", text: $run, unit: $runUnit)
            }
            Section("Layout") {
                if let r = result {
                    ResultRow("Risers", "\(r.risers)")
                    ResultRow("Riser height", String(format: "%.3f in", r.riserH))
                    ResultRow("Treads", "\(r.treads)")
                    ResultRow("Tread depth", String(format: "%.3f in", r.treadD))
                    ResultRow("Total run", String(format: "%.1f in (%.2f ft)", r.runIn, r.runIn / 12))
                    if r.riserH > 7.75 || r.riserH < 6.0 {
                        Label("Riser height is outside the typical 6–7¾″ — try one more riser.", systemImage: "exclamationmark.triangle")
                            .font(.footnote).foregroundStyle(.orange)
                    }
                } else {
                    Text("Enter the total rise (floor to floor). Total run is optional — leave it blank to size from a 10″ tread.")
                        .foregroundStyle(.secondary)
                }
            }
            Section {
                Text("Rule of thumb: riser ≈ 7″ (max 7¾″), tread ≥ 10″, and there's always one more riser than tread.")
                    .font(.footnote).foregroundStyle(.secondary)
            }
        }
        .navigationTitle("Stairs")
        .navigationBarTitleDisplayMode(.inline)
    }
}

/// Circle measures + a round-column / cylinder volume.
struct CircleCalcView: View {
    @State private var diameter = ""
    @State private var diameterUnit: LenUnit = .feet
    @State private var depth = ""
    @State private var depthUnit: LenUnit = .inch

    private var circle: (radius: Double, circ: Double, area: Double)? {
        guard let dm = meters(diameter, diameterUnit) else { return nil }
        let r = dm / 2
        return (r, Double.pi * dm, Double.pi * r * r)
    }
    private var volume: Double? {
        guard let c = circle, let h = meters(depth, depthUnit) else { return nil }
        return c.area * h
    }

    var body: some View {
        Form {
            Section("Circle") {
                DimField(label: "Diameter", text: $diameter, unit: $diameterUnit)
            }
            Section("Results") {
                if let c = circle {
                    ResultRow("Radius", String(format: "%.3f m (%.2f ft)", c.radius, c.radius / 0.3048))
                    ResultRow("Circumference", String(format: "%.3f m (%.2f ft)", c.circ, c.circ / 0.3048))
                    ResultRow("Area", String(format: "%.3f m² (%.1f ft²)", c.area, c.area * 10.7639104))
                } else {
                    Text("Enter a diameter.").foregroundStyle(.secondary)
                }
            }
            Section("Column / cylinder volume") {
                DimField(label: "Depth / height", text: $depth, unit: $depthUnit)
                if let v = volume {
                    ResultRow("Volume", String(format: "%.3f m³ (%.2f yd³)", v, v * 1.307950619))
                } else {
                    Text("Add a depth for a round footing or column pour.").foregroundStyle(.secondary)
                }
            }
        }
        .navigationTitle("Circle")
        .navigationBarTitleDisplayMode(.inline)
    }
}

/// Wall framing: stud + plate counts, plus drywall sheets.
struct FramingCalcView: View {
    @State private var length = ""
    @State private var lengthUnit: LenUnit = .feet
    @State private var height = ""
    @State private var heightUnit: LenUnit = .feet
    @State private var spacing = 16

    private var studs: Int? {
        guard let lm = meters(length, lengthUnit) else { return nil }
        return Int(((lm / 0.0254) / Double(spacing)).rounded(.down)) + 2 // +1 end, +1 corner
    }
    private var plateFeet: Double? {
        guard let lm = meters(length, lengthUnit) else { return nil }
        return (lm / 0.3048) * 3 // bottom + double top plate
    }
    private var drywallSheets: Int? {
        guard let lm = meters(length, lengthUnit), let hm = meters(height, heightUnit) else { return nil }
        return Int(((lm / 0.3048) * (hm / 0.3048) / 32.0).rounded(.up)) // 4×8 = 32 ft², one side
    }

    var body: some View {
        Form {
            Section("Wall") {
                DimField(label: "Length", text: $length, unit: $lengthUnit)
                DimField(label: "Height", text: $height, unit: $heightUnit)
                Picker("Stud spacing", selection: $spacing) {
                    Text("12\" o.c.").tag(12)
                    Text("16\" o.c.").tag(16)
                    Text("24\" o.c.").tag(24)
                }
            }
            Section("Framing") {
                if let s = studs, let p = plateFeet {
                    ResultRow("Studs", "\(s)")
                    ResultRow("Plate", String(format: "%.0f lin ft", p))
                } else {
                    Text("Enter the wall length.").foregroundStyle(.secondary)
                }
            }
            Section("Drywall") {
                if let d = drywallSheets {
                    ResultRow("4×8 sheets (one side)", "\(d)")
                    ResultRow("Both sides", "\(d * 2)")
                } else {
                    Text("Add a height for a drywall sheet count.").foregroundStyle(.secondary)
                }
            }
        }
        .navigationTitle("Framing")
        .navigationBarTitleDisplayMode(.inline)
    }
}

/// Rebar grid for a slab.
struct RebarCalcView: View {
    @State private var length = ""
    @State private var lengthUnit: LenUnit = .feet
    @State private var width = ""
    @State private var widthUnit: LenUnit = .feet
    @State private var spacing = 16

    private var result: (lengthwise: Int, widthwise: Int, total: Int, linFt: Double)? {
        guard let lm = meters(length, lengthUnit), let wm = meters(width, widthUnit) else { return nil }
        let lFt = lm / 0.3048, wFt = wm / 0.3048, sFt = Double(spacing) / 12.0
        let acrossWidth = Int((wFt / sFt).rounded(.down)) + 1  // bars that run the length
        let acrossLength = Int((lFt / sFt).rounded(.down)) + 1 // bars that run the width
        let linFt = Double(acrossWidth) * lFt + Double(acrossLength) * wFt
        return (acrossWidth, acrossLength, acrossWidth + acrossLength, linFt)
    }

    var body: some View {
        Form {
            Section("Slab") {
                DimField(label: "Length", text: $length, unit: $lengthUnit)
                DimField(label: "Width", text: $width, unit: $widthUnit)
                Picker("Grid spacing", selection: $spacing) {
                    Text("12\" o.c.").tag(12)
                    Text("16\" o.c.").tag(16)
                    Text("18\" o.c.").tag(18)
                    Text("24\" o.c.").tag(24)
                }
            }
            Section("Rebar grid") {
                if let r = result {
                    ResultRow("Bars (run lengthwise)", "\(r.lengthwise)")
                    ResultRow("Bars (run widthwise)", "\(r.widthwise)")
                    ResultRow("Total bars", "\(r.total)")
                    ResultRow("Total length", String(format: "%.0f ft", r.linFt))
                } else {
                    Text("Enter the slab length and width.").foregroundStyle(.secondary)
                }
            }
        }
        .navigationTitle("Rebar")
        .navigationBarTitleDisplayMode(.inline)
    }
}

/// Roofing materials from a plan footprint + pitch.
struct RoofingCalcView: View {
    @State private var length = ""
    @State private var lengthUnit: LenUnit = .feet
    @State private var width = ""
    @State private var widthUnit: LenUnit = .feet
    @State private var pitch = 6

    private var pitchFactor: Double {
        let slope = Double(pitch) / 12.0
        return (1.0 + slope * slope).squareRoot()
    }
    private var result: (areaFt2: Double, squares: Double, bundles: Int)? {
        guard let lm = meters(length, lengthUnit), let wm = meters(width, widthUnit) else { return nil }
        let roofFt2 = (lm / 0.3048) * (wm / 0.3048) * pitchFactor
        let squares = roofFt2 / 100.0
        return (roofFt2, squares, Int((squares * 3.0).rounded(.up)))
    }

    var body: some View {
        Form {
            Section("Roof footprint (plan view)") {
                DimField(label: "Length", text: $length, unit: $lengthUnit)
                DimField(label: "Width", text: $width, unit: $widthUnit)
                Picker("Pitch", selection: $pitch) {
                    ForEach([0, 3, 4, 6, 8, 10, 12], id: \.self) { p in Text("\(p)/12").tag(p) }
                }
            }
            Section("Materials") {
                if let r = result {
                    ResultRow("Roof area", String(format: "%.0f ft² (%.1f m²)", r.areaFt2, r.areaFt2 * 0.092903))
                    ResultRow("Squares", String(format: "%.1f", r.squares))
                    ResultRow("Shingle bundles", "\(r.bundles)")
                } else {
                    Text("Enter the building length and width (plan view).").foregroundStyle(.secondary)
                }
            }
            Section {
                Text("Plan area × pitch factor = sloped area. 3 bundles ≈ 1 square (100 ft²); add ~10% for waste, starter and ridge.")
                    .font(.footnote).foregroundStyle(.secondary)
            }
        }
        .navigationTitle("Roofing")
        .navigationBarTitleDisplayMode(.inline)
    }
}
