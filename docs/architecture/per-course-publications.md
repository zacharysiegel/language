# Per-course publications

A plan. Each course is published, retained and removed on its own. Publishing one course never changes another course's published files, its pointer, or what a client shows for it. A small course index lists the courses; it carries no publication label and changes only when a course is added, renamed or removed. Recordings stay content-addressed and shared by every course. Phase 7 folds the lasting decisions into `overview.md` and deletes this file.

## Decisions

- One publication per course: its own manifest, publication label (`<UTC date>+<city>`), latest pointer, and retained publications.
- Publication labels are unique within a course, not across courses. Every key carries the course id.
- Clients decide whether a course's content changed by comparing manifest bytes, never labels alone.
- The course index has no labels. A client learns that a course has newer course content only from that course's latest pointer.
- A course's latest pointer is fetched only when that course is opened, or refreshed while it is open, and only through an index read from a repository.
- Adding a course to a repository is explicit: `publish <target> --course <id>`, or `--include-unlisted-courses`. A publish with neither updates only courses the target's index already lists.
- Removing a course is `remove-course`. Leaving a course out of a build or a publish changes nothing.
- Every command that writes a repository holds that repository's lock for its whole run.
- Publishing is resumable: rerunning it for the same bundle completes whatever a failed run left undone.
- Test courses (`test_course: true`) reach the local repository only. Cloudflare R2 and the embedded bundle refuse them, and the R2 index never lists one.
- No index schema pointer. A later index schema adds `course-index.<n>.json` beside `course-index.json`, which keeps serving schema 1.
- The app is not released. No deployed client reads the schema-2 layout, so nothing keeps `latest/manifest.json` or `latest/manifest.2.json` alive.

## Repository layout and keys

```
course-index.json                                              the course index (pointer headers)
courses/<course id>/latest/manifest.json                       the course's latest pointer (pointer headers)
courses/<course id>/latest/manifest.<manifest schema>.json     the course's pointer for one manifest schema (pointer headers)
courses/<course id>/<label>/manifest.json                      a publication's manifest (immutable)
courses/<course id>/<label>/recordings.json                    the recordings the publication's shards name; producer-only (immutable)
courses/<course id>/<label>/data/course-<sha256>.sqlite.br     (immutable)
courses/<course id>/<label>/data/unit-<unit id>-<sha256>.sqlite.br   (immutable)
audio/<sha256>.m4a                                             shared by every course (immutable)
.repository-lock                                               R2 only: the lease a writing command holds
```

- Key functions, in `shared/src/artifact/repository_key.rs` (new):
  - `COURSE_INDEX_KEY = "course-index.json"`.
  - `COURSES_DIRECTORY = "courses"`, `course_directory_key(course_id)` (ends in `/`).
  - `course_latest_manifest_key(course_id)`, `course_schema_pointer_key(latest_manifest_key, schema)` (the sibling `manifest.<schema>.json` of the pointer the index names).
  - `publication_directory_key(course_id, label)` (ends in `/`), `publication_manifest_key`, `publication_recordings_key`, `publication_file_key(course_id, label, relative_path)`.
  - `REPOSITORY_LOCK_KEY = ".repository-lock"`.
  - `audio_key(sha256)` moves here from `manifest.rs` in phase 7.
- Every listing prefix ends in `/`, so `2026-10-08+charlotte/` never matches `2026-10-08+charlottesville/`.
- Headers:
  - Pointers (`course-index.json`, anything under `courses/<course id>/latest/`): `public, max-age=300`, fetched with `HttpCacheMode::Reload`.
  - Everything else: `public, max-age=31536000, immutable`.
  - One rule in shared: `repository_key::is_pointer_key(key)` and `cache_control_for_key(key)`, with the two `CACHE_CONTROL_*` constants moved into shared. `course_builder publish` picks headers through it, and `web/src/server.rs` `set_repository_cache_control` calls it in place of the `latest/` prefix test.
- Discovery is unchanged (`DISCOVERY_SCHEMA_VERSION` stays 1). The index is `<repository_base_url>/course-index.json`; the discovery document does not name it.

### Course index

```json
{
  "course_index_schema_version": 1,
  "courses": {
    "pt_br.en.foundations": {
      "title": "Português do Brasil",
      "target_language": "pt-BR",
      "source_language": "en",
      "latest_manifest_key": "courses/pt_br.en.foundations/latest/manifest.json",
      "test_course": false
    }
  }
}
```

