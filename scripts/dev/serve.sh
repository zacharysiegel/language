#!/usr/bin/env bash

# Serves the app at http://127.0.0.1:3000 with rebuild on change, and the local repository at http://127.0.0.1:3002.
# Builds and publishes the courses, the test courses included, to the local repository first when it has no
# course index.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly COURSE_INDEX_PATH="${REPO_ROOT}/build/repository/course-index.json"
readonly REPOSITORY_SERVER_BINARY="${REPO_ROOT}/target/debug/repository_server"

if test ! -f "${COURSE_INDEX_PATH}"; then
    "${REPO_ROOT}/scripts/dev/build-local-repository.sh"
fi

cd "${REPO_ROOT}"

cargo build --package repository_server

"${REPOSITORY_SERVER_BINARY}" &
readonly REPOSITORY_SERVER_PID="$!"
trap 'kill "${REPOSITORY_SERVER_PID}" 2> /dev/null || true' EXIT

cd "${REPO_ROOT}/web"

cargo leptos watch
