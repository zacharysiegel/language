#!/usr/bin/env bash

# Builds the Rust core as an arm64 Android shared object in target/uniffi/android/jniLibs/ and generates its Kotlin
# bindings into target/uniffi/kotlin/. The Gradle build runs it before every build.
#
# Usage:
#   ./scripts/android/build-android-core.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

source "${SCRIPT_DIR}/android-environment.sh"

readonly ANDROID_ABI="arm64-v8a"
readonly RUST_TARGET="aarch64-linux-android"
readonly LIBRARY_NAME="liblanguage_core.so"
readonly UNIFFI_DIRECTORY="${REPO_ROOT}/target/uniffi"
readonly JNI_LIBS_DIRECTORY="${UNIFFI_DIRECTORY}/android/jniLibs"
readonly KOTLIN_DIRECTORY="${UNIFFI_DIRECTORY}/kotlin"
MIN_SDK="$(android_gradle_property languageMinSdk)"
readonly MIN_SDK

cd "${REPO_ROOT}"

cargo ndk --target "${ANDROID_ABI}" --platform "${MIN_SDK}" --output-dir "${JNI_LIBS_DIRECTORY}" build --package ffi --release

readonly LIBRARY="${REPO_ROOT}/target/${RUST_TARGET}/release/${LIBRARY_NAME}"

rm -rf "${KOTLIN_DIRECTORY}"
mkdir -p "${KOTLIN_DIRECTORY}"

cargo run --quiet --package uniffi_bindgen_kotlin -- generate --library "${LIBRARY}" --language kotlin --no-format --out-dir "${KOTLIN_DIRECTORY}"

echo "built ${UNIFFI_DIRECTORY}/android"