- Types in `shared/src/artifact/course_index.rs`: `CourseIndex { course_index_schema_version, courses: BTreeMap<String, CourseIndexEntry> }`, `CourseIndexEntry { title, target_language, source_language, latest_manifest_key, test_course }`.
- `COURSE_INDEX_SCHEMA_VERSION = 1`, field `course_index_schema_version`, read through `schema_version::require_schema_version`.
- `parse_course_index` rejects:
  - a course id that fails `course_identity::is_well_formed_course_id` (extracted from `course_id_issue`: three dot-separated parts of lowercase letters, digits and underscores);
  - a `latest_manifest_key` that contains `..`, does not start with `course_directory_key(id)`, or does not end in `/manifest.json`.
- `latest_manifest_key` is relative to the repository base and joined with `fetch::join_url`, never `resolve_reference`.
- `test_course` lets the publisher check the index it writes. Clients ignore it.
- Index entries are written in course-id order (`BTreeMap`), so an unchanged index serializes to the same bytes.

### Course manifest

```json
{
  "manifest_schema_version": 3,
  "course_id": "pt_br.en.foundations",
  "publication_label": "2026-10-08+duluth",
  "artifact_created": "2026-10-08T14:02:11Z",
  "title": "Português do Brasil",
  "target_language": "pt-BR",
  "source_language": "en",
  "test_course": false,
  "outline": { "relative_path": "data/course-<sha256>.sqlite.br", "size_bytes": 0, "sha256": "<sha256>" },
  "units": { "a1-02-sounds-alphabet": { "relative_path": "data/unit-a1-02-sounds-alphabet-<sha256>.sqlite.br", "size_bytes": 0, "sha256": "<sha256>" } },
  "voice_pipeline_revisions": { "kokoro": 3 }
}
```

- `CourseManifest` and `COURSE_MANIFEST_SCHEMA_VERSION = 3` in `shared/src/artifact/course_manifest.rs` (new). They keep these names after phase 7. `manifest.rs` keeps the shared pieces (`MANIFEST_FILENAME`, `MANIFEST_SCHEMA_VERSION_FIELD`, `SUBDIR_*`, file extensions, `ManifestEntry`, `validate_entry`).
- `relative_path` is relative to the publication directory and drops the course id. The file name still ends in the SHA-256, which the flat client caches key on.
- The manifest carries title and languages because the publisher derives the index entry from it. Clients take titles from the index.
- `voice_pipeline_revisions`: `ClipRecordings.pipeline_revision_by_engine_code` filtered to the engines in the course's `clip_audio` entries.
- `parse_course_manifest(bytes, expected_course_id: Option<&str>)` rejects a manifest for a different course than the pointer it came from, a malformed course id, and a `publication_label` containing `/` or `..`.
- `CourseManifest::schema_fallback_key(latest_manifest_key, bytes)`.

### Recordings list

- `recordings.json`: `{ "recording_sha256s": ["<sha256>", ...] }`, sorted, written by the builder from the shards' `clip_audio` tables.
- Publish derives the set again from the verified shards (`referenced_audio_sha256s`) and refuses a bundle whose list differs.
- Retention and recording removal read only the lists, so neither opens shards or downloads them from R2.
- A publication whose `recordings.json` cannot be read keeps every recording.

## Builder output

### Bundles

```
build/artifacts/<course id>/<label>/
    manifest.json
    recordings.json
    data/course-<sha256>.sqlite.br
    data/unit-<unit id>-<sha256>.sqlite.br
    audio/<sha256>.m4a            hard links into build/clips
build/artifacts/<course id>/.partial-<label>/
```

- `build` plans and synthesizes audio once for every selected course (unchanged), then writes one bundle per course.
- `write_course_bundle` returns `CourseBundleOutcomeKind { Written(BuiltBundle), Unchanged(BuiltBundle) }`; `Unchanged` carries the course's existing newest bundle.
- Unchanged: the new manifest equals the newest bundle's apart from `publication_label` and `artifact_created`. The partial directory is deleted and the build logs `course content unchanged; [course_id=… publication_label=…]`.
- Each course gets its own label. The taken set for course X: the names under `build/artifacts/<X>/`, the publications under `build/repository/courses/<X>/`, and X's label in the embedded bundle. Publish adds the target's labels (see relabeling).
- Retention: `BUNDLES_KEPT_PER_COURSE = 2`, applied to the courses this build wrote, whose `.partial-*` directories are also removed. `build/artifacts/<id>/` of a course id absent from `data/courses/` is removed.
- Listing a course's bundles looks only at `build/artifacts/<course id>/`, skipping dot-directories, ordered by `artifact_created`. Directories directly under `build/artifacts/` that are not course ids (the old `<label>/` bundles) are ignored.
- `remove_clips_no_course_uses` is unchanged.
- `version_label` becomes `publication_label` (module, error text). `generate` is unchanged.
- Workspace helpers in `workspace.rs`: `course_bundles_directory(course_id)`, `local_repository_course_directory(course_id)`, `embedded_course_directory(course_id)`.
- `build` no longer writes the embedded bundle; publishing does.

