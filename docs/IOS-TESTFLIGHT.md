# Shipping Construction Calc to TestFlight — without a Mac

The iOS app is a native SwiftUI front-end over the Rust `calc-core` engine (via the
`calc-uniffi` Swift bindings). Building/signing/packaging a native iOS app *requires*
Apple's macOS toolchain — there is no Linux path. But you never need to **own** a
Mac: the `.github/workflows/ios.yml` pipeline does the whole thing on GitHub's
**macOS cloud runners**, and every prerequisite below is created from Linux + a
browser.

```
push tag ios-v*  ──►  GitHub macOS runner
                        ├─ build CalcEngine.xcframework (Rust → iOS)   scripts/build-ios.sh
                        ├─ xcodegen generate (project.yml → .xcodeproj)
                        ├─ sign with your cert + profile (from secrets)
                        └─ fastlane → upload to TestFlight (App Store Connect API key)
```

> **One honest caveat.** The SwiftUI sources have never been through a Swift
> compiler (see `ios/README.md`). The pipeline gives you the *build environment*;
> the first run or two will likely surface Swift errors to fix. You can iterate
> through the CI logs (slow), or rent a cloud Mac for a few hours (MacinCloud ~$1/hr,
> AWS EC2 mac) for the one-time shakedown with live Xcode, then hand all future
> builds back to CI. See **Troubleshooting**.

---

## One-time setup (all from Linux + browser)

You already have an Apple Developer Program membership. Have your **Team ID** handy
(developer.apple.com → Membership — a 10-char string like `A1B2C3D4E5`).

### 1. Register the App ID + create the app record

- **developer.apple.com → Certificates, IDs & Profiles → Identifiers → +** → *App
  IDs* → *App* → Description "Construction Calc", Bundle ID **explicit**
  `com.nuniesmith.constructioncalc`. (No special capabilities needed.)
- **appstoreconnect.apple.com → Apps → + → New App** → pick that bundle ID, name
  *Construction Calc*, primary language, an SKU (any string, e.g. `constructioncalc`),
  and set the price to your **$4.99** tier later under *Pricing and Availability*.

### 2. App Store Connect API key (for uploading)

- **appstoreconnect.apple.com → Users and Access → Integrations → App Store Connect
  API → +** → role **App Manager**.
- Download the **`AuthKey_XXXXXXXXXX.p8`** (you can only download it once). Note the
  **Key ID** and the **Issuer ID** shown on that page.

### 3. Distribution certificate (no Mac — OpenSSL)

```bash
# a) Private key + Certificate Signing Request
openssl genrsa -out ios_dist.key 2048
openssl req -new -key ios_dist.key -out ios_dist.csr \
  -subj "/CN=Construction Calc Distribution/O=nuniesmith/C=CA"
```

- Upload `ios_dist.csr` at **developer.apple.com → Certificates → + → Apple
  Distribution**. Download the resulting `distribution.cer`.

```bash
# b) Convert the cert + key into a password-protected .p12
openssl x509 -in distribution.cer -inform DER -out distribution.pem -outform PEM
openssl pkcs12 -export -legacy \
  -inkey ios_dist.key -in distribution.pem \
  -name "Apple Distribution" \
  -out ios_dist.p12 -passout pass:CHOOSE_A_P12_PASSWORD
```

### 4. App Store provisioning profile

- **developer.apple.com → Profiles → + → App Store Connect** (Distribution) → App ID
  `com.nuniesmith.constructioncalc` → select the distribution cert from step 3 →
  give it a **name** (remember it exactly) → download the `.mobileprovision`.

### 5. Base64-encode the binaries for the secrets

```bash
base64 -w0 ios_dist.p12                       > p12.b64
base64 -w0 *.mobileprovision                  > profile.b64
base64 -w0 AuthKey_XXXXXXXXXX.p8              > asckey.b64
```

### 6. Set the GitHub repository secrets

**Repo → Settings → Secrets and variables → Actions → New repository secret** for
each:

| Secret | Value |
|---|---|
| `BUILD_CERTIFICATE_BASE64` | contents of `p12.b64` |
| `P12_PASSWORD` | the password from step 3b |
| `BUILD_PROVISION_PROFILE_BASE64` | contents of `profile.b64` |
| `PROVISIONING_PROFILE_NAME` | the profile name from step 4 |
| `KEYCHAIN_PASSWORD` | any random string |
| `APPLE_TEAM_ID` | your 10-char Team ID |
| `ASC_KEY_ID` | the API Key ID from step 2 |
| `ASC_ISSUER_ID` | the Issuer ID from step 2 |
| `ASC_KEY_CONTENT` | contents of `asckey.b64` |

> Delete the local `*.key`, `*.p12`, `*.p8`, `*.b64` files afterward — the secrets
> live in GitHub now.

---

## Build a beta

- **Manual:** GitHub → **Actions → "iOS TestFlight" → Run workflow**.
- **By tag:** `git tag ios-v0.1.0 && git push origin ios-v0.1.0`.

On success the build lands in **App Store Connect → TestFlight** after ~5–15 min of
Apple-side processing. Add yourself under **Internal Testing** (no review, installs
via the TestFlight app immediately). External testers need a one-time, quick beta
review.

> **Export compliance:** this app uses no non-exempt encryption. The first build
> per version, answer "No" to the encryption question in App Store Connect (or it
> may prompt in TestFlight). That's a one-time toggle per version.

---

## Troubleshooting (first-build shakedown)

The Swift has never compiled, so budget for a couple of red runs. Download the
**`ios-build` artifact** (gym logs) from the failed run to read the real errors.

- **`No such module 'CalcEngine'`** — the framework-module route didn't link.
  Fallback to a single app target: in `ios/project.yml`, delete the `CalcEngine`
  framework target, add `- path: Sources/CalcEngine` to the app target's `sources`,
  and remove the `import CalcEngine` lines from the five app files that have them
  (`grep -rl 'import CalcEngine' ios/ConstructionCalc`). The engine types then live
  in the app module directly.
- **Swift compile errors in the app sources** — expected for never-compiled code.
  The engine *binding* usage was reviewed and is correct against the generated API;
  errors are most likely SwiftUI layout/availability nits. Fix and re-run, or do the
  one-time cloud-Mac shakedown (live Xcode previews make this 10× faster).
- **Code-signing failure** — confirm the provisioning profile (step 4) was built
  from the *same* distribution cert (step 3) and the `com.nuniesmith.constructioncalc`
  App ID, and that `APPLE_TEAM_ID` / `PROVISIONING_PROFILE_NAME` match exactly.
- **`-legacy` openssl flag** — older OpenSSL (1.x) doesn't need/accept `-legacy`;
  newer OpenSSL (3.x) needs it so Apple's tooling can read the `.p12`. Drop it if your
  `openssl` rejects it.

Once the first build is green, every subsequent push of an `ios-v*` tag ships a new
TestFlight build with zero Mac involvement.
