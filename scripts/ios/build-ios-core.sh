#!/usr/bin/env bash

# Builds the Rust core for the iOS device and simulator, generates its Swift bindings, and packages both slices as
# target/uniffi/LanguageCore.xcframework, with the Swift sources in target/uniffi/swift/.
#
# Usage:
#   ./scripts/ios/build-ios-core.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly LIBRARY_NAME="liblanguage_core.a"
readonly DEVICE_TARGET="aarch64-apple-ios"
readonly SIMULATOR_TARGET="aarch64-apple-ios-sim"
readonly UNIFFI_DIRECTORY="${REPO_ROOT}/target/uniffi"
readonly HEADERS_DIRECTORY="${UNIFFI_DIRECTORY}/headers"
readonly SWIFT_DIRECTORY="${UNIFFI_DIRECTORY}/swift"
readonly XCFRAMEWORK_PATH="${UNIFFI_DIRECTORY}/LanguageCore.xcframework"
readonly STAGING_DIRECTORY="${UNIFFI_DIRECTORY}/staging"

# Matches the app's deployment target, so the C code built into the library targets the same iOS release.
export IPHONEOS_DEPLOYMENT_TARGET=18.0

cd "${REPO_ROOT}"

cargo build --package ffi --release --target "${DEVICE_TARGET}"
cargo build --package ffi --release --target "${SIMULATOR_TARGET}"

readonly DEVICE_LIBRARY="${REPO_ROOT}/target/${DEVICE_TARGET}/release/${LIBRARY_NAME}"
readonly SIMULATOR_LIBRARY="${REPO_ROOT}/target/${SIMULATOR_TARGET}/release/${LIBRARY_NAME}"

# The previous output stays in place until its replacement is complete.
replace_with_staged() {
    local generated_path="$1"
    local staged_path="${STAGING_DIRECTORY}/$(basename "${generated_path}")"

    rm -rf "${generated_path}"
    mv "${staged_path}" "${generated_path}"
}

readonly STAGED_HEADERS_DIRECTORY="${STAGING_DIRECTORY}/$(basename "${HEADERS_DIRECTORY}")"
readonly STAGED_SWIFT_DIRECTORY="${STAGING_DIRECTORY}/$(basename "${SWIFT_DIRECTORY}")"
readonly STAGED_XCFRAMEWORK_PATH="${STAGING_DIRECTORY}/$(basename "${XCFRAMEWORK_PATH}")"

rm -rf "${STAGING_DIRECTORY}"
mkdir -p "${STAGED_HEADERS_DIRECTORY}" "${STAGED_SWIFT_DIRECTORY}"

cargo run --quiet --package uniffi_bindgen_swift -- "${DEVICE_LIBRARY}" "${STAGED_SWIFT_DIRECTORY}" --swift-sources
cargo run --quiet --package uniffi_bindgen_swift -- "${DEVICE_LIBRARY}" "${STAGED_HEADERS_DIRECTORY}" --headers
cargo run --quiet --package uniffi_bindgen_swift -- "${DEVICE_LIBRARY}" "${STAGED_HEADERS_DIRECTORY}" --modulemap --module-name LanguageCoreFFI --modulemap-filename module.modulemap

xcodebuild -create-xcframework -library "${DEVICE_LIBRARY}" -headers "${STAGED_HEADERS_DIRECTORY}" -library "${SIMULATOR_LIBRARY}" -headers "${STAGED_HEADERS_DIRECTORY}" -output "${STAGED_XCFRAMEWORK_PATH}" > /dev/null

replace_with_staged "${HEADERS_DIRECTORY}"
replace_with_staged "${SWIFT_DIRECTORY}"
replace_with_staged "${XCFRAMEWORK_PATH}"
rmdir "${STAGING_DIRECTORY}"

echo "built ${XCFRAMEWORK_PATH}"
