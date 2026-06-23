import SwiftUI

/// The button grid. Faithful port of `Keypad.svelte`: a page picker
/// (Rafter / Trig / Miter / Mem / Calc) over a 6-column grid of shared rows,
/// plus the 4-column number pad. Long-pressing a key surfaces its help entry;
/// the backspace key holds-to-repeat. A "Compact" preference tightens the pad.
struct KeypadView: View {
    @Binding var helpId: String?
    /// Set when a "Calc" page key is tapped, so the root view can present that
    /// estimator as a sheet.
    @Binding var estimatorRoute: EstimatorRoute?
    @Environment(\.calcTheme) private var theme
    @AppStorage(Preferences.Key.compactKeypad) private var compact = false

    @State private var page: KeypadPage = .rafter
    /// When armed by the "2nd" key, a key with a red secondary label sends that
    /// secondary event instead of its primary one.
    @State private var secondMode = false

    /// The function-page row plus the shared unit / control rows — the compact
    /// 6-column zone above the number pad.
    private var topRows: [[KeypadButton]] {
        [KeypadModel.functionRow(for: page)] + KeypadModel.topSharedRows
    }

    /// Tighter gaps in compact mode so more of the pad fits.
    private var gap: CGFloat { compact ? 3 : 4 }

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
            Grid(horizontalSpacing: gap, verticalSpacing: gap) {
                ForEach(Array(topRows.enumerated()), id: \.offset) { _, row in
                    GridRow {
                        ForEach(row) { button in
                            KeyButton(button: button, helpId: $helpId,
                                      secondMode: $secondMode, estimatorRoute: $estimatorRoute,
                                      compact: compact)
                                .gridCellColumns(button.columns)
                        }
                    }
                }
            }

            // Number pad as a real 4-column calculator block (three big digit
            // keys + an operator per row). In the comfortable layout the digit
            // keys stretch to fill the leftover height; compact keeps them
            // uniform so the whole pad sits higher and shows more at once.
            VStack(spacing: gap) {
                ForEach(Array(KeypadModel.calcBlock.enumerated()), id: \.offset) { _, row in
                    HStack(spacing: gap) {
                        ForEach(row) { button in
                            KeyButton(button: button, helpId: $helpId,
                                      secondMode: $secondMode, estimatorRoute: $estimatorRoute,
                                      stretch: !compact, compact: compact)
                        }
                    }
                    .frame(maxHeight: compact ? nil : .infinity)
                }
            }
            .frame(maxHeight: compact ? nil : .infinity)
        }
        .padding(compact ? 4 : 6)
        .background(
            RoundedRectangle(cornerRadius: 12).fill(theme.panel)
        )
    }
}

/// Press feedback: a quick scale + dim so keys feel responsive under a thumb,
/// like a physical pad.
private struct CalcKeyStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .scaleEffect(configuration.isPressed ? 0.93 : 1)
            .opacity(configuration.isPressed ? 0.82 : 1)
            .animation(.easeOut(duration: 0.09), value: configuration.isPressed)
    }
}

