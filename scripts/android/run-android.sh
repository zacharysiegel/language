#!/usr/bin/env bash

# Installs the debug build on the attached device or emulator, with the Mac's servers reachable from it, and starts it.
# ANDROID_SERIAL picks a device when several are attached.
#
# Usage:
#   ./scripts/android/run-android.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

source "${SCRIPT_DIR}/android-environment.sh"
source "${SCRIPT_DIR}/android-device.sh"

require_attached_device
reverse_local_server_ports

cd "${REPO_ROOT}/android"
./gradlew installDebug

adb shell am start -n ro.zach.language/.app.MainActivity
