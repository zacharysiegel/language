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

`./scripts/build/deploy-site.sh --build` writes `target/deploy-site/` with `./scripts/build/build-site.sh`: a release build with content-hashed file names, the static shell (`index.html`, written by `target/release/web prerender`) that every route serves and the client renders over, `web/static` (the embedded bundle and `_headers` included), and the committed discovery document with `repository_base_url` set to https://repository.language.zach.ro (or `LANGUAGE_REPOSITORY_BASE_URL`, which is also compiled into the client). `./scripts/build/verify-site-tree.sh` checks the tree, and `npx wrangler deploy` uploads it to Cloudflare Workers Assets at https://language.zach.ro (`web/wrangler.toml`). The build leaves `target/site`, which `cargo leptos watch` serves, alone. It needs `cargo-leptos`, `jq` and Node; authenticate once with `npx wrangler login`. `--dry-run` stops before uploading.

## Releasing the apps

Before building the web, iOS or Android app for release, publish to Cloudflare R2 after any local publish, since every publish refreshes the embedded bundle, then run `./scripts/release/check-embedded-bundle.sh`. It fails unless the embedded bundle lists exactly the courses R2's index lists, each at the publication R2's latest pointer names.

## iOS app

`ios/` is a SwiftUI app over the same Rust core, reached through UniFFI. It needs Xcode and XcodeGen (`brew install xcodegen`):

```sh
./scripts/dev/build-local-repository.sh --skip-audio
./scripts/ios/sync-ios-embedded-bundle.sh
./scripts/ios/build-ios-core.sh
cd ios && xcodegen generate && open Language.xcodeproj
```

Building in Xcode reruns the last two scripts. The app reads `/discovery` from `LANGUAGE_ORIGIN` (a build setting in `ios/project.yml`, `http://127.0.0.1:3000` for the simulator and https://language.zach.ro for a device), looks for the course index at `LANGUAGE_REPOSITORY_BASE_URL` when discovery cannot be read, opens the bundle shipped inside it when no repository answers, and keeps shards and played recordings in its cache. `xcodebuild test -project ios/Language.xcodeproj -scheme Language -destination 'platform=iOS Simulator,name=iPhone 17 Pro'` runs its tests; the UI tests read the unit files in `data/` to answer a whole unit's practice. With `./scripts/dev/serve.sh` serving a local repository built by `./scripts/dev/build-local-repository.sh`, the app lists the test courses too, and the UI tests choose `pt_br.en.foundations` from the course list. `./scripts/ios/install-on-device.sh` builds the app in release configuration, signs it with the development team, installs it on the paired iPhone and launches it; a free developer account's provisioning profile lapses after 7 days, and running the script renews it without opening Xcode.

## Android app

`android/` is a Kotlin and Jetpack Compose app over the same Rust core, reached through UniFFI's Kotlin bindings. Android Studio is not needed: the committed Gradle wrapper, the SDK command-line tools, the emulator and `adb` do everything. It needs the Android SDK in `~/Library/Android/sdk` (`platform-tools`, `emulator`, `cmdline-tools;latest`, the platform, build tools and NDK versions `android/app/build.gradle.kts` and `android/gradle.properties` name, and the system image `android-36;google_apis_playstore;arm64-v8a`), JDK 21, the Rust target `aarch64-linux-android` (`rustup target add aarch64-linux-android`) and `cargo-ndk` (`cargo install cargo-ndk`). `./scripts/android/android-environment.sh`, which every Android script sources, sets `JAVA_HOME`, `ANDROID_HOME` and `ANDROID_NDK_HOME`.

Every Gradle build of the app builds the Rust core and its bindings (`./scripts/android/build-android-core.sh`) and packages the embedded bundle from `web/static/embedded/`, so build the local repository first:

```sh
./scripts/dev/build-local-repository.sh --skip-audio
cd android && bash -c 'source ../scripts/android/android-environment.sh && ./gradlew assembleDebug testDebugUnitTest'
```

The debug build reads `/discovery` from `http://127.0.0.1:3000` and the static repository at `http://127.0.0.1:3002`; the run and test scripts map those ports of the device to the Mac's with `adb reverse`, so `./scripts/dev/serve.sh` serves the app on the emulator and on a phone alike. The release build reads https://language.zach.ro and https://repository.language.zach.ro.

### On the emulator

Create the virtual device once (then set `hw.lcd.height=2340` in `~/.android/avd/language_s25.avd/config.ini`, for the S25's 1080x2340 screen at 420 dpi), then start the adb server and the emulator from a terminal:

```sh
~/Library/Android/sdk/cmdline-tools/latest/bin/avdmanager create avd --name language_s25 --package 'system-images;android-36;google_apis_playstore;arm64-v8a' --device pixel_8
~/Library/Android/sdk/platform-tools/adb start-server
~/Library/Android/sdk/emulator/emulator -avd language_s25
```

Then `./scripts/android/run-android.sh` installs the debug build and starts it. `./scripts/android/test-android.sh` builds the core, packages the embedded bundle, assembles the app and its tests, runs the JVM unit tests and the instrumented suite on the attached device, and copies the suite's screenshots to `/tmp/language-android-screenshots`; its arguments go to the instrumented run, for example `-Pandroid.testInstrumentationRunnerArguments.class=ro.zach.language.app.RootScreenEndToEndTest`. Like the iOS suite, it passes with or without `./scripts/dev/serve.sh`; with it, the test courses appear and the tests choose `pt_br.en.foundations` from the course list.

### On a Samsung Galaxy S25 over USB

1. On the phone, open Settings, About phone, Software information, and tap Build number seven times.
2. In Settings, Developer options, turn on USB debugging.
3. Connect the phone with a USB-C cable and accept the computer's key on the phone. `~/Library/Android/sdk/platform-tools/adb devices` then lists it as `device`.
4. Run `./scripts/android/run-android.sh` for the debug build against the Mac's servers. The `adb reverse` mapping lasts until the phone disconnects, so run the script again after reconnecting. `./scripts/android/test-android.sh` runs the suite on the phone.

With the emulator and the phone both attached, `ANDROID_SERIAL=<serial from adb devices>` picks one. Wireless debugging (Developer options, Wireless debugging, `adb pair <address>`) works the same once paired. A release build installed with `cd android && ./gradlew installRelease` reads the deployed site; it needs the signing properties `languageReleaseStoreFile`, `languageReleaseStorePassword`, `languageReleaseKeyAlias` and `languageReleaseKeyPassword` in `~/.gradle/gradle.properties`, and without them `assembleRelease` writes an unsigned APK.

## Layout

- `data/courses/<course id>/curriculum.json` describes a course (its languages, levels and units); `units/<id>.json` beside it holds one unit each. The format is described in `docs/authoring.md`.
- `shared/` has the course model and its validation, the shard schema and reader, the artifact contract (manifest, discovery, verified fetch and cache), answer checking, the review schedule and the progress model.
- `tools/course_builder/` validates the data, synthesizes the audio, writes each course's bundles, and publishes and removes courses.
- `web/` is the Leptos app and the server that hosts it and `/discovery`.
- `tools/repository_server/` serves the local repository.
- `ffi/` exposes the core to the native apps through UniFFI; `tools/uniffi_bindgen_swift/` and `tools/uniffi_bindgen_kotlin/` generate the Swift and Kotlin bindings.
- `ios/` is the iOS app.
- `android/` is the Android app.

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
