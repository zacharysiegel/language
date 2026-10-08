#!/usr/bin/env bash

# Copies the embedded bundle written by `course_builder build` into the iOS app's resources.
#
# Usage:
#   ./scripts/ios/sync-ios-embedded-bundle.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SOURCE_DIRECTORY="${REPO_ROOT}/web/static/embedded"
readonly DESTINATION_DIRECTORY="${REPO_ROOT}/ios/Language/Resources/embedded_bundle"

if test ! -f "${SOURCE_DIRECTORY}/manifest.json"; then
    echo "error: ${SOURCE_DIRECTORY} has no bundle; run ./scripts/build/build-course.sh first" >&2
    exit 1
fi

rm -rf "${DESTINATION_DIRECTORY}"
mkdir -p "${DESTINATION_DIRECTORY}"
cp -R "${SOURCE_DIRECTORY}/" "${DESTINATION_DIRECTORY}/"

echo "synced the embedded bundle into ${DESTINATION_DIRECTORY}"
