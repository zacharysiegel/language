# Sourced by the Android scripts: points JAVA_HOME at JDK 21, ANDROID_HOME at the SDK and ANDROID_NDK_HOME at the NDK
# version android/gradle.properties names, and puts cargo's binaries on PATH.
#
# Usage:
#   source ./scripts/android/android-environment.sh

ANDROID_ENVIRONMENT_REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ANDROID_GRADLE_PROPERTIES="${ANDROID_ENVIRONMENT_REPO_ROOT}/android/gradle.properties"

# Prints the value of a property in android/gradle.properties.
android_gradle_property() {
    local property_name="$1"
    local property_line
    property_line="$(grep "^${property_name}=" "${ANDROID_GRADLE_PROPERTIES}")"
    printf '%s' "${property_line#*=}"
}

if test -z "${JAVA_HOME:-}"; then
    JAVA_HOME="$(/usr/libexec/java_home -v 21)"
fi
export JAVA_HOME

export ANDROID_HOME="${ANDROID_HOME:-${HOME}/Library/Android/sdk}"

ANDROID_NDK_VERSION="$(android_gradle_property languageNdkVersion)"
export ANDROID_NDK_HOME="${ANDROID_HOME}/ndk/${ANDROID_NDK_VERSION}"

export PATH="${HOME}/.cargo/bin:${ANDROID_HOME}/platform-tools:${PATH}"
