# Architecture: per-course SQLite publications in an object store

The course follows Eafora's architecture (`/Users/singularity/eafora/docs/architecture/overview.md` and `client.md`): a producer builds immutable, content-addressed SQLite shards and publishes them to an object store; every client is a reader of those files through the shared Rust core, and never calls an application server for course data.

## Data flow

```
data/courses/<course id>/{curriculum.json, units/*.json} (authoring source, in git)
        |  course_builder build: validate, synthesize missing clips
        v
build/clips/<engine>/<clip_key>.m4a          clip store, one directory per speech engine
        |  course_builder build: write one bundle per course
        v
build/artifacts/<course id>/<publication label>/
        manifest.json
        recordings.json                       the recordings the shards name
        data/course-<sha256>.sqlite.br        the course's identity, languages, levels, units, vocabulary, their clips
        data/unit-<id>-<sha256>.sqlite.br     one unit's lesson and its clips
        audio/<sha256>.m4a                    every recording the shards name (hard links into the clip store)
        |  course_builder publish local | cloudflare-r2 | dry
        v
repository:  audio/<sha256>.m4a                                       shared by every course
             courses/<course id>/<publication label>/manifest.json, recordings.json, data/...
             courses/<course id>/latest/manifest.<schema>.json, latest/manifest.json
             course-index.json (written last)
        v
clients: /discovery -> course-index.json -> course list; opening a course -> its latest pointer -> its outline shard -> its unit shards on demand -> recordings when played
```

## Artifact contract

- Each course is published on its own. A publication is a directory `courses/<course id>/<publication label>/`, labelled `<UTC date>+<city>` (a random United States city, unique within the course, for uniqueness and for people to tell publications apart). It holds `manifest.json`, `recordings.json` and the shards under `data/`. Shard names end in the SHA-256 of their bytes.
- Recordings are named by the SHA-256 of their bytes and sit in `audio/`, outside every course and publication, so a publish uploads only the ones the repository lacks. They are not stored in SQLite. `recordings.json` lists the recordings a publication's shards name; only the producer reads it.
- `course-index.json` leads with `course_index_schema_version`, then the courses by course id, each with its title, target and source languages, `latest_manifest_key` (relative to the repository base) and whether it is a test course. It names no publication label, so it changes only when a course is added, renamed or removed. A course list needs only the index. `shared::artifact::CourseIndex` is the canonical type.
- A course manifest leads with `manifest_schema_version`, then `course_id`, `publication_label`, `artifact_created`, the title and languages, whether it is a test course, its outline shard, its unit shards by unit id, and the pipeline revision of each voice engine its recordings come from. Paths are relative to the publication directory. `shared::artifact::CourseManifest` is the canonical type for producer and clients.
- A course's latest pointer, `courses/<course id>/latest/manifest.json`, is a byte-for-byte copy of the manifest of the publication clients should open. Clients decide whether a course's content changed by comparing manifest bytes.
- A shard is SQLite with `application_id` `LANG` and `user_version` set to the shard schema version, compressed with brotli because neither R2 nor a static file server compresses a generic binary type. The manifest hash covers the compressed bytes.
- Every shard has a `clip` table (clip key, persona, spoken text) and a `clip_audio` table (clip key, engine, recording hash, size). The clip key is a hash of the language, the persona and the text, so an unchanged sentence keeps its recordings across builds; the recording hash is what a client fetches.
- Headers: `course-index.json` and everything under `courses/<course id>/latest/` are pointers, `public, max-age=300`, which clients fetch bypassing the HTTP cache; everything else is `public, max-age=31536000, immutable`. `shared::artifact::repository_key::cache_control_for_key` is the one rule, which `course_builder publish` and `repository_server` both apply.
- Publish order, per course: the recordings the repository lacks, the shards, `recordings.json`, the publication's `manifest.json`, `latest/manifest.<schema>.json`, `latest/manifest.json`, then `course-index.json`. A client that reads a pointer finds every file it names.
- A client that cannot read the schema of a course's latest pointer falls back to `manifest.<its schema>.json` beside it.
- Discovery: the app's origin serves `/discovery` (`public, max-age=3600`) with `schema_version`, `repository_base_url`, `minimum_client_version` and `sunset`. The course index is `<repository_base_url>/course-index.json`. A document with an unknown `schema_version` is ignored in favor of the static repository (`LANGUAGE_REPOSITORY_BASE_URL`: on the web compiled in when set at build time, as the site build sets it, otherwise the committed document's `repository_base_url`; on iOS a build setting), whose course index the client requests in parallel and reuses when discovery agrees. `minimum_client_version` is informational: an older client logs it and continues.
- Versioning, as in Eafora: every reader accepts exactly its own course index, manifest, discovery and shard schema versions, and reports whether a mismatched document is older or newer than itself. A later course index schema is published as `course-index.<n>.json` beside `course-index.json`.

