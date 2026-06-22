import SwiftUI

/// Horizontal strip of display-format chips above the keypad. Mirrors
/// `FormatStrip.svelte`. The fraction chips double as rounding controls.
struct FormatStripView: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Environment(\.calcTheme) private var theme

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 6) {
                ForEach(Array(FormatOption.all.enumerated()), id: \.element.id) { index, option in
                    Button {
                        vm.pickFormat(index)
                    } label: {
                        Text(option.label)
                            .font(.caption)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(
                                RoundedRectangle(cornerRadius: 7)
                                    .fill(vm.activeFormatIndex == index
                                          ? theme.accent
                                          : theme.chipInactive)
                            )
                            .foregroundStyle(vm.activeFormatIndex == index ? theme.keyText : theme.chipText)
                    }
                    .buttonStyle(.plain)
                }
            }
            .padding(.horizontal, 4)
            .padding(.vertical, 4)
        }
        .background(
            RoundedRectangle(cornerRadius: 8).fill(theme.panel)
        )
    }
}
