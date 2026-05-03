#!/usr/bin/env bash
# One-shot setup for a fresh dev environment. Idempotent: safe to re-run.
#
# Used by:
#   - .devcontainer/devcontainer.json (postCreateCommand)
#   - first-time clones on Sullivan/Freddy/Mac
#
# What it does:
#   1. Adds the wasm32 target if missing
#   2. Installs wasm-pack if missing
#   3. Installs frontend deps
#   4. Builds the wasm bundle once so `npm run dev` is fast on first start

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "==> Ensuring wasm32 target"
rustup target add wasm32-unknown-unknown

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "==> Installing wasm-pack"
  curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
fi

echo "==> Installing frontend deps"
cd frontend
if [ -f package-lock.json ]; then
  npm ci
else
  npm install
fi

echo "==> Building wasm bundle"
npm run build:wasm

echo
echo "Setup complete. To start dev server:"
echo "  cd frontend && npm run dev"
echo
echo "To run the engine tests:"
echo "  cargo test -p calc-core"