### Embedded bundle

```
web/static/embedded/
    course-index.json
    courses/<course id>/latest/manifest.json
    courses/<course id>/<label>/data/*.sqlite.br
```

- Same keys as a repository, one publication per course, no recordings, no schema pointers. The loader reads it with the same functions as a repository, with base `/embedded` (web) or the bundle directory (iOS).
- `publish local` and `publish cloudflare-r2` replace `courses/<X>/` and X's index entry with the publication they just published or resumed, so every embedded label is one a repository holds. Other courses are untouched.
- `refresh_embedded_course` refuses a manifest with `test_course: true`; publishing a test course skips the refresh.
- Every build and publish drops embedded courses whose id is absent from `data/courses/` or is a test course there.
- `remove-course <target>` drops the course from the embedded bundle.
- The embedded publication keeps its label. `embedded_manifest_for`, `EmbeddedBundleRefreshKind` and `decide_embedded_bundle_refresh` are deleted.
- `scripts/ios/sync-ios-embedded-bundle.sh` (the iOS build's pre-build phase) checks for `course-index.json` and copies the directory as before.

### Scripts

- `scripts/build/build-course.sh`: comment describes per-course bundles.
- `scripts/publish/publish-course.sh <target> [--course <id>]... [--include-unlisted-courses]`.
- `scripts/publish/remove-course.sh <target> --course <id>` (new): wraps `remove-course`.
- `scripts/dev/build-local-repository.sh`: build with `--include-test-courses`, then `publish local --include-unlisted-courses`, so it publishes every course `data/courses/` holds. A course removed from the local repository stays out once it is deleted from `data/courses/`.
- `scripts/dev/serve.sh`: probes `build/repository/course-index.json`.

## Publishing

- CLI (`tools/course_builder/src/main.rs`):
  - `publish <local|cloudflare-r2|dry> [--course <id>]... [--bundle <dir>] [--include-unlisted-courses]`. `--bundle` names one course's bundle; with `--course` as well, the ids must match.
  - `remove-course <local|cloudflare-r2> --course <id>`.
  - `remove-unused-recordings <local|cloudflare-r2>`.
  - `unlock cloudflare-r2`.
  - Help text says "publication" and "publication label", never a bare "version".
- The publish, retention, removal and index functions are generic over `impl ArtifactRepository` (static dispatch). `publish_command.rs` and `remove_course_command.rs` build the concrete repository and run the local-only recording removal.

### Repository lock

- Held by `publish`, `remove-course` and `remove-unused-recordings` from before their first read to after their last write. `publish dry` takes none.
- Local: exclusive `std::fs::File::lock` on `<root>/.repository.lock`; the operating system releases it when the process exits.
- R2: `PutObject` of `.repository-lock` with `If-None-Match: *`. The body names the host, process id and acquisition time. Deleted when the command ends. A command that finds a lease fails and names it; `unlock cloudflare-r2` deletes a lease left by a crash.
- Index and pointer writes stay conditional on the entity tag read under the lock (`If-Match`, or `If-None-Match: *` when absent). A failed precondition means a writer ignored the lock, and the command fails. A 412 is `WriteOutcomeKind::PreconditionFailed`.
- Local entity tag: the SHA-256 of the file; the compare and `filesystem::write_atomically` happen under the lock.

### One course (`artifact::publish::publish_course_bundle`)

1. Read the index with its entity tag (absent counts as empty). Cloudflare R2: refuse a test-course bundle, and refuse when the index lists a course whose stored flag or curriculum in `data/courses/` marks it a test course. The error names `remove-course cloudflare-r2 --course <id>`. Nothing is uploaded first.
2. Derive the recordings from the verified shards; refuse when they differ from `recordings.json`.
3. Read `courses/<id>/<label>/manifest.json`:
   - Bytes equal the bundle's: the publication is complete; go to step 4.
   - Different bytes: another publication holds the label. Relabel the bundle (a fresh label, with this target's `courses/<id>/` listing added to the taken set; the directory renamed and `manifest.json` rewritten) and repeat step 3.
   - Absent: upload the recordings `audio/` lacks, the shards the publication directory lacks, `recordings.json`, then `manifest.json`.
4. Read `courses/<id>/latest/manifest.json` with its entity tag. Bytes equal the bundle's: skip. Otherwise write `manifest.<schema>.json`, then `manifest.json` conditionally.
   - `--course` or `--bundle`: the pointer always moves to the bundle.
   - Every-course publish: the pointer moves only to a publication this run uploaded, or when there is no pointer. A pointer on another label is logged and left.
5. Derive X's index entry from the manifest. Equal to the current entry: no write. Added or changed: write the index with X's entry replaced, conditionally.
6. Apply retention for this course.
7. Refresh the embedded bundle (local and R2 targets; not test courses).

Each step skips work already done, so a rerun after a failure at any step completes the publish, including a first publish whose index write failed.

### Every course

- Courses considered: those the target's index lists and `data/courses/` holds. `--course` adds named courses; `--include-unlisted-courses` adds every built course in `data/courses/`. On Cloudflare R2, test-course bundles are skipped with a log line.
- An indexed course absent from `data/courses/` is logged with the `remove-course` command that removes it.
- Each course's newest bundle goes through the steps above; a complete one only verifies.
- Courses are published one after another. A failure is reported, the rest continue, and the command exits non-zero.
- `PublishReport` is per course: course id, publication label, latest pointer URL, `CourseIndexChangeKind { Unchanged, Added, Updated }`, `PublicationUploadKind { Uploaded, AlreadyPresent }`, shard and recording counts.

### Retention

- `PUBLICATIONS_KEPT_PER_COURSE = 2`, for local and R2. A course's publications are ordered by the upload time of their `manifest.json` (local: modification time; R2: `LastModified`).
- Kept:
  - the newest `PUBLICATIONS_KEPT_PER_COURSE`;
  - every publication named by any `courses/<id>/latest/manifest*.json`;
  - R2 only: every publication whose next newer publication was uploaded less than `PUBLICATION_GRACE_PERIOD` (7 days) ago.
- The rest are deleted (`delete_keys` over their listing).
- Local only: then remove the recordings `recordings_named_by_publications` does not return.
- `recordings_named_by_publications(repository)`: the union of `recordings.json` across every publication under `courses/`, listed or not in the index; an unreadable list keeps every recording. Local retention, local `remove-course` and `remove-unused-recordings` all use it.
- R2 recordings are removed only by `remove-unused-recordings cloudflare-r2`, which holds the lock.

### Removing a course (`remove-course <target> --course <id>`)

1. Rewrite the index without the course (conditional write).
2. Delete `courses/<id>/latest/`, then every object under `courses/<id>/`.
3. Drop the course from the embedded bundle.
4. Local: remove unused recordings. R2: recordings are left for `remove-unused-recordings`.

- A course the index does not list still has `courses/<id>/` deleted if present, and the command says so.
- `build/artifacts/<id>/` is left. An every-course publish without `--include-unlisted-courses` does not consider an unlisted course, so removal holds.
- Renaming a course is changing `title` in `curriculum.json`, building and publishing. A changed course id is a removal plus an addition.

### Repository trait (`artifact_repository.rs`)

- `get_object(key) -> Option<StoredObject { bytes, entity_tag }>`.
- `put_bytes(key, bytes, headers, condition: WriteConditionKind { Unconditional, Absent, MatchesEntityTag(String) }) -> WriteOutcomeKind { Written, PreconditionFailed }`.
- `list_objects(prefix) -> Vec<ListedObject { key, uploaded: DateTime<Utc> }>` replaces `existing_keys`. Local lists recursively, keys relative to the root.
- `delete_keys(keys)`. Local also removes directories left empty, up to the root.
- Publications are listed by their `manifest.json` keys, not by directory names.
- Dry: `get_object` returns `None` and `list_objects` is empty, so `publish dry` logs every upload, pointer write and index write as if the repository were empty. It applies no retention.

## Shared loader

- Sources (`load.rs`):
  - `CourseIndexSource { origin: ArtifactOriginKind { Repository, Embedded }, repository_base_url, course_index, course_index_bytes, sunset }`.
  - `CourseManifestSource { origin, repository_base_url, manifest: CourseManifest, manifest_bytes }`.
- Index:
  - `discover_course_index(http_fetch, discovery_url, static_base_url)`: discovery and `<static>/course-index.json` in parallel; reuse the second when discovery agrees, else fetch from the discovered base.
  - `load_embedded_course_index(http_fetch, embedded_base_url)`, `parse_cached_course_index(bytes, base)`.
- Course manifest:
  - `fetch_latest_course_manifest(http_fetch, course_index: &CourseIndexSource, course_id)`: errors for an embedded index or a course the index lacks; otherwise the pointer the entry names (Reload) under the index's base, schema fallback to its sibling pointer, `parse_course_manifest` with the expected course id.
  - `load_embedded_course_manifest(http_fetch, embedded_base_url, course_id, entry)`, `parse_cached_course_manifest(bytes, base, course_id)`.
  - Called only for a course being opened or refreshed; the course list needs only the index.
- Shards: `load_publication_shard` / `load_publication_shard_bytes` take a `CourseManifestSource`; the URL is `publication_file_key(course_id, publication_label, relative_path)`. Cache lookup and verification are unchanged. The old `load_shard` / `load_shard_bytes` stay until phase 7.
- Cache retention: `ArtifactCache::retain` keeps its keep-exactly semantics. Callers pass `relative_paths_to_keep(manifests)`, the union of the entries of every remembered course manifest. A retain that races a load only causes a refetch.
- Failure is per course: each loader returns its own `Result`; there is no all-courses call.
- Index order: the cached index (repository origin), else the embedded index, shown at once; the repository index replaces it when it loads. A failed fetch changes nothing.
- Course order:
  - The cached manifest, else the embedded publication when the embedded index lists the course, shown at once.
  - Once the repository index load has finished, the pointer is read through the fresh repository index, else the cached one. With neither, there is no pointer step.
  - The pointer's publication is always adopted and remembered. When its bytes equal the shown manifest's, outline and unit state are kept and only the source and the recordings base change.
- A shard or recording fetch that fails from a repository refetches that course's pointer once and adopts the result before reporting the failure.
- A parsed repository index is authoritative for the course list. Courses it omits are forgotten on the client (stored manifest deleted; their shards go at the next retain). Progress is kept.
- `send_bound` covers every new exported load.

## Clients

### Web

- `course_loading.rs`:
  - `LoadedCourseIndex { source, audio_base_url }`, `LoadedCoursePublication { source, audio_base_url }` replace `LoadedVersion`.
  - localStorage: `language.artifact.course-index`, `language.artifact.course-index-base-url`, `language.artifact.course-manifest.<course id>`, `language.artifact.course-manifest-base-url.<course id>`.
  - `remember_course_index` stores the index and forgets courses it omits; `remember_course_publication` stores one manifest, then retains OPFS to the union of every stored manifest.
  - Server stubs mirror every browser function.
- `course_data.rs` `CourseStore`:
  - `index: RwSignal<CourseIndexLoadState>` (`Loading`, `Loaded(Arc<LoadedCourseIndex>)`, `Failed`), and `repository_index: RwSignal<RepositoryIndexLoadKind { Pending, Loaded, Failed }>`.
  - `publications: RwSignal<BTreeMap<course id, CoursePublicationLoadState>>`, compared by `Arc::ptr_eq` like `ShardLoadState`.
  - `outlines` and `units` unchanged.
  - `show_publication(course_id, publication)` replaces that course's entry; it clears that course's outline and `UnitKey` entries only when the manifest bytes differ.
  - `is_still_shown(course_id, manifest)` drops late results per course.
  - `ensure_publication_loaded(course_id)`: once per page load per course; cached or embedded at once, then the pointer after `repository_index` leaves `Pending`. `ensure_outline_loaded` / `ensure_unit_loaded` wait for it.
- Gates:
  - `course_index_gate` replaces `version_gate` (CoursesPage).
  - `course_gate`: "That course does not exist." only once `repository_index` is `Loaded` or `Failed` and the shown index lacks the course; before that, the loading status. Then the course's publication, then its outline.
  - `failure_view` wording names the course content, not the build.
- `CoursesPage` and `published_course_title` (app.rs) read the index. `NavigationBar` hides the course links when the active course is not in the index.
- `platform.rs`, browser and server stubs: no `reset_clip_audio`. `register_clip_audio(audio_base_url, entries)` merges, storing the base with each clip key; entries are content-addressed, so an entry from an earlier publication still plays.
- First paint: the server render stays `Loading`; the orders under "Shared loader" apply once the client starts.
- The web app shows no publication label.

### iOS and FFI

- FFI (`ffi/src/course_library.rs`):
  - `PublishedCourseIndex` object: `courses() -> Vec<CourseSummaryRecord>`, `is_embedded()`, `repository_base_url()`.
  - `CoursePublication` object (replaces `CourseVersion`): `course_id()`, `publication_label()`, `manifest_sha256()`, `repository_base_url()`, `is_embedded()`, `load_course()`, `load_unit(unit_id)`.
  - `CourseLibrary`: `load_cached_course_index()`, `load_latest_course_index()`, `load_embedded_course_index()`, `load_cached_course_publication(course_id)`, `load_latest_course_publication(course_index: &PublishedCourseIndex, course_id)` (errors for an embedded index), `load_embedded_course_publication(course_id)`, `recording_path(clip_key, engine, spoken_text, voice_choice)`.
  - The clip registry moves from the publication to `CourseLibrary`: a `DashMap` from clip key to, per engine, the recording's SHA-256 and the audio base of the publication whose shard registered it.
  - Test seam: internal `open_latest_course_index(&self, http_fetch: &impl HttpFetch)`, `open_latest_course_publication(&self, http_fetch: &impl HttpFetch, …)`, and the shard and recording fetches likewise. Exported functions pass the reqwest client; tests pass `DirectoryFetch` over a temporary fixture tree.
  - Cache layout: `course-index.json`, `course-index-base-url`, `courses/<course id>/manifest.json`, `courses/<course id>/repository-base-url`; `shards/` and `audio/` shared. `audio/` stays untrimmed.
  - A `tokio::sync::Mutex<()>` in `CourseLibrary` serializes every remember-and-retain.
  - `load_latest_course_index` forgets courses the index omits (deletes `courses/<id>/`).
  - Legacy `current-manifest.json` and `current-repository-base-url` are deleted in `CourseLibrary::new`.
  - Errors name the URL.
- `CourseStore.swift`:
  - `indexState: CourseIndexLoadState`, `repositoryIndexState`, `publicationStates: [String: CoursePublicationLoadState]`, `updateMessages: [String: String]`.
  - `start()`: cached index, else embedded index, shown at once; then the repository index.
  - `ensurePublicationLoaded(courseId:)`: cached, else embedded, shown at once; then, once the repository index load has finished, the pointer once per launch.
  - `refresh(courseId:)`: the index and that course. `LanguageApp` start and scene activation share one in-flight refresh task.
  - `show(publication)` clears that course's `outlineStates` and `unitStates` only when `manifestSha256()` differs; the identity guard becomes `shownPublication(courseId:) === publication`.
  - Messages per course: "Downloaded the course content published as <label>.", "The course content is up to date.", "The course server could not be reached."
- Audio: `AudioPlayer` holds the `CourseLibrary`; `play` / `playSequence` lose the `version` parameter. Changed call sites: `Audio/PlayButton.swift` (both buttons), `Practice/ExerciseViews.swift` (two), `Review/ReviewPage.swift`, `Practice/ExerciseSession.swift`. `PlayButton` and `PlayAllButton` keep their parameters, so their call sites are unchanged.
- Views:
  - `VersionGate` becomes `CourseIndexGate`; `CourseGate` gates the course's publication and outline, says the course does not exist only after the repository index load has finished, and its retry refreshes that course.
  - `RootView` and `UnitPage`: `onChange(of: courseStore.shownPublication(courseId:)?.manifestSha256())`.
  - `RootView.activeCourseId` and `CoursesPage` read the index.
- Settings (`SettingsSheet.courseContentSection`): for the active course, "Published as <label>", then "The copy included in the app" or "Downloaded from <base>"; "Check for updated course content" refreshes that course; its message shows below. No course open: the section is hidden.

## Migration

- Assumption: the app is not released, so no client outside this machine reads the schema-2 layout.
- Local, once: `rm -rf ./build/artifacts ./build/repository ./web/static/embedded`, then `./scripts/dev/build-local-repository.sh --skip-audio`. `build/clips` is kept. The iOS build's pre-build phase copies the new embedded bundle.
- Web: the old keys `language.artifact.manifest` and `language.artifact.repository-base-url` are deleted on start. OPFS shards keep their file names (`course-<sha256>.sqlite.br`), so unchanged shards survive; the rest go at the first retain.
- iOS: legacy files deleted by `CourseLibrary::new`; shards as for the web.
- R2: nothing has been published, so the new layout starts on an empty bucket.

## Test plan

- shared:
  - `course_index`: round trip, schema mismatch both directions, a malformed course id rejected, `latest_manifest_key` outside the course's directory or with `..` rejected.
  - `course_manifest`: course id mismatch rejected, malformed label rejected, per-course schema fallback key beside the named pointer.
  - `repository_key`: `is_pointer_key` for the index, a course pointer, a schema pointer, a publication manifest, a shard, a recording.
  - `load`: speculative index reuse and discovered-base refetch; `fetch_latest_course_manifest` refuses an embedded index; a course pointer error leaves another course's load working; shard URL carries course id and label; `relative_paths_to_keep` is the union; `send_bound` for each new export.
- course_builder (a recording mock and a fail-on-call mock repository, through the generic functions):
  - Per-course bundle layout and `recordings.json`; two courses get independent labels; an unchanged course returns `Unchanged`; revisions cover only the course's engines.
  - Pruning one course leaves another course's bundles.
  - Publish order (the mock logs the key order); unchanged entry writes no index; renamed entry rewrites it.
  - Resume: a failure injected at each write, then a rerun, leaves the same repository as an uninterrupted publish; a first publish whose index write failed adds the course on rerun.
  - Relabel: a label the target holds with other bytes is replaced and published.
  - A `recordings.json` that differs from the shards is refused.
  - Every-course publish continues after one course fails and exits non-zero; it does not add an unlisted course; it leaves a pointer on another label.
  - Cloudflare R2: the course selection skips test-course bundles; a test-course bundle, and an index listing a test course, are refused before any upload.
  - `remove-course` writes the index before any delete; then an every-course publish leaves the course absent.
  - Retention keeps the pointer's publication and the publications within the grace period; retention of course A leaves course B's publications; local retention of A keeps recordings only B names; an unreadable `recordings.json` keeps every recording.
  - Local lock: a second writer waits while the first holds it. R2 lease: a held lease fails the second command.
  - Local `delete_keys` leaves no empty directory.
  - Prefix: publishing `…+charlotte` beside an existing `…+charlottesville`.
  - Embedded: publish refreshes only the published course, never writes a test course, keeps the label; build drops a course absent from `data/courses/`.
  - Rewritten existing tests: `publish.rs` `publish_bundle_writes_every_file_points_latest_at_the_version_and_reuses_recordings` (the second build is `Unchanged`) and `publish_bundle_refuses_a_version_the_repository_already_has` (a rerun resumes); `artifact_bundle.rs` `refresh_embedded_bundle_leaves_out_test_courses_the_bundle_holds` and `refresh_embedded_bundle_keeps_the_bundle_version_when_no_course_is_left_out`; `build_command.rs` `decide_embedded_bundle_refresh_refreshes_only_when_every_course_but_the_test_courses_is_built` (deleted); `publish_command.rs` guard tests over `CourseManifest`.
- web server: cache headers for the index, a course pointer, a publication file.
- web: `cargo check --target wasm32-unknown-unknown` and `cargo leptos build`; no new `wasm_bindgen_test`. Browser checks (Chrome DevTools):
  - The course list issues `/discovery` and `course-index.json` only.
  - Opening a course issues one pointer request for that course.
  - Publishing the test course locally while a foundations unit is open leaves the unit on screen; reload keeps the foundations shards in OPFS.
  - Storage cleared and `/repository` unreachable: the embedded index and publication open.
  - A deep link to a course the cached index lacks shows the loading status, then the course.
- ffi: the embedded probe checks `COURSE_INDEX_KEY`; embedded index plus publication open from `web/static/embedded`; refreshing course A keeps course B's cached manifest and shards (fixture tree through `DirectoryFetch`); an index without a course forgets it; legacy files are deleted.
- iOS: `CoreIntegrationTests` rewritten for the new API (embedded index and publication, latest publication plus a recording, cached manifest equals latest per course, unreachable origin). `CourseUITests` still opens the course list. By hand: Settings shows the active course's "Published as".

## Implementation phases

Each phase is a branch stacked on the previous one, compiles, and passes `cargo test` on its own. Phases 4 to 6 merge together: from phase 4 the producer writes only the new layout, which the clients read from phases 5 and 6.

1. Contract types and keys (additive).
   - `shared/src/artifact/{course_index.rs, course_manifest.rs, repository_key.rs}` (new), `mod.rs`; `shared/src/course/course_identity.rs` (`is_well_formed_course_id`). `audio_key` stays in `manifest.rs`.
2. Shared loader (additive; new names beside the old functions).
   - `shared/src/artifact/{load.rs, fetch.rs}`.
3. Repository operations and the lock (additive).
   - `tools/course_builder/src/artifact/repository/{artifact_repository.rs, local_artifact_repository.rs, cloudflare_r2_artifact_repository.rs, dry_artifact_repository.rs, repository_lock.rs (new)}`.
4. Producer: per-course bundles, publish with resume and relabeling, index updates, retention, embedded refresh, `remove-course`, `remove-unused-recordings`, `unlock`, server pointer rule.
   - `tools/course_builder/src/{main.rs, build_command.rs, publish_command.rs, remove_course_command.rs (new), workspace.rs, test_courses.rs}`, `artifact/{artifact_bundle.rs, publish.rs, course_index_update.rs (new), retention.rs (new)}`, `version_label/` renamed `publication_label/`.
   - `web/src/server.rs` (cache rule through `repository_key`).
   - `scripts/build/build-course.sh`, `scripts/publish/{publish-course.sh, remove-course.sh}`, `scripts/dev/{serve.sh, build-local-repository.sh}`, `scripts/ios/sync-ios-embedded-bundle.sh`.
   - `docs/architecture/overview.md` producer, layout and contract sections; `README.md`.
5. Web client.
   - `web/src/course_data/{course_data.rs, course_loading.rs}`, `web/src/courses/courses_page.rs`, `web/src/app.rs`, `web/src/platform/platform.rs` (browser and server stubs), pages using `course_gate`.
   - `overview.md` "Consumer" section, web bullet.
6. FFI and iOS (one change; the generated Swift breaks otherwise).
   - `ffi/src/{course_library.rs, course_records.rs}`, `ffi/Cargo.toml` (`dashmap`).
   - `ios/Language/CourseData/CourseStore.swift`, `App/{RootView.swift, LanguageApp.swift}`, `Unit/UnitPage.swift`, `Settings/SettingsSheet.swift`, `Courses/CoursesPage.swift`, `Home/HomePage.swift`, `Audio/{AudioPlayer.swift, PlayButton.swift}`, `Practice/{ExerciseViews.swift, ExerciseSession.swift}`, `Review/ReviewPage.swift`, `ios/LanguageTests/CoreIntegrationTests.swift`, `ios/LanguageUITests/CourseUITests.swift`.
   - `overview.md` "Consumer" section, iOS bullet.
7. Remove the single-manifest contract.
   - `Manifest`, `ManifestCourse`, `MANIFEST_SCHEMA_VERSION`, `MANIFEST_LATEST_KEY`, `LATEST_POINTER_DIRECTORY`, `schema_pointer_key`, the free `schema_fallback_key`, `versioned_manifest_key`, `versioned_file_key`, `parse_manifest`, `ManifestSource`, `discover_latest_manifest`, `load_embedded_manifest`, `parse_cached_manifest`, `load_shard`, `load_shard_bytes`, `fetch::fetch_latest_manifest`, `fetch::fetch_versioned_file`, `fetch::fetch_embedded_manifest`, and their tests.
   - `audio_key` moves to `repository_key.rs`; callers `artifact_bundle.rs`, `publish.rs` and `ffi/src/course_library.rs` follow (`artifact::audio_key` resolves through the re-export).
   - `CLAUDE.md`: the schema sentence names `COURSE_MANIFEST_SCHEMA_VERSION` in `course_manifest.rs` and `COURSE_INDEX_SCHEMA_VERSION` in `course_index.rs`.
   - Fold the lasting decisions into `overview.md` (data flow diagram, deviations) and delete this file.

## Release check

- `check-embedded-bundle cloudflare-r2` (new command, wrapped by `scripts/release/check-embedded-bundle.sh`): for every course in the embedded index, the embedded `courses/<id>/latest/manifest.json` must equal, byte for byte, the manifest R2's `courses/<id>/latest/manifest.json` names, and the embedded index must list exactly the courses R2's index lists. It reads only; it takes no lock. Any difference fails with the course id and both publication labels.
- Run before building the app for release; README's release steps name it.
- Tests: equal manifests pass; a differing manifest, a course missing on either side, and an embedded test course each fail.

## Decisions taken with the user

- Several people may publish to the same repository, so the repository lock, conditional writes, relabelling and the R2 grace period stay.
- Nothing has been published to Cloudflare R2.
- When the repository index stops listing a course, clients forget it and delete its cached files at once; progress is kept.
- A release requires every embedded publication to match R2's current pointer (the release check above).
- `build-local-repository.sh` publishes every course in `data/courses/` with `--include-unlisted-courses`.

## Deviations

- Phase 1: `repository_key.rs` also has `COURSE_LATEST_DIRECTORY`, `course_latest_directory_key(course_id)` (ends in `/`) and `RECORDINGS_FILENAME`; `CACHE_CONTROL_POINTER` and `CACHE_CONTROL_IMMUTABLE` are added to shared beside `cache_control_for_key`, and the copies in `web/src/server.rs` and `artifact_repository.rs` are left for phase 4 to delete.
- Phase 1: `parse_course_manifest` also rejects an empty `publication_label`.
- Phase 1: `CourseManifest` has `file_entries()` and `find_unit(unit_id)`, and `course_index.rs` has `COURSE_INDEX_SCHEMA_VERSION_FIELD`; `manifest::validate_entry` is now `pub`.
