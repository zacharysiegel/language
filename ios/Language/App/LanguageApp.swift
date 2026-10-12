import SwiftUI

@main
struct LanguageApp: App {
    @State private var courseStore: CourseStore
    @State private var progressStore: ProgressStore
    @State private var audioPlayer: AudioPlayer
    @Environment(\.scenePhase) private var scenePhase

    init() {
        let environment: CoreEnvironment = CoreEnvironment.standard()
        let courseStore: CourseStore = CourseStore(environment: environment)

        _courseStore = State(initialValue: courseStore)
        _progressStore = State(initialValue: ProgressStore(environment: environment))
        _audioPlayer = State(initialValue: AudioPlayer(library: courseStore.courseLibrary()))
    }

    var body: some Scene {
        WindowGroup {
            RootView()
                .environment(courseStore)
                .environment(progressStore)
                .environment(audioPlayer)
                .preferredColorScheme(.light)
                .task { await courseStore.start() }
                .onChange(of: scenePhase) { _, phase in
                    if phase == .active {
                        progressStore.advanceReviewClock()
                        Task { await courseStore.refresh(courseId: courseStore.activeCourseId(lastCourseId: progressStore.lastCourseId())) }
                    }
                }
        }
    }
}
