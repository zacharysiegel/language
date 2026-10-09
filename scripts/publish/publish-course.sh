#!/usr/bin/env bash

# Publishes courses' newest bundles under build/artifacts/ to a repository: `local` (build/repository/, which
# `repository_server` serves), `cloudflare-r2`, or `dry` (logs each write without making it). Without
# --course, it publishes every course the repository's index lists; --include-unlisted-courses adds every built
# course the index does not list yet. Publishing to `local` or `cloudflare-r2` also refreshes those courses in the
# embedded bundle. Cloudflare R2 refuses a test course.
#
# Usage:
#   ./scripts/publish/publish-course.sh <local|cloudflare-r2|dry> [--course <course id>]... [--include-unlisted-courses]
#   ./scripts/publish/publish-course.sh local
#   ./scripts/publish/publish-course.sh cloudflare-r2 --course pt_br.en.foundations
#   ./scripts/publish/publish-course.sh local --include-unlisted-courses

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- publish "$@"
