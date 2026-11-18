#!/usr/bin/env sh
# Run the Android instrumentation suites on a booted emulator.
# Usage: run_android_instrumentation.sh <expected page size in bytes>
set -eu
page_size="$1"

adb shell cat /proc/meminfo
if [ "$page_size" = '16384' ]; then
  test "$(adb shell getconf PAGE_SIZE | tr -d '\r')" = '16384'
fi

run_suite() {
  # $1 = instrumentation runner, $2 = output file
  adb shell am instrument -w "$1" | tee "$2"
  if ! grep -Eq 'OK \([0-9]+ tests?\)' "$2"; then
    adb logcat -d -b crash
    adb logcat -d -t 200
    exit 1
  fi
}

adb install dist/android/apk/androidTest/debug/android-debug-androidTest.apk
run_suite com.fatoora.test/androidx.test.runner.AndroidJUnitRunner instrumentation.txt
adb install dist/android-consumer/debug/android-consumer-debug.apk
adb install dist/android-consumer/androidTest/debug/android-consumer-debug-androidTest.apk
run_suite com.fatoora.consumer.test/androidx.test.runner.AndroidJUnitRunner consumer.txt
