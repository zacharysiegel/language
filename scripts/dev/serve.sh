#!/usr/bin/env bash

# Serves the app at http://127.0.0.1:3000 with rebuild on change. The compiled course and its audio must
# exist first; this builds them when the compiled course is missing.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly COMPILED_COURSE_PATH="${REPO_ROOT}/web/static/course/course.json"

if test ! -f "${COMPILED_COURSE_PATH}"; then
    "${REPO_ROOT}/scripts/build/build-course.sh"
fi

cd "${REPO_ROOT}/web"

exec cargo leptos watch