## Courses

- A course is one curriculum in one target language taught in one source language; its id is `<target language>.<source language>.<curriculum name>` (`pt_br.en.foundations`). Several courses can share a language pair.
- Text fields are `target` (the language taught), `source` (the language it is taught in) and `source_literal` (the target text word for word in the source language). Levels are the curriculum's own, each saying whether its units need readings.
- Learner progress is kept per course; the study streak counts activity in every course.
- A target language may add validation rules, and needs speech voices before its recordings can be made.

## Producer

- Authoring source stays JSON in git; there is no canonical database. The data is hand-authored and small, so a database would add nothing. This is the deliberate deviation from Eafora.
- `course_builder build` validates, synthesizes each engine's missing clips with the engines running at the same time, and writes one bundle per course to `build/artifacts/<course id>/<publication label>/`. A course whose new manifest equals its newest bundle's, apart from the label and creation time, keeps that bundle. It keeps each course's two newest bundles, removes the bundles of course ids no longer in `data/`, and removes the recorded clips that no unit of any course in `data/` uses, built or not. A test course is built only with `--include-test-courses`.
- `course_builder publish <local|cloudflare-r2|dry>` has three targets, as in Eafora: `local` copies into a directory (default `build/repository/`); `cloudflare-r2` uploads through the S3 API with credentials from `.env` and the secret access key from `secrets.yaml` (decrypted with `secr`); `dry` logs every write as if the repository were empty.
  - Without `--course`, it publishes the newest bundle of every course the target's index lists and `data/` holds, and logs the `remove-course` command for an indexed course `data/` lacks. `--include-unlisted-courses` adds every built course the index does not list. `--course <id>` (repeatable) publishes only the named courses, listed or not; `--bundle <dir>` publishes one bundle.
  - Courses are published one after another; one course's failure is reported, the rest continue, and the command exits non-zero.
  - Publishing is resumable: a rerun uploads every file of the publication the target lacks, even under a `manifest.json` already in place, and skips every pointer and index entry already in place. A bundle whose label the target holds for other course content is relabelled and published under the new label.
  - A named course's pointer always moves to its bundle. A publish of every course moves a pointer only to course content built and uploaded after the publication the pointer names, and leaves a pointer whose label or build time it cannot read. An every-course publish skips an indexed course that has no local bundle.
  - When a pointer is left on another publication, the course's index entry is taken from that publication's manifest and its embedded copy is left as it is, so an embedded publication is always one a pointer names; when this builder cannot read that manifest, the existing index entry is kept.
  - The index is rewritten only when the course's entry changes.
  - A publication's `manifest.json` is written only if absent. An already-current pointer still has a missing or differing `manifest.<schema>.json` beside it rewritten.
  - Retention, per course: the two newest publications by upload time and every publication a pointer names are kept; on Cloudflare R2, so is a publication whose next newer one was uploaded less than seven days ago. A course with a pointer whose publication label cannot be read keeps every publication. A deleted publication loses its other files before its `manifest.json`. Locally, the recordings no publication's `recordings.json` names are then deleted; on R2 that is `remove-unused-recordings cloudflare-r2`.
  - `publish dry` takes no lock, applies no retention and leaves the embedded bundle alone.
