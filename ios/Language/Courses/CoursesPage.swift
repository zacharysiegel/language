import SwiftUI

/// Every published course; choosing one makes it the course the other pages show.
struct CoursesPage: View {
    let onChoose: () -> Void

    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 0) {
                PageHeader(caption: "\(courseStore.courses().count) courses", title: "Courses")
                    .padding(.bottom, .spaceMd)

                HairlineRule()

                ForEach(courseStore.courses(), id: \.id) { course in
                    Button {
                        progressStore.rememberCourse(courseId: course.id)
                        onChoose()
                    } label: {
                        VStack(alignment: .leading, spacing: .spaceXs) {
                            Text(course.title).font(.bodyText).foregroundStyle(Color.accentLink)
                            Text("\(course.targetLanguageName) for \(course.sourceLanguageName) speakers")
                                .font(.captionText)
                                .foregroundStyle(Color.inkMuted)
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .padding(.vertical, .spaceSm + .spaceXs)
                        .contentShape(Rectangle())
                    }
                    .buttonStyle(.plain)
                    .accessibilityIdentifier("course-\(course.id)")

                    HairlineRule()
                }
            }
            .padding(.spaceMd)
            .readableWidth()
        }
        .navigationTitle("Courses")
        .navigationBarTitleDisplayMode(.inline)
    }
}
