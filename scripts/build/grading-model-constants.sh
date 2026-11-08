# Sourced by the scripts that read the grading model's identity from shared/src/grading/grading_model.rs.
#
# Usage:
#   source ./scripts/build/grading-model-constants.sh

readonly GRADING_MODEL_SOURCE_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/shared/src/grading/grading_model.rs"

# Prints the value of a `pub const <name>` in grading_model.rs, without quotes, and fails when the file defines none.
function read_grading_model_constant {
    local constant_name="$1"
    local value
    value="$(grep -E "^pub const ${constant_name}: " "${GRADING_MODEL_SOURCE_PATH}" | sed -E 's/^[^=]*= *"?([^";]*)"?;$/\1/' || true)"

    if test -z "${value}"; then
        printf 'error: %s defines no %s\n' "${GRADING_MODEL_SOURCE_PATH}" "${constant_name}" >&2
        return 1
    fi

    printf '%s\n' "${value}"
}
