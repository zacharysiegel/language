#!/usr/bin/env bash

# Builds every course, the test courses included, and publishes the bundle to the local repository
# (build/repository/), which the web server serves at /repository. Arguments are passed to the build.
#
# Usage:
#   ./scripts/dev/build-local-repository.sh
#   ./scripts/dev/build-local-repository.sh --skip-audio
#   ./scripts/dev/build-local-repository.sh --engine macos-say

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

"${REPO_ROOT}/scripts/build/build-course.sh" --include-test-courses "$@"
"${REPO_ROOT}/scripts/publish/publish-course.sh" local
