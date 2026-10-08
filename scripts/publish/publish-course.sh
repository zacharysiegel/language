#!/usr/bin/env bash

# Publishes the newest bundle under build/artifacts/ to a repository: `local` (build/repository/, which the
# web server serves at /repository), `cloudflare-r2`, or `dry` (logs each upload without making it). Cloudflare R2
# refuses a bundle that holds a test course.
#
# Usage:
#   ./scripts/publish/publish-course.sh local
#   ./scripts/publish/publish-course.sh cloudflare-r2
#   ./scripts/publish/publish-course.sh dry

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- publish "$@"
