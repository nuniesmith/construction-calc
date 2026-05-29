#!/usr/bin/env bash
# Build CalcEngine.xcframework for iOS from crates/calc-uniffi.
#
# Run on macOS with Xcode + the iOS Rust targets installed:
#
#   rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
#
# Output: ios/CalcEngine.xcframework + ios/Sources/CalcEngine/CalcEngine.swift
# Drag the xcframework + .swift file into the Xcode project to import.
#
# This script intentionally has no dependency on cargo-swift or any other
# external tool beyond cargo, xcodebuild, and lipo (all shipped with
# Xcode + a default Rust install). Keeping the build hand-rolled means
# we don't get blocked by a third-party tool drifting out of date.

set -euo pipefail

cd "$(dirname "$0")/.."

LIB_NAME="libcalc_uniffi.a"
FRAMEWORK="CalcEngine"
OUT_DIR="ios"
SWIFT_OUT="${OUT_DIR}/Sources/${FRAMEWORK}"
XCF_OUT="${OUT_DIR}/${FRAMEWORK}.xcframework"
IOS_TARGETS=(aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios)

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "error: build-ios.sh must run on macOS (need Xcode + lipo)" >&2
    exit 1
fi

if ! xcode-select -p >/dev/null 2>&1; then
    echo "error: Xcode command-line tools not found. Run: xcode-select --install" >&2
    exit 1
fi

echo "==> ensuring iOS Rust targets are installed"
# Idempotent — a no-op if the target is already present.
rustup target add "${IOS_TARGETS[@]}"

echo "==> building static libs for iOS targets"
for target in "${IOS_TARGETS[@]}"; do
    cargo build --release -p calc-uniffi --target "$target"
done

echo "==> lipo: combining simulator slices into one fat archive"
SIM_DIR="target/universal-ios-sim/release"
mkdir -p "${SIM_DIR}"
lipo -create \
    "target/aarch64-apple-ios-sim/release/${LIB_NAME}" \
    "target/x86_64-apple-ios/release/${LIB_NAME}" \
    -output "${SIM_DIR}/${LIB_NAME}"

echo "==> generating Swift bindings"
mkdir -p "${SWIFT_OUT}"
cargo run --quiet --release -p calc-uniffi --bin uniffi-bindgen -- \
    generate \
    --library "target/aarch64-apple-ios/release/${LIB_NAME}" \
    --language swift \
    --out-dir "${SWIFT_OUT}"

# UniFFI emits a .modulemap that needs to live inside the xcframework
# headers dir so Xcode can find it during the Swift import step.
HEADERS_DIR="${OUT_DIR}/headers"
rm -rf "${HEADERS_DIR}"
mkdir -p "${HEADERS_DIR}"
mv "${SWIFT_OUT}/${FRAMEWORK}FFI.h" "${HEADERS_DIR}/"
mv "${SWIFT_OUT}/${FRAMEWORK}FFI.modulemap" "${HEADERS_DIR}/module.modulemap"

echo "==> assembling xcframework"
rm -rf "${XCF_OUT}"
xcodebuild -create-xcframework \
    -library "target/aarch64-apple-ios/release/${LIB_NAME}" \
    -headers "${HEADERS_DIR}" \
    -library "${SIM_DIR}/${LIB_NAME}" \
    -headers "${HEADERS_DIR}" \
    -output "${XCF_OUT}"

echo
echo "done."
echo "  framework: ${XCF_OUT}"
echo "  swift:     ${SWIFT_OUT}/${FRAMEWORK}.swift"
echo
echo "Drag both into your Xcode project's General > Frameworks list."
