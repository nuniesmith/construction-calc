# todo.md

Roadmap for **Construction Calc** — short-term cleanup, feature parity with
Construction Master Pro / Calculated Industries class apps, and the path
to a paid iOS App Store release at **$4.99 (one-time, forever — no
subscription, no IAP)**.

Status legend: `[x]` done · `[ ]` planned · `[~]` in progress

---

## 0. Project review (May 2026)

The architecture is in good shape and the discipline is holding:

- **`calc-core`** is pure Rust, `no_std`-capable, no I/O, no UI deps —
  exactly the right shape to ship as a Swift/Kotlin library via UniFFI.
  The math is exact (`Rational64` inches), so 1/3 + 1/3 + 1/3 = 1 with no
  float drift. That's a real differentiator vs. cheaper store calculators.
  `Value` is a tagged union — `Scalar / Length / Area / Volume / Angle` —
  with dimension-aware promotion (`Length × Length = Area`,
  `Area × Length = Volume`, `Volume ÷ Area = Length`, …).
- **`calc-wasm`** drives the web app from the same engine (JSON event
  protocol). Good for the try-before-you-buy demo and a single source of
  truth.
- **`calc-uniffi`** exports a *typed* event API (no JSON) for Swift /
  Kotlin. Bindings regenerate cleanly; verified on Linux against the `.so`.
- **`calc-cli`** drives the engine from a terminal REPL — great for
  end-to-end testing without a UI.
- **Frontend (SvelteKit)** is the reference UI and the marketing demo /
  PWA. The native iOS app is SwiftUI on top of the UniFFI bindings.
- **iOS** sources are scaffolded (`ios/ConstructionCalc/`, ~920 lines of
  hand-written SwiftUI against the real generated binding names) but have
  **never hit a Swift compiler** — that step needs a Mac.

**Tests (all green as of this review):**

- **78 Rust tests** (73 `calc-core` + 5 `calc-uniffi`) — includes the
  angle-mode coverage from PR #13.
- **40 frontend TypeScript tests**.
- **CI**: 4 jobs — engine (fmt + clippy + test), wasm build, frontend
  (typecheck + vitest + build), docker image — plus GitGuardian. All
  passing. Keep this discipline; App Review rewards crash-never apps.

