import Foundation

/// Visual category for a key, driving its tint. Mirrors the `style` field on
/// the web `Keypad.svelte` button specs.
enum KeyStyle {
    case function
    case op
    case num
    case unit
    case control
    case filler
}

/// One key in the pad. `event == nil` marks a filler cell that pads a row to
/// the full grid width and does nothing when tapped.
struct KeypadButton: Identifiable {
    let id = UUID()
    let label: String
    let event: KeyEvent?
    let style: KeyStyle
    /// How many of the 6 grid columns this key spans.
    let columns: Int
    /// Key into `HelpText.entries` for the long-press overlay.
    let helpId: String?
    /// Secondary ("2nd"-shift) label, shown small in red above the key — the
    /// ProjectCalc convention. Nil = no secondary.
    let sub: String?
    /// Event sent instead of `event` when the 2nd shift is armed.
    let secondary: KeyEvent?
    /// True only for the "2nd" key itself, which toggles secondary mode.
    let shift: Bool
    /// A guided estimator this key opens instead of sending an event — used by
    /// the "Calc" page. Nil for ordinary keys.
    let estimator: EstimatorRoute?

    init(_ label: String, _ event: KeyEvent?, _ style: KeyStyle,
         columns: Int = 1, help: String? = nil,
         sub: String? = nil, secondary: KeyEvent? = nil, shift: Bool = false,
         estimator: EstimatorRoute? = nil) {
        self.label = label
        self.event = event
        self.style = style
        self.columns = columns
        self.helpId = help
        self.sub = sub
        self.secondary = secondary
        self.shift = shift
        self.estimator = estimator
    }

    /// A blank, non-interactive cell.
    static func filler() -> KeypadButton {
        KeypadButton("", nil, .filler)
    }
}

/// The switchable function pages — Rafter / Trig / Miter / Mem — plus a "Calc"
/// page of one-tap estimator launchers. Mirrors the page tabs in `Keypad.svelte`.
enum KeypadPage: String, CaseIterable, Identifiable {
    case rafter = "Rafter"
    case trig = "Trig"
    case miter = "Miter"
    case memory = "Mem"
    case calc = "Calc"

    var id: String { rawValue }
}

/// Static keypad definition. Pure data so it reads like the web component
/// and stays trivially testable.
enum KeypadModel {
    static func functionRow(for page: KeypadPage) -> [KeypadButton] {
        switch page {
        case .rafter:
            return [
                fn("Pitch", .pitch, "pitch"),
                fn("Rise", .rise, "rise"),
                fn("Run", .run, "run"),
                fn("Diag", .diagonal, "diag"),
                fn("Hip/V", .hipValley, "hipv"),
                fn("Jack", .jack, "jack")
            ]
        case .trig:
            return [
                fn("sin", .sin, "sin"),
                fn("cos", .cos, "cos"),
                fn("tan", .tan, "tan"),
                fn("asin", .asin, "asin"),
                fn("acos", .acos, "acos"),
                fn("atan", .atan, "atan")
            ]
        case .miter:
            return [
                fn("Corner", .corner, "corner"),
                fn("Spring", .spring, "spring"),
                fn("Miter", .miter, "miter"),
                fn("Bevel", .bevel, "bevel"),
                .filler(),
                .filler()
            ]
        case .memory:
            return [
                KeypadButton("MS", .memory(op: .store(slot: 0)), .function, help: "memstore"),
                KeypadButton("MR", .memory(op: .recall(slot: 0)), .function, help: "memrecall"),
                KeypadButton("M+", .memory(op: .addTo(slot: 0)), .function, help: "memplus"),
                KeypadButton("MC", .memory(op: .clear(slot: 0)), .function, help: "memclear"),
                KeypadButton("MC All", .memory(op: .clearAll), .function, help: "memclearall"),
                .filler()
            ]
        case .calc:
            // Each key jumps straight to a guided estimator sheet.
            return EstimatorRoute.allCases.map { est($0) }
        }
    }

    /// Rows shared across every page. Laid out as a 6-column grid exactly
    /// like the web `sharedRows`.
    static let sharedRows: [[KeypadButton]] = [
        [
            KeypadButton("Yds", .unit(unit: .yards), .unit, help: "yd",
                         sub: "yd³", secondary: .convertVolume(format: .cubicYards(precision: 2))),
            KeypadButton("Feet", .unit(unit: .feet), .unit, help: "ft",
                         sub: "ft³", secondary: .convertVolume(format: .cubicFeet(precision: 2))),
            KeypadButton("Inch", .unit(unit: .inch), .unit, help: "in",
                         sub: "in³", secondary: .convertVolume(format: .cubicInches(precision: 1))),
            KeypadButton("m", .unit(unit: .meters), .unit, help: "m",
                         sub: "m³", secondary: .convertVolume(format: .cubicMeters(precision: 3))),
            KeypadButton("Cost", .costPerUnit, .unit, help: "cost"),
            KeypadButton("2nd", nil, .op, help: "shift", shift: true)
        ],
        [
            KeypadButton("C", .clear, .control, help: "c"),
            KeypadButton("AC", .clearAll, .control, help: "ac"),
            KeypadButton("⌫", .backspace, .control, help: "bs"),
            KeypadButton("/", .slash, .op, help: "slash"),
            KeypadButton("±", .negate, .op, help: "negate"),
            KeypadButton("÷", .op(op: .div), .op)
        ],
        [
            KeypadButton("√", .function(function: .sqrt), .function, help: "sqrt"),
            KeypadButton("x²", .function(function: .square), .function, help: "square",
                         sub: "ft²", secondary: .convertArea(format: .squareFeet(precision: 2))),
            KeypadButton("1/x", .function(function: .reciprocal), .function, help: "recip"),
            KeypadButton("%", .function(function: .percent), .function, help: "percent"),
            KeypadButton("×", .op(op: .mul), .op),
            KeypadButton("−", .op(op: .sub), .op)
        ],
        [
            digit(7), digit(8), digit(9),
            digit(4, sub: "lb", secondary: .weightUnit(unit: .pounds)),
            digit(5, sub: "tn", secondary: .weightUnit(unit: .tons)),
            digit(6, sub: "t", secondary: .weightUnit(unit: .tonnes))
        ],
        [
            digit(1, sub: "kg", secondary: .weightUnit(unit: .kilograms)),
            digit(2, sub: "Acre", secondary: .convertArea(format: .acres(precision: 2))),
            digit(3),
            KeypadButton("0", .digit(value: 0), .num, columns: 2),
            KeypadButton(".", .decimal, .num)
        ],
        [
            KeypadButton("+", .op(op: .add), .op),
            KeypadButton("=", .equals, .op, columns: 5)
        ]
    ]

    // MARK: - Builders

    private static func fn(_ label: String, _ key: FunctionKey, _ help: String) -> KeypadButton {
        KeypadButton(label, .function(function: key), .function, help: help)
    }

    private static func digit(_ value: UInt8, sub: String? = nil, secondary: KeyEvent? = nil) -> KeypadButton {
        KeypadButton(String(value), .digit(value: value), .num, sub: sub, secondary: secondary)
    }

    /// A "Calc" page key that opens a guided estimator sheet.
    private static func est(_ route: EstimatorRoute) -> KeypadButton {
        KeypadButton(route.keyLabel, nil, .function, help: "calc", estimator: route)
    }
}
