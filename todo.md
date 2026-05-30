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
- **Tests** — 75 Rust tests (71 in `calc-core` + 4 in `calc-uniffi`)
  and 38 frontend TypeScript tests, all passing as of the latest
  audit. Keep this discipline; App Review prefers crashes-never apps,
  and Rust + property-style testing get you most of the way.

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

- [x] **Preferences screen** at `/preferences`, backed by
      `localStorage` (key `cc.preferences.v1`). Settings: default
      fraction resolution (1/4, 1/8, 1/16), default length format,
      degrees vs radians. Applied on app startup via a `Convert`
      KeyEvent — first render already shows the chosen mode. The
      `Preferences` shape is the contract iOS will mirror against
      `UserDefaults`.
- [x] **Saved tapes** at `/tapes`, backed by `localStorage` (key
      `cc.tapes.v1`). `💾 Save` button in the tape toolbar prompts
      for a name and persists the engine's JSON. The tapes page
      lists newest-first with Load / Delete actions and a two-step
      "Delete all". Size cap 1 MB per tape, 200 tapes total.
- [x] **Polygon & Circle** surfaced via EZ Calc forms (/ez/polygon,
      /ez/circle). Engine math at `operations/polygon.rs` /
      `operations/circle.rs` shadowed in JS for live preview; results
      pushed to tape on Save. A future improvement would add a
      `PartialPolygon` / `PartialCircle` state machine so the keypad
      can drive them directly the way Rafter does.
- [x] **Board feet** EZ Calc form (/ez/board-feet) wraps
      `operations/board_feet::board_feet`. Includes optional
      cost-per-bf to compute total cost.
- [x] **Memory keys (MS, MR, M+, MC, MC-All)** exposed via a new "Mem"
      tab on the keypad. The engine has 4 slots — UI uses slot 0
      since that's the universal calculator convention.
- [ ] **Cost-per-unit** as a first-class engine function (currently a
      board-feet-only field). Engine TODO from existing roadmap;
      essential for "estimating" apps.
- [ ] **dms ↔ deg** angle conversion key (visible in the CMPro
      screenshot — the `dms◄►deg` label). The `Angle` type already
      decomposes to DMS; just need a Mode toggle + button.
- [ ] **Save/share tape** — JSON export already exists in `calc-wasm`;
      wire up a Share Sheet on iOS, a download button on web.
- [ ] **Compound miter** is partially implemented — finish wiring the
      Corner / Spring / Miter / Bevel keys (they exist as enum variants
      but the keypad Miter page just has those four buttons; verify the
      flow end-to-end).
- [ ] **Material estimates** (sheets, studs, roofing bundles) —
      drywall is in /ez/drywall; add `/ez/studs` and `/ez/roofing` for
      the remaining two engine functions.

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
- [x] Drywall estimation (in EZ Calc)
- [x] Concrete volume (slab, column, cone) — engine module +
      `/ez/concrete` form with cubic-yard output and waste factor
- [x] Rebar spacing & count — `/ez/rebar` with bar size weight tables
- [x] Baluster spacing — `/ez/baluster` enforcing IRC 4" sphere code
- [x] Roof: roofing bundles + footprint × pitch multiplier
      (`/ez/roofing`), stud count (`/ez/studs`)
- [ ] Column / cone lateral surface area (currently volume only)
- [ ] Equal-spacing on-center divider (for joists, picket fences,
      etc.) — generalized form of baluster
- [ ] Sheathing sheets, plates, headers — extend `/ez/studs` or
      add a dedicated framing form

### UI / UX

- [x] Tape view
- [x] Long-press help overlay
- [x] Format strip
- [x] Physical keyboard support (web)
- [x] Saved tapes list (named, persisted at `/tapes`)
- [x] Settings tab (`/preferences`)
- [x] PWA install support — `static/manifest.webmanifest` +
      icons, installable on iOS Safari (Add to Home Screen),
      Android Chrome (Install app), and desktop Chrome / Edge
- [x] About / marketing page at `/about` for the App Store
      cross-link
- [ ] Per-key secondary functions exposed via the small red label above
      the key — like CMPro's `Slope` over `Pitch`, `R/Wall` over `Rise`,
      etc. On iOS this is a long-press; the existing long-press
      infrastructure can be repurposed to *invoke* the secondary key
      instead of just showing help.
- [ ] Help tab (full reference manual)
- [ ] Real PNG app icon (currently an SVG placeholder; iOS Safari
      uses it for Add-to-Home-Screen but Apple prefers 180×180 PNG)
- [ ] Service worker for true offline support (currently relies on
      browser HTTP cache)

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

- [x] **Create `crates/calc-uniffi`.** Proc-macro mode UniFFI wrapper
      around `calc-core` exporting `Calculator`, `KeyEvent`,
      `LengthFormat`, `Unit`, `FunctionKey`, `MemoryOp`, `Snapshot`,
      `CalcFfiError`. 4 end-to-end tests confirm the wrapper works
      identically to the WASM version.
