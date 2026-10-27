#!/usr/bin/env bash

# Runs the iOS tests, all of them or the ones that match, several at a time on cloned simulators.
#
# Usage:
#   ./scripts/ios/run-tests.sh [--matching <word>]... [--only <test id>]... [--parallel <workers>] [--simulator <name>] [--list]
# e.g.
#   ./scripts/ios/run-tests.sh
#   ./scripts/ios/run-tests.sh --matching glossary --matching rail
#   ./scripts/ios/run-tests.sh --only LanguageTests/GlossaryRailTests --parallel 1
#
# --matching selects every test whose Target/Class/method name contains the word, ignoring case; repeat it to
# add words. --only names a test as Target/Class or Target/Class/method. --list prints the selection without
# running it. Test names carry their feature, so no list of tests needs keeping.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly DERIVED_DATA_DIRECTORY="${REPO_ROOT}/build/ios-test"
readonly TEST_LOG_PATH="/tmp/language-ios-tests.log"
readonly SCHEME="Language"
readonly DEFAULT_WORKER_COUNT=4

SIMULATOR_NAME="iPhone 17"
WORKER_COUNT="${DEFAULT_WORKER_COUNT}"
LIST_ONLY=false
MATCHING_WORDS=()
EXPLICIT_TESTS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --matching)
            MATCHING_WORDS+=("${2:?--matching needs a word}")
            shift 2
            ;;
        --only)
            EXPLICIT_TESTS+=("${2:?--only needs a test id}")
            shift 2
            ;;
        --parallel)
            WORKER_COUNT="${2:?--parallel needs a number of workers}"
            shift 2
            ;;
        --simulator)
            SIMULATOR_NAME="${2:?--simulator needs a simulator name}"
            shift 2
            ;;
        --list)
            LIST_ONLY=true
            shift
            ;;
        *)
            sed -n '3,16p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//' >&2
            exit 64
            ;;
    esac
done

function fail {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

# Every test the project defines, as Target/Class/method, one per line.
function list_defined_tests {
    local target
    local source_path
    local class_name

    for target in LanguageTests LanguageUITests; do
        for source_path in "${REPO_ROOT}/ios/${target}"/*.swift; do
            class_name="$(grep -oE 'class [A-Za-z0-9_]+: XCTestCase' "${source_path}" | head -1 | sed -E 's/class ([A-Za-z0-9_]+):.*/\1/')"

            if test -z "${class_name}"; then
                continue
            fi

            grep -oE 'func test[A-Za-z0-9_]*' "${source_path}" | sed -E "s|func (.*)|${target}/${class_name}/\\1|"
        done
    done
}

selected_tests=""

if [[ ${#EXPLICIT_TESTS[@]} -gt 0 ]]; then
    selected_tests="$(printf '%s\n' "${EXPLICIT_TESTS[@]}")"
fi

if [[ ${#MATCHING_WORDS[@]} -gt 0 ]]; then
    defined_tests="$(list_defined_tests)"
    matched_tests=""

    for matching_word in "${MATCHING_WORDS[@]}"; do
        matched_for_word="$(printf '%s\n' "${defined_tests}" | grep -i -F -- "${matching_word}" || true)"
        matched_tests="$(printf '%s\n%s\n' "${matched_tests}" "${matched_for_word}")"
    done

    selected_tests="$(printf '%s\n%s\n' "${selected_tests}" "${matched_tests}" | awk 'NF' | sort -u)"

    if test -z "${selected_tests}"; then
        fail "no test name contains: ${MATCHING_WORDS[*]}"
    fi
fi

if test -z "${selected_tests}"; then
    printf 'running every test\n'
else
    printf 'running %s test(s):\n%s\n' "$(printf '%s\n' "${selected_tests}" | wc -l | tr -d ' ')" "$(printf '%s\n' "${selected_tests}" | sed 's/^/  /')"
fi

if [[ "${LIST_ONLY}" == true ]]; then
    exit 0
fi

only_testing_arguments=()

if test -n "${selected_tests}"; then
    while IFS= read -r selected_test; do
        only_testing_arguments+=("-only-testing:${selected_test}")
    done <<< "${selected_tests}"
fi

parallel_arguments=(-parallel-testing-enabled NO)

if test "${WORKER_COUNT}" -gt 1; then
    parallel_arguments=(-parallel-testing-enabled YES -parallel-testing-worker-count "${WORKER_COUNT}" -maximum-parallel-testing-workers "${WORKER_COUNT}")
fi

installed_xcodegen="$(command -v xcodegen || true)"

if test -z "${installed_xcodegen}"; then
    fail "xcodegen is not installed; run: brew install xcodegen"
fi

cd "${REPO_ROOT}/ios"
xcodegen generate > /dev/null

# A fixed derived data directory keeps the build products, so a repeat run rebuilds only what changed.
test_status=0

xcodebuild test \
    -project Language.xcodeproj \
    -scheme "${SCHEME}" \
    -destination "platform=iOS Simulator,name=${SIMULATOR_NAME}" \
    -derivedDataPath "${DERIVED_DATA_DIRECTORY}" \
    "${parallel_arguments[@]}" \
    ${only_testing_arguments[@]+"${only_testing_arguments[@]}"} \
    > "${TEST_LOG_PATH}" 2>&1 || test_status=$?

grep -E "Test Case .* (passed|failed)|: error: |Executed [0-9]+ tests?|TEST (SUCCEEDED|FAILED)" "${TEST_LOG_PATH}" | sed -E 's/^.*Test Case .(.*). (passed|failed).*$/\2 \1/' | sort -u || true
printf 'full log: %s\n' "${TEST_LOG_PATH}"

exit "${test_status}"
