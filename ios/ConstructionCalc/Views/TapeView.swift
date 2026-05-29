import SwiftUI

/// The paper-tape sheet. Mirrors `Tape.svelte` + `TapeToolbar.svelte`:
/// shows the running tape and offers share (Markdown / JSON) and clear.
/// Saved-tape persistence (the web `/tapes` page) is a follow-up; see
/// `todo.md`.
struct TapeView: View {
    @Environment(CalculatorViewModel.self) private var vm
    @Environment(\.dismiss) private var dismiss
    @State private var confirmClear = false

    var body: some View {
        NavigationStack {
            Group {
                if vm.tapeMarkdown.isEmpty {
                    ContentUnavailableView(
                        "No tape yet",
                        systemImage: "list.bullet.rectangle",
                        description: Text("Press = to record a result.")
                    )
                } else {
                    ScrollView {
                        Text(vm.tapeMarkdown)
                            .font(.system(.callout, design: .monospaced))
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .textSelection(.enabled)
                            .padding()
                    }
                }
            }
            .navigationTitle("Tape")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Button("Done") { dismiss() }
                }
                ToolbarItemGroup(placement: .topBarTrailing) {
                    ShareLink(item: vm.exportMarkdown()) {
                        Image(systemName: "square.and.arrow.up")
                    }
                    .disabled(vm.tapeMarkdown.isEmpty)

                    Button(role: .destructive) {
                        confirmClear = true
                    } label: {
                        Image(systemName: "trash")
                    }
                    .disabled(vm.tapeMarkdown.isEmpty)
                }
            }
            .confirmationDialog("Clear the tape?", isPresented: $confirmClear, titleVisibility: .visible) {
                Button("Clear tape", role: .destructive) { vm.clearTape() }
                Button("Cancel", role: .cancel) {}
            }
        }
    }
}
