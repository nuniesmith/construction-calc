import Foundation

/// One context-help entry, shown after a long-press. Mirrors the `HelpEntry`
/// shape and content of `frontend/src/lib/help.ts`.
struct HelpEntry {
    let title: String
    let body: String
    var formula: String? = nil
    var example: String? = nil
}

/// Static help table keyed by the `helpId` on each keypad button.
enum HelpText {
    static func entry(for id: String) -> HelpEntry? { entries[id] }

    static let entries: [String: HelpEntry] = [
        // -------- Rafter --------
        "pitch": HelpEntry(
            title: "Pitch",
            body: "Roof slope as inches of rise per 12 inches of run. Enter a number then press Pitch — small integers are interpreted as \"X/12\".",
            formula: "pitch = rise ÷ run",
            example: "6 [Pitch] = 6/12 pitch"
        ),
        "rise": HelpEntry(
            title: "Rise",
            body: "Vertical change. With pitch and run already entered, pressing Rise solves and shows the rise."
        ),
        "run": HelpEntry(
            title: "Run",
            body: "Horizontal distance. With pitch and rise entered, pressing Run solves for the run."
        ),
        "diag": HelpEntry(
            title: "Diagonal",
            body: "Common rafter length / hypotenuse. With any two of pitch, rise, run, this returns the diagonal.",
            formula: "diag² = rise² + run²"
        ),
        "hipv": HelpEntry(
            title: "Hip / Valley",
            body: "Length of a regular (square) hip or valley rafter. Hip run = run × √2."
        ),
        "jack": HelpEntry(
            title: "Jack rafter",
            body: "Difference in length between consecutive jack rafters at the current on-center spacing."
        ),
        // -------- Trig --------
        "sin": HelpEntry(title: "Sine", body: "sin of the displayed angle."),
        "cos": HelpEntry(title: "Cosine", body: "cos of the displayed angle."),
        "tan": HelpEntry(title: "Tangent", body: "tan of the displayed angle."),
        "asin": HelpEntry(title: "Arcsine", body: "Angle whose sine is the displayed value. Domain: −1 to 1."),
        "acos": HelpEntry(title: "Arccosine", body: "Angle whose cosine is the displayed value. Domain: −1 to 1."),
        "atan": HelpEntry(title: "Arctangent", body: "Angle whose tangent is the displayed value."),
        // -------- Math --------
        "sqrt": HelpEntry(title: "Square root", body: "Square root of the displayed value. Area → Length, Scalar → Scalar."),
        "square": HelpEntry(title: "Square", body: "Multiplies the displayed value by itself. Length × Length = Area."),
        "recip": HelpEntry(title: "Reciprocal", body: "1 ÷ x"),
        "percent": HelpEntry(
            title: "Percent",
            body: "After an operator, treats the entered number as a percent of the previous operand.",
            example: "200 + 5 % = adds 5% of 200 = 210"
        ),
        // -------- Compound miter --------
        "corner": HelpEntry(
            title: "Corner angle",
            body: "Wall corner viewed from above. 90° for a standard inside or outside corner. Enter a value (degrees) then press Corner.",
            example: "90 [Corner]"
        ),
        "spring": HelpEntry(
            title: "Spring angle",
            body: "Angle between the back of the molding and the wall. 38° and 45° are typical for crown — usually printed on the box.",
            example: "38 [Spring]"
        ),
        "miter": HelpEntry(title: "Miter angle", body: "Saw rotation. Press after entering both Corner and Spring to get the miter setting."),
        "bevel": HelpEntry(title: "Bevel angle", body: "Blade tilt off vertical. Press after entering both Corner and Spring to get the bevel setting."),
        // -------- Units, control --------
        "ft": HelpEntry(title: "Feet", body: "Tag the typed number as feet. Press multiple unit keys to build a compound length like 5 ft 6 in."),
        "in": HelpEntry(title: "Inch", body: "Tag the typed number as inches. Use the / key to enter a fraction like 3 / 8 [Inch]."),
        "yd": HelpEntry(title: "Yards", body: "Tag the typed number as yards."),
        "m": HelpEntry(title: "Meters", body: "Tag the typed number as meters. Use the format strip above the display to render results in m."),
        "slash": HelpEntry(
            title: "Fraction divider",
            body: "Use during entry to type a fraction. After typing 3 / 8, you have 3/8. After 5, /, 3, /, 8, you have 5-3/8."
        ),
        "negate": HelpEntry(title: "Toggle sign", body: "Flip the sign of the entry buffer."),
        "bs": HelpEntry(title: "Backspace", body: "Remove the last typed digit, decimal point, or fraction component."),
        "c": HelpEntry(title: "Clear entry", body: "Reset the entry buffer to zero. Pending operators and memory are preserved."),
        "ac": HelpEntry(title: "All clear", body: "Reset everything: entry, pending op, rafter state, miter state. Memory is preserved."),
        // -------- Memory --------
        "memstore": HelpEntry(title: "Memory store (MS)", body: "Overwrite memory slot 1 with the current display value. The engine reserves four slots; the keypad uses slot 1."),
        "memrecall": HelpEntry(title: "Memory recall (MR)", body: "Load the value from memory slot 1 onto the display."),
        "memplus": HelpEntry(title: "Memory add (M+)", body: "Add the current display value into memory slot 1 (in-place)."),
        "memclear": HelpEntry(title: "Memory clear (MC)", body: "Zero out memory slot 1."),
        "memclearall": HelpEntry(title: "Clear all memory", body: "Zero out every memory slot the engine tracks.")
    ]
}
