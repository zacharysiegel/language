#!/usr/bin/env bash

# Removes a course from a repository: `local` (build/repository/) or `cloudflare-r2`. The course leaves the
# repository's index first, then its publications are deleted and it is dropped from the embedded bundle. Locally,
# the recordings no other publication names are deleted too; on Cloudflare R2 they stay until
# `course_builder remove-unused-recordings cloudflare-r2`.
#
# Usage:
#   ./scripts/publish/remove-course.sh <local|cloudflare-r2> --course <course id>

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- remove-course "$@"
