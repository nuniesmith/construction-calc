import SwiftUI

/// Horizontal strip of display-format chips above the keypad. Mirrors
/// `FormatStrip.svelte`. The fraction chips double as rounding controls.
struct FormatStripView: View {
    @Environment(CalculatorViewModel.self) private var vm

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 6) {
                ForEach(Array(FormatOption.all.enumerated()), id: \.element.id) { index, option in
                    let isActive = vm.activeFormatIndex == index
                    Button {
                        vm.pickFormat(index)
                    } label: {
                        Text(option.label)
                            .font(.caption)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(
                                RoundedRectangle(cornerRadius: 7)
                                    .fill(isActive ? Color(red: 0.85, green: 0.47, blue: 0.02)
                                                   : Color.white.opacity(0.04))
                            )
                            .foregroundStyle(isActive ? .white : Color(white: 0.8))
                    }
                    .buttonStyle(.plain)
                }
            }
            .padding(.horizontal, 4)
            .padding(.vertical, 4)
        }
        .background(
            RoundedRectangle(cornerRadius: 8).fill(Color.white.opacity(0.03))
        )
    }
}
