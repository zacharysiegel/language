#!/usr/bin/env bash

# Measures how well local language models grade a learner's written answer. Models run through Ollama, or through
# cross-llama with `run --runtime cross-llama`.
#
# Usage:
#   ./scripts/dev/evaluate-grading.sh pull small|large           download the candidate models
#   ./scripts/dev/evaluate-grading.sh run-candidates             grade with every installed candidate model, then report
#   ./scripts/dev/evaluate-grading.sh run-thinking [arguments]   grade both case sets through cross-llama with thinking
#                                                                off, on, and on with a 128-token budget, then report;
#                                                                the arguments go to each run
#   ./scripts/dev/evaluate-grading.sh <grading_evaluation arguments>
# e.g.
#   ./scripts/dev/evaluate-grading.sh check-cases
#   ./scripts/dev/evaluate-grading.sh run --model gemma3:4b --prompt-variant plain --only a2-
#   ./scripts/dev/evaluate-grading.sh run --runtime cross-llama --thinking on --thinking-budget 128
#   ./scripts/dev/evaluate-grading.sh report

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

readonly HOLDOUT_CASES_PATH="tools/grading_evaluation/cases/pt_br.en.holdout.json"
readonly THINKING_RUNS_DIRECTORY="tools/grading_evaluation/results/thinking_comparison"
readonly THINKING_HOLDOUT_RUNS_DIRECTORY="tools/grading_evaluation/results/thinking_comparison_holdout"
readonly THINKING_PROMPT_VARIANT="few_shot_six"

readonly SMALL_MODELS=(
    "gemma3:1b"
    "qwen3:1.7b"
    "llama3.2:3b"
    "gemma3:4b"
    "qwen3:4b"
    "phi4-mini"
)

readonly LARGE_MODELS=(
    "qwen3:8b"
    "llama3.1:8b"
    "gemma3:12b"
    "aya-expanse:8b"
)

function pull_models {
    local model
    for model in "$@"; do
        printf 'pulling %s\n' "${model}"
        ollama pull "${model}"
    done
}

function run_installed_candidates {
    local installed_names
    installed_names="$(ollama list | awk 'NR > 1 { print $1 }' | sed 's/:latest$//')"

    local model_arguments=()
    local model
    for model in "${SMALL_MODELS[@]}" "${LARGE_MODELS[@]}"; do
        local installed_match
        installed_match="$(printf '%s\n' "${installed_names}" | grep -x -F "${model}" || true)"

        if test -n "${installed_match}"; then
            model_arguments+=(--model "${model}")
        fi
    done

    if test "${#model_arguments[@]}" -eq 0; then
        printf 'no candidate model is installed; run: %s pull small\n' "$0" >&2
        exit 1
    fi

    cd "${REPO_ROOT}"
    cargo run --release --package grading_evaluation -- run "${model_arguments[@]}" > /dev/null
    cargo run --release --package grading_evaluation -- report
}

function run_cross_llama_on_both_case_sets {
    cargo run --release --package grading_evaluation -- run --runtime cross-llama --prompt-variant "${THINKING_PROMPT_VARIANT}" --output "${THINKING_RUNS_DIRECTORY}" "$@" > /dev/null
    cargo run --release --package grading_evaluation -- run --runtime cross-llama --prompt-variant "${THINKING_PROMPT_VARIANT}" --cases "${HOLDOUT_CASES_PATH}" --output "${THINKING_HOLDOUT_RUNS_DIRECTORY}" "$@" > /dev/null
}

function run_thinking_comparison {
    cd "${REPO_ROOT}"
    run_cross_llama_on_both_case_sets --thinking off "$@"
    run_cross_llama_on_both_case_sets --thinking on "$@"
    run_cross_llama_on_both_case_sets --thinking on --thinking-budget 128 "$@"
    run_cross_llama_on_both_case_sets --thinking on --thinking-budget 256 "$@"
    run_cross_llama_on_both_case_sets --thinking on --thinking-budget 512 "$@"
    cargo run --release --package grading_evaluation -- report --runs "${THINKING_RUNS_DIRECTORY}"
    cargo run --release --package grading_evaluation -- report --runs "${THINKING_HOLDOUT_RUNS_DIRECTORY}" --cases "${HOLDOUT_CASES_PATH}"
}

subcommand="${1:-}"

if test "${subcommand}" == "pull"; then
    pull_group="${2:-}"

    if test "${pull_group}" == "small"; then
        pull_models "${SMALL_MODELS[@]}"
    elif test "${pull_group}" == "large"; then
        pull_models "${LARGE_MODELS[@]}"
    else
        printf 'usage: %s pull small|large\n' "$0" >&2
        exit 64
    fi
elif test "${subcommand}" == "run-candidates"; then
    run_installed_candidates
elif test "${subcommand}" == "run-thinking"; then
    run_thinking_comparison "${@:2}"
else
    cd "${REPO_ROOT}"
    cargo run --release --package grading_evaluation -- "$@"
fi
