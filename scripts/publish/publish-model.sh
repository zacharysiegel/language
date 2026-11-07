#!/usr/bin/env bash

# Splits the grading model into llama.cpp's split format and publishes the parts a repository lacks: `local`
# (build/repository/, which `repository_server` serves) or `cloudflare-r2`. The model file must have the SHA-256
# `shared::grading::GRADING_MODEL_SHA256` names; Ollama stores gemma4:12b-it-qat's text GGUF under that hash.
#
# Usage:
#   ./scripts/publish/publish-model.sh <local|cloudflare-r2> [--model-file <path>]
#   ./scripts/publish/publish-model.sh local
#   ./scripts/publish/publish-model.sh cloudflare-r2 --model-file ~/Downloads/gemma4-12b-it-qat.gguf

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly OLLAMA_MODEL_FILE="${HOME}/.ollama/models/blobs/sha256-faff1a63667fac17ac5e777f47114688fcefea96e220e211aaa8d62c2c4561f1"

cd "${REPO_ROOT}"

target="${1:?usage: publish-model.sh <local|cloudflare-r2> [--model-file <path>]}"
shift

if test "$#" -eq 0; then
    set -- --model-file "${OLLAMA_MODEL_FILE}"
fi

exec cargo run --release --package course_builder -- publish-model "${target}" "$@"