/// A single key. Tap sends the event through the view model; long-press raises
/// the help id (with a haptic); the backspace key holds-to-repeat instead.
/// Filler cells render invisibly so the grid stays aligned. Keys with a `sub`
/// label show it small above the main label (red normally, accent while the
/// "2nd" shift is armed).
private struct KeyButton: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Environment(\.calcTheme) private var theme
    let button: KeypadButton
    @Binding var helpId: String?
    @Binding var secondMode: Bool
    @Binding var estimatorRoute: EstimatorRoute?
    /// When true the key grows to fill extra vertical space (used by the number
    /// pad so the digit keys read tall and chunky like a physical calculator).
    var stretch: Bool = false
    /// Denser sizing for the compact layout preference.
    var compact: Bool = false

    /// Swallows the tap that lands on release after a long-press (help) or a
    /// hold-repeat, so neither also fires a stray key event.
    @State private var suppressTap = false
    @State private var repeatTimer: Timer?

    private var minHeight: CGFloat { compact ? 44 : 48 }

    var body: some View {
        if button.style == .filler {
            // Fixed height (not just a minimum): a bare Color is greedy and would
            // otherwise balloon its grid row to absorb all slack, leaving a gap
            // on pages whose function row has empty cells (Miter, Mem).
            Color.clear.frame(maxWidth: .infinity, minHeight: minHeight, maxHeight: minHeight)
        } else if button.repeats {
            // Hold-to-repeat (backspace): a long-press starts the repeat, a drag
            // end (finger lift) stops it. Both run alongside the button's tap, so
            // a quick tap still deletes exactly one. Simultaneous gestures avoid
            // the button swallowing the press the way `.onLongPressGesture` does.
            keyButton
                .simultaneousGesture(
                    LongPressGesture(minimumDuration: 0.35, maximumDistance: 50)
                        .onEnded { _ in startRepeat() }
                )
                .simultaneousGesture(
                    DragGesture(minimumDistance: 0).onEnded { _ in stopRepeat() }
                )
                .onDisappear { stopRepeat() }
        } else {
            keyButton
                // A *simultaneous* long-press runs alongside the button's tap so
                // a still hold fires it right away (no drag needed).
                .simultaneousGesture(
                    LongPressGesture(minimumDuration: 0.35, maximumDistance: 40)
                        .onEnded { _ in longPress() }
                )
        }
    }

    private var keyButton: some View {
        Button {
            if suppressTap { suppressTap = false } else { tap() }
        } label: {
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
                    .foregroundStyle(theme.keyText)
            }
            .frame(maxWidth: .infinity, minHeight: minHeight, maxHeight: stretch ? .infinity : nil)
            .background(
                RoundedRectangle(cornerRadius: 8).fill(fill)
            )
        }
        .buttonStyle(CalcKeyStyle())
    }

    /// Routes the tap: the "2nd" key toggles shift; an armed shift fires a key's
    /// secondary event (then disarms); a "Calc" key opens its estimator;
    /// otherwise the primary event fires.
    private func tap() {
        if button.shift {
            secondMode.toggle()
        } else if secondMode, let secondary = button.secondary {
            vm.send(secondary)
            secondMode = false
        } else if let route = button.estimator {
            estimatorRoute = route
            if secondMode { secondMode = false }
        } else if let event = button.event {
            vm.send(event)
            if secondMode { secondMode = false }
        }
    }

    /// Long-press → firm haptic + the key's help entry. Keys without a help
    /// entry ignore it.
    private func longPress() {
        guard let id = button.helpId else { return }
        suppressTap = true
        Haptics.shared.longPress()
        helpId = id
    }

    /// Begin hold-to-repeat: fire once (with its haptic), then repeat silently
    /// on a timer until release. The timer is added to the common run-loop modes
    /// so it keeps firing while the finger is down (touch tracking).
    private func startRepeat() {
        guard let event = button.event else { return }
        suppressTap = true
        vm.send(event)
        repeatTimer?.invalidate()
        let timer = Timer(timeInterval: 0.09, repeats: true) { _ in
            vm.send(event, haptic: false)
        }
        RunLoop.main.add(timer, forMode: .common)
        repeatTimer = timer
    }

    private func stopRepeat() {
        repeatTimer?.invalidate()
        repeatTimer = nil
    }

    /// Secondary caption tint: the accent (yellow / amber) while armed so the
    /// live alternate functions stand out; a muted red otherwise.
    private var subColor: Color {
        secondMode ? theme.accent : theme.subNormal
    }

    /// Key fill. The "2nd" key glows in the accent colour while armed;
    /// everything else uses its category tint from the theme.
    private var fill: Color {
        if button.shift {
            return secondMode ? theme.accent : theme.opKey
        }
        return theme.keyFill(button.style)
    }
}
