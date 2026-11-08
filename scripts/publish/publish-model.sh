#!/usr/bin/env bash

# Splits the grading model into llama.cpp's split format and publishes the parts a repository lacks: `local`
# (build/repository/, which `repository_server` serves) or `cloudflare-r2`. The model file must have the SHA-256
# `shared::grading::GRADING_MODEL_SHA256` names; without --model-file, `course_builder` reads Ollama's copy.
#
# Usage:
#   ./scripts/publish/publish-model.sh <local|cloudflare-r2> [--model-file <path>]
#   ./scripts/publish/publish-model.sh local
#   ./scripts/publish/publish-model.sh cloudflare-r2 --model-file ~/Downloads/gemma4-12b-it-qat.gguf

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

exec cargo run --release --package course_builder -- publish-model "$@"