**Recently landed — angle mode (PR #13):** the degrees/radians
**angle-mode** preference is now wired end to end
(`KeyEvent::SetAngleMode(bool)` → core → wasm → uniffi → CLI → web →
iOS). This closed the one real engine gap found while scaffolding iOS.

**Corrections to the previous roadmap (things that were already done but
listed as TODO):**

- ~~"Length × Length × Length → Volume — need a Volume variant of
  `Value`."~~ **`Value::Volume` already exists** and `mul` promotes
  Length³ → Volume today. The *actual* remaining gap is **display**:
  Area always renders as `sq in`, Volume always as `cu in` — there's no
  way to convert the display to ft²/yd³/acres/gallons. See §1.
- ~~"Acres key — need an Area type for land."~~ The `Area` type exists;
  what's missing is an **area display format** (ft², yd², acres) and a
  convert key, same shape as `LengthFormat`.

---

## 1. Next up — prioritized

This is the recommended order of attack. Items in **Track A need no Mac**
and harden the engine/web *before* the FFI surface freezes into Swift, so
you don't re-port later. **Track B is the iOS-native path**, blocked on
having a Mac with Xcode.

### Track A — engine + web (do now, no Mac required)

1. **[x] Angle mode (degrees/radians)** — landed in PR #13. Everything
   below builds on it.
2. **[x] Area & Volume display formats + convert keys** (PR #15).
   `AreaFormat` (in²/ft²/yd²/m²/acres) and `VolumeFormat`
   (in³/ft³/yd³/m³/gal/L) with exact rational conversion, `ConvertArea` /
   `ConvertVolume` events, and a `dimension` snapshot tag. Wired through
   wasm + uniffi + CLI + the dimension-aware web format strip. `10' × 12'`
   now shows `120 sq ft` in the main calculator.
3. **[x] Cost-per-unit as a first-class engine function.** New
   `Value::Money` + `KeyEvent::CostPerUnit`: `price × quantity-in-shown-
   unit = $total` over any dimension ($/ft, $/sq ft, $/cu yd, $/each).
   Wired through wasm + uniffi + CLI (`cost`) + a web `Cost` key. Replaces
   the board-feet-only field.
4. **[x] dms ↔ deg angle display toggle.** New `AngleFormat`
   (`DegMinSec` / `DecimalDegrees`) + `KeyEvent::ConvertAngle`, with a
   `Mode.angle_format` independent of the trig-input `angle_in_degrees`.
   Wired through wasm + uniffi + CLI (`afmt dms|dd`) + the format strip
   (`°′″` / `deg`). Reuses the exact `Angle` Display for DMS.
5. **[x] Tape export / share.** Web already had `.md` / `.json` download
   + clipboard copy + load; iOS already had a Markdown `ShareLink`. Added
   the missing pieces: a **Web Share API** button (`navigator.share`) so
   mobile web gets a real OS share sheet (falls back to download), the iOS
   share now offers **Markdown *and* JSON**, and `canShare` / `shareText`
   helpers are unit-tested. Possible follow-up: share a `.md`/`.json`
   *file* (not just text) where `navigator.canShare({ files })` allows.
6. **[ ] Verify compound miter end-to-end.** The Corner/Spring/Miter/Bevel
   keys exist as enum variants and the keypad Miter page has the four
   buttons — drive the full flow (enter 90° corner, 38° spring → confirm
   ~31.6° miter / ~33.9° bevel) on web and add an integration test if one
   is missing. Confirm it's a finished feature, not a stub.
7. **[x] Material-estimate gaps.** Added `/ez/framing` (sheathing panels,
   wall plates, headers) and `/ez/spacing` — a generalized equal-spacing
   divider with both max-gap (pickets/balusters) and on-center
   (joists/studs) modes. Unlike the older inline-math forms, the math is
   extracted into unit-tested `$lib/framing.ts` and `$lib/spacing.ts`
   (16 new vitest tests).
8. **[x] Weight dimension** (pounds, kg, tons, metric tons). New
   `Value::Weight` (exact pounds) + `WeightUnitKey` input keys
   (`WeightUnit` event) + `WeightFormat` + `ConvertWeight`. Wired through
   wasm + uniffi + CLI (`lb`/`kg`/`ton`/`tonne` + `wfmt`) + a `Wt` keypad
   page + dimension-aware format strip. Integrates with cost-per-unit
   ($/lb, $/ton). Conversions are exact (1 lb = 0.453 592 37 kg).

### Track B — iOS native (blocked on a Mac)

9. **[ ] Day one on the Mac** — `bash scripts/build-ios.sh` → create the
   Xcode project → drag in `CalcEngine.xcframework` + the scaffolded
   sources → run in the simulator. See §3 for the full checklist.
10. **[ ] Real 1024×1024 PNG app icon** (currently an SVG placeholder).
    Needed for both the App Store and a polished PWA install. Can be
    designed off-Mac.
11. **[ ] Port the 9 EZ Calc forms to SwiftUI**, then App Store Connect
    setup + submission (§3.3–3.5).

---

## 2. Near-term engine + web polish (detail)

Expanded detail for the Track A items above plus the smaller polish.

- [x] **Preferences screen** at `/preferences`, backed by `localStorage`
      (`cc.preferences.v1`): default fraction resolution (1/4, 1/8, 1/16),
      default length format, degrees vs radians. Applied on startup. The
      `Preferences` shape is the contract iOS mirrors against
      `UserDefaults`.
- [x] **Saved tapes** at `/tapes`, backed by `localStorage`
      (`cc.tapes.v1`). `💾 Save` persists the engine's JSON; list is
      newest-first with Load / Delete and a two-step "Delete all". Caps:
      1 MB/tape, 200 tapes.
- [x] **Polygon & Circle** EZ Calc forms (`/ez/polygon`, `/ez/circle`).
      A future improvement: a `PartialPolygon` / `PartialCircle` state
      machine so the keypad can drive them directly the way Rafter does.
- [x] **Board feet** EZ Calc form with optional cost-per-bf.
- [x] **Memory keys** (MS, MR, M+, MC, MC-All) on a "Mem" keypad tab.
- [x] **Angle mode (degrees/radians)** — wired end to end in **PR #13**.
- [x] **Area & Volume display formats + convert** (PR #15) — ft²/yd²/acres,
      cu yd/gal/L in the main calculator, with a dimension-aware strip.
- [x] **Cost-per-unit** first-class engine function — `Value::Money` +
      `CostPerUnit`; prices by the shown unit ($/ft, $/sq ft, $/cu yd).
- [x] **dms ↔ deg** angle display toggle — `AngleFormat` + `ConvertAngle`,
      with a format-strip toggle (`°′″` / `deg`).
- [x] **Save/share tape** — web download + clipboard + Web Share API;
      iOS `ShareLink` (Markdown + JSON).
- [x] **Compound miter** end-to-end verification (PR #19) — see §1.6.
- [x] **Material estimates** — `/ez/framing` (sheathing/plates/headers) +
      `/ez/spacing` (generalized equal-spacing divider) — see §1.7.
- [ ] **Service worker** for true offline support. The PWA currently
      relies on the browser HTTP cache; a service worker makes it work
      fully offline (and is a credible "works on a job site with no
      signal" selling point).

---

## 3. Feature parity with Construction Master Pro

What CMPro exposes that we don't yet. `[x]` = shipped; `[ ]` = gap. Several
gaps are engine-complete and just need a button + help text.

### Length / area / volume

- [x] Feet-inch-fraction with rounding (1/4, 1/8, 1/16)
- [x] Meters
- [x] Yards
- [x] `Value::Area` / `Value::Volume` types with dimension promotion
      (Length² → Area, Length³ → Volume) — **engine done**
- [x] **Area display formats** (in²/ft²/yd²/m²/acres) + convert key (PR #15)
- [x] **Volume display formats** (in³/ft³/yd³/m³/gal/L) + convert key (PR #15)
- [ ] **Weight** dimension: pounds, kilograms, tons, metric tons
      (`Value::Weight` + format + convert)

### Construction math

- [x] Pitch / Rise / Run / Diagonal solver
- [x] Hip / Valley
- [x] Jack rafter difference
- [x] Stair layout
- [x] Compound miter (corner + spring → miter + bevel) — verified
      end-to-end (§1.6); also exposed in the CLI (`corner spring miter bevel`)
- [x] Circle (radius, diameter, circumference, area, arc, chord, segment)
- [x] Polygon (equilateral)
- [x] Board feet
- [x] Drywall estimation (`/ez/drywall`)
- [x] Concrete volume (slab, column, cone) — `/ez/concrete`, cubic-yard
      output + waste factor
- [x] Rebar spacing & count — `/ez/rebar` with bar-size weight tables
- [x] Baluster spacing — `/ez/baluster`, enforces IRC 4" sphere code
- [x] Roofing bundles + footprint × pitch (`/ez/roofing`); stud count
      (`/ez/studs`)
- [x] Column / cone **lateral surface area** — `concrete::{column,cone}_lateral_area`,
      surfaced in `/ez/concrete` (form/wrap material)
- [x] **Equal-spacing on-center divider** (joists, pickets, fences) —
      `/ez/spacing`, generalized from the baluster math (max-gap + OC modes)
- [x] **Sheathing sheets, plates, headers** — `/ez/framing`
- [x] **Cost-per-unit** estimating across any dimension — `Value::Money` +
      `CostPerUnit`, priced by the shown unit

### UI / UX

- [x] Tape view
- [x] Long-press help overlay
- [x] Format strip
- [x] Physical keyboard support (web)
- [x] Saved tapes list (named, persisted at `/tapes`)
- [x] Settings tab (`/preferences`)
- [x] PWA install support (`manifest.webmanifest` + icons; installable on
      iOS Safari, Android Chrome, desktop Chrome/Edge)
- [x] About / marketing page at `/about`
- [ ] **Per-key secondary functions** via the small red label above a key
      (CMPro's `Slope` over `Pitch`, `R/Wall` over `Rise`, …). On iOS this
      is a long-press; repurpose the existing long-press infra to *invoke*
      the secondary key instead of only showing help.
- [ ] **Help tab** — a full in-app reference manual (the long-press help
      strings already exist; collect them into a browsable screen).
- [ ] **Real PNG app icon** — 1024×1024 master + the auto-generated iOS
      sizes; Apple prefers 180×180 PNG over the current SVG placeholder.
- [ ] **Service worker** for true offline (§2).

---

## 4. iOS App Store release plan

Goal: **paid app, $4.99 USD one-time, worldwide.** No subscription, no
IAP, no ads.

### 4.1 Architecture

```
crates/calc-core      ← done, pure Rust
crates/calc-uniffi    ← done, UniFFI wrapper → Swift bindings
ios/                  ← Xcode project, SwiftUI app (scaffolded, uncompiled)
```

- [x] **`crates/calc-uniffi`** — proc-macro UniFFI wrapper exporting
      `Calculator`, `KeyEvent`, `LengthFormat`, `Unit`, `FunctionKey`,
      `MemoryOp`, `Snapshot`, `CalcFfiError`. 4 end-to-end tests (5 with
      PR #13's angle-mode test).
- [x] **`scripts/build-ios.sh`** — builds arm64-device + arm64-sim +
      x86_64-sim slices, lipos the sim pair, runs `uniffi-bindgen
      generate --language swift`, assembles `CalcEngine.xcframework`.
      Auto-installs the iOS Rust targets (idempotent). macOS-only; bindings
      generation verified on Linux against the `.so`.
- [ ] **Install iOS Rust targets on the Mac** — handled by the script;
      for reference: `aarch64-apple-ios`, `aarch64-apple-ios-sim`,
      `x86_64-apple-ios`.
- [ ] **Create the Xcode project** under `ios/ConstructionCalc/`, SwiftUI,
      iOS 17+ target.
- [ ] **Wire up the bindings** — import `CalcEngine.xcframework`,
      instantiate `Calculator` once in an `@Observable` view model, send
      `KeyEvent`s on taps.

#### Day-one-on-the-Mac checklist

1. `xcode-select --install` (or full Xcode from the App Store).
2. From repo root: `bash scripts/build-ios.sh`. Installs targets, builds
   all three slices, generates Swift bindings, assembles
   `ios/CalcEngine.xcframework` (artifacts are gitignored).
3. Likely first-run snags (none reproducible off a Mac):
   - **`panic = "abort"` vs UniFFI `catch_unwind`.** The release profile
     in `Cargo.toml` sets `panic = "abort"` (good for wasm size). UniFFI
     turns Rust panics into Swift errors via `catch_unwind`; with `abort`
     a panic crashes the app instead. The engine returns `Result` on every
     fallible path and overflow *wraps* (not panics) in release, so this
     is unlikely — but if you see hard crashes on device, add an
     iOS-specific profile with `panic = "unwind"` and rebuild.
   - **Min deployment target** — set the framework's min iOS version to
     match the app target.
4. Drag `ios/CalcEngine.xcframework` into *General → Frameworks, Libraries,
   and Embedded Content*; add `ios/Sources/CalcEngine/CalcEngine.swift` to
   the target.
5. Smoke test: `let calc = Calculator(); calc.handle(event: .digit(value:
   5))` → `displayString()` returns `"5"`.

The TypeScript shapes the iOS UI mirrors are settled: see
`frontend/src/lib/calc.ts` (`Key`/`Unit`/`FunctionKey`) and
`frontend/src/lib/preferences.ts` (`Preferences` → `UserDefaults`).
`Keypad.svelte` / `FormatStrip.svelte` are the reference layouts.

### 4.2 SwiftUI app structure

- [x] **Scaffolded** under `ios/ConstructionCalc/` (compiles once
      `CalcEngine.xcframework` is linked; written against the regenerated
      binding API). See `ios/README.md` for the click-by-click assembly.

```
ios/ConstructionCalc/
├── ConstructionCalcApp.swift     ← @main entrypoint                 [done]
├── CalculatorViewModel.swift     ← @Observable wrapper over UniFFI  [done]
├── Haptics.swift                 ← light tap feedback               [done]
├── Model/
│   ├── KeypadModel.swift          ← button grid (Keypad.svelte)     [done]
│   ├── FormatOption.swift         ← format strip                    [done]
│   ├── Preferences.swift          ← UserDefaults (preferences.ts)   [done]
│   └── HelpText.swift             ← long-press help (help.ts)       [done]
└── Views/
    ├── CalculatorView.swift       ← root screen                     [done]
    ├── DisplayView.swift                                            [done]
    ├── FormatStripView.swift                                        [done]
    ├── KeypadView.swift           ← Grid w/ long-press help         [done]
    ├── TapeView.swift             ← share / clear                   [done]
    ├── HelpOverlayView.swift                                        [done]
    └── PreferencesView.swift      ← settings sheet                  [done]
```

- [x] **Haptics on key press** — `UIImpactFeedbackGenerator(.light)`.
- [x] **Angle-mode preference applied at launch + live** — via PR #13
      (`Preferences.toAngleModeKey()`, `setAngleMode(degrees:)`).
- [ ] **EZ Calc forms** — the 9 web forms aren't ported yet; pure-Swift
      arithmetic + a tape note, easy to add as a navigation list once the
      core app builds.
- [ ] **Keyboard support** via `.keyboardShortcut` (iPad hardware keyboard).
- [ ] **iPad layout** — wider keypad, two-pane (calculator + tape).
      Universal binary; one purchase covers iPhone + iPad.
- [ ] **App icon + Assets.xcassets** — needs the 1024×1024 PNG.
- [ ] **Light mode** — dark-locked today (`preferredColorScheme(.dark)`);
      add a light palette.

### 4.3 App Store Connect setup

- [ ] **Apple Developer Program** ($99/year). developer.apple.com.
- [ ] **App ID** — `com.<yourname>.constructioncalc`.
- [ ] **App Store Connect listing.**
  - Name: `Construction Calc` (check availability — may need `…Pro` or
    `BuildCalc`)
  - Subtitle (30 chars): e.g. `Exact rational math for builders`
  - Category: Productivity (primary), Utilities (secondary)
  - Age rating: 4+
- [ ] **Pricing.** Tier 5 (≈ $4.99 USD), "Available in all countries". No
      introductory pricing, no auto-renew.
- [ ] **App icon** — 1024×1024 + the auto-generated set. Must read well at
      60×60.
- [ ] **Screenshots.** 6.7" iPhone (1290×2796) + 6.9" (1320×2868). iPad
      13" (2064×2752) strongly recommended (universal app). 3–10 each.
- [ ] **App Preview video** (optional, ~+20% conversion): 15–30s.
- [ ] **Privacy nutrition label.** "Data Not Collected" — no analytics,
      login, or network. Lean into it.
- [ ] **Privacy Policy URL** (required even for no-data apps) — host a
      one-pager, e.g. `nuniesmith.github.io/construction-calc/privacy`.
- [ ] **Support URL** — GitHub issues or a contact page.

### 4.4 Pre-submission checklist

- [ ] **No crashes** in a 10-min manual smoke test on a physical device
      (all keypad pages, all unit combos, save/share tape, settings
      persistence).
- [ ] **No `print()` / `NSLog`** in the shipped binary.
- [ ] **Memory leak check** with Instruments → Leaks (Rust side is
      leak-free by construction; check Swift).
- [ ] **VoiceOver / accessibility** — every key has an
      `accessibilityLabel`.
- [ ] **Dynamic Type** — display text scales with system font size.
- [ ] **Light + dark mode** both correct.
- [ ] **Smallest device** (iPhone SE 3rd gen, 4.7") — keypad must not
      overflow.
- [ ] **App Review Guidelines** §4 (Design) + §5 (Legal). Calculators are
      low-risk; rejections usually = missing privacy policy, broken links,
      or unneeded permissions.

### 4.5 Submission & rollout

- [ ] First TestFlight build to yourself + 1–2 trades friends (they'll
      find bugs).
- [ ] At least one full week of internal TestFlight before review.
- [ ] Submit for review (24–48h turnaround in 2026).
- [ ] **Manual release** so you can pick a launch day (Tue/Wed is typical).
- [ ] Launch-day artifacts:
  - [ ] README App Store badge + link
  - [ ] r/Construction, r/HomeImprovement, HN Show
  - [ ] Tweet / LinkedIn
  - [ ] (Optional) ToolGuyd, This Old House, Pro Tool Reviews outreach

### 4.6 Why $4.99 one-time will work

Competitors (Construction Master Pro, BuildCalc Pro) are $24.99–$29.99
one-time or $4.99/month. Positioning:

- **Sub-$5 impulse buy** — below the think-twice threshold.
- **No subscription** — every competitor's reviews complain about the
  recurring charge. This is the wedge.
- **Forever updates** — paid-app StoreKit gives buyers all future updates
  free. "Buy once. Yours forever."
- **Local-only, no tracking** — the privacy label is a marketing asset.

Counter the "too cheap to be good" risk with a polished icon, real
screenshots of the exact-rational result (`1/3 + 1/3 + 1/3 = 1"`, not
`0.999"`), and "exact rational math" in the subtitle / description.

---

## 5. Android (after iOS is stable)

- [ ] `crates/calc-uniffi` already produces Kotlin bindings — reuse.
- [ ] Compose Multiplatform UI, or Jetpack Compose Android-only.
- [ ] Google Play submission ($25 one-time dev fee), similar checklist.
- [ ] Price-match $4.99.

---

## 6. Long-term ideas

- [ ] **iCloud sync of saved tapes** via CloudKit (no server; opt-in).
- [ ] **Tape templates** — save a calc as a template ("garage slab"),
      recall later with new inputs.
- [ ] **PDF export** of a tape for a client / inspector.
- [ ] **Apple Watch companion** — read-only last result; voice memo to
      start a tape.
- [ ] **macOS Catalyst** build — same codebase, free extra platform.
- [ ] **Open-source `calc-core`** under MIT, keep the app shells closed.
      Engine OSS is a credibility signal.
- [ ] **Property-based tests** (e.g. `proptest`) for the engine — random
      dimensional expressions that must round-trip / preserve invariants.
      Cheap insurance for a "never wrong" calculator before the paid
      launch.
