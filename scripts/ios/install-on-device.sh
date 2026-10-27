#!/usr/bin/env bash

# Builds the app in release configuration, signs it with the development team, installs it on a paired iPhone and
# launches it. A free developer account's provisioning profile lapses after 7 days; running this renews it.
#
# Usage:
#   ./scripts/ios/install-on-device.sh [--device <name or identifier>] [--no-launch]
# e.g.
#   ./scripts/ios/install-on-device.sh
#   ./scripts/ios/install-on-device.sh --device "Zachary iPhone 17"
#
# Without --device, the one paired iPhone is used; with several, the script lists them and stops. The phone must be
# unlocked, and on the same network or connected by cable.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly IOS_DIRECTORY="${REPO_ROOT}/ios"
readonly DERIVED_DATA_DIRECTORY="${REPO_ROOT}/build/ios-device"
readonly DEVICE_LIST_PATH="/tmp/language-ios-devices.json"
readonly BUNDLE_IDENTIFIER="ro.zach.language"
readonly SCHEME="Language"
readonly CONFIGURATION="Release"

REQUESTED_DEVICE=""
LAUNCH_AFTER_INSTALL=true

while [[ $# -gt 0 ]]; do
    case "$1" in
        --device)
            REQUESTED_DEVICE="${2:?--device needs a name or identifier}"
            shift 2
            ;;
        --no-launch)
            LAUNCH_AFTER_INSTALL=false
            shift
            ;;
        *)
            echo "usage: $0 [--device <name or identifier>] [--no-launch]" >&2
            exit 64
            ;;
    esac
done

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

installed_jq="$(command -v jq || true)"

if test -z "${installed_jq}"; then
    fail "jq is not installed; run: brew install jq"
fi

installed_xcodegen="$(command -v xcodegen || true)"

if test -z "${installed_xcodegen}"; then
    fail "xcodegen is not installed; run: brew install xcodegen"
fi

xcrun devicectl list devices --json-output "${DEVICE_LIST_PATH}" > /dev/null 2>&1 || fail "could not list devices; is Xcode installed?"

paired_iphones="$(jq -r '.result.devices[] | select(.hardwareProperties.platform == "iOS" and .connectionProperties.pairingState == "paired") | [.identifier, .deviceProperties.name, .hardwareProperties.udid] | @tsv' "${DEVICE_LIST_PATH}")"

if test -z "${paired_iphones}"; then
    fail "no paired iPhone; connect the phone once by cable, unlock it and trust this Mac"
fi

if test -n "${REQUESTED_DEVICE}"; then
    chosen_iphone="$(printf '%s\n' "${paired_iphones}" | awk -F '\t' -v wanted="${REQUESTED_DEVICE}" '$1 == wanted || $2 == wanted || $3 == wanted' | head -1)"
else
    paired_iphone_count="$(printf '%s\n' "${paired_iphones}" | wc -l | tr -d ' ')"

    if test "${paired_iphone_count}" -gt 1; then
        printf '%s\n' "${paired_iphones}" | awk -F '\t' '{ printf "  %s  %s\n", $2, $1 }' >&2
        fail "several paired iPhones; pass --device with a name or identifier"
    fi

    chosen_iphone="${paired_iphones}"
fi

if test -z "${chosen_iphone}"; then
    fail "no paired iPhone matches '${REQUESTED_DEVICE}'"
fi

device_identifier="$(printf '%s' "${chosen_iphone}" | cut -f1)"
device_name="$(printf '%s' "${chosen_iphone}" | cut -f2)"
device_udid="$(printf '%s' "${chosen_iphone}" | cut -f3)"

printf 'building for %s\n' "${device_name}"
cd "${IOS_DIRECTORY}"
xcodegen generate > /dev/null

# -allowProvisioningUpdates lets xcodebuild renew the lapsed profile with the account Xcode is signed into.
xcodebuild build \
    -project Language.xcodeproj \
    -scheme "${SCHEME}" \
    -configuration "${CONFIGURATION}" \
    -destination "platform=iOS,id=${device_udid}" \
    -derivedDataPath "${DERIVED_DATA_DIRECTORY}" \
    -allowProvisioningUpdates \
    -quiet

app_path="${DERIVED_DATA_DIRECTORY}/Build/Products/${CONFIGURATION}-iphoneos/Language.app"

if test ! -d "${app_path}"; then
    fail "the build produced no app at ${app_path}"
fi

printf 'installing on %s\n' "${device_name}"
xcrun devicectl device install app --device "${device_identifier}" "${app_path}" > /dev/null

printf 'installed %s on %s\n' "${BUNDLE_IDENTIFIER}" "${device_name}"

if [[ "${LAUNCH_AFTER_INSTALL}" == true ]]; then
    launch_status=0
    launch_output="$(xcrun devicectl device process launch --device "${device_identifier}" --terminate-existing "${BUNDLE_IDENTIFIER}" 2>&1)" || launch_status=$?

    if test "${launch_status}" -ne 0; then
        launch_reason="$(printf '%s\n' "${launch_output}" | grep -m1 'NSLocalizedFailureReason' || true)"
        printf 'warning: the app is installed but did not launch;%s\n' "${launch_reason#*=}" >&2
        printf 'if the phone does not trust the developer certificate: Settings, General, VPN & Device Management, the Apple Development entry, Trust\n' >&2
    fi
fi