- The embedded bundle, `web/static/embedded/`, has the repository's keys with one publication per course and no recordings or schema pointers: `course-index.json`, `courses/<course id>/latest/manifest.json` and the publication's shards. `publish local` and `publish cloudflare-r2` replace each published course in it, keeping the publication label, so every embedded label is one a repository holds. It never holds a test course, and every build and publish drops a course `data/` lacks or marks a test course.
- Every command that writes a repository holds its lock for the whole run: locally an exclusive lock on `<root>/.repository.lock`; on R2 a lease object, `.repository-lock`, created only if absent and naming the host, process and time. A command that finds a lease fails and names it; `unlock cloudflare-r2` removes a lease a crashed command left. Index and pointer writes are also conditional on the entity tag read under the lock, so a writer that ignored the lock makes the command fail.
- `course_builder remove-course <local|cloudflare-r2> --course <id>` rewrites the index without the course, deletes its pointers and then its publications, and drops it from the embedded bundle; locally it also deletes the recordings no publication names. A course left out of a build or a publish is not removed.
- `course_builder remove-unused-recordings <local|cloudflare-r2>` deletes the recordings no publication names, and keeps every recording when a publication's `recordings.json` is unreadable.
- `course_builder check-embedded-bundle cloudflare-r2` (`scripts/release/check-embedded-bundle.sh`, run before building the app for release) reads R2 only, takes no lock, and fails unless the embedded index lists exactly the courses R2's index lists, none of them a test course, and each embedded manifest equals R2's latest pointer byte for byte.
- Cloudflare R2 never holds a test course: publishing there refuses a test course's bundle and an index that lists a course its entry or `data/` marks a test course, before any upload.

## Consumer

- `shared` holds the client side of the contract: course index, course manifest and discovery types, verified fetch, the cache trait, brotli decoding, the shard reader, and a read-only in-memory SQLite (rusqlite on native targets, `sqlite-wasm-rs` with a read-only memory VFS on `wasm32`).
- Web: on start the app shows the course index it cached last, else the embedded one, then the repository's index when it loads; a course the repository's index omits is forgotten, and its progress kept. Opening a course shows the publication cached for it, else the embedded one, then reads that course's latest pointer once through a repository index and adopts the publication it names, keeping the loaded outline and units when the manifest bytes and recordings base are unchanged. A shard that fails to load from a repository makes the course's pointer be read again before the failure is shown. Shards are cached in the origin private file system by name, and the cache is trimmed to the files of every course manifest the browser stored. A unit's shard is fetched when the unit is visited. A recording stays playable from the repository of the publication whose shard named it. The Voice menu picks a recording per clip from `clip_audio`, falling back to the other engine, then to the browser's speech synthesis.
- iOS: a SwiftUI app (`ios/`) over the same core, compiled as a static library for device and simulator, packaged as an xcframework, and reached through Swift bindings that UniFFI generates from `ffi/`. The core does the work the web app does in Rust: discovery, the course index, each course's latest pointer, verified shard loading, caching under `Library/Caches` (the index and one manifest per course, beside shard and recording directories every course shares, the shards trimmed to the files of every remembered manifest), recordings downloaded and verified on first play from the repository of the publication whose shard named them, the embedded bundle included in the app as the floor, progress saved to a file, answer checking and review scheduling. On launch the app shows the cached course index, else the embedded one, then the repository's; a course the repository's index omits is forgotten, and its progress kept. Opening a course shows its cached publication, else the embedded one, then reads its latest pointer once through a repository index, keeping the loaded outline and units when the manifest bytes and recordings base are unchanged; returning to the app reads the index and the open course's pointer again. Settings shows the open course's publication label. Swift draws the screens and plays audio (`AVAudioPlayer`, and the system pt-BR voice when a clip has no recording).
- Android: a Kotlin and Jetpack Compose app (`android/`) over the same core, compiled as a shared library for `arm64-v8a` with `cargo ndk` and reached through Kotlin bindings that UniFFI generates from `ffi/`. It loads courses as the iOS app does, keeps its cache under the app's cache directory, copies the embedded bundle out of its assets once per install, and plays recordings through Media3 ExoPlayer and the system pt-BR `TextToSpeech` voice when a clip has no recording.
- Every client: a course's latest pointer is read only when the course is opened or refreshed, and only through a course index read from a repository; a pointer read for a course the remembered index no longer lists is not remembered. A publication from a repository fetches its recordings from that repository, and the embedded publication from the static repository (`load::recordings_base_url`). Only a failed shard load reads the pointer again; a failed recording falls back to the platform's speech synthesis.
- Learner progress stays on the device.

