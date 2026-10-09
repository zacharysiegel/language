#!/usr/bin/env bash

# Checks, before building an app for release, that the embedded bundle holds exactly the courses Cloudflare R2's
# course index lists, each at the publication R2's latest pointer names. Reads R2 only.
#
# Usage:
#   ./scripts/release/check-embedded-bundle.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- check-embedded-bundle cloudflare-r2
