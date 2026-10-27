#!/usr/bin/env bash

# Runs only the iOS tests that the changed files can affect, as mapped in ios/test-selection.txt. A changed file the
# map does not cover runs every test.
#
# Usage:
#   ./scripts/ios/test-changed.sh [--base <git ref>] [--simulator <name>] [--list]
#   ./scripts/ios/test-changed.sh --only <test id>... [--simulator <name>]
#   ./scripts/ios/test-changed.sh --all
#   ./scripts/ios/test-changed.sh --check
# e.g.
#   ./scripts/ios/test-changed.sh                       tests for the uncommitted changes
#   ./scripts/ios/test-changed.sh --base origin/main    tests for everything since origin/main
#   ./scripts/ios/test-changed.sh --only LanguageTests/GlossaryRailTests
#
# A test id is Target/Class or Target/Class/method. --list prints the selection without running it, and --check
# reports tests missing from the map and map entries that name no test.

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SELECTION_MAP_PATH="${REPO_ROOT}/ios/test-selection.txt"
readonly DERIVED_DATA_DIRECTORY="${REPO_ROOT}/build/ios-test"
readonly SCHEME="Language"

BASE_REFERENCE="HEAD"
SIMULATOR_NAME="iPhone 17"
LIST_ONLY=false
RUN_EVERYTHING=false
CHECK_MAP=false
EXPLICIT_TESTS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --base)
            BASE_REFERENCE="${2:?--base needs a git ref}"
            shift 2
            ;;
        --simulator)
            SIMULATOR_NAME="${2:?--simulator needs a simulator name}"
            shift 2
            ;;
        --only)
            EXPLICIT_TESTS+=("${2:?--only needs a test id}")
            shift 2
            ;;
        --list)
            LIST_ONLY=true
            shift
            ;;
        --all)
            RUN_EVERYTHING=true
            shift
            ;;
        --check)
            CHECK_MAP=true
            shift
            ;;
        *)
            sed -n '3,17p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//' >&2
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

# Every test id the map names, one per line, without the wildcard and the no-test marker.
function list_mapped_tests {
    grep -vE '^[[:space:]]*(#|$)' "${SELECTION_MAP_PATH}" | sed -E 's/^[^:]*://' | tr ' ' '\n' | awk 'NF && $0 != "*" && $0 != "-"' | sort -u
}

if [[ "${CHECK_MAP}" == true ]]; then
    defined_tests="$(list_defined_tests | sort -u)"
    mapped_tests="$(list_mapped_tests)"
    problem_count=0

    while IFS= read -r defined_test; do
        covered="$(printf '%s\n' "${mapped_tests}" | awk -v wanted="${defined_test}" '{ if (wanted == $0 || index(wanted, $0 "/") == 1) found = 1 } END { if (found) print "yes" }')"

        if test -z "${covered}"; then
            printf 'not in the map: %s\n' "${defined_test}" >&2
            problem_count=$((problem_count + 1))
        fi
    done <<< "${defined_tests}"

    while IFS= read -r mapped_test; do
        exists="$(printf '%s\n' "${defined_tests}" | awk -v wanted="${mapped_test}" '{ if (wanted == $0 || index($0, wanted "/") == 1) found = 1 } END { if (found) print "yes" }')"

        if test -z "${exists}"; then
            printf 'names no test: %s\n' "${mapped_test}" >&2
            problem_count=$((problem_count + 1))
        fi
    done <<< "${mapped_tests}"

    if test "${problem_count}" -ne 0; then
        fail "${problem_count} problem(s) in ios/test-selection.txt"
    fi

    printf 'ios/test-selection.txt covers every test\n'
    exit 0
fi

selected_tests=""
run_every_test=false

if [[ "${RUN_EVERYTHING}" == true ]]; then
    run_every_test=true
