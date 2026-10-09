#!/usr/bin/env bash

# Copies the embedded bundle, which `course_builder publish local` and `publish cloudflare-r2` write, into the iOS
# app's resources.
#
# Usage:
#   ./scripts/ios/sync-ios-embedded-bundle.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SOURCE_DIRECTORY="${REPO_ROOT}/web/static/embedded"
readonly DESTINATION_DIRECTORY="${REPO_ROOT}/ios/Language/Resources/embedded_bundle"

if test ! -f "${SOURCE_DIRECTORY}/course-index.json"; then
    echo "error: ${SOURCE_DIRECTORY} has no course index; run ./scripts/dev/build-local-repository.sh first" >&2
    exit 1
fi

rm -rf "${DESTINATION_DIRECTORY}"
mkdir -p "${DESTINATION_DIRECTORY}"
cp -R "${SOURCE_DIRECTORY}/" "${DESTINATION_DIRECTORY}/"

echo "synced the embedded bundle into ${DESTINATION_DIRECTORY}"
