#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Reading Ruler"
PROCESS_NAME="reading-ruler"
MANIFEST="src-tauri/Cargo.toml"
MODE="${1:-}"

if pgrep -x "$PROCESS_NAME" >/dev/null 2>&1; then
  pkill -x "$PROCESS_NAME" || true
fi

cargo build --manifest-path "$MANIFEST"

BIN_PATH="src-tauri/target/debug/$PROCESS_NAME"
"$BIN_PATH" >/tmp/reading-ruler.log 2>&1 &
APP_PID=$!

sleep 1

if [[ "$MODE" == "--verify" ]]; then
  if pgrep -x "$PROCESS_NAME" >/dev/null 2>&1; then
    echo "$APP_NAME is running with PID $APP_PID"
  else
    echo "$APP_NAME did not stay running. Recent log output:"
    tail -n 80 /tmp/reading-ruler.log || true
    exit 1
  fi
elif [[ "$MODE" == "--logs" ]]; then
  tail -f /tmp/reading-ruler.log
else
  echo "$APP_NAME launched with PID $APP_PID"
  echo "Log: /tmp/reading-ruler.log"
fi