## Hosting

- Local (the Mac mini): `cargo leptos serve --release` (or `serve.sh` for development) serves the app and the committed `web/static/discovery` at `/discovery`; `repository_server` (started by `serve.sh`) serves `build/repository/` at http://127.0.0.1:3002, the address that document names, with the repository's headers and cross-origin reads allowed. Nothing else is needed.
- R2: the bucket, token and public domain are set up by hand in the Cloudflare dashboard, as for Eafora. `course_builder set-artifact-cors` applies `tools/course_builder/r2-cors.json` to the bucket (the token needs bucket admin permission), then publish with `cloudflare-r2`. The bucket's public domain is https://repository.language.zach.ro.
- Site: `./scripts/build/deploy-site.sh` deploys `target/deploy-site/` to Cloudflare Workers Assets at https://language.zach.ro, as Eafora deploys `app.eafora.org`: a release build of the app, one static shell for every route (any path that is not a file is answered with `index.html`), `web/static` and the committed discovery document rewritten to name https://repository.language.zach.ro, which is also compiled into the client as its static repository. Over the static shell the client renders from scratch; over the dev server's render it hydrates.

## Deviations from the original proposal

- One course shard (outline and vocabulary) replaced the proposed separate index and glossary shards; both pages need the same records.
- Shards are brotli-compressed, as Eafora's are.
- Recordings sit outside the course and publication directories, keyed by hash, rather than under `audio/<engine>/` per publication, so an unchanged recording is uploaded once.
- Publication labels are `<date>+<city>` rather than `<date>+<Nobel laureate>`, unique within a course rather than globally, and checked against the course's local bundles, its publications in the local repository and the embedded bundle, and, at publish, the target's publications of the course, since there is no database.
- The CORS policy is applied through the S3 API by `course_builder set-artifact-cors` rather than by a `wrangler` script.
- `build` synthesizes; there is no separate `synthesize` command.
- The deployed site serves one static shell at every route rather than a prerendered document per route, since a page's content comes from the course data the client loads.
- The local repository is `build/repository/`, served by its own server, rather than `web/static/repository/` served with the app's static files: cargo-leptos copies the whole static directory again on every change in it, and the repository holds every recording.
- The embedded bundle holds every shard and no recordings, rather than three foundation units.
- The repository trait reads objects with their entity tags, writes conditionally, lists objects with their upload times and deletes, which Eafora's does not, for resumable publishing, retention and the repository lock.
- iOS: HTTP (reqwest) and the cache (the filesystem) run inside the Rust core rather than in Swift through `URLSession` and `FileManager`, so iOS and the web share one loader. The FFI surface is a separate `ffi` crate rather than a module inside the core, so the wasm32 build never compiles UniFFI. The generated Swift is compiled into the app target, and the static library's module map is a plain module rather than a framework module.
- Android: TLS in the core is rustls with the Mozilla roots rather than the platform verifier, which on Android needs a JNI context handed to Rust at startup. The generated Kotlin is build output, compiled into the app module from `target/uniffi/kotlin/`.
