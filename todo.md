# todo.md

Roadmap for **Construction Calc** — short term cleanup, feature parity with
Construction Master Pro / Calculated Industries class apps, and the path
to a paid iOS App Store release at **$4.99 (one-time, forever — no
subscription, no IAP)**.

Status legend: `[x]` done · `[ ]` planned · `[~]` in progress

---

## 0. Project review (May 2026)

The architecture is in good shape for what you want to do:

- **`calc-core`** is pure Rust, no I/O, no UI deps — exactly the right
  shape to ship as a Swift library via UniFFI. The math is exact
  (`Rational64` inches), so 1/3 + 1/3 + 1/3 = 1 with no float drift.
  That's a real differentiator vs. cheaper calculators on the store.
- **`calc-wasm`** lets the same engine drive the web app — useful for
  marketing (try-before-you-buy demo) and for keeping a single source of
  truth.
- **`calc-cli`** is great for end-to-end testing without a UI.
- **Frontend (SvelteKit)** is the current "reference UI." For iOS it
  becomes a marketing site / web demo; the actual app will be native
  SwiftUI on top of UniFFI bindings.
- **Tests** — 67 Rust tests + 21 frontend tests, all passing as of the
  latest cleanup. Keep this discipline; App Review prefers crashes-never
  apps, and Rust + property-style testing get you most of the way.

**Recently shipped (this branch):**

- [x] Removed `mm` and `cm` units across engine, parser, formatter,
      WASM bindings, CLI, and UI. `m` (meters) is the only metric unit
      that remains.
- [x] Removed 1/32" and 1/64" display resolutions. Added 1/4" and 1/8"
      alongside the existing 1/16". Validation in `LengthFormat`
      tightened to `denom ∈ {2, 4, 8, 16}`.
- [x] The fraction buttons on the format strip now double as rounding
      controls — pick 1/4" and any sum of whole + fraction renders
      rounded to the nearest quarter inch. Internal value stays exact.

---

## 1. Near-term engine + web polish (1–2 weeks)

These are the gaps to close *before* starting on the iOS app, so the
engine you bind into Swift is the version you actually want to ship.

- [ ] **Preferences screen** (web + later iOS). A persistent settings
      pane backed by `localStorage` on web, `UserDefaults` on iOS.
      Settings:
      - Default fraction resolution (1/4, 1/8, 1/16)
      - Default angle mode (degrees vs radians)
      - Default unit (feet-inch vs meters)
      - Tape auto-save on/off
- [ ] **Polygon & Circle keys** exposed in the keypad UI. Engine
      already supports them (see `operations/polygon.rs`,
      `operations/circle.rs`); just need buttons + help entries.
- [ ] **Memory keys (Store, Rcl, M+, M-)** exposed in the keypad. The
      engine already has 4 memory slots — currently only reachable from
      a physical keyboard / programmatic events.
- [ ] **dms ↔ deg** angle conversion key (visible in the CMPro
      screenshot — the `dms◄►deg` label).
- [ ] **Cost-per-unit** function. Engine TODO from existing roadmap;
      essential for "estimating" apps.
- [ ] **Save/share tape** — JSON export already exists in `calc-wasm`;
      wire up a Share Sheet on iOS, a download button on web.
- [ ] **Compound miter** is partially implemented — finish wiring the
      Corner / Spring / Miter / Bevel keys (they exist as enum variants
      but the keypad Miter page just has those four buttons; verify the
      flow end-to-end).
- [ ] **Board feet** key in keypad UI. Engine implemented.
- [ ] **Material estimates** (sheets, studs, roofing bundles) — engine
      done; surface in the EZ Calc forms layer.

---

## 2. Feature parity with Construction Master Pro

From the screenshot you sent, here's what CMPro has that we don't yet
expose. Implement the ones marked `[ ]` to match the feature checklist
reviewers compare on the store. Some are already in the engine and just
need a button + help text.

### Length / area / volume

- [x] Feet-inch-fraction with rounding (1/4, 1/8, 1/16)
- [x] Meters
- [x] Yards
- [ ] Acres key (engine: scalar × scalar → area; need an Area type for
      land if we want it tagged)
- [ ] Weight: pounds, kilograms, tons, metric tons (engine extension)
- [ ] Volume: cubic yards (for concrete), gallons, liters
- [ ] Length × Length × Length → Volume (currently returns generic; need
      a Volume variant of `Value`)

### Construction math

