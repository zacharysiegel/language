import SwiftUI

@main
struct LanguageApp: App {
    @State private var courseStore: CourseStore
    @State private var progressStore: ProgressStore
    @State private var audioPlayer: AudioPlayer = AudioPlayer()
    @Environment(\.scenePhase) private var scenePhase

    init() {
        let environment: CoreEnvironment = CoreEnvironment.standard()

        _courseStore = State(initialValue: CourseStore(environment: environment))
        _progressStore = State(initialValue: ProgressStore(environment: environment))
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
                        Task { await courseStore.refresh() }
                    }
                }
        }
    }
}
