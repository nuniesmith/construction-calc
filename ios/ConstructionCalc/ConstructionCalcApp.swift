import SwiftUI

/// App entry point. Owns the single `CalculatorViewModel` for the whole
/// session and hands it down through the environment so the calculator,
/// tape, and preferences screens all drive the same engine instance.
@main
struct ConstructionCalcApp: App {
    @State private var vm = CalculatorViewModel()

    var body: some Scene {
        WindowGroup {
            CalculatorView()
                .environment(vm)
                .preferredColorScheme(.dark)
        }
    }
}
