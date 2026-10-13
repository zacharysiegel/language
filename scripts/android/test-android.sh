#!/usr/bin/env bash

# Runs the JVM unit tests and the instrumented suite on the attached device or emulator, then copies the suite's
# screenshots to /tmp/language-android-screenshots. ANDROID_SERIAL picks a device when several are attached; extra
# arguments go to Gradle, such as
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

require_attached_device
reverse_local_server_ports

cd "${REPO_ROOT}/android"
# Left installed, so the screenshots in the app's external files survive the run.
./gradlew testDebugUnitTest connectedDebugAndroidTest -Pandroid.injected.androidTest.leaveApksInstalledAfterRun=true "$@"

rm -rf "${SCREENSHOT_DIRECTORY}"
mkdir -p "${SCREENSHOT_DIRECTORY}"

device_screenshots="$(adb shell ls "${DEVICE_SCREENSHOT_DIRECTORY}" 2> /dev/null || true)"

if test -z "${device_screenshots}"; then
    echo "the suite saved no screenshots"
    exit 0
fi

adb pull "${DEVICE_SCREENSHOT_DIRECTORY}/." "${SCREENSHOT_DIRECTORY}" > /dev/null

echo "screenshots in ${SCREENSHOT_DIRECTORY}"
