#!/usr/bin/env bash

# Publishes the grading model, checked against the size and SHA-256 in shared/src/grading/grading_model.rs: `local`
# links the file into build/repository/, which `repository_server` serves; `cloudflare-r2` uploads it in llama.cpp's
# split format, and does nothing while R2 model hosting is turned off. Without --model-file, `course_builder` reads
# models/<its file name>, where ./scripts/dev/fetch-grading-model.sh downloads it.
#
# Usage:
#   ./scripts/publish/publish-model.sh <local|cloudflare-r2> [--model-file <path>]
#   ./scripts/publish/publish-model.sh local

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- publish-model "$@"
