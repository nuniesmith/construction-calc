# syntax=docker/dockerfile:1.7

# =============================================================================
# Stage 1 — build the WASM module from the Rust workspace.
# =============================================================================
# IMPORTANT: keep this >= 1.85. The workspace declares `edition = "2024"`,
# which only stabilized in Rust 1.85. Older toolchains will fail with
# `feature edition2024 is required`. Bump in lockstep with rust-toolchain.toml.
ARG RUST_VERSION=1.94.1
FROM rust:${RUST_VERSION}-bookworm AS wasm-builder

ENV CARGO_TERM_COLOR=always \
    CARGO_NET_RETRY=10

# Install wasm-pack via the precompiled installer — much faster than
# `cargo install wasm-pack` (which would compile from source).
RUN curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh

WORKDIR /build

COPY Cargo.toml ./
COPY rust-toolchain.toml ./
COPY crates ./crates

# Cache mounts keep the cargo registry and target dir warm between builds.
# --release for code size and runtime perf.
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

# Install deps first so this layer caches independently of source changes.
COPY frontend/package.json frontend/package-lock.json* ./
RUN if [ -f package-lock.json ]; then npm ci; else npm install; fi

# Copy frontend source and drop the pre-built wasm into place.
# We invoke vite directly (not `npm run build`) because the npm script
# also runs `build:wasm`, which would need a Rust toolchain we don't
# have in this stage — and we already have the wasm output from stage 1.
COPY frontend/ ./
COPY --from=wasm-builder /build/wasm-out ./src/lib/wasm

RUN npx vite build

# =============================================================================
# Stage 3 — tiny runtime: nginx serving the static build.
# =============================================================================
FROM nginx:1.27-alpine AS runtime

# SPA-friendly config in place of the default.
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf

# Copy the static build (SvelteKit's adapter-static writes to ./build).
COPY --from=frontend-builder /app/build /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost/healthz || exit 1
