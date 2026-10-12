#!/usr/bin/env bash

# Checks that a site tree is safe to deploy, and says what is wrong when it is not.
#
# Usage:
#   ./scripts/build/verify-site-tree.sh [<site dir>]

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SITE_DIR="${1:-${REPO_ROOT}/target/deploy-site}"
readonly DOCUMENT_PATH="${SITE_DIR}/index.html"
readonly DISCOVERY_PATH="${SITE_DIR}/discovery"
readonly HEADERS_PATH="${SITE_DIR}/_headers"
readonly EMBEDDED_COURSE_INDEX_PATH="${SITE_DIR}/embedded/course-index.json"

# STATIC_SHELL_ELEMENT_ID in web/src/app.rs.
readonly STATIC_SHELL_MARKER='id="static-shell"'

# cargo-leptos inserts a 22-character base64url digest between the stem and the extension.
readonly HASHED_NAME_PATTERN='\.[A-Za-z0-9_-]{22}\.[A-Za-z0-9]+$'

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

installed_jq="$(command -v jq || true)"

if test -z "${installed_jq}"; then
    fail "jq is required to verify a site tree; run: brew install jq"
fi

if test ! -d "${SITE_DIR}"; then
    fail "${SITE_DIR} does not exist; run ./scripts/build/build-site.sh"
fi

if test ! -f "${DOCUMENT_PATH}"; then
    fail "${DOCUMENT_PATH} is missing, so the deploy would serve no app; run ./scripts/build/build-site.sh"
fi

# The client renders from scratch only over the static shell; over any other document it hydrates.
static_shell_marker_count="$(grep -F -c "${STATIC_SHELL_MARKER}" "${DOCUMENT_PATH}" || true)"

if test "${static_shell_marker_count}" -eq 0; then
    fail "${DOCUMENT_PATH} is not the static shell; run ./scripts/build/build-site.sh"
fi

if test ! -d "${SITE_DIR}/pkg"; then
    fail "${SITE_DIR}/pkg is missing, so the shell would load no client; run ./scripts/build/build-site.sh"
fi

reference_count=0
wasm_path=""

while IFS= read -r referenced_path; do
    # A reference the tree cannot satisfy deploys as a 404 for the client or the stylesheet: a page stuck loading.
    if test ! -f "${SITE_DIR}${referenced_path}"; then
        fail "the shell references ${referenced_path}, which is not in ${SITE_DIR}"
    fi

    # _headers serves /pkg/* as immutable, which a stable name would pin in caches for a year.
    hashed_name="$(printf '%s' "${referenced_path}" | grep -E "${HASHED_NAME_PATTERN}" || true)"

    if test -z "${hashed_name}"; then
        fail "the shell references ${referenced_path}, whose name carries no content hash; build with ./scripts/build/build-site.sh"
    fi

    if [[ "${referenced_path}" == *.wasm ]]; then
        wasm_path="${SITE_DIR}${referenced_path}"
    fi

    reference_count=$((reference_count + 1))
done < <(grep -o '/pkg/[A-Za-z0-9._-]*' "${DOCUMENT_PATH}" | sort -u)

if test "${reference_count}" -eq 0; then
    fail "the shell references no /pkg/ assets, so it would load nothing"
fi

if test -z "${wasm_path}"; then
    fail "the shell references no .wasm module"
fi

if test ! -f "${HEADERS_PATH}"; then
    fail "${HEADERS_PATH} is missing, so /pkg/* would be revalidated and /discovery served with no Content-Type"
fi

if test ! -f "${EMBEDDED_COURSE_INDEX_PATH}"; then
    fail "${EMBEDDED_COURSE_INDEX_PATH} is missing, so a client that reaches no repository lists no courses"
fi

# The page head links these; without them the browser falls back to another site's icon.
for icon_file in favicon-32.png apple-touch-icon.png favicon.ico; do
    if test ! -f "${SITE_DIR}/${icon_file}"; then
        fail "${SITE_DIR}/${icon_file} is missing"
    fi
done

if test ! -f "${DISCOVERY_PATH}"; then
    fail "${DISCOVERY_PATH} is missing, so a client has nothing to resolve the repository from"
fi

repository_base_url="$(jq -r '.repository_base_url' "${DISCOVERY_PATH}")"

# A device cannot reach the local repository the committed document names.
if [[ "${repository_base_url}" != https://* ]]; then
    fail "the discovery document names ${repository_base_url}, not an https repository"
fi

# The client falls back to the repository compiled into it, and serves the embedded bundle's recordings from there.
compiled_in_count="$(grep -F -a -c "${repository_base_url}" "${wasm_path}" || true)"

if test "${compiled_in_count}" -eq 0; then
    fail "${wasm_path} was not compiled with the repository the discovery document names; [repository_base_url=${repository_base_url}]"
fi

printf '%s: static shell present, %d referenced assets present and content-hashed\n' "${SITE_DIR}" "${reference_count}"
printf '%s: discovery and the client both name %s\n' "${SITE_DIR}" "${repository_base_url}"
