# construction-calc

A construction-math calculator with exact-rational dimensional arithmetic.

The engine is a pure Rust crate (`calc-core`) with no I/O and no UI
dependencies. A WebAssembly wrapper (`calc-wasm`) drives a SvelteKit
frontend that doubles as a PWA — installable on iOS / Android / desktop
from the browser's "add to home screen" menu. A CLI (`calc-cli`) drives
the engine from the terminal. A UniFFI wrapper crate (`calc-uniffi`)
generates Swift / Kotlin bindings for the upcoming iOS app — same
engine, native UI.

Roadmap and iOS App Store launch plan: [`todo.md`](todo.md).
Marketing pitch: [`/about`](frontend/src/routes/about/+page.svelte) on
the web app.

## Why exact rationals?

The display register holds an exact `Rational64` of inches, not an `f64`.
That means `1/3 + 1/3 + 1/3 == 1` *exactly*, with no float drift. Framers
checking stud layouts will trust the result. Trig is the unavoidable f64
island — when its results feed back into a length, we reify by rounding
to the user's display precision (default 1/64").

## Layout

```
construction-calc/
├── crates/
│   ├── calc-core/     pure-Rust engine + state machine + tests
│   ├── calc-wasm/     wasm-bindgen wrapper for the web
│   └── calc-cli/      terminal REPL
└── frontend/          SvelteKit app, imports calc-wasm
```

## Build

### Engine + tests

```bash
cargo test -p calc-core
```

### CLI

```bash
cargo run -p calc-cli
> 5 ft 6 in + 2 ft 7 in =
= 8' 1"
> 6 pitch 10 ft run rise
= 5'
```

### Web app

You'll need [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
and Node 18+.

```bash
cd frontend
npm install
npm run dev          # builds wasm + serves vite dev
```

Open http://localhost:5173.

### Production build

```bash
cd frontend
npm run build
# Output goes in frontend/build/ - serve with any static host.
```

### Docker

A multi-stage `Dockerfile` builds the WASM module, builds the SvelteKit
site, and produces a small nginx image (~30 MB) that serves the static
output.

```bash
docker compose build
docker compose up -d
# Visit http://localhost:8099
```

The compose file binds to `127.0.0.1:8099` only — external access is
expected to come via Tailscale, matching the FKS pattern. (Port 8080
is intentionally avoided since it commonly collides with qBittorrent /
other local services.) Change the host port at the top of
`docker-compose.yml` if it collides.

To rebuild on source changes:

```bash
docker compose build --no-cache web   # full rebuild
# or with BuildKit cache mounts (default), incremental:
docker compose build web
```

The container is locked down: read-only rootfs, no new privileges,
memory cap of 128 MB, healthcheck on `/healthz`. nginx writes to two
tmpfs mounts so the read-only rootfs doesn't break it.

## Roadmap

- [x] Exact `Length` arithmetic
- [x] Format: feet-inch-fraction, decimal feet, decimal inches, meters, yards
- [x] Calculator state machine: digits, ops, units, equals, memory
- [x] Right-angle/rafter solver (any 2 of pitch/rise/run/diagonal)
- [x] Stair layout solver
- [x] Board feet
- [x] Trig keys (sin/cos/tan/asin/acos/atan)
- [x] Math keys (√, x², 1/x, %)
- [x] Polygon (equal-sided) area + diagonals
- [x] Circle (radius/diameter/circumference/area), arc length, chord, segment
- [x] Material estimates (sheets, studs, roofing bundles)
- [x] WASM bindings + Svelte UI
- [x] Long-press context help
- [x] Tape display
- [x] Display-format strip (1/4, 1/8, 1/16, decimal, meters, yards)
- [x] Physical keyboard support

See [`todo.md`](todo.md) for the full roadmap including the iOS App Store
release plan.

## Engine test count

```bash
$ cargo test -p calc-core
   ...
   test result: ok. 71 passed; 0 failed
```

The full workspace (`cargo test --workspace`) runs 75 Rust tests
(adds the `calc-uniffi` binding tests), and the frontend
(`cd frontend && npx vitest run`) runs 38 TypeScript tests.

## License

MIT
