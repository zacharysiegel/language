import SwiftUI

enum RootTabKind: Hashable {
    case course
    case review
    case glossary
    case progress
}

/// A unit of a course, and the tab to open it on.
struct UnitDestination: Hashable {
    let courseId: String
    let unitId: String
    let tab: UnitTabKind
}

struct RootView: View {
    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore
    @State private var selectedTab: RootTabKind = .course
    @State private var settingsPresented: Bool = false
    @State private var coursesPresented: Bool = false

    var body: some View {
        let courseId: String? = activeCourseId()

        TabView(selection: $selectedTab) {
            Tab("Course", systemImage: "book", value: RootTabKind.course) {
                navigationRoot(courseId: courseId) { outline in HomePage(outline: outline) }
            }

            Tab("Review", systemImage: "rectangle.stack", value: RootTabKind.review) {
                navigationRoot(courseId: courseId) { outline in ReviewPage(outline: outline) }
            }
            .badge(courseId.map { courseId in progressStore.dueCardCount(courseId: courseId) } ?? 0)

            Tab("Glossary", systemImage: "character.book.closed", value: RootTabKind.glossary) {
                navigationRoot(courseId: courseId) { outline in GlossaryPage(outline: outline) }
            }

            Tab("Progress", systemImage: "chart.bar", value: RootTabKind.progress) {
                navigationRoot(courseId: courseId) { outline in ProgressPage(outline: outline) }
            }
        }
        .tint(Color.accentActive)
        .sheet(isPresented: $settingsPresented) {
            SettingsSheet()
        }
        .sheet(isPresented: $coursesPresented) {
            NavigationStack {
                CoursesPage(onChoose: { coursesPresented = false })
            }
        }
    }

    /// The course the learner opened last, else the only published course.
    private func activeCourseId() -> String? {
        let courses: [CourseSummaryRecord] = courseStore.courses()

        if let lastCourseId = progressStore.lastCourseId(), courses.contains(where: { course in course.id == lastCourseId }) {
            return lastCourseId
        }

        return courses.count == 1 ? courses[0].id : nil
    }

    private func courseTitle(courseId: String) -> String? {
        courseStore.courses().first { course in course.id == courseId }?.title
    }

    private func navigationRoot<Content: View>(courseId: String?, @ViewBuilder content: @escaping (CourseOutlineRecord) -> Content) -> some View {
        NavigationStack {
            VersionGate {
                if let courseId {
                    CourseGate(courseId: courseId, content: content)
                } else {
                    CoursesPage(onChoose: {})
                }
            }
            .navigationDestination(for: UnitDestination.self) { destination in
                UnitPage(courseId: destination.courseId, unitId: destination.unitId, initialTab: destination.tab)
            }
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    if let courseTitle = courseId.flatMap(courseTitle(courseId:)) {
                        Button {
                            coursesPresented = true
                        } label: {
                            Text(courseTitle).lineLimit(1)
                        }
                        .accessibilityIdentifier("courses")
                    }
                }

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

/// Shows its content once a version of the courses has loaded, and a status message before that.
struct VersionGate<Content: View>: View {
    @ViewBuilder let content: () -> Content

    @Environment(CourseStore.self) private var courseStore

    var body: some View {
        switch courseStore.loadState() {
        case .loading:
            StatusMessage(text: "Loading the courses…")
        case .failed(let message):
            LoadFailure(message: message) {
                Task { await courseStore.start() }
            }
        case .loaded:
            content()
        }
    }
}

/// Shows its content with the course's outline, loading it if needed.
struct CourseGate<Content: View>: View {
    let courseId: String
    @ViewBuilder let content: (CourseOutlineRecord) -> Content

    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        Group {
            switch courseStore.outlineState(courseId: courseId) {
            case .loaded(let outline):
                content(outline)
            case .failed(let message):
                LoadFailure(message: message) {
                    Task { await courseStore.refresh() }
                }
            case .loading, .none:
                StatusMessage(text: "Loading the course…")
            }
        }
        .onAppear {
            courseStore.ensureOutlineLoaded(courseId: courseId)
            progressStore.rememberCourse(courseId: courseId)
        }
        .onChange(of: courseStore.shownVersion()?.version()) { courseStore.ensureOutlineLoaded(courseId: courseId) }
    }
}

struct StatusMessage: View {
    let text: String

    var body: some View {
        Text(text)
            .font(.bodyText)
            .foregroundStyle(Color.inkMuted)
            .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

struct LoadFailure: View {
    let message: String
    let onRetry: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            Text("The course could not be loaded.")
                .font(.titleText)
                .foregroundStyle(Color.accentActive)
            Text(message)
                .font(.captionText)
                .foregroundStyle(Color.inkMuted)
            Button("Try again", action: onRetry)
                .buttonStyle(SquareButtonStyle(isPrimary: true))
        }
        .padding(.spaceMd)
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
    }
}
