#!/usr/bin/env bash

# Downloads the grading model from its Hugging Face repository into models/, resuming a partial download, and checks
# its size and SHA-256 against shared/src/grading/grading_model.rs. Does nothing when the checked file is there.
#
# Usage:
#   ./scripts/dev/fetch-grading-model.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly GRADING_MODEL_SOURCE_PATH="${REPO_ROOT}/shared/src/grading/grading_model.rs"
readonly MODELS_DIR="${REPO_ROOT}/models"

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

# Prints the value of a `pub const <name>` in grading_model.rs, without quotes.
function read_grading_model_constant {
    local constant_name="$1"
    local value
    value="$(grep -E "^pub const ${constant_name}: " "${GRADING_MODEL_SOURCE_PATH}" | sed -E 's/^[^=]*= *"?([^";]*)"?;$/\1/')"

    if test -z "${value}"; then
        fail "${GRADING_MODEL_SOURCE_PATH} defines no ${constant_name}"
    fi

    printf '%s\n' "${value}"
}

function file_size_bytes {
    stat -f '%z' "$1"
}

readonly FILE_NAME="$(read_grading_model_constant GRADING_MODEL_FILE_NAME)"
readonly SIZE_BYTES="$(read_grading_model_constant GRADING_MODEL_SIZE_BYTES | tr -d '_')"
readonly SHA256="$(read_grading_model_constant GRADING_MODEL_SHA256)"
readonly MODEL_URL="$(read_grading_model_constant GRADING_MODEL_HUGGING_FACE_URL)"
readonly MODEL_PATH="${MODELS_DIR}/${FILE_NAME}"
readonly PARTIAL_PATH="${MODEL_PATH}.partial"

if test -f "${MODEL_PATH}"; then
    printf 'the grading model is present; [path=%s]\n' "${MODEL_PATH}"
    exit 0
fi

mkdir -p "${MODELS_DIR}"

partial_size_bytes="$(file_size_bytes "${PARTIAL_PATH}" 2> /dev/null || printf '0')"

# A complete partial file is one whose check did not finish; asking the server to resume it would fail with 416.
if test "${partial_size_bytes}" != "${SIZE_BYTES}"; then
    printf 'downloading the grading model; [url=%s path=%s]\n' "${MODEL_URL}" "${PARTIAL_PATH}"
    curl --fail --location --continue-at - --output "${PARTIAL_PATH}" "${MODEL_URL}"
fi

downloaded_size_bytes="$(file_size_bytes "${PARTIAL_PATH}")"

if test "${downloaded_size_bytes}" != "${SIZE_BYTES}"; then
    fail "the download has ${downloaded_size_bytes} bytes, not ${SIZE_BYTES}; delete ${PARTIAL_PATH} and run again"
fi

printf 'checking the SHA-256; [path=%s]\n' "${PARTIAL_PATH}"
downloaded_sha256="$(shasum -a 256 "${PARTIAL_PATH}" | cut -d' ' -f1)"

if test "${downloaded_sha256}" != "${SHA256}"; then
    fail "the download has the SHA-256 ${downloaded_sha256}, not ${SHA256}; delete ${PARTIAL_PATH} and run again"
fi

mv "${PARTIAL_PATH}" "${MODEL_PATH}"
printf 'downloaded the grading model; [path=%s]\n' "${MODEL_PATH}"
