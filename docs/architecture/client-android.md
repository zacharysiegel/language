# Android client

Status: implemented.

A Kotlin and Jetpack Compose app in `android/` over the same Rust core the iOS app uses, reached through UniFFI's Kotlin bindings. The first release does everything the iOS app does; `ios/Language` is the reference for every screen and behaviour. Android Studio is not needed: Gradle, the SDK command-line tools, the emulator and `adb` do everything.

## Decisions

- One app module, `android/app`; Kotlin packages by feature, mirroring `ios/Language/<Feature>/`.
- Application id `ro.zach.language` for every build type.
- `minSdk` 29 (Android 10), `targetSdk` and `compileSdk` 36. Every platform API the app uses (`AudioFocusRequest`, `UtteranceProgressListener`, ExoPlayer's playback speed) exists at 29 without compatibility branches; the S25 runs far above it.
- ABI `arm64-v8a` only: the S25 and the `language_s25` AVD are both arm64, and so is every phone sold for years. Adding `x86_64` later is one `-t` argument and one ABI filter.
- The debug build reaches the Mac's servers at `http://127.0.0.1:3000` and `http://127.0.0.1:3002` through `adb reverse`, on the emulator and on the S25 alike. Not `10.0.2.2`: the committed `web/static/discovery` names `http://127.0.0.1:3002` as the repository, so an emulator using `10.0.2.2` reads discovery, then fails to fetch the course index from its own loopback and never shows the local repository (test courses included). `adb reverse` makes the device's `127.0.0.1` the Mac's, so discovery's address is right everywhere, and no LAN address is needed.
- The release build reads https://language.zach.ro and https://repository.language.zach.ro, like the iOS device build.
- All HTTP is the core's (reqwest over native sockets). Android's network security config governs only the platform's Java HTTP stacks, so it cannot restrict the core; each build type's origins are what keep release traffic on HTTPS. The debug config still declares cleartext for `127.0.0.1` only, so any platform-stack use stays within it.
- TLS on Android is rustls (aws-lc-rs) with the Mozilla roots from `webpki-root-certs`, through reqwest's `tls_certs_only`. Not the platform verifier: on Android it needs a JNI context handed to Rust at startup and an AAR taken from the cargo registry. Other targets keep native-tls. A probe crate with `reqwest 0.13.5` (`rustls`) and `rusqlite` (`bundled`) built for `aarch64-linux-android` with `cargo ndk --platform 29` and NDK 30 without changes.
- Compose Foundation for every control, drawn to `docs/design.md`: square, hairline, no ripple, no elevation. Material 3 only for the pull-to-refresh state on the course page, with the app's own indicator.
- No navigation library: each root tab keeps its own back stack of unit destinations, as each iOS tab has its own `NavigationStack`.
- Recordings play through Media3 ExoPlayer on a dedicated audio thread; the system voice is `TextToSpeech` in `pt-BR`. The app manages audio focus itself, so recordings and the system voice share one interruption state machine, as on iOS.
- The embedded bundle ships as APK assets and is copied once per install into `noBackupFilesDir`, since the core reads it through `DirectoryFetch` (plain file paths).
- Launch overrides (origin, static repository, progress file name, cache directory name) are Intent extras, honoured only in debuggable builds; the instrumentation tests use them as the iOS UI tests use launch arguments.

## Toolchain

- Android SDK `~/Library/Android/sdk`: platform-tools, emulator, `platforms;android-36`, `build-tools;36.1.0`, `ndk;30.0.16248370`, system image `android-36;google_apis_playstore;arm64-v8a`.
- JDK 21 (`/usr/libexec/java_home -v 21` finds the Apple JDK).
- Rust target `aarch64-linux-android` for the project toolchain, and `cargo-ndk`.
- Gradle through the committed wrapper only; the Homebrew `gradle` generates the wrapper once.
- AVD `language_s25` (1080x2340, 420 dpi, API 36, arm64).
- `scripts/android/android-environment.sh`, sourced by every Android script: exports `JAVA_HOME` (from `/usr/libexec/java_home -v 21` when unset), `ANDROID_HOME` (default `~/Library/Android/sdk`), `ANDROID_NDK_HOME` from the NDK version in `android/gradle.properties`, and prepends `${HOME}/.cargo/bin` to `PATH`.

The emulator and the adb server are started from the owner's terminal:

```sh
~/Library/Android/sdk/platform-tools/adb start-server
```

```sh
~/Library/Android/sdk/emulator/emulator -avd language_s25
```

## Rust core for Android

### Build

`scripts/android/build-android-core.sh`, the twin of `scripts/ios/build-ios-core.sh`:

- Reads `languageMinSdk` and `languageNdkVersion` from `android/gradle.properties`, the one place they are written.
- `cargo ndk --target arm64-v8a --platform "${MIN_SDK}" --output-dir target/uniffi/android/jniLibs build --package ffi --release`.
- The library lands at `target/aarch64-linux-android/release/liblanguage_core.so`; cargo-ndk copies it to `target/uniffi/android/jniLibs/arm64-v8a/liblanguage_core.so`.
- Generates the bindings into `target/uniffi/kotlin/` (cleared first), mirroring `target/uniffi/swift/`.
- Prints `built target/uniffi/android`.
- NDK r28 and later link arm64 libraries for 16 KB pages by default, which Android 15 devices may use.
- AGP strips the library with the NDK named by `ndkVersion`; the Cargo release profile is unchanged.

### Kotlin bindings

- New tool `tools/uniffi_bindgen_kotlin` (workspace member), binary `uniffi-bindgen-kotlin`, `main` calling `uniffi::uniffi_bindgen_main()`; `uniffi = { workspace = true, features = ["cli"] }`.
- Invocation, from the repository root so cargo metadata finds `ffi/uniffi.toml`:

```sh
cargo run --quiet --package uniffi_bindgen_kotlin -- generate --library target/aarch64-linux-android/release/liblanguage_core.so --language kotlin --no-format --out-dir target/uniffi/kotlin
```

- Output: one file, `target/uniffi/kotlin/ro/zach/language/core/language_core.kt`. Probed against the iOS static library: suspend functions for every async export, `CoreException.Failure`, sealed classes for `ExerciseRecord`, `VoiceChoiceRecord`, `GroupedTableRecord`, `PausedPracticeStateRecord`, data classes for records.
- Runtime dependencies of the generated code: JNA (`net.java.dev.jna:jna`, the `@aar` artifact), `kotlinx-coroutines-core`, `androidx.annotation` (for `android = true`).
- Cancelling the coroutine of a generated suspend call frees, and so drops, the Rust future.
- The generated sources are build output, never committed.

### Changes to the core

- `ffi/Cargo.toml`: `crate-type = ["staticlib", "cdylib", "lib"]`. Android loads a shared object; iOS keeps the static library.
- `ffi/uniffi.toml`, a `[bindings.kotlin]` table: `package_name = "ro.zach.language.core"`, `cdylib_name = "language_core"`, `android = true`.
- `ffi/uniffi.toml`, `[bindings.kotlin.rename]` with `"CoreError.Failure.message" = "failureMessage"`. A Kotlin error field named `message` collides with `Throwable.message` and the generated class does not compile. Verified with a global `--config` file keyed `[crates.language_core.bindings.kotlin.rename]`; check that the in-crate table produces `val failureMessage` too. Swift is unaffected; its pattern binds positionally.
- TLS per target, in `Cargo.toml` and `shared/Cargo.toml`:
  - Workspace `reqwest` keeps `default-features = false` and names no TLS feature.
  - `shared`: `[target.'cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))'.dependencies]` takes `reqwest` with `native-tls`; `[target.'cfg(target_os = "android")'.dependencies]` takes `reqwest` with `rustls` plus `webpki-root-certs = "1.0.*"`. Resolver 2 keeps each target's features to itself.
  - `shared/src/http/reqwest_fetch.rs`: `ReqwestHttpFetch::create` passes the builder through a trust-root function with two `#[cfg]`-gated bodies, the Android one calling `tls_certs_only` with `webpki_root_certs::TLS_SERVER_ROOT_CERTS` converted by `reqwest::Certificate::from_der`. Each gate carries its one-line reason per `docs/conventions/conditional-compilation.md`.
- Logging: `ffi/src/core_logging.rs` exports `initialize_core_logging()` under `#[cfg(target_os = "android")]`, installing `android_logger` (tag `language_core`, `log::LevelFilter::Info`) so the core's `log` lines reach logcat; the app calls it once before opening the library. iOS has no equivalent call and is unaffected.
- No runtime change: `uniffi`'s `tokio` feature drives every async export on async-compat's tokio runtime, on Android as on iOS.
- No path change: the app passes `cacheDir/artifacts` as `cache_directory`, the installed embedded bundle's directory as `embedded_bundle_directory`, and `filesDir/language-progress.json` to `ProgressBook.open`.
- Checks: `cargo test`, `cargo check --package ffi --target aarch64-apple-ios`, `cargo check --package web --target wasm32-unknown-unknown --features hydrate` (unchanged behaviour), and the Android build script.

## Gradle project

### Layout

```
android/
  settings.gradle.kts            rootProject.name = "language", include(":app"), google() and mavenCentral()
  build.gradle.kts               plugins declared with apply false
  gradle.properties              android.useAndroidX, JVM args, languageMinSdk, languageNdkVersion
  gradle/libs.versions.toml      every dependency and plugin, pinned
  gradle/wrapper/                gradle-wrapper.properties and gradle-wrapper.jar
  gradlew, gradlew.bat
  app/
    build.gradle.kts
    proguard-rules.pro           keeps JNA and ro.zach.language.core
    src/main/AndroidManifest.xml
    src/main/java/ro/zach/language/<feature>/...
    src/main/res/                launcher icon, glyph vector drawables, backup rules, theme
    src/debug/AndroidManifest.xml          adds the network security config
    src/debug/res/xml/network_security_config.xml
    src/test/java/ro/zach/language/<feature>/...
    src/androidTest/java/ro/zach/language/<feature>/...
```

- `.gitignore` adds `/android/.gradle/`, `/android/.kotlin/`, `/android/build/`, `/android/app/build/`, `/android/local.properties`.
- The SDK location comes from `ANDROID_HOME`; no committed `local.properties`.
- Generate the wrapper before any build file exists, so `gradle wrapper` resolves no plugins: `gradle wrapper --gradle-version 9.4.1 --distribution-type bin` in an empty `android/`.

### Version catalog

`android/gradle/libs.versions.toml`, exact versions only. Resolved from Google Maven on 2026-10-13:

- Android Gradle plugin 9.4.1 (built-in Kotlin support; no separate `kotlin-android` plugin).
- Compose BOM 2026.09.00: `ui`, `foundation`, `runtime`, `material3` (pull to refresh only), `ui-test-junit4`, `ui-test-manifest`, `ui-tooling-preview`.
- `androidx.activity:activity-compose` 1.13.0, `androidx.lifecycle:lifecycle-viewmodel-compose` and `lifecycle-runtime-compose` 2.11.0, `androidx.core:core-ktx` 1.19.1, `androidx.annotation:annotation` 1.11.0.
- `androidx.media3:media3-exoplayer` 1.11.1.
- `androidx.test:runner` 1.7.0, `androidx.test:core` 1.7.0, `androidx.test.ext:junit` 1.3.0.

From Maven Central and the Gradle plugin portal, pinned to the latest stable release once those hosts are reachable (see "Hosts the build needs"):

- Kotlin Compose compiler plugin `org.jetbrains.kotlin.plugin.compose`, at the Kotlin version AGP 9.4.1 supports.
- `org.jetbrains.kotlinx:kotlinx-coroutines-android` and `-test`.
- `net.java.dev.jna:jna` (`@aar` for the app, the jar for JVM unit tests).
- `junit:junit` 4.13.2.

### App module

- `namespace = "ro.zach.language"`, `applicationId = "ro.zach.language"`, `versionName = "0.1.0"` (the iOS `MARKETING_VERSION`), `versionCode = 1`.
- `minSdk` from `languageMinSdk`, `ndkVersion` from `languageNdkVersion`, `ndk.abiFilters += "arm64-v8a"`.
- `buildFeatures { compose = true; buildConfig = true }`.
- Kotlin `jvmToolchain(21)`; public declarations carry explicit types.
- `sourceSets["main"].jniLibs.srcDir("../../target/uniffi/android/jniLibs")` and the Kotlin source directory `../../target/uniffi/kotlin`.
- Task `buildRustCore` (`Exec`, runs `scripts/android/build-android-core.sh`, never up to date, as the iOS pre-build script runs on every build); `preBuild` depends on it. Cargo's own incremental build keeps a no-change run short.
- Task `syncEmbeddedBundle` (`Sync`) from `web/static/embedded` into `build/generated/embedded_bundle_assets/embedded_bundle`, added to every variant's assets with `androidComponents.onVariants { variant.sources.assets?.addGeneratedSourceDirectory(...) }`. It fails, naming `./scripts/dev/build-local-repository.sh`, when `course-index.json` is missing, as `sync-ios-embedded-bundle.sh` does.
- `sourceSets["androidTest"].assets.srcDir("../../data")`, so the instrumentation tests read `courses/<course id>/curriculum.json` and `units/*.json` as the iOS UI tests read `data/`.
- `testOptions.unitTests.isReturnDefaultValues = true` for the JVM tests.

### Build types and origins

- `debug`: `buildConfigField("String", "LANGUAGE_ORIGIN", "\"http://127.0.0.1:3000\"")`, `LANGUAGE_REPOSITORY_BASE_URL` `"http://127.0.0.1:3002"`.
- `release`: `https://language.zach.ro` and `https://repository.language.zach.ro`; `isMinifyEnabled = true`, `proguard-rules.pro` keeping `com.sun.jna.**` and `ro.zach.language.core.**` (JNA finds them by reflection).
- Release signing from Gradle properties outside the repository (`languageReleaseStoreFile`, `languageReleaseStorePassword`, `languageReleaseKeyAlias`, `languageReleaseKeyPassword` in `~/.gradle/gradle.properties`); without them the release build is unsigned and `installRelease` is unavailable.

### Manifest and network security config

- `android.permission.INTERNET`; nothing else.
- `<application android:allowBackup="true" android:dataExtractionRules="@xml/data_extraction_rules" android:fullBackupContent="@xml/backup_rules">`: the progress file in `filesDir` is backed up (iOS Application Support is), the cache and `noBackupFilesDir` are not.
- One activity, `ro.zach.language.app.MainActivity`, launcher, `android:windowSoftInputMode="adjustResize"`, portrait and landscape, as on iOS.
- Debug only, `src/debug/res/xml/network_security_config.xml`: `cleartextTrafficPermitted="false"` in `base-config`, and a `domain-config` permitting cleartext for `127.0.0.1` and `localhost`. Main declares none, so release keeps the API 28+ default of no cleartext.

## Embedded bundle

- Packaged by `syncEmbeddedBundle` as assets under `embedded_bundle/`, the same tree as `ios/Language/Resources/embedded_bundle/`. No sync script: the Gradle task runs on every build, which is when `sync-ios-embedded-bundle.sh` runs in Xcode.
- `coursedata/EmbeddedBundleInstaller.kt`: on `Dispatchers.IO`, before the first embedded read, copies the asset tree to `noBackupFilesDir/embedded_bundle/` when `noBackupFilesDir/embedded_bundle.installed` does not hold this package's `PackageInfo.lastUpdateTime`. Copies into a sibling temporary directory, then renames over the old one, then writes the marker, so an interrupted copy is redone.
- `CourseStore.start` awaits the installer before `showCachedOrEmbeddedIndex`; a failed install is the embedded index's failure and reads as on iOS ("the copy included in the app: ...").
- `./scripts/release/check-embedded-bundle.sh` applies unchanged before a release build.

## Design tokens and components

`design/DesignTokens.kt` is the only place colors, spaces and type sizes are written, mirroring `ios/Language/Design/DesignTokens.swift` (which itself follows `web/style/_tokens.scss`):

- `object LanguageColors`: `paper`, `ink`, `accentActive` (#d50000), `accentHover` (#b50000), `accentLink` (#0050ff), `rule` (#d4d4d4), `accentTint` (#fbe6e6), `inkMuted` (#555555), `goodText` (the iOS green).
- `object LanguageSpacing`: `spaceXs` 4.dp, `spaceSm` 8, `spaceMd` 16, `spaceLg` 32, `spaceXl` 48, `hairline` 1, `buttonIconSize` 30, `measure` 640, `boxShadowOffset` 4, `smallBoxShadowOffset` 2, `floatingTipMaximumWidth` 320, `indexRailWidth` 20, `indexRailEdgeInset` 6, `indexRailEntryMaximumHeight` 21, `tableFieldLabelWidth` 96.
- `object LanguageType`: `bodyText` 15.sp, `dataText` 14 tabular (`fontFeatureSettings = "tnum"`), `labelText` 11 semibold, `captionText` 12, `indexEntryText` 10 tabular, `titleText` 17 semibold, `headingText` 22 semibold, `figureText` 38 semibold tabular, `targetText` 16 medium.
- The compact width threshold, 600.dp (Android's compact window width class), stands in for iOS's compact horizontal size class.

Components, one Kotlin file per iOS file:

- `Design/Controls.swift` to `design/Controls.kt`: `MicroLabel`, `HairlineRule`, `SquareButton` (primary, compact, `SquareButtonPendingKind`), `ChoiceButton` (`ChoiceMarkKind`), `Tag`, `Modifier.panel()`, `Modifier.shadowedBox()` (a solid ink block offset behind, drawn with `drawBehind`), `Modifier.readableWidth()`, `SquareProgressBar`, `Paragraphs`, `PageHeader`, `UnitStatusTag`, `SquareToggle`, `SquareSegmentedControl`. Pressed state from `collectIsPressedAsState` (rule fill, accent hover for primary), `indication = null`. `FlowLayout` becomes Compose's `FlowRow`. `UnitStatusRecord.label()` and `accessibilityName()` become extension functions.
- `Design/PendingIndication.swift` to `design/PendingIndication.kt`: `PendingIndication.delay` from `pendingIndicationDelayMilliseconds()`, `waitForDelay()` returning `PendingDelayOutcome`, `Modifier.pendingGlyphSwap`, `PendingGlyph` (an ellipsis vector drawable), `Modifier.pendingAccessibility` (`stateDescription` plus a polite live region), `PendingActivity` (Compose state, one coroutine at a time), `PendingStatusMessage`.
- `Design/FloatingTip.swift` to `design/FloatingTip.kt`: the anchor reported by `onGloballyPositioned` into a page-level `FloatingTipState`; the tip drawn in an overlay layer above the content, clamped to the window less `spaceMd` and the shadow; a full-size transparent layer under it consumes the first down event and dismisses, so the press never reaches the content; auto close after its duration.
- `Design/IndexRail.swift` and `Design/ScrollMomentumStopper.swift` to `design/IndexRail.kt` and `design/IndexRailTracker.kt`:
  - `IndexRailEntry`, `IndexRail` (fewer than `unitItemIndexMinimumItemCount()` entries shows nothing), drawn over the trailing edge with `Modifier.align(Alignment.CenterEnd)`, taking no layout width from the page.
  - `IndexRailTracker` with two targets, `ColumnScrollTargets` (positions from `onGloballyPositioned` inside a `verticalScroll` column, for the unit page) and `LazyListScrollTargets` (item indexes in a `LazyColumn`, for the glossary). The reading line is `spaceLg` below the visible top; a jumped entry is held until the learner scrolls.
  - Shown while the page scrolls (`isScrollInProgress`), while dragged, or while an entry holds accessibility or keyboard focus; hidden `idleHideDelay` (1500 ms) after; 200 ms fade, the iOS overlay behaviour. Hidden, it takes no pointer input, so touches reach the page.
  - One pointer handler serves taps and drags; the entry under the finger jumps; a light haptic (`HapticFeedbackType.SegmentTick`) on each new entry.
  - The momentum stop needs no scroll view lookup: a jump calls `stopScroll()` on the scroll state, which cancels the fling, then `scrollTo` (no animation).
  - Semantics per entry: content description "Go to note A1.1.4", `selected` for the current one, test tag `<rail identifier>-<visible text>`, custom accessibility actions "Next entry" and "Previous entry" in place of iOS's adjustable action.
- Glyphs: vector drawables in `res/drawable/` (`glyph_course`, `glyph_review`, `glyph_glossary`, `glyph_progress`, `glyph_settings`, `glyph_play`, `glyph_stop`, `glyph_chevron`, `glyph_check`, `glyph_pending`), standing in for the SF Symbols.
- Theme: light only (`preferredColorScheme(.light)`); edge to edge with dark system bar icons; window insets padded with `WindowInsets.safeDrawing`.
- Launcher icon from `ios/Language/Resources/Assets.xcassets/AppIcon.appiconset/AppIcon.png`, scaled with `sips` into `mipmap-*dpi/ic_launcher.png` once and committed.

## Screens and behaviours

Every screen, from its iOS file to its Kotlin file. Test tags equal the iOS accessibility identifiers, and the root sets `testTagsAsResourceId = true`.

### App, data and diagnostics

- `App/LanguageApp.swift` to `app/MainActivity.kt` and `app/LanguageViewModel.kt`:
  - The view model owns `CourseStore`, `ProgressStore`, `AudioPlayer` and the root navigation state, built from `CoreEnvironment.standard(context, intent)`; it survives rotation and is cleared when the activity finishes, so a relaunch reopens the progress file as an iOS relaunch does.
  - `initializeCoreLogging()` once per process, before the library opens.
  - On `Lifecycle.Event.ON_RESUME` (iOS `scenePhase == .active`): `progressStore.advanceReviewClock()` and `courseStore.refresh(activeCourseId)`.
- `App/RootView.swift` to `app/RootScreen.kt`:
  - Bottom bar of four square tabs (Course, Review, Glossary, Progress; test tags `root-tab-course` and so on), the Review tab badged with the due count.
  - Top bar: the course title button (`courses`, opens the course list overlay), a back button when the tab's stack holds a unit (`back`), the page title, the settings button (`settings`, content description "Settings").
  - Settings and the course list are full-screen overlay layers with their own Done; the system back closes an overlay, then pops the tab's stack.
  - `CourseIndexGate`, `CourseGate` (missing course, publication and outline states), `StatusMessage`, `LoadFailure` (with `try-again` and its pending state) as on iOS.
  - `UnitDestination(courseId, unitId, tab)`; `RootTabKind`.
- `CourseData/CoreEnvironment.swift` to `coursedata/CoreEnvironment.kt`: origin and static repository from `BuildConfig`, overridden in debuggable builds by the extras `ro.zach.language.extra.ORIGIN`, `REPOSITORY_BASE_URL`, `PROGRESS_FILE`, `CACHE_DIRECTORY`; cache directory `cacheDir/<name or "artifacts">`, progress `filesDir/<name or "language-progress.json">`.
- `CourseData/CourseStore.swift` to `coursedata/CourseStore.kt`: the same states as sealed interfaces (`CourseIndexLoadState`, `RepositoryIndexLoadKind`, `CoursePublicationLoadState`, `OutlineLoadState`, `UnitLoadState`) held in Compose snapshot state, the same flow: cached index, else embedded, then the repository's; a course's publication once per launch, cached else embedded, then latest once the repository index read finishes; refresh reads already under way are joined through shared `Deferred`s; repository index waiters are a `CompletableDeferred<Unit>`; the shard failure path rereads the pointer once; the same update messages ("Downloaded the course content published as ...", "The course content is up to date.", "The course server could not be reached."). Coroutines on the main dispatcher in the view model's scope.
- `ProgressData/ProgressStore.swift` to `progressdata/ProgressStore.kt`: a `changeCount` snapshot state read by every getter, so composables recompose after any change; the review clock (`reviewClockMilliseconds` state, a coroutine waiting until the next due card, at most one hour) advances after every change and on resume. The `ProgressBook` calls are synchronous and small, made on the main thread as on iOS.
- `Diagnostics/CoreErrorReport.swift` and `LogSubsystem.swift` to `diagnostics/CoreErrorReport.kt` and `diagnostics/LogTag.kt`: `Log.e` with the activity, the message and the Rust backtrace from `CoreException.Failure`; returns the message to show.

### Course list and course page

- `Courses/CoursesPage.swift` to `courses/CoursesPage.kt`: caption "N courses", a row per course (`course-<id>`), "<target> for <source> speakers"; choosing remembers the course.
- `Home/HomePage.swift` to `home/HomePage.kt`: header with "N units", "completed / total units completed", Continue (`continue`) to the first incomplete unit; per level "A1 · <title>" and unit rows (`unit-<id>`) with order, title, status tag and chevron; pull to refresh calls `courseStore.refresh(outline.id)`.

### Unit page

- `Unit/UnitPage.swift` to `unit/UnitPage.kt`:
  - `UnitTabKind` (labels Overview, Principles, Dialogues, Reading, Vocabulary, Practice; Reading only when the unit has readings), test tags `tab-<raw value>`, the horizontally scrolling tab row `unit-tabs`, the previous and next tab links (`next-tab`).
  - Header "A1 · 3" and title, the status panel, the section, the tab links, in a `verticalScroll` column with `readableWidth`; each tab change resets the scroll and calls `recordContentChange`.
  - Index rail over the trailing edge for Principles, Dialogues and Reading, entries "1" to "n", accessibility label "Go to <noun> <item label>", rail tag `unit-item-index`.
  - Leaving the page or changing tab stops audio and closes the help tip.
  - Prefetch: `LaunchedEffect(RecordingPrefetchRequest(tab, engine, contentKey))` runs `RecordingPrefetch.prefetchTab`.
- `Unit/UnitStatusPanel.swift` to `unit/UnitStatusPanel.kt`: "Status" label, the "?" button (`unit-status-help`, the floating tip anchor), the segmented control (`unit-status-not-started`, `-in-progress`, `-completed`, each `selected` in semantics); the help tip (`unit-status-help-tip`) with the two numbered rules (pass mark from `passPercent()`), close button (`unit-status-help-close`), auto close after 8 s.
- `Unit/LessonSections.swift` to `unit/LessonSections.kt`:
  - Overview: objectives, "Add the N words to my review deck" (`add-words`) and its message, the culture note.
  - Vocabulary: rows with play buttons for the item and its example, part of speech tag, literal gloss, note.
  - Principles: "Show translations" toggle (`show-translations`), per note the item label (`unit-item-label`, tabular), title, paragraphs, table, examples; each note is rail target n.
  - Dialogues: label, title, context, Play all and Stop, lines with speaker; every line of a dialogue carries `VoiceChoiceRecord.WholeDialogue` with all its line texts, so the whole dialogue plays from one recording set.
  - Readings: `ReadingPanel` with Play all, paragraphs, and "Answer the comprehension questions" (`comprehension-questions`) opening an `ExerciseSession` that can only be quit.
  - `UnitOutlineRecord.itemLabel(itemNumber)` from `unitItemLabel(...)`.
- `Unit/PhraseRow.swift` to `unit/PhraseRow.kt`: `TranslationToggle`, `PhraseRow` (target, translation and "literally: ..." when translations are shown).
- `Unit/GrammarTable.swift` to `unit/GrammarTable.kt`: below the compact width, `GroupedTableView` from `groupedTable(table)`: the key (`grouped-table-key`, uppercase headers over an ink rule), group headings (`grouped-table-heading`, heading semantics), rows (`grouped-table-row`, merged semantics with the spoken text "<header>: <value>." per field); at regular width the hairline paradigm grid, scrolling sideways.
- `Unit/PracticeSection.swift` to `unit/PracticeSection.kt`: best score summary, pass mark, the notice (`practice-notice`, including "The practice changed since you paused, so it starts over."), Resume full practice "(question n of m)" (`resume-full-practice`) and Start over (`start-over`) when paused, else Full practice (`full-practice`), Quick practice (`quick-practice`); full practice saves each answer and pauses, quick practice quits; finishing records the outcome and reports words added.

### Practice

- `Practice/ExerciseSession.swift` to `practice/ExerciseSession.kt`: `ExerciseResult`, `ExerciseSessionExitKind` (Quit `quit`, Pause `pause`), `ExerciseRecord.clipKey()`, `explanation()`, `speaksAfterAnswer()`; question position (`question-position`), progress bar, instruction, the exercise body keyed by retry and index, feedback (`feedback-heading`, Hear it, Continue `continue-exercise`), the summary (`session-summary`, Retry missed, Done `session-done`), completion reported once.
- `Practice/ExerciseViews.swift` to `practice/ExerciseViews.kt`: `ExerciseBody`, `ListeningControls` (plays once on appearing), `ChoiceList` (`choice`, shuffled once), `TypedAnswerView` and `AnswerField` (`answer-field`, focused on appearing, no capitalisation or autocorrect, IME Done checks), `FillBlankView`, `OrderWordsView` (`answer-area` with a dashed hairline, `bank-token`, shuffle avoiding sentence order), `MatchView` (`match-left`, `match-right`, the pending glyph on a left choice whose clip waits, 500 ms wrong flash), Check (`check`).

### Review, glossary, progress, settings

- `Review/ReviewPage.swift` to `review/ReviewPage.kt`: due figure (`due-count`), "scheduled for later", the explanations, the direction picker (a square drop-down menu of the three directions), Start review (`start-review`, 20 cards, mixed direction picked per card); the session (`flashcard-prompt`, `show-answer`, Forgot `forgot`, Remembered `remembered`, `review-complete`); the target plays when it shows.
- `Glossary/GlossaryPage.swift` and `GlossaryRail.swift` to `glossary/GlossaryPage.kt` and `glossary/GlossaryRail.kt`: a search field "Search <target> or <source>" folded with `searchKey`, the level segmented control ("All levels", each level code), a `LazyColumn` of runs; the rail lists levels (`glossary-level-index`) with all levels shown and the chosen level's units (`glossary-unit-index`) with one; rows link to the unit's Vocabulary tab. `GlossaryRail.dividing` and `GlossaryRailKind` are plain Kotlin, unit tested.
- `ProgressPage/ProgressPage.swift` to `progresspage/ProgressPage.kt`: summary, units with status tags, the review deck by box from `reviewIntervalDays()`, `ProgressTransfer` (Export copies to the clipboard through `ClipboardManager`, Import from the text box, the monospaced text box `transfer-text`, reset with a second tap).
- `Settings/SettingsSheet.swift` to `settings/SettingsSheet.kt`: Voice (`voice-auto`, `voice-kokoro`, `voice-macos_say`, same titles and details), Speed (Slow audio), Course content ("Published as <label>" `publication-label`, "The copy included in the app" or "Downloaded from <url>", "Check for updated course content" `check-for-updated-content` with its pending state and "Checking for updated course content…", then the store's update message).

### Pending states

- Every pending indication waits `pendingIndicationDelayMilliseconds()` from the core, as on iOS: loading page messages (`pending-status`), Try again, the course content check, play buttons, Play all, match choices.
- Pending play controls show the ellipsis in red in place of the glyph and carry the state description "Loading the recording".

## Audio

- `Audio/AudioPlayer.swift` to `audio/AudioPlayer.kt`: `AudioClip`, `PlaybackControlKind`, `PlaybackPendingTarget`, the playback generation, one playback job (starting a clip or a sequence stops the last), the pending request with serials, `isIndicatedPending`, `hasPendingRequest`, `stop`; recording lookup through `CourseLibrary.recordingPath(clipKey, engine, spokenText, voiceChoice)`; a recording that fails falls back to the system voice; normal rate 1.0, slow 0.7.
- `Audio/AudioOutput.swift` to `audio/AudioOutput.kt`:
  - One `HandlerThread` ("language-audio"); its looper is ExoPlayer's application looper and its `Handler.asCoroutineDispatcher()` serializes every call, as the Swift actor does. Nothing touches the player on the main thread.
  - One `ExoPlayer` reused per play (`setMediaItem` of the local file, `PlaybackParameters(rate)`), `AudioAttributes` usage media and content type speech, `handleAudioFocus = false`.
  - One `TextToSpeech`, created at start (`prepareWithoutActivating`), language `Locale.forLanguageTag("pt-BR")`, `setSpeechRate(rate)`, `UtteranceProgressListener` with per-utterance ids (`onStart` sound started, `onDone` finished, `onStop` stopped, `onError` failed). A missing `pt-BR` voice is logged and the default voice speaks, as iOS falls back to the default voice.
  - `PlaybackCompletion` becomes a `CompletableDeferred<PlaybackOutcome>`; the first completion wins; `PlaybackOutcome` (finished, failed, stopped, interrupted).
  - Audio focus: `AudioFocusRequest` with `AUDIOFOCUS_GAIN_TRANSIENT`, `setWillPauseWhenDucked(true)`, requested before a sound starts (iOS session activation) and abandoned when the output goes idle. `AudioInterruptionEvent` from the focus change: transient loss is `Began`, gain after it is `EndedShouldResume`, permanent loss is `EndedShouldNotResume`; `observeInterruption(event)` drives the same state machine as iOS (a recording pauses and resumes; speech, which cannot pause on Android, ends `Interrupted`). `ACTION_AUDIO_BECOMING_NOISY` ends playback `Interrupted`.
- `Audio/PlaybackTiming.swift` to `audio/PlaybackTiming.kt`: the same steps (`recording_lookup`, `focus_request` for session activation, `player_preparation`, `play_start`, `system_voice_lookup`, `system_voice_speak_start`) as `android.os.Trace` async sections for Perfetto and a debug line each: `playback step finished; [step=... duration_ms=... main_thread=...]`.
- `Audio/RecordingPrefetch.swift` to `audio/RecordingPrefetch.kt`: `prefetchTab` runs `publication.prefetchUnitTabRecordings(...)` in the view model's scope and awaits it; when the calling coroutine is cancelled it calls `PrefetchCancellation.cancel()` and still awaits, so downloads already started finish, as on iOS. (Kotlin cancellation alone would drop the Rust future mid-download.)
- `Audio/PlayButton.swift` to `audio/PlayButton.kt`: `PlayButton` (`play`, content description "Play audio: <text>" or "Play slowly: <text>", the "0.7×" glyph for slow), `PlayAllButton` (`play-all`, `stop`).

## Tests

### Unit tests (JVM, `./gradlew testDebugUnitTest`)

Plain Kotlin logic only; nothing that calls into the core, whose library is built for Android.

- `glossary/GlossaryRailTest.kt`: the five iOS `GlossaryRailTests` cases.
- `design/IndexRailTrackerTest.kt`: the entry passed at a reading line, a jump held until the learner scrolls, the entry number under a drag position.
- `practice/ExerciseRecordsTest.kt`: `speaksAfterAnswer`, `clipKey`, `explanation` per kind; the order-words shuffle never returns sentence order.
- `unit/GroupedTableRowTest.kt`: the spoken text of a row.
- `audio/AudioInterruptionEventTest.kt`: focus change codes to interruption events.

### Instrumented tests (`./gradlew connectedDebugAndroidTest`)

- `coursedata/CoreIntegrationTest.kt`, from `CoreIntegrationTests.swift`: the installed embedded bundle opens an index, a publication, an outline and a unit; latest publications and a recording from the local servers (skipped with `assumeTrue` when `http://127.0.0.1:3000` does not answer); an unreachable repository raises `CoreException.Failure` naming it, with a backtrace; the progress book records and persists; typed answers; the spoken text of a fill-blank exercise.
- `audio/AudioPlayerPendingTest.kt`: the system voice starting ends the pending request; `stop` ends it.
- `audio/AudioOutputInterruptionTest.kt`: a 5 s silent WAV written to the test's cache plays and, on `Began` then `EndedShouldNotResume`, ends `Interrupted`.
- `progressdata/ProgressStoreReviewClockTest.kt`: the due count recomposes when a card comes due (a snapshot observer instead of `withObservationTracking`).

### End-to-end suite

Compose UI tests driving `MainActivity` through `ActivityScenario` with `createEmptyComposeRule()`, each test with its own progress file extra, reading the course data from the test APK's assets. `testsupport/CourseAppDriver.kt` launches, relaunches keeping progress, relaunches with an unreachable repository (`http://192.0.2.1:3002`, TEST-NET-1) and its own cache, chooses `pt_br.en.foundations` when the course list opens, opens units and tabs, solves exercises from their JSON, and saves screenshots with `UiAutomation.takeScreenshot()` into the app's external files directory under the iOS names. `testsupport/CourseData.kt` reads units and the curriculum with `org.json`.

Gestures that iOS runs without waiting for idle run with `mainClock.autoAdvance = false`, advancing frames by hand, so a fling is still moving when the rail is tapped.

From `CourseUITests.swift`, one Android test per iOS test, in the feature package that owns the screen:

- `unit/UnitPageEndToEndTest.kt`
  - `unitPage_opensEveryTab` (home opens a unit and every tab)
  - `unitStatus_isSetByTheStatusControlAndPractice`
  - `unitStatusHelp_floatsOverThePageAndCloses` (tab row bounds unchanged; a press outside closes without adding words; auto close)
- `unit/LessonSectionsEndToEndTest.kt`
  - `principles_notesAreLabelledAndTheIndexRailJumpsToThem` (tap and drag)
  - `indexRail_isShownOnlyWhileScrollingAndTakesNoWidth` (touches pass through when hidden; content inset equally; shadow clears the edge)
  - `indexRail_jumpsDuringMomentumScrolling`
  - `principlesTables_areGroupedByTheirFirstColumnAtCompactWidth`
  - `principlesTables_withoutARepeatedFirstColumnAreStacked`
  - `playingClips_leavesThePageUsable`
  - `play_waitingForItsRecordingShowsPendingUntilStopped` (single play and Play all)
- `unit/PracticeSectionEndToEndTest.kt`
  - `fullPractice_scoresEveryAnswerCorrect`
  - `pausedFullPractice_resumesAfterTheAppIsRelaunched`
  - `startOver_replacesThePausedFullPractice`
  - `quickPractice_isQuitAndLeavesThePausedFullPractice`
  - `readingQuestions_canBeAnswered`
- `review/ReviewEndToEndTest.kt`: `review_addsWordsAndGradesACard`.
- `glossary/GlossaryEndToEndTest.kt`
  - `glossaryLevelRail_jumpsToTheFirstWordOfALevel`
  - `glossaryUnitRail_ofOneLevelJumpsToTheFirstWordOfAUnit`
- `app/RootScreenEndToEndTest.kt`
  - `glossaryProgressAndSettings_open` (publication label starts "Published as ")
  - `navigation_showsTheCourseTitleAndOpensTheCourseList`

### Script

`scripts/android/test-android.sh`:

- Sources the environment; reads `adb devices`; with no device, prints how to start the emulator and exits non-zero.
- `adb reverse tcp:3000 tcp:3000` and `adb reverse tcp:3002 tcp:3002` (`ANDROID_SERIAL` picks a device when several are attached).
- `./gradlew testDebugUnitTest connectedDebugAndroidTest` in `android/`.
- `adb pull` of the screenshots to `/tmp/language-android-screenshots`, printing the path.
- Like the iOS suite it passes with or without `./scripts/dev/serve.sh` running; with it, the local test courses appear and the course list opens first.

`scripts/android/run-android.sh`: environment, `adb reverse` for both ports, `./gradlew installDebug`, `adb shell am start -n ro.zach.language/.app.MainActivity`.

## Running on the Samsung S25

- On the phone: Settings, About phone, Software information, tap Build number seven times; then Developer options, USB debugging on.
- Connect over USB, accept the computer's key on the phone; `~/Library/Android/sdk/platform-tools/adb devices` lists it as `device`.
- Debug build against the Mac's servers: `./scripts/android/run-android.sh` (it sets up `adb reverse`, which lasts until the phone disconnects).
- Against the deployed site: a signed release build, `cd android && ./gradlew installRelease`.
- Wireless debugging (Developer options, Wireless debugging, `adb pair <address>`) works the same once paired, `adb reverse` included.
- With the emulator and the phone both attached, `ANDROID_SERIAL=<serial>` picks one.

## Hosts the build needs

- Reachable now: `dl.google.com` (Google Maven, AGP and AndroidX), crates.io for the Rust dependencies.
- Refused by the egress proxy on 2026-10-13 (curl exit 56): `repo.maven.apache.org` (Kotlin, coroutines, JNA, JUnit), `repo1.maven.org`, `plugins.gradle.org` (plugin portal), `services.gradle.org` (the wrapper's distribution download). Each needs an entry in `~/.claude/apple/dangerous_allowed_domains.csv` before the first Gradle build from an agent shell.

## Implementation steps

In dependency order. Steps 1 to 3 are sequential; the four tracks of step 4 run in parallel, each owning the files it names; step 5 closes.

1. Rust core for Android
   - Files: `Cargo.toml`, `shared/Cargo.toml`, `shared/src/http/reqwest_fetch.rs`, `ffi/Cargo.toml`, `ffi/uniffi.toml`, `ffi/src/core_logging.rs`, `ffi/src/lib.rs`, `tools/uniffi_bindgen_kotlin/` (new), `scripts/android/android-environment.sh`, `scripts/android/build-android-core.sh`, `.gitignore`.
   - Checks: `cargo test`, the iOS and wasm checks above, `./scripts/ios/build-ios-core.sh`, `./scripts/android/build-android-core.sh` producing the `.so` and `language_core.kt` with `failureMessage`.
2. Gradle foundation and app shell
   - Files: everything under `android/` except the track-owned files below; `app/`, `coursedata/`, `progressdata/`, `diagnostics/`, `design/` (tokens, controls, pending indication, floating tip, index rail and tracker), `courses/CoursesPage.kt`, `home/HomePage.kt`, `unit/UnitPage.kt`, `unit/UnitStatusPanel.kt`; `testsupport/`; `coursedata/CoreIntegrationTest.kt`, `progressdata/ProgressStoreReviewClockTest.kt`, `design/IndexRailTrackerTest.kt`, `unit/UnitPageEndToEndTest.kt`, `app/RootScreenEndToEndTest.kt` (the navigation test; the settings assertions join in track D); `scripts/android/test-android.sh`, `scripts/android/run-android.sh`.
   - Track-owned composables exist from this step with their final signatures and a `PageHeader` body, so every track compiles on its own: `OverviewSection`, `GrammarSection`, `DialoguesSection`, `ReadingsSection`, `VocabularySection` (`unit/LessonSections.kt`), `PracticeSection`, `ExerciseSession`, `ReviewPage`, `GlossaryPage`, `ProgressPage`, `SettingsSheet`. Each track replaces its bodies.
   - Checks: `./gradlew assembleDebug testDebugUnitTest`, and the instrumented suite when a device is attached.
3. Audio
   - Files: `audio/` (all six), `audio/AudioPlayerPendingTest.kt`, `audio/AudioOutputInterruptionTest.kt`, `audio/AudioInterruptionEventTest.kt`; the view model gains the player.
4. Parallel tracks
   - A. Lesson sections: `unit/LessonSections.kt`, `unit/PhraseRow.kt`, `unit/GrammarTable.kt`, `unit/GroupedTableRowTest.kt`, `unit/LessonSectionsEndToEndTest.kt`.
   - B. Practice: `practice/ExerciseSession.kt`, `practice/ExerciseViews.kt`, `unit/PracticeSection.kt`, `practice/ExerciseRecordsTest.kt`, `unit/PracticeSectionEndToEndTest.kt` (the reading questions test needs track A's readings; it lands with whichever of A and B finishes second).
   - C. Review and glossary: `review/ReviewPage.kt`, `glossary/GlossaryPage.kt`, `glossary/GlossaryRail.kt`, `glossary/GlossaryRailTest.kt`, `review/ReviewEndToEndTest.kt`, `glossary/GlossaryEndToEndTest.kt`.
   - D. Progress and settings: `progresspage/ProgressPage.kt`, `settings/SettingsSheet.kt`, the settings test in `app/RootScreenEndToEndTest.kt`.
5. Closing
   - The full suite on `language_s25` with `./scripts/android/test-android.sh`; a debug run on the S25; a release build.
   - `README.md` gains an Android section (the commands above); `docs/architecture/overview.md` names the Android client beside iOS; `CLAUDE.md` lists `android/`.
   - This file's status changes to implemented, with its deviations recorded.

## Deviations

Steps 1 and 2 (the foundation):

- Gradle 9.8.1, not 9.4.1: AGP 9.4.1 requires Gradle 9.6.0 or later.
- `compileSdk` 37 (`platforms;android-37.0`, installed with `sdkmanager`); `targetSdk` stays 36. Compose UI 1.12.1, from BOM 2026.09.00, requires compiling against API 37. `buildToolsVersion` is pinned to the installed 36.1.0.
- Versions pinned once the hosts resolved: Kotlin (the Compose compiler plugin) 2.4.21, kotlinx-coroutines 1.11.0, JNA 5.19.1.
- No allowlist change was needed: this machine's `~/.gradle/init.gradle` points every Gradle build at the Artifactory mirror on artifacts.apple.com, which serves Maven Central, the plugin portal and Google Maven. The committed settings name only `google()`, `mavenCentral()` and `gradlePluginPortal()`.
- The embedded bundle task is per variant, `syncEmbeddedBundle<Variant>`, since `addGeneratedSourceDirectory` gives each variant its own output.
- `PendingIndication.waitForDelay()` returns nothing: coroutine cancellation ends the wait, so there is no `PendingDelayOutcome`.
- `StatusMessage` lives in `design/Controls.kt`, since `PendingStatusMessage` draws it.
- `UnitOutlineRecord.itemLabel` is in `unit/UnitItemLabel.kt`, since the unit page's rail needs it before the lesson sections exist.
- `GrammarSection`, `DialoguesSection` and `ReadingsSection` take the page's `ColumnScrollTargets`, through which each box marks itself as its rail entry's target.
- `SettingsSheet(courseId)` draws only its content; the root's overlay layer draws the title and Done (test tag `done`), for the course list as well.
- Instrumented tests use `androidx.compose.ui.test.junit4.v2.createEmptyComposeRule`; the first one is deprecated.
- `scripts/android/android-device.sh` holds the device check and `adb reverse`, shared by the run and test scripts; the test script passes `android.injected.androidTest.leaveApksInstalledAfterRun=true` so the screenshots survive the run.
- Not yet written: `coursedata/CoreIntegrationTest.kt`, `progressdata/ProgressStoreReviewClockTest.kt`, and the status tests of `unit/UnitPageEndToEndTest.kt`; the unit page does not yet stop audio or prefetch recordings (step 3).

The glossary, progress and settings step:

- The glossary's search field sits in the list's first item with the header and the level control; Android has no navigation bar search field.
- Glossary rows have no play button yet, since `audio/` has not landed; the row takes `PlayButton(AudioClip(clipKey, target))` as its first child once it does.
- `GlossaryRun` carries `firstEntryIndex` rather than an id, which the page offsets by the header item to map rail entries to `LazyColumn` items. `GlossaryEntry` and `matchingGlossaryEntries` live in `glossary/GlossaryEntry.kt`.
- The level control's segments are tagged `glossary-level-filter-all` and `glossary-level-filter-<level code>`; the iOS test finds them by label.
- `design/SquareTextField.kt` is new (the glossary search and the progress text box), with the tokens `LanguageSpacing.transferTextHeight` and `LanguageType.transferText`.
- A failed export or reset shows "Could not export" or "Could not reset" with the store's error, where iOS reports success.
- `settings/SettingsEndToEndTest.kt` is new: with the repository unreachable, the update check shows "Checking" and the checking message.
- The progress page stays in `progresspage/`, as the foundation named it.

The practice and review step:

- `audio/` has not landed, so `practice/AudioStandIns.kt` holds internal stand-ins with the audio step's names (`AudioClip`, `PlaybackControlKind`, `AudioPlayer`, `LocalAudioPlayer`, `PlayButton`) that draw the controls and play nothing. Listening choice and dictation have no sound until then. The audio step deletes the file and imports `ro.zach.language.audio` in `practice/ExerciseSession.kt`, `practice/ExerciseViews.kt` and `review/ReviewSession.kt`.
- `practice/ExerciseSessionState.kt` is new: the session's place, saved with `rememberSaveable` so a session survives a tab change, as iOS keeps its tabs alive. `ExerciseResult` and `ReviewItem` are `Serializable` for the same reason. `practice/ExerciseSessionStateTest.kt` is new.
- The `ExerciseRecord` extensions are in `practice/ExerciseRecords.kt`.
- The review is split into `review/ReviewPage.kt`, `review/ReviewSession.kt` and `review/ReviewItems.kt` (`VocabularyIndex`, `buildReviewItems`); `ReviewCardSide` replaces `promptIsTarget` and `ReviewGradeKind` the remembered flag. `review/ReviewItemsTest.kt` is new.
- `design/SquareDropDown.kt` (the direction menu, in a `Popup`) and `design/TextLink.kt` are new, with the tokens `LanguageSpacing.answerAreaMinimumHeight` and `LanguageSpacing.answerAreaDashLength`.
- Test tags beyond the iOS identifiers: `retry-missed`, `review-direction`, `direction-<direction>`, and `quit` on the review card.
- `practice/ExerciseSolver.kt` in `androidTest` answers exercises from their JSON, for the practice and reading question tests.
- `ReadingQuestionsSession` in `unit/IntegrationStubs.kt` draws `ExerciseSession` with the quit exit; `readingQuestions_canBeAnswered` is in `unit/PracticeSectionEndToEndTest.kt`.
- `review/ReviewEndToEndTest.kt` also checks that the due count and the Review tab's badge agree before and after grading, and that the source-to-target direction prompts with the translation.

The audio and integration step:

- `audio/AudioInterruptionEvent.kt` holds the focus change mapping on its own, so the JVM test reaches it without the output.
- `AudioPlayer` takes its `AudioOutput` and coroutine scope as constructor parameters; the instrumented test builds one with no library.
- The output abandons the audio focus 1 s after it goes idle, so the clips of a Play all keep it between them.
- A permanent focus loss ends playback `Interrupted` even without a transient loss before it, which Android allows and iOS does not.
- `PlaybackTiming` has one more step, `player_creation`, since one ExoPlayer is created once and reused.
- The manifest queries the `TTS_SERVICE` intent, since package visibility otherwise hides the system voice's engine.
- The prefetch runs in the unit page's `LaunchedEffect`, keyed by `RecordingPrefetchRequest`; the core call runs on `Dispatchers.IO` under `NonCancellable`, and a cancelled caller calls `PrefetchCancellation.cancel()`.
- The design token `LanguageSpacing.playGlyphSize` is new.
- `ReadingQuestionsSession` moved into `unit/LessonSections.kt`; `unit/IntegrationStubs.kt` and `practice/AudioStandIns.kt` are gone.
- `CoreIntegrationTest` and `ProgressStoreReviewClockTest` use `http://127.0.0.1:9` as the unreachable origin; nothing listens on port 9, so the connection is refused at once.
- `scripts/android/test-android.sh` builds the core, packages the embedded bundle, assembles the app and its tests and runs the JVM unit tests before the instrumented suite, and copies the screenshots whether or not the suite passed.
- `CLAUDE.md` does not yet list `android/`.
