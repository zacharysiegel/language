#!/usr/bin/env bash

# Serves the app at http://127.0.0.1:3000 with rebuild on change, and the local repository at /repository.
# Builds and publishes the course locally first when the local repository has no published version.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly LATEST_MANIFEST_PATH="${REPO_ROOT}/build/repository/latest/manifest.json"

if test ! -f "${LATEST_MANIFEST_PATH}"; then
    "${REPO_ROOT}/scripts/build/build-course.sh"
    "${REPO_ROOT}/scripts/publish/publish-course.sh" local
fi

cd "${REPO_ROOT}/web"

exec cargo leptos watch
