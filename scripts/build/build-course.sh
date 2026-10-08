#!/usr/bin/env bash

# Validates the course data, synthesizes any missing audio with every speech engine, and writes a bundle of
# SQLite shards, recordings and a manifest under build/artifacts/. Recordings that already exist are kept, so
# a re-run only speaks the clips that changed. Test courses are left out unless --include-test-courses is given.
#
# Usage:
#   ./scripts/build/build-course.sh
#   ./scripts/build/build-course.sh --skip-audio
#   ./scripts/build/build-course.sh --engine kokoro
#   ./scripts/build/build-course.sh --include-test-courses

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- build "$@"
