#!/usr/bin/env bash
# Wrapper for phi-tk-cli with optional Xvfb/audio setup for headless systems.

set -euo pipefail

#需要修改:
APP_DIR="/usr/lib/phi-tk-cli"
REAL_BIN="$APP_DIR/phi-tk-cli"
DEFAULT_ASSETS="/usr/lib/ptkc-assets"

cleanup() {
  if [ -n "${PHI_TK_XVFB_PID:-}" ]; then
    kill "$PHI_TK_XVFB_PID" >/dev/null 2>&1 || true
    wait "$PHI_TK_XVFB_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT INT TERM

has_arg() {
  local needle="$1"
  shift
  for arg in "$@"; do
    case "$arg" in
      "$needle"|"$needle"=*) return 0 ;;
    esac
  done
  return 1
}

if [ -z "${DISPLAY:-}" ]; then
  if ! command -v Xvfb >/dev/null 2>&1; then
    echo "phi-tk-cli: DISPLAY is not set and Xvfb is not installed." >&2
    exit 1
  fi

  display_number="${PHI_TK_DISPLAY:-99}"
  max_display_number=$((display_number + 20))
  xvfb_log="${PHI_TK_XVFB_LOG:-/tmp/phi-tk-cli-xvfb.log}"
  # Try to create/truncate the log file; if not writable, use /dev/null silently.
  if ! ( : > "$xvfb_log" ) 2>/dev/null; then
    xvfb_log="/dev/null"
  fi
  while [ "$display_number" -le "$max_display_number" ]; do
    if [ ! -e "/tmp/.X${display_number}-lock" ]; then
      Xvfb ":$display_number" -screen 0 "${PHI_TK_SCREEN:-1920x1080x24}" -ac +extension GLX +render -nolisten tcp >>"$xvfb_log" 2>&1 &
      PHI_TK_XVFB_PID=$!
      sleep "${PHI_TK_XVFB_WAIT:-1}"
      if kill -0 "$PHI_TK_XVFB_PID" >/dev/null 2>&1; then
        export DISPLAY=":$display_number"
        break
      fi
      wait "$PHI_TK_XVFB_PID" >/dev/null 2>&1 || true
      unset PHI_TK_XVFB_PID
    fi
    display_number=$((display_number + 1))
  done

  if [ -z "${DISPLAY:-}" ]; then
    echo "phi-tk-cli: failed to start Xvfb. Check /tmp/.X11-unix permissions or set DISPLAY manually. Xvfb log: $xvfb_log" >&2
    exit 1
  fi
fi
#INTERNET YAMERO MASTER 13 SSS 100.4523%

export SDL_AUDIODRIVER="${SDL_AUDIODRIVER:-alsa}"
export ALSA_PCM_NAME="${ALSA_PCM_NAME:-default}"

if [ -z "${PULSE_SERVER:-}" ] && [ -S "/run/user/$(id -u)/pulse/native" ]; then
  export PULSE_SERVER="unix:/run/user/$(id -u)/pulse/native"
fi

if [ -d "$DEFAULT_ASSETS" ] && ! has_arg "--assets" "$@"; then
  "$REAL_BIN" --assets "$DEFAULT_ASSETS" "$@"
else
  "$REAL_BIN" "$@"
fi
#NEEDY GIRL OVERDOZE
