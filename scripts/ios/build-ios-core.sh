#!/usr/bin/env bash

# Builds the Rust core for the iOS device and simulator, generates its Swift bindings, and packages both slices as
# target/uniffi/PortugueseCore.xcframework, with the Swift sources in target/uniffi/swift/.
#
# Usage:
#   ./scripts/ios/build-ios-core.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly LIBRARY_NAME="libportuguese_core.a"
readonly DEVICE_TARGET="aarch64-apple-ios"
readonly SIMULATOR_TARGET="aarch64-apple-ios-sim"
readonly UNIFFI_DIRECTORY="${REPO_ROOT}/target/uniffi"
readonly HEADERS_DIRECTORY="${UNIFFI_DIRECTORY}/headers"
readonly SWIFT_DIRECTORY="${UNIFFI_DIRECTORY}/swift"
readonly XCFRAMEWORK_PATH="${UNIFFI_DIRECTORY}/PortugueseCore.xcframework"

# Matches the app's deployment target, so the C code built into the library targets the same iOS release.
export IPHONEOS_DEPLOYMENT_TARGET=18.0

cd "${REPO_ROOT}"

cargo build --package ffi --release --target "${DEVICE_TARGET}"
cargo build --package ffi --release --target "${SIMULATOR_TARGET}"

readonly DEVICE_LIBRARY="${REPO_ROOT}/target/${DEVICE_TARGET}/release/${LIBRARY_NAME}"
readonly SIMULATOR_LIBRARY="${REPO_ROOT}/target/${SIMULATOR_TARGET}/release/${LIBRARY_NAME}"

rm -rf "${HEADERS_DIRECTORY}" "${SWIFT_DIRECTORY}" "${XCFRAMEWORK_PATH}"
mkdir -p "${HEADERS_DIRECTORY}" "${SWIFT_DIRECTORY}"

cargo run --quiet --package uniffi_bindgen_swift -- "${DEVICE_LIBRARY}" "${SWIFT_DIRECTORY}" --swift-sources
cargo run --quiet --package uniffi_bindgen_swift -- "${DEVICE_LIBRARY}" "${HEADERS_DIRECTORY}" --headers
cargo run --quiet --package uniffi_bindgen_swift -- "${DEVICE_LIBRARY}" "${HEADERS_DIRECTORY}" --modulemap --module-name PortugueseCoreFFI --modulemap-filename module.modulemap

xcodebuild -create-xcframework -library "${DEVICE_LIBRARY}" -headers "${HEADERS_DIRECTORY}" -library "${SIMULATOR_LIBRARY}" -headers "${HEADERS_DIRECTORY}" -output "${XCFRAMEWORK_PATH}" > /dev/null

echo "built ${XCFRAMEWORK_PATH}"
