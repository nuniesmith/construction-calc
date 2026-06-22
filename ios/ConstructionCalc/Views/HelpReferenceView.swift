import SwiftUI

/// Browsable reference of what every function/key does — the discoverable
/// companion to the long-press help (which users rarely find on their own).
/// Grouped like the keypad; entries come straight from `HelpText`.
struct HelpReferenceView: View {
    @Environment(\.dismiss) private var dismiss

    private struct HelpGroup {
        let name: String
        let ids: [String]
    }

    private let groups: [HelpGroup] = [
        HelpGroup(name: "Rafter", ids: ["pitch", "rise", "run", "diag", "hipv", "jack"]),
        HelpGroup(name: "Trig", ids: ["sin", "cos", "tan", "asin", "acos", "atan"]),
        HelpGroup(name: "Math", ids: ["sqrt", "square", "recip", "percent"]),
        HelpGroup(name: "Compound miter", ids: ["corner", "spring", "miter", "bevel"]),
        HelpGroup(name: "Units & entry", ids: ["yd", "ft", "in", "m", "slash", "negate", "bs", "c", "ac"]),
        HelpGroup(name: "Memory", ids: ["memstore", "memrecall", "memplus", "memclear", "memclearall"]),
        HelpGroup(name: "Estimators", ids: ["concrete"]),
    ]

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Text("Tip: long-press any key on the calculator to pop up its help directly.")
                        .font(.footnote)
                        .foregroundStyle(.secondary)
                }
                ForEach(groups, id: \.name) { group in
                    Section(group.name) {
                        ForEach(group.ids, id: \.self) { id in
                            if let e = HelpText.entry(for: id) {
                                VStack(alignment: .leading, spacing: 3) {
                                    Text(e.title).fontWeight(.semibold)
                                    Text(e.body)
                                        .font(.subheadline)
                                        .foregroundStyle(.secondary)
                                    if let f = e.formula {
                                        Text(f).font(.caption).monospaced().foregroundStyle(.tertiary)
                                    }
                                    if let ex = e.example {
                                        Text(ex).font(.caption).foregroundStyle(.tertiary)
                                    }
                                }
                                .padding(.vertical, 2)
                            }
                        }
                    }
                }
            }
            .navigationTitle("Function Guide")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }
}
