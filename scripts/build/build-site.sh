#!/usr/bin/env bash

# Builds the deployable site tree into target/deploy-site: a release build of the app, the static shell every
# route serves, the static assets, the header rules, and a discovery document naming the production repository.
#
# Usage:
#   ./scripts/build/build-site.sh
#   LANGUAGE_REPOSITORY_BASE_URL=<url> ./scripts/build/build-site.sh
#
# The release build writes its own site root rather than target/site, which `cargo leptos watch` serves, so it
# can run while the dev server does.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly WEB_DIR="${REPO_ROOT}/web"
readonly COMMITTED_DISCOVERY_PATH="${WEB_DIR}/static/discovery"
readonly EMBEDDED_COURSE_INDEX_PATH="${WEB_DIR}/static/embedded/course-index.json"
readonly SITE_ROOT="target/deploy-site"
readonly SITE_DIR="${REPO_ROOT}/${SITE_ROOT}"
readonly SERVER_BINARY="${REPO_ROOT}/target/release/web"
readonly PRODUCTION_REPOSITORY_BASE_URL="https://repository.language.zach.ro"

# cargo-leptos's own variables, which the server binary's prerender reads too, so both agree on the tree and its
# file names. cargo-leptos re-hashes only on a full build, so hashed names are a release setting.
export LEPTOS_SITE_ROOT="${SITE_ROOT}"
export LEPTOS_HASH_FILES=true

# The committed discovery document names the local repository. This one is compiled into the client as the static
# repository base and written into the deployed discovery document.
export LANGUAGE_REPOSITORY_BASE_URL="${LANGUAGE_REPOSITORY_BASE_URL:-${PRODUCTION_REPOSITORY_BASE_URL}}"

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

installed_cargo_leptos="$(command -v cargo-leptos || true)"

if test -z "${installed_cargo_leptos}"; then
    fail "cargo-leptos is not installed; run: cargo install cargo-leptos --locked"
fi

installed_jq="$(command -v jq || true)"

if test -z "${installed_jq}"; then
    fail "jq is not installed; run: brew install jq"
fi

# The build copies web/static into the tree, and the embedded bundle there is generated rather than committed.
if test ! -f "${EMBEDDED_COURSE_INDEX_PATH}"; then
    fail "${EMBEDDED_COURSE_INDEX_PATH} is missing; publish the courses first"
fi

rm -rf "${SITE_DIR}"

printf 'building the release site; [site_dir=%s repository_base_url=%s]\n' "${SITE_DIR}" "${LANGUAGE_REPOSITORY_BASE_URL}"
cd "${WEB_DIR}"
cargo leptos build --release

if test ! -x "${SERVER_BINARY}"; then
    fail "the release build produced no server binary at ${SERVER_BINARY}"
fi

# Run in place: the render reads the content-hash file from the directory holding the binary.
printf '\nwriting the static shell\n'
"${SERVER_BINARY}" prerender

# Rewrites the one field rather than committing a second document, so the schema stays in one file.
jq --arg base_url "${LANGUAGE_REPOSITORY_BASE_URL}" '.repository_base_url = $base_url' "${COMMITTED_DISCOVERY_PATH}" > "${SITE_DIR}/discovery"

printf '\n'
"${SCRIPT_DIR}/verify-site-tree.sh" "${SITE_DIR}"
