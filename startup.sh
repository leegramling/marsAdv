#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cleanup() {
    trap - SIGINT SIGTERM EXIT
    kill "${SERVER_PID:-}" "${CLIENT_PID:-}" 2>/dev/null || true
    wait "${SERVER_PID:-}" "${CLIENT_PID:-}" 2>/dev/null || true
}

trap cleanup SIGINT SIGTERM EXIT

echo "Starting Axum server..."
cargo run --manifest-path "$ROOT_DIR/server/Cargo.toml" &
SERVER_PID=$!

echo "Starting Svelte client..."
npm --prefix "$ROOT_DIR/client" run dev &
CLIENT_PID=$!

wait -n "$SERVER_PID" "$CLIENT_PID"
