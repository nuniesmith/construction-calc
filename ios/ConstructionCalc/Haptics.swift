import UIKit

/// Light tactile feedback on key presses — costs nothing and makes the pad
/// feel like a real calculator. Pre-warming the generator keeps the first
/// tap from feeling laggy.
struct Haptics {
    private let generator = UIImpactFeedbackGenerator(style: .light)

    init() {
        generator.prepare()
    }

    func tap() {
        generator.impactOccurred()
        generator.prepare()
    }
}
