import SwiftUI

enum RootTabKind: Hashable {
    case course
    case review
    case glossary
    case progress
}

/// A unit and the tab to open it on.
struct UnitDestination: Hashable {
    let unitId: String
    let tab: UnitTabKind
}

struct RootView: View {
    @Environment(ProgressStore.self) private var progressStore
    @State private var selectedTab: RootTabKind = .course
    @State private var settingsPresented: Bool = false

    var body: some View {
        TabView(selection: $selectedTab) {
            Tab("Course", systemImage: "book", value: RootTabKind.course) {
                navigationRoot { HomePage() }
            }

            Tab("Review", systemImage: "rectangle.stack", value: RootTabKind.review) {
                navigationRoot { ReviewPage() }
            }
            .badge(progressStore.dueCardCount())

            Tab("Glossary", systemImage: "character.book.closed", value: RootTabKind.glossary) {
                navigationRoot { GlossaryPage() }
            }

            Tab("Progress", systemImage: "chart.bar", value: RootTabKind.progress) {
                navigationRoot { ProgressPage() }
            }
        }
        .tint(Color.accentActive)
        .sheet(isPresented: $settingsPresented) {
            SettingsSheet()
        }
    }

    private func navigationRoot<Content: View>(@ViewBuilder content: @escaping () -> Content) -> some View {
        NavigationStack {
            CourseGate {
                content()
            }
            .navigationDestination(for: UnitDestination.self) { destination in
                UnitPage(unitId: destination.unitId, initialTab: destination.tab)
            }
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button {
                        settingsPresented = true
                    } label: {
                        Image(systemName: "gearshape")
                    }
                    .accessibilityLabel("Settings")
                    .accessibilityIdentifier("settings")
                }
            }
            .background(Color.paper)
        }
    }
}

/// Shows its content once the course has loaded, and a status message before that.
struct CourseGate<Content: View>: View {
    @ViewBuilder let content: () -> Content

    @Environment(CourseStore.self) private var courseStore

    var body: some View {
        switch courseStore.loadState() {
        case .loading:
            statusView(Text("Loading the course…"))
        case .failed(let message):
            VStack(alignment: .leading, spacing: .spaceMd) {
                Text("The course could not be loaded.")
                    .font(.titleText)
                    .foregroundStyle(Color.accentActive)
                Text(message)
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)
                Button("Try again") {
                    Task { await courseStore.start() }
                }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
            }
            .padding(.spaceMd)
            .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        case .loaded:
            content()
        }
    }

    private func statusView(_ text: Text) -> some View {
        text
            .font(.bodyText)
            .foregroundStyle(Color.inkMuted)
            .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}
