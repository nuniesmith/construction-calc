import SwiftUI

/// The big read-out at the top of the calculator. Mirrors the web
/// `Display.svelte`: right-aligned, monospaced, with a transient error
/// banner when the last key failed.
struct DisplayView: View {
    let display: String
    let error: String?

    var body: some View {
        VStack(alignment: .trailing, spacing: 4) {
            if let error {
                Text(error)
                    .font(.footnote)
                    .foregroundStyle(.red)
                    .lineLimit(2)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .transition(.opacity)
            }
            Text(display)
                .font(.system(size: 48, weight: .light, design: .monospaced))
                .lineLimit(1)
                .minimumScaleFactor(0.4)
                .frame(maxWidth: .infinity, alignment: .trailing)
                .contentTransition(.numericText())
        }
        .padding(.horizontal, 16)
        .padding(.vertical, 20)
        .frame(maxWidth: .infinity, minHeight: 120)
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color(red: 0.05, green: 0.09, blue: 0.16))
        )
        .animation(.easeInOut(duration: 0.15), value: display)
        .animation(.easeInOut(duration: 0.15), value: error)
    }
}

#Preview {
    DisplayView(display: "8' 1-5/8\"", error: nil)
        .padding()
        .background(Color.black)
}
