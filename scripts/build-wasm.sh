#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="wasm32-unknown-unknown"
FEATURES="wasm-bindings"
OUT_DIR="${ROOT_DIR}/pkg"
WASM_PATH="${ROOT_DIR}/target/${TARGET}/release/labelize.wasm"

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo is required" >&2
  exit 1
fi

if ! rustup target list --installed | grep -qx "${TARGET}"; then
  echo "error: missing Rust target '${TARGET}'" >&2
  echo "run: rustup target add ${TARGET}" >&2
  exit 1
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "error: wasm-bindgen CLI is required" >&2
  echo "run: cargo install wasm-bindgen-cli" >&2
  exit 1
fi

echo "Building labelize for ${TARGET}..."
cargo build \
  --manifest-path "${ROOT_DIR}/Cargo.toml" \
  --release \
  --lib \
  --target "${TARGET}" \
  --no-default-features \
  --features "${FEATURES}"

mkdir -p "${OUT_DIR}"

echo "Generating JS bindings into ${OUT_DIR}..."
wasm-bindgen \
  --target web \
  --out-dir "${OUT_DIR}" \
  "${WASM_PATH}"

echo "Done. Generated:"
echo "  ${OUT_DIR}/labelize.js"
echo "  ${OUT_DIR}/labelize_bg.wasm"
