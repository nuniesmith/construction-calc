import UIKit

/// Light tactile feedback on key presses — costs nothing and makes the pad
/// feel like a real calculator. Pre-warming the generators keeps the first
/// tap from feeling laggy.
struct Haptics {
    /// Shared instance so views (e.g. a key recognising a long-press) can fire
    /// feedback without each owning a generator.
    static let shared = Haptics()

    private let light = UIImpactFeedbackGenerator(style: .light)
    private let medium = UIImpactFeedbackGenerator(style: .medium)

    init() {
        light.prepare()
        medium.prepare()
    }

    /// A light tick for an ordinary key press.
    func tap() {
        light.impactOccurred()
        light.prepare()
    }

    /// A firmer bump when a long-press is recognised (e.g. opening help), so it
    /// feels distinct from a tap and confirms the gesture took.
    func longPress() {
        medium.impactOccurred()
        medium.prepare()
    }
}
