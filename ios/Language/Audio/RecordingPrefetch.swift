import Foundation

/// Downloads the recordings a unit tab can play into the recording cache while the calling task runs.
enum RecordingPrefetch {
    /// Returns once every download has finished, or once the calling task is cancelled and the downloads already started have.
    static func prefetchTab(_ tab: UnitTabKind, unitId: String, publication: CoursePublication, engine: AudioEngineRecord) async {
        let cancellation: PrefetchCancellation = PrefetchCancellation()

        await withTaskCancellationHandler {
            await publication.prefetchUnitTabRecordings(unitId: unitId, tab: tab.record(), engine: engine, cancellation: cancellation)
        } onCancel: {
            cancellation.cancel()
        }
    }
}