- [x] Pitch / Rise / Run / Diagonal solver
- [x] Hip / Valley
- [x] Jack rafter difference
- [x] Stair layout
- [x] Compound miter (corner + spring → miter + bevel)
- [x] Circle (radius, diameter, circumference, area, arc, chord, segment)
- [x] Polygon (equilateral)
- [x] Board feet
- [ ] Drywall estimation (in EZ Calc, expand it)
- [ ] Concrete volume (slab, footing, column, cone)
- [ ] Rebar spacing & count
- [ ] Baluster spacing
- [ ] Column / cone (volume + lateral area)
- [ ] Roof: studs, sheathing sheets, plates, roofing bundles
- [ ] Equal-spacing on-center divider (for studs, joists, etc.)

### UI / UX

- [x] Tape view
- [x] Long-press help overlay
- [x] Format strip
- [x] Physical keyboard support (web)
- [ ] Saved tapes list (named, persisted)
- [ ] Per-key secondary functions exposed via the small red label above
      the key — like CMPro's `Slope` over `Pitch`, `R/Wall` over `Rise`,
      etc. On iOS this is a long-press; the existing long-press
      infrastructure can be repurposed to *invoke* the secondary key
      instead of just showing help.
- [ ] Settings tab (Preferences)
- [ ] Help tab (full reference manual)

---

## 3. iOS App Store release plan

Goal: **paid app, $4.99 USD one-time, available worldwide.** No
subscription, no in-app purchase, no ads.

### 3.1 Architecture

```
crates/calc-core           ← already done, pure Rust
crates/calc-uniffi (new)   ← UniFFI wrapper, exports Swift bindings
ios/                       ← Xcode project, SwiftUI app
```

- [ ] **Create `crates/calc-uniffi`.** A thin crate that depends on
      `calc-core` and exposes a UniFFI-friendly API. Key types to
      export: `Calculator`, `KeyEvent`, `LengthFormat`, `LengthUnitKey`,
      `Snapshot { display, tape, error }`. Mirror what `calc-wasm` does
      but using UniFFI's `udl` (or proc-macro) interface.
- [ ] **Add `cargo-swift` or hand-rolled `xcframework` build script.**
      Output: `CalcEngine.xcframework` containing arm64 (device) +
      arm64-sim + x86_64-sim slices. Add a `scripts/build-ios.sh` that
      runs `cargo build --release --target aarch64-apple-ios` etc and
      assembles the xcframework.
- [ ] **Create the Xcode project** under `ios/ConstructionCalc/`. Use
      SwiftUI. iOS 17+ target is fine (lets you use latest APIs; cuts
      out only ~5% of users at this point).
- [ ] **Wire up the bindings.** Import `CalcEngine.xcframework`,
      instantiate `Calculator` once in an `@Observable` view model,
      send `KeyEvent`s on button taps.

### 3.2 SwiftUI app structure

```
ios/ConstructionCalc/
├── ConstructionCalcApp.swift     ← @main entrypoint
├── Views/
│   ├── CalculatorView.swift      ← Display + Keypad + FormatStrip
│   ├── KeypadView.swift          ← grid of buttons w/ long-press
│   ├── DisplayView.swift
│   ├── FormatStripView.swift
│   ├── TapeView.swift
│   ├── HelpOverlayView.swift
│   ├── PreferencesView.swift     ← settings tab
│   └── EZCalcListView.swift      ← form-based tools
├── ViewModels/
│   └── CalculatorVM.swift        ← @Observable wrapper around UniFFI
├── Engine/
│   └── CalcEngine.xcframework    ← built from crates/calc-uniffi
└── Assets.xcassets/              ← icon, colors
```

- [ ] **Haptics on key press.** `UIImpactFeedbackGenerator(style:
      .light)` — costs nothing and makes the app feel premium.
- [ ] **Keyboard support** via `.keyboardShortcut` on each button (for
      iPad with hardware keyboard).
- [ ] **iPad layout** — wider keypad, two-pane (calculator + tape side
      by side). Universal binary; same purchase covers iPhone + iPad.
- [ ] **Dark mode + light mode** — auto-switch via `@Environment(\
      .colorScheme)`.

### 3.3 App Store Connect setup

- [ ] **Apple Developer Program** membership ($99/year — required to
      publish). Sign up at developer.apple.com.
- [ ] **App ID** — `com.<yourname>.constructioncalc` (replace with
      your reverse-DNS).
- [ ] **App Store Connect listing.**
  - Name: `Construction Calc` (check availability — may need a
    qualifier like `Construction Calc Pro` or `BuildCalc`)
  - Subtitle (30 chars): something like `Exact rational math for builders`
  - Category: Productivity (primary), Utilities (secondary)
  - Age rating: 4+
- [ ] **Pricing.** Tier 5 (= $4.99 USD). Set to "Available in all
      countries" — App Store maps the tier to local currency
      automatically. No introductory pricing, no auto-renewing.
