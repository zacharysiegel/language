#!/usr/bin/env bash

# Validates the course data, compiles it into web/static/course/course.json and synthesizes any missing
# audio with the macOS Brazilian Portuguese voices. Audio that already exists is kept, so a re-run only
# speaks the clips that changed.
#
# Usage:
#   ./scripts/build/build-course.sh
#   ./scripts/build/build-course.sh --skip-audio

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- build "$@"