- [x] **Hand-rolled xcframework build script** at `scripts/build-ios.sh`.
      Builds arm64-device + arm64-sim + x86_64-sim slices, lipos the
      sim slices, runs `uniffi-bindgen generate --language swift`, and
      assembles the final `CalcEngine.xcframework`. No external tool
      dependencies beyond Xcode + Rust. Runs on macOS only — verified
      bindings generation end-to-end in CI on Linux against the
      `.so` (Swift output looks correct: `Calculator` class +
      `handle(event:)` method + all enums).
- [ ] **Install iOS Rust targets on the build Mac.** `scripts/build-ios.sh`
      now does this for you (`rustup target add` is idempotent), but for
      reference the targets are: `aarch64-apple-ios`,
      `aarch64-apple-ios-sim`, `x86_64-apple-ios`.
- [ ] **Create the Xcode project** under `ios/ConstructionCalc/`. Use
      SwiftUI. iOS 17+ target is fine (lets you use latest APIs; cuts
      out only ~5% of users at this point).
- [ ] **Wire up the bindings.** Import `CalcEngine.xcframework`,
      instantiate `Calculator` once in an `@Observable` view model,
      send `KeyEvent`s on button taps.

#### Day-one-on-the-Mac checklist

Everything that *can* be verified off-Mac has been. When the laptop
arrives, work top to bottom:

1. `xcode-select --install` (or install full Xcode from the App Store).
2. From the repo root: `bash scripts/build-ios.sh`. It installs the
   Rust targets, builds all three slices, generates the Swift
   bindings, and assembles `ios/CalcEngine.xcframework`. The generated
   artifacts are already gitignored.
3. If the build fails, the most likely culprits (none reproducible off
   a Mac, so flagged here):
   - **`panic = "abort"` vs UniFFI.** The workspace release profile in
     `Cargo.toml` sets `panic = "abort"` (great for wasm size). UniFFI
     wraps exported calls in `catch_unwind` to turn Rust panics into
     Swift errors — with `abort`, a panic crashes the whole app
     instead. The engine returns `Result` for every fallible path and
     overflow wraps (not panics) in release, so this is *unlikely* to
     bite, but if you see hard crashes on device, add an
     iOS-specific profile with `panic = "unwind"` and rebuild.
   - **Bitcode / minimum deployment target** mismatches — set the
     framework's min iOS version to match the app target.
4. Drag `ios/CalcEngine.xcframework` into the Xcode project's
   *General → Frameworks, Libraries, and Embedded Content*, and add
   `ios/Sources/CalcEngine/CalcEngine.swift` to the target's sources.
5. Smoke-test in the simulator: `import CalcEngine`, then
   `let calc = Calculator(); calc.handle(event: .digit(value: 5))` and
   confirm `displayString()` returns `"5"`.

The TypeScript shapes the iOS UI should mirror are already settled and
won't drift: see `frontend/src/lib/calc.ts` (`Key`/`Unit`/`FunctionKey`)
and `frontend/src/lib/preferences.ts` (`Preferences` →
`UserDefaults`). The web `Keypad.svelte` / `FormatStrip.svelte` are the
reference layouts to port.

### 3.2 SwiftUI app structure

- [x] **Scaffolded the SwiftUI sources** under `ios/ConstructionCalc/`
      (committed; compiles once `CalcEngine.xcframework` is linked).
      Written against the *exact* generated binding API (verified by
      regenerating the Swift from the `.so` on Linux). See
      `ios/README.md` for the click-by-click Xcode assembly steps.

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

- [x] **Haptics on key press** — `UIImpactFeedbackGenerator(.light)`
      in `Haptics.swift`, fired from the view model's `send`.
- [ ] **EZ Calc forms** — the 9 web forms (`/ez/*`) aren't ported yet;
      they're pure-Swift arithmetic + a tape note, easy to add as a
      navigation list once the core app builds.
- [ ] **Keyboard support** via `.keyboardShortcut` on each button (for
      iPad with hardware keyboard).
- [ ] **iPad layout** — wider keypad, two-pane (calculator + tape side
      by side). Universal binary; same purchase covers iPhone + iPad.
- [ ] **App icon + Assets.xcassets** — reuse the web SVG mark as a
      starting point; needs a 1024×1024 PNG for the store.
- [ ] **Light mode** — the app is dark-locked today
      (`preferredColorScheme(.dark)`); add a light palette later.

> **Engine gap CLOSED (angle mode).** The degrees/radians preference is
> now wired end to end: `KeyEvent::SetAngleMode(bool)` in `calc-core`
> (with handler + tests), surfaced through `calc-wasm`, `calc-uniffi`,
> and the CLI (`angle deg|rad`). The web app and iOS both send it at
> launch and live from the preferences screen, so the toggle actually
> changes how trig keys read a plain number. Verified by tests that
> compare `sin(30)` in degrees (0.5) vs radians (≈ −0.988).

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