- [ ] **App icon.** Required sizes: 1024×1024 (App Store) plus the
      auto-generated iOS sizes via an `AppIcon.appiconset`. Design
      should look good at 60×60 home-screen size.
- [ ] **Screenshots.** Required: 6.7" iPhone (1290×2796) + 6.9" iPhone
      (1320×2868). iPad 13" (2064×2752) strongly recommended since the
      app is universal. 3–10 screenshots per device.
- [ ] **App Preview video** (optional but boosts conversion ~20%):
      15–30s screen recording.
- [ ] **Privacy nutrition label.** Almost certainly "Data Not
      Collected" — we don't have analytics, login, or network calls.
      That's a selling point; lean into it on the listing.
- [ ] **Privacy Policy URL** — required even for no-data apps. Host a
      one-page policy at e.g. `nuniesmith.github.io/construction-calc/
      privacy`.
- [ ] **Support URL** — link to a GitHub issues page or simple contact
      page.

### 3.4 Pre-submission checklist

- [ ] **No crashes** in a 10-minute manual smoke test on a physical
      device. Test all keypad pages, all unit combinations, save/share
      tape, settings persistence.
- [ ] **No `print()` / `NSLog` calls** in the shipped binary.
- [ ] **Memory leak check** with Instruments → Leaks. The Rust engine
      is leak-free by construction; check Swift side.
- [ ] **VoiceOver / accessibility pass** — each key has an
      `accessibilityLabel`. Apple checks this in review.
- [ ] **Dynamic Type** support — display text scales with the system
      font size setting.
- [ ] **Light + dark mode** both look correct.
- [ ] **Test on the smallest supported device** (iPhone SE 3rd gen,
      4.7") — keypad mustn't overflow.
- [ ] **App Store guidelines pass** — read sections 4 (Design) and 5
      (Legal) of the App Review Guidelines. Calculators are generally
      low-risk but reviewers reject for missing privacy policy, broken
      links, or asking for unnecessary permissions.

### 3.5 Submission & rollout

- [ ] First TestFlight build to yourself + 1-2 trusted testers
      (friends in trades — they'll find bugs the testing won't).
- [ ] Internal TestFlight: at least one full week of beta before
      submitting for review.
- [ ] Submit for review. Expected turnaround: 24–48 hours in 2026.
- [ ] **Auto-release after approval** OR **manual release** — choose
      manual so you can pick a launch day (Tuesday or Wednesday is
      typical for press attention).
- [ ] Launch-day artifacts:
  - [ ] Update README with App Store badge + link
  - [ ] Post on r/Construction, r/HomeImprovement, HN Show
  - [ ] Tweet / LinkedIn announcement
  - [ ] (Optional) reach out to ToolGuyd, This Old House, Pro Tool
        Reviews — they cover construction apps occasionally

### 3.6 Why $4.99 one-time will work

The competitors (Construction Master Pro, BuildCalc Pro, etc.) are
$24.99–$29.99 one-time or $4.99/month subscriptions. Positioning:

- **Sub-$5 impulse buy** — the price where people don't think twice
- **No subscription** — every Amazon review of competitor apps
  complains about the recurring charge. This is your wedge.
- **Forever updates** — Apple's StoreKit model means paid app
  customers get all future updates for free. Embrace it as marketing
  copy: "Buy once. Yours forever."
- **Local-only, no tracking** — the privacy nutrition label is a
  marketing asset.

The risk is that $4.99 is below the threshold where customers assume
quality. Counter that with:
- A polished icon
- Real screenshots showing the exact-rational result (`1/3 + 1/3 + 1/3
  = 1"`, not `0.999"`)
- Mention "exact rational math" in the subtitle / description

---

## 4. Android (later, after iOS is stable)

- [ ] `crates/calc-uniffi` already produces Kotlin bindings — re-use.
- [ ] Compose Multiplatform UI, or Jetpack Compose Android-only.
- [ ] Google Play submission: $25 one-time dev fee, similar checklist.
- [ ] Price-match $4.99 on Play.

---

## 5. Long-term ideas

- [ ] Cloud-free **iCloud sync of saved tapes** via CloudKit (no
      server needed; opt-in).
- [ ] **Tape templates** — save a calculation as a template ("garage
      slab"), recall later with new inputs.
- [ ] **PDF export** of a tape for handing to a client / inspector.
- [ ] **Apple Watch companion** — read-only display of the last
      result, voice memo to start a new tape.
- [ ] **macOS Catalyst** build — same codebase, free additional
      platform.
- [ ] **Open-source the engine** (`calc-core`) under MIT, keep the
      iOS/Android app shells closed-source. Engine OSS is a credibility
      signal and useful for contributions to obscure formulas.
