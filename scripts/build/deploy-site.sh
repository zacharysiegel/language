#!/usr/bin/env bash

# Deploys the site tree in target/deploy-site to Cloudflare Workers Assets at https://language.zach.ro.
#
# Usage:
#   ./scripts/build/deploy-site.sh [--build] [--dry-run]
# e.g.
#   ./scripts/build/deploy-site.sh --build
#   ./scripts/build/deploy-site.sh --dry-run
#
# Deploys whatever is already in target/deploy-site; pass --build to produce it first. Either way the discovery
# document is checked before anything is uploaded.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly WEB_DIR="${REPO_ROOT}/web"
readonly DISCOVERY_PATH="${REPO_ROOT}/target/deploy-site/discovery"

RUN_BUILD=false
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --build)
            RUN_BUILD=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        *)
            echo "usage: $0 [--build] [--dry-run]" >&2
            exit 64
            ;;
    esac
done

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

if [[ "${RUN_BUILD}" == true ]]; then
    "${SCRIPT_DIR}/build-site.sh"
    printf '\n'
fi

if test ! -f "${DISCOVERY_PATH}"; then
    fail "${DISCOVERY_PATH} is missing; pass --build"
fi

repository_base_url="$(jq -r '.repository_base_url' "${DISCOVERY_PATH}")"

# A device cannot reach the local repository the committed document names.
if [[ "${repository_base_url}" != https://* ]]; then
    fail "the discovery document names ${repository_base_url}, not an https repository"
fi

printf 'deploying; [repository_base_url=%s]\n' "${repository_base_url}"
cd "${WEB_DIR}"

if [[ "${DRY_RUN}" == true ]]; then
    npx wrangler deploy --dry-run
else
    npx wrangler deploy
fi
