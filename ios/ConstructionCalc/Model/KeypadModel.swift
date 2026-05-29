import Foundation
import CalcEngine

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

    init(_ label: String, _ event: KeyEvent?, _ style: KeyStyle,
         columns: Int = 1, help: String? = nil) {
        self.label = label
        self.event = event
        self.style = style
        self.columns = columns
        self.helpId = help
    }

    /// A blank, non-interactive cell.
    static func filler() -> KeypadButton {
        KeypadButton("", nil, .filler)
    }
}

/// The switchable function pages — Rafter / Trig / Miter / Mem — mirroring
/// the page tabs in `Keypad.svelte`.
enum KeypadPage: String, CaseIterable, Identifiable {
    case rafter = "Rafter"
    case trig = "Trig"
    case miter = "Miter"
    case memory = "Mem"

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
        }
    }

    /// Rows shared across every page. Laid out as a 6-column grid exactly
    /// like the web `sharedRows`.
    static let sharedRows: [[KeypadButton]] = [
        [
            KeypadButton("Yds", .unit(unit: .yards), .unit, help: "yd"),
            KeypadButton("Feet", .unit(unit: .feet), .unit, help: "ft"),
            KeypadButton("Inch", .unit(unit: .inch), .unit, help: "in"),
            KeypadButton("m", .unit(unit: .meters), .unit, help: "m"),
            .filler(),
            .filler()
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
            KeypadButton("x²", .function(function: .square), .function, help: "square"),
            KeypadButton("1/x", .function(function: .reciprocal), .function, help: "recip"),
            KeypadButton("%", .function(function: .percent), .function, help: "percent"),
            KeypadButton("×", .op(op: .mul), .op),
            KeypadButton("−", .op(op: .sub), .op)
        ],
        [
            digit(7), digit(8), digit(9), digit(4), digit(5), digit(6)
        ],
        [
            digit(1), digit(2), digit(3),
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

    private static func digit(_ value: UInt8) -> KeypadButton {
        KeypadButton(String(value), .digit(value: value), .num)
    }
}
