# Language

Language courses that teach the principles first (sounds and spelling, stress and accents, the structure of the language, then grammar), with dialogues, readings, exercises and vocabulary, spoken audio for every sentence in the language taught, and a spaced-repetition review deck. Each course is one curriculum in a target language taught in a source language, in `data/courses/<course id>/`; the first is Brazilian Portuguese for English speakers, from A1 to B2. Progress is kept on the device.

## Run it

```sh
./scripts/dev/serve.sh
```

On the first run that builds the courses, the test courses included, and publishes them to the local repository (`build/repository/`), then serves the app at http://127.0.0.1:3000 with `cargo leptos watch` and the local repository at http://127.0.0.1:3002 with `repository_server`. The audio is spoken by the Kokoro neural voices, run locally through ONNX Runtime with eSpeak NG for the phonemes, and by the macOS Brazilian Portuguese voices; the two engines synthesize at the same time. The first run downloads pinned model files into `models/` (checked against checksums) and synthesizes every clip with each engine, which takes tens of minutes; later runs synthesize only the clips that changed. It needs `cmake` (eSpeak NG is built from source), `ffmpeg`, and Apple silicon.

The Voice menu chooses between the two recording sets: Auto (the macOS voice for clips of one or two words, where the neural voice is unreliable, and the neural voice for longer ones; every line of a dialogue plays from the same recording set, the neural one when any line is longer), Neural, or macOS. `./scripts/build/build-course.sh --engine macos-say` synthesizes only the macOS set, which needs no download.

To rebuild the local repository later, for example after editing `data/`, run `./scripts/dev/build-local-repository.sh`, which builds with the test courses and publishes every course in `data/courses/` locally; its arguments go to the build (`--skip-audio`, `--engine macos-say`). The running repository server then serves the newly published course content.

## How the data flows

- `course_builder build` validates `data/`, synthesizes missing clips into `build/clips/<engine>/`, and writes one bundle per course to `build/artifacts/<course id>/<publication label>/`, a publication of that course's content labelled `<date>+<city>`: a `manifest.json`, a `recordings.json`, one brotli-compressed SQLite shard for the course outline and one per unit (`data/*.sqlite.br`, named by their SHA-256), and the recordings (`audio/<sha256>.m4a`). A course whose content is unchanged keeps its newest bundle.
- `course_builder publish local | cloudflare-r2 | dry` publishes each course on its own: the recordings the repository lacks, the shards, the publication's `manifest.json`, the course's `courses/<course id>/latest/manifest.json`, then the course index, `course-index.json`, which lists the courses. Without `--course` it publishes every course the repository's index lists; `--course <id>` publishes the named courses, and `--include-unlisted-courses` adds every built course. A failed publish completes when rerun. Publishing to `local` or `cloudflare-r2` also copies each published course's manifest and shards, never a test course's, to `web/static/embedded/`, the bundle the app falls back to when no repository answers.
- `course_builder remove-course local | cloudflare-r2 --course <id>` (`./scripts/publish/remove-course.sh`) takes a course out of a repository and the embedded bundle. Leaving a course out of a build or a publish does not remove it.
- The app reads `/discovery`, the committed `web/static/discovery`, for the repository's address, fetches the course index, fetches a course's latest manifest when the course is opened, opens the course shard, and opens a unit's shard when the unit is visited. Every download is checked against its hash and cached in the browser, so the app opens offline once visited.

## Test courses

- A course whose `curriculum.json` sets `"test_course": true` exists to test the apps, for example with more than one course; `pt_br.en.test_course` is one.
- `course_builder build` leaves test courses out unless given `--include-test-courses`; `--course <test course id>` without the flag is an error. `course_builder validate` always checks them.
- The local flow includes them: `./scripts/dev/serve.sh` on its first run, and `./scripts/dev/build-local-repository.sh`.
- The embedded bundle never holds a test course, and `publish cloudflare-r2` refuses a test course's bundle, and a Cloudflare R2 course index that lists one.

## Publishing to Cloudflare R2

Copy `template.env` to `.env` and fill in the R2 settings. Put the publish token's secret access key in `secrets.yaml`, encrypted with your master key:

