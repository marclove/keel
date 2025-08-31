#!/usr/bin/env bash
set -euo pipefail
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

# Base URL for the app; default to Spin's default if not provided
E2E_URL="${E2E_URL:-http://127.0.0.1:3000}"
RUST_LOG="${RUST_LOG:-info}"
SPIN_HTTP_LISTEN_ADDR="${SPIN_HTTP_LISTEN_ADDR:-127.0.0.1:3000}"

APP_DIR="apps/e2e-keel"
LOG_DIR="$APP_DIR/.spin"
PID_FILE="$LOG_DIR/e2e-run.pid"
LOG_FILE="$LOG_DIR/e2e-run.log"

BUILD_LOG_FILE="$LOG_DIR/build.log"
echo "[e2e-run] Building app (logging to $BUILD_LOG_FILE)..."
mkdir -p "$LOG_DIR"
# Capture build output; if it fails, print the full log then exit
if ! (cd "$APP_DIR" && spin build 2>&1 | tee ".spin/build.log"); then
  echo "[e2e-run] Build failed." >&2
  # If the log is huge, avoid flooding CI: show last 400 lines
  lines=$(wc -l < "$BUILD_LOG_FILE" || echo 0)
  if [ "$lines" -gt 400 ]; then
    echo "[e2e-run] Build log is $lines lines; showing last 400 lines:" >&2
    tail -n 400 "$BUILD_LOG_FILE" || true
  else
    echo "[e2e-run] Full build log:" >&2
    cat "$BUILD_LOG_FILE" || true
  fi
  # The build already failed above, we have the full log
  echo "[e2e-run] Build failed (see log above)" >&2
  exit 1
fi

echo "[e2e-run] Using RUST_LOG=$RUST_LOG, SPIN_HTTP_LISTEN_ADDR=$SPIN_HTTP_LISTEN_ADDR"
echo "[e2e-run] spin version: $(spin --version || echo 'spin not found')"
echo "[e2e-run] Starting app in background (logging to $LOG_FILE)..."
mkdir -p "$LOG_DIR"
(
  cd "$APP_DIR"
  RUST_LOG="$RUST_LOG" SPIN_HTTP_LISTEN_ADDR="$SPIN_HTTP_LISTEN_ADDR" \
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
  if command -v ss >/dev/null 2>&1; then
    echo "[e2e-run] Listening sockets (ss -ltn):"; ss -ltn || true
  elif command -v netstat >/dev/null 2>&1; then
    echo "[e2e-run] Listening sockets (netstat -tln):"; netstat -tln || true
  fi
  exit 1
fi

echo "[e2e-run] Running smoke tests..."
if just e2e-smoke; then
  rc=0
else
  rc=$?
  echo "[e2e-run] Smoke tests failed (exit $rc). Recent app log:"
  tail -n 200 "$APP_DIR/.spin/e2e-run.log" || true
  if command -v ss >/dev/null 2>&1; then
    echo "[e2e-run] Listening sockets (ss -ltn):"; ss -ltn || true
  elif command -v netstat >/dev/null 2>&1; then
    echo "[e2e-run] Listening sockets (netstat -tln):"; netstat -tln || true
  fi
fi

exit "$rc"
