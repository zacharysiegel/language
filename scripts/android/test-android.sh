#!/usr/bin/env bash

# Builds the Rust core, packages the embedded bundle, assembles the app and its tests, runs the JVM unit tests, then
# runs the instrumented suite on the attached device or emulator and copies the suite's screenshots to
# /tmp/language-android-screenshots, whether or not the suite passed. ANDROID_SERIAL picks a device when several are
# attached; extra arguments go to the instrumented run, such as
# -Pandroid.testInstrumentationRunnerArguments.class=ro.zach.language.app.RootScreenEndToEndTest.
#
# Usage:
#   ./scripts/android/test-android.sh [gradle arguments...]

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly DEVICE_SCREENSHOT_DIRECTORY="/sdcard/Android/data/ro.zach.language/files/language-android-screenshots"
readonly SCREENSHOT_DIRECTORY="/tmp/language-android-screenshots"

source "${SCRIPT_DIR}/android-environment.sh"
source "${SCRIPT_DIR}/android-device.sh"

# Copies the screenshots the suite saved on the device, replacing those of an earlier run.
pull_screenshots() {
    rm -rf "${SCREENSHOT_DIRECTORY}"
    mkdir -p "${SCREENSHOT_DIRECTORY}"

    local device_screenshots
    device_screenshots="$(adb shell ls "${DEVICE_SCREENSHOT_DIRECTORY}" 2> /dev/null || true)"

    if test -z "${device_screenshots}"; then
        echo "the suite saved no screenshots"
        return
    fi

    adb pull "${DEVICE_SCREENSHOT_DIRECTORY}/." "${SCREENSHOT_DIRECTORY}" > /dev/null
    echo "screenshots in ${SCREENSHOT_DIRECTORY}"
}

require_attached_device
reverse_local_server_ports
adb shell rm -rf "${DEVICE_SCREENSHOT_DIRECTORY}"

cd "${REPO_ROOT}/android"
./gradlew buildRustCore syncEmbeddedBundleDebug assembleDebug assembleDebugAndroidTest testDebugUnitTest

suite_status=0
# Left installed, so the screenshots in the app's external files survive the run.
./gradlew connectedDebugAndroidTest -Pandroid.injected.androidTest.leaveApksInstalledAfterRun=true "$@" || suite_status=$?

pull_screenshots

exit "${suite_status}"
