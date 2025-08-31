#!/usr/bin/env bash
set -euo pipefail

# Base URL for the app; default to Spin's default if not provided
E2E_URL="${E2E_URL:-http://127.0.0.1:3000}"

APP_DIR="apps/e2e-keel"
LOG_DIR="$APP_DIR/.spin"
PID_FILE="$LOG_DIR/e2e-run.pid"
LOG_FILE="$LOG_DIR/e2e-run.log"

echo "[e2e-run] Building app..."
just spin-build "$APP_DIR"

echo "[e2e-run] Starting app in background (logging to $LOG_FILE)..."
mkdir -p "$LOG_DIR"
(
  cd "$APP_DIR"
  spin up > ".spin/e2e-run.log" 2>&1 & echo $! > ".spin/e2e-run.pid"
)
PID="$(cat "$APP_DIR/.spin/e2e-run.pid")"
cleanup() {
  if kill -0 "$PID" >/dev/null 2>&1; then
    echo "[e2e-run] Stopping app (pid $PID)"
    kill "$PID" || true
    # Give Spin a moment to shutdown
    sleep 0.5 || true
  fi
}
trap cleanup EXIT

echo "[e2e-run] Waiting for readiness at $E2E_URL/users ..."
ready=0
for i in $(seq 1 60); do
  if curl -fsS "$E2E_URL/users" >/dev/null 2>&1; then ready=1; break; fi
  sleep 0.25
done

if [ "$ready" -ne 1 ]; then
  echo "[e2e-run] App did not become ready. Recent log:"
  tail -n 100 "$APP_DIR/.spin/e2e-run.log" || true
  exit 1
fi

echo "[e2e-run] Running smoke tests..."
if just e2e-smoke; then
  rc=0
else
  rc=$?
fi

exit "$rc"
