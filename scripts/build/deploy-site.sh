#!/usr/bin/env bash

# Deploys the site tree in target/deploy-site to Cloudflare Workers Assets at https://language.zach.ro.
#
# Usage:
#   ./scripts/build/deploy-site.sh [--build] [--dry-run]
# e.g.
#   ./scripts/build/deploy-site.sh --build
#   ./scripts/build/deploy-site.sh --dry-run
#
# Deploys whatever is already in target/deploy-site; pass --build to produce it first. Either way the tree is
# verified before anything is uploaded.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly WEB_DIR="${REPO_ROOT}/web"
readonly SITE_DIR="${REPO_ROOT}/target/deploy-site"

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

if [[ "${RUN_BUILD}" == true ]]; then
    "${SCRIPT_DIR}/build-site.sh"
    printf '\n'
fi

"${SCRIPT_DIR}/verify-site-tree.sh" "${SITE_DIR}"

printf '\ndeploying %s\n' "${SITE_DIR}"
cd "${WEB_DIR}"

if [[ "${DRY_RUN}" == true ]]; then
    npx wrangler deploy --dry-run
else
    npx wrangler deploy
fi
