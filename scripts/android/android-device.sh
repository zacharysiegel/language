# Sourced by the Android scripts that need a device, after android-environment.sh.
#
# Usage:
#   source ./scripts/android/android-device.sh

readonly ANDROID_LOCAL_SERVER_PORTS=(3000 3002)

# Exits, saying how to start the emulator, when adb lists no attached device.
require_attached_device() {
    local device_lines
    device_lines="$(adb devices | tail -n +2 | grep -w "device" || true)"

    if test -z "${device_lines}"; then
        echo "error: no Android device is attached; start the emulator with" >&2
        echo "  ${ANDROID_HOME}/emulator/emulator -avd language_s25" >&2
        echo "or connect the phone with USB debugging on" >&2
        exit 1
    fi
}

# Makes the device's 127.0.0.1 ports of the web server and the repository server the Mac's.
reverse_local_server_ports() {
    local port

    for port in "${ANDROID_LOCAL_SERVER_PORTS[@]}"; do
        adb reverse "tcp:${port}" "tcp:${port}" > /dev/null
    done
}
