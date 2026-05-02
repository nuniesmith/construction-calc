# syntax=docker/dockerfile:1.7

# =============================================================================
# Stage 1 — build the WASM module from the Rust workspace.
# =============================================================================
# We install wasm-pack via its precompiled installer (faster than `cargo
# install wasm-pack`, which compiles from source). BuildKit cache mounts
# keep the cargo registry and target dir warm between builds.
FROM rust:1.83-bookworm AS wasm-builder

ENV CARGO_TERM_COLOR=always \
    CARGO_NET_RETRY=10 \
    RUSTFLAGS="-D warnings"

RUN curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh

WORKDIR /build

# Copy the workspace. Source is small and `.dockerignore` keeps target/
# and node_modules/ out, so a single COPY is fine and simpler than the
# manifest-first dance.
COPY Cargo.toml ./
COPY crates ./crates

# Build the wasm bundle. Cache mounts make rebuilds fast; --release for
# code size and runtime perf.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    wasm-pack build crates/calc-wasm \
        --release \
        --target web \
        --out-dir /build/wasm-out

# =============================================================================
# Stage 2 — build the SvelteKit static site.
# =============================================================================
FROM node:20-alpine AS frontend-builder

WORKDIR /app

# Install JS deps first so this layer caches independently of source changes.
COPY frontend/package.json frontend/package-lock.json* ./
RUN if [ -f package-lock.json ]; then npm ci; else npm install; fi

# Copy frontend source and drop the pre-built wasm into place. We bypass
# `npm run build` because that script invokes `build:wasm` (which needs
# the Rust toolchain we don't have in this stage) — we already have the
# wasm output from stage 1.
COPY frontend/ ./
COPY --from=wasm-builder /build/wasm-out ./src/lib/wasm

RUN npx vite build

# =============================================================================
# Stage 3 — tiny runtime: nginx serving the static build.
# =============================================================================
FROM nginx:1.27-alpine AS runtime

# Drop our SPA-friendly config in place of the default.
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy the static build (SvelteKit's adapter-static writes to ./build).
COPY --from=frontend-builder /app/build /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost/ || exit 1
