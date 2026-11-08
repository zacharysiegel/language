#!/usr/bin/env bash

# Writes wllama's browser files into web/static/wllama/, which the app serves at /wllama/, with the fetch-wllama.sh
# that ships in the cross-llama crate the web client depends on. Does nothing when the directory already holds the
# wllama version that script pins.
#
# Usage:
#   ./scripts/build/prepare-wllama.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly WLLAMA_DIR="${REPO_ROOT}/web/static/wllama"
readonly VERSION_PATH="${WLLAMA_DIR}/VERSION"

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

installed_jq="$(command -v jq || true)"

if test -z "${installed_jq}"; then
    fail "jq is not installed; run: brew install jq"
fi

cd "${REPO_ROOT}"

# cross-llama is a dependency of the client build only.
cross_llama_manifest_path="$(cargo metadata --format-version 1 --features web/hydrate --filter-platform wasm32-unknown-unknown | jq -r '.packages[] | select(.name == "cross-llama") | .manifest_path')"

if test -z "${cross_llama_manifest_path}"; then
    fail "cargo metadata lists no cross-llama package for the web client"
fi

readonly FETCH_SCRIPT_PATH="$(dirname "${cross_llama_manifest_path}")/scripts/fetch-wllama.sh"
wanted_version="$(grep -E '^readonly WLLAMA_VERSION=' "${FETCH_SCRIPT_PATH}" | cut -d'"' -f2)"

if test -z "${wanted_version}"; then
    fail "${FETCH_SCRIPT_PATH} names no wllama version"
fi

fetched_version="$(cat "${VERSION_PATH}" 2> /dev/null || true)"

if test "${fetched_version}" = "${wanted_version}"; then
    exit 0
fi

printf 'fetching wllama; [version=%s directory=%s]\n' "${wanted_version}" "${WLLAMA_DIR}"
rm -rf "${WLLAMA_DIR}"
bash "${FETCH_SCRIPT_PATH}" "${WLLAMA_DIR}"
printf '%s\n' "${wanted_version}" > "${VERSION_PATH}"