```sh
secr encrypt --key "$MASTER_SECRET" --name cloudflare.r2.publish.secret_access_key <secret-access-key>
```

Add the app's origin to `tools/course_builder/r2-cors.json` and apply it with `cargo run --release --package course_builder -- set-artifact-cors` (the token needs bucket admin permission). Then build without `--include-test-courses` and publish: `./scripts/build/build-course.sh`, then `./scripts/publish/publish-course.sh cloudflare-r2 --course <course id>` to add a course, or `./scripts/publish/publish-course.sh cloudflare-r2` to publish every course R2 already lists.

Every command that writes R2 holds a lease for its whole run. If one stops without releasing it, `cargo run --release --package course_builder -- unlock cloudflare-r2` removes the lease. Recordings no publication names stay on R2 until `cargo run --release --package course_builder -- remove-unused-recordings cloudflare-r2`.

## Deploying the site

`./scripts/build/deploy-site.sh --build` writes `target/deploy-site/`, the committed discovery document with `repository_base_url` set to https://repository.language.zach.ro (or `LANGUAGE_REPOSITORY_BASE_URL`) and `web/static/_headers`, and deploys it with `npx wrangler deploy` to Cloudflare Workers Assets at https://language.zach.ro (`web/wrangler.toml`). It needs `jq` and Node; authenticate once with `npx wrangler login`. `--dry-run` stops before uploading.

## Releasing the apps

Before building the web or iOS app for release, publish to Cloudflare R2 after any local publish, since every publish refreshes the embedded bundle, then run `./scripts/release/check-embedded-bundle.sh`. It fails unless the embedded bundle lists exactly the courses R2's index lists, each at the publication R2's latest pointer names.

## iOS app

`ios/` is a SwiftUI app over the same Rust core, reached through UniFFI. It needs Xcode and XcodeGen (`brew install xcodegen`):

```sh
./scripts/dev/build-local-repository.sh --skip-audio
./scripts/ios/sync-ios-embedded-bundle.sh
./scripts/ios/build-ios-core.sh
cd ios && xcodegen generate && open Language.xcodeproj
```

Building in Xcode reruns the last two scripts. The app reads `/discovery` from `LANGUAGE_ORIGIN` (a build setting in `ios/project.yml`, `http://127.0.0.1:3000` for the simulator and https://language.zach.ro for a device), looks for the latest manifest at `LANGUAGE_REPOSITORY_BASE_URL` when discovery cannot be read, opens the bundle shipped inside it when no repository answers, and keeps shards and played recordings in its cache. `xcodebuild test -project ios/Language.xcodeproj -scheme Language -destination 'platform=iOS Simulator,name=iPhone 17 Pro'` runs its tests; the UI tests read the unit files in `data/` to answer a whole unit's practice. With `./scripts/dev/serve.sh` serving a local repository built by `./scripts/dev/build-local-repository.sh`, the app lists the test courses too, and the UI tests choose `pt_br.en.foundations` from the course list.

## Layout

- `data/courses/<course id>/curriculum.json` describes a course (its languages, levels and units); `units/<id>.json` beside it holds one unit each. The format is described in `docs/authoring.md`.
- `shared/` has the course model and its validation, the shard schema and reader, the artifact contract (manifest, discovery, verified fetch and cache), answer checking, the review schedule and the progress model.
- `tools/course_builder/` validates the data, synthesizes the audio, writes each course's bundles, and publishes and removes courses.
- `web/` is the Leptos app and the server that hosts it and `/discovery`.
- `tools/repository_server/` serves the local repository.
- `ffi/` exposes the core to the native apps through UniFFI; `tools/uniffi_bindgen_swift/` generates the Swift bindings.
- `ios/` is the iOS app.

## Commands

```sh
cargo run --package course_builder -- validate --all
./scripts/build/build-course.sh
./scripts/publish/publish-course.sh local --include-unlisted-courses
./scripts/publish/remove-course.sh local --course <course id>
./scripts/dev/build-local-repository.sh
./scripts/release/check-embedded-bundle.sh
./scripts/build/deploy-site.sh --build
cargo test
```