elif [[ ${#EXPLICIT_TESTS[@]} -gt 0 ]]; then
    selected_tests="$(printf '%s\n' "${EXPLICIT_TESTS[@]}")"
else
    cd "${REPO_ROOT}"
    tracked_changes="$(git diff --name-only "${BASE_REFERENCE}")"
    untracked_files="$(git ls-files --others --exclude-standard)"
    changed_files="$(printf '%s\n%s\n' "${tracked_changes}" "${untracked_files}" | grep -v '^$' | sort -u || true)"

    if test -z "${changed_files}"; then
        printf 'no changed files since %s; nothing to test\n' "${BASE_REFERENCE}"
        exit 0
    fi

    selection="$(printf '%s\n' "${changed_files}" | awk -F ':' '
        FNR == NR {
            if ($0 ~ /^[[:space:]]*#/ || $0 ~ /^[[:space:]]*$/) next
            prefix = $1
            gsub(/[[:space:]]/, "", prefix)
            tests = $0
            sub(/^[^:]*:/, "", tests)
            rule_count++
            rule_prefix[rule_count] = prefix
            rule_tests[rule_count] = tests
            next
        }
        {
            matched = 0
            for (rule = 1; rule <= rule_count; rule++) {
                if (index($0, rule_prefix[rule]) == 1) {
                    matched = 1
                    part_count = split(rule_tests[rule], parts, " ")
                    for (part = 1; part <= part_count; part++) print "TEST " parts[part]
                }
            }
            if (!matched) print "UNMAPPED " $0
        }
    ' "${SELECTION_MAP_PATH}" - )"

    unmapped_files="$(printf '%s\n' "${selection}" | sed -n 's/^UNMAPPED //p')"
    wildcard_rules="$(printf '%s\n' "${selection}" | grep -c '^TEST \*$' || true)"

    if test -n "${unmapped_files}"; then
        printf 'not in ios/test-selection.txt, so every test runs:\n%s\n' "${unmapped_files}" >&2
        run_every_test=true
    elif test "${wildcard_rules}" -gt 0; then
        printf 'a changed file affects every test\n'
        run_every_test=true
    else
        selected_tests="$(printf '%s\n' "${selection}" | sed -n 's/^TEST //p' | grep -vx -- '-' | sort -u || true)"
    fi
fi

if [[ "${run_every_test}" == false && -z "${selected_tests}" ]]; then
    printf 'the changes need no iOS tests\n'
    exit 0
fi

if [[ "${run_every_test}" == true ]]; then
    printf 'running every test\n'
else
    printf 'running %s test(s):\n%s\n' "$(printf '%s\n' "${selected_tests}" | wc -l | tr -d ' ')" "$(printf '%s\n' "${selected_tests}" | sed 's/^/  /')"
fi

if [[ "${LIST_ONLY}" == true ]]; then
    exit 0
fi

only_testing_arguments=()

if [[ "${run_every_test}" == false ]]; then
    while IFS= read -r selected_test; do
        only_testing_arguments+=("-only-testing:${selected_test}")
    done <<< "${selected_tests}"
fi

installed_xcodegen="$(command -v xcodegen || true)"

if test -z "${installed_xcodegen}"; then
    fail "xcodegen is not installed; run: brew install xcodegen"
fi

cd "${REPO_ROOT}/ios"
xcodegen generate > /dev/null

# A fixed derived data directory keeps the build products, so a repeat run rebuilds only what changed.
readonly TEST_LOG_PATH="/tmp/language-ios-test-changed.log"
test_status=0

xcodebuild test \
    -project Language.xcodeproj \
    -scheme "${SCHEME}" \
    -destination "platform=iOS Simulator,name=${SIMULATOR_NAME}" \
    -derivedDataPath "${DERIVED_DATA_DIRECTORY}" \
    ${only_testing_arguments[@]+"${only_testing_arguments[@]}"} \
    > "${TEST_LOG_PATH}" 2>&1 || test_status=$?

grep -E "Test Case .* (passed|failed)|: error: |Executed [0-9]+ tests?|TEST (SUCCEEDED|FAILED)" "${TEST_LOG_PATH}" | sed -E 's/^.*Test Case .(.*). (passed|failed).*$/\2 \1/' | sort -u || true
printf 'full log: %s\n' "${TEST_LOG_PATH}"

exit "${test_status}"
