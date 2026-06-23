import Foundation
import Observation

/// Observable wrapper around the UniFFI `Calculator`.
///
/// Mirrors the responsibilities of `frontend/src/lib/calc.ts`: it hides the
/// engine handle, exposes reactive display / tape / error state, and is the
/// single funnel every key press flows through. SwiftUI views observe the
/// published properties and never touch the engine directly.
@Observable
final class CalculatorViewModel {
    /// The formatted display register (already rendered for the chosen mode).
    private(set) var display: String = "0"

    /// The full tape, rendered as Markdown by the engine.
    private(set) var tapeMarkdown: String = ""

    /// Set when the last key produced an error (e.g. divide by zero). The
    /// display is still valid; views can surface this as a transient banner.
    private(set) var errorMessage: String?

    /// Index of the active chip in the format strip, for highlighting.
    var activeFormatIndex: Int = FormatOption.defaultIndex

    /// The engine. A reference type from UniFFI; `@ObservationIgnored` keeps
    /// the observation machinery from trying to track it.
    @ObservationIgnored private let calc = Calculator()

    @ObservationIgnored private let haptics = Haptics()

    init() {
        applyStoredPreferences()
        display = calc.displayString()
    }

    /// Funnel for every key press. Sends the event, then republishes the
    /// snapshot the engine returns.
    func send(_ event: KeyEvent) {
        haptics.tap()
        let snapshot = calc.handle(event: event)
        display = snapshot.display
        tapeMarkdown = snapshot.tapeMarkdown
        errorMessage = snapshot.error
    }

    /// The current display value as decimal feet when it's a length — used to
    /// seed the guided estimators from the calculator. Nil for any other
    /// dimension (or an empty/zero display).
    var displayLengthFeet: Double? { calc.displayLengthFeet() }

    /// Pick a display format from the strip. Records the active index so the
    /// chip highlights, then sends the matching convert event.
    func pickFormat(_ index: Int) {
        guard FormatOption.all.indices.contains(index) else { return }
        activeFormatIndex = index
        send(FormatOption.all[index].event)
    }

    // MARK: - Tape

    func exportMarkdown() -> String { calc.exportMarkdown() }

    func exportJson() -> String? { try? calc.exportJson() }

    func clearTape() {
        calc.clearTape()
        tapeMarkdown = calc.exportMarkdown()
    }

    func loadTape(json: String) -> Bool {
        do {
            try calc.loadJsonTape(json: json)
            tapeMarkdown = calc.exportMarkdown()
            return true
        } catch {
            errorMessage = "Could not load tape: \(error.localizedDescription)"
            return false
        }
    }

    // MARK: - Preferences

    /// Apply the stored display preference at launch so the first render is
    /// already in the user's chosen format — same behaviour as the web app's
    /// `+page.svelte` onMount.
    private func applyStoredPreferences() {
        let prefs = Preferences.load()
        _ = calc.handle(event: prefs.toConvertKey())
        _ = calc.handle(event: prefs.toAngleModeKey())
        activeFormatIndex = FormatOption.index(matching: prefs)
    }

    /// Set whether trig keys read a plain number as degrees or radians.
    /// Called live from the preferences screen. The display doesn't change,
    /// so there's nothing to republish.
    func setAngleMode(degrees: Bool) {
        _ = calc.handle(event: .setAngleMode(degrees: degrees))
    }

    /// Re-read preferences and apply the display format to the live engine.
    /// Called from the preferences screen so a format change takes effect
    /// immediately rather than waiting for the next launch. Deliberately
    /// silent (no haptic) since it's a settings change, not a key press.
    func reapplyDisplayPreference() {
        let prefs = Preferences.load()
        let snapshot = calc.handle(event: prefs.toConvertKey())
        display = snapshot.display
        tapeMarkdown = snapshot.tapeMarkdown
        errorMessage = snapshot.error
        activeFormatIndex = FormatOption.index(matching: prefs)
    }
}
