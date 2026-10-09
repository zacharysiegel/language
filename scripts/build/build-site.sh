#!/usr/bin/env bash

# Builds the deployable site tree into target/deploy-site: the discovery document, naming the production
# repository, and the header rules the edge applies to it.
#
# Usage:
#   ./scripts/build/build-site.sh
#   LANGUAGE_REPOSITORY_BASE_URL=<url> ./scripts/build/build-site.sh

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly STATIC_DIR="${REPO_ROOT}/web/static"
readonly SITE_DIR="${REPO_ROOT}/target/deploy-site"
readonly PRODUCTION_REPOSITORY_BASE_URL="https://repository.language.zach.ro"

# The committed discovery document names the local repository.
readonly REPOSITORY_BASE_URL="${LANGUAGE_REPOSITORY_BASE_URL:-${PRODUCTION_REPOSITORY_BASE_URL}}"

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

installed_jq="$(command -v jq || true)"

if test -z "${installed_jq}"; then
    fail "jq is not installed; run: brew install jq"
fi

rm -rf "${SITE_DIR}"
mkdir -p "${SITE_DIR}"

cp "${STATIC_DIR}/_headers" "${SITE_DIR}/_headers"

# Rewrites the one field rather than committing a second document, so the schema stays in one file.
jq --arg base_url "${REPOSITORY_BASE_URL}" '.repository_base_url = $base_url' "${STATIC_DIR}/discovery" > "${SITE_DIR}/discovery"

printf 'built %s; [repository_base_url=%s]\n' "${SITE_DIR}" "${REPOSITORY_BASE_URL}"
