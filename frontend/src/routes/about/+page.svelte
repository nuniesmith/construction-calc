<script lang="ts">
  // Marketing / about page. Acts as the landing for people who follow
  // a link from the App Store listing or a forum post — explains what
  // makes this calculator different in 30 seconds.
</script>

<svelte:head>
  <title>About · Construction Calc</title>
</svelte:head>

<main>
  <a class="back" href="/">← back to calculator</a>
  <h1>Construction Calc</h1>
  <p class="lead">Exact-rational math for builders. Built so framers, finishers, and concrete crews can trust the result.</p>

  <section class="card">
    <h2>Why this calculator</h2>
    <p>
      Most calculators store values as 64-bit floats. That's fine for
      science but lies on construction sites — <code>1/3 + 1/3 + 1/3</code>
      doesn't equal <code>1</code>, it equals
      <code>0.9999999999999999</code>. Over a stair stringer or a long
      stud layout that drift adds up.
    </p>
    <p>
      This calculator stores every length as an <strong>exact rational</strong>
      number of inches. <code>1/3 + 1/3 + 1/3 = 1"</code> on the dot.
      The same goes for <code>1/16" + 1/16" + 1/16" + ... = 1"</code>
      after sixteen presses — no rounding error, no compounding drift.
    </p>
  </section>

  <section class="card">
    <h2>What's in the box</h2>
    <ul>
      <li><strong>Feet-inch-fraction arithmetic</strong> with selectable rounding (1/4", 1/8", 1/16")</li>
      <li><strong>Rafter solver</strong> — any two of pitch / rise / run / diagonal</li>
      <li><strong>Stair layout</strong>, <strong>compound miter</strong>, <strong>polygon</strong>, <strong>circle / arc</strong></li>
      <li><strong>Trig keys</strong> (sin/cos/tan + inverses) with degrees / DMS preservation</li>
      <li><strong>Memory slots</strong>, <strong>tape</strong> with markdown / JSON export</li>
      <li><strong>EZ Calc forms</strong> for drywall, board feet, concrete (slab/column/cone), studs, roofing, balusters, rebar</li>
    </ul>
  </section>

  <section class="card">
    <h2>Privacy</h2>
    <p>
      Nothing leaves your device. No accounts, no analytics, no
      tracking. Saved tapes live in your browser's local storage.
      The iOS app will be the same — local-only by design.
    </p>
  </section>

  <section class="card">
    <h2>Coming to iOS</h2>
    <p>
      The same engine that powers this web app is wrapped in
      <a href="https://mozilla.github.io/uniffi-rs/" target="_blank" rel="noopener">UniFFI</a>
      for the upcoming iOS app — same math, native SwiftUI keypad,
      Apple Pencil / hardware keyboard support, planned at
      <strong>$4.99 one-time. No subscription, no ads, no
      in-app purchase.</strong>
    </p>
    <p>
      Buy once, yours forever. Want to be notified when it ships?
      <a href="https://github.com/nuniesmith/construction-calc" target="_blank" rel="noopener">Watch the repo on GitHub</a>.
    </p>
  </section>

  <section class="card">
    <h2>Install as a web app</h2>
    <p>
      On iOS Safari: Share → <em>Add to Home Screen</em>.<br />
      On Android Chrome: Menu → <em>Install app</em>.<br />
      On desktop Chrome / Edge: address-bar install icon.
    </p>
    <p class="small">
      The web version is free, open source (MIT), and works offline
      after first load. Source on
      <a href="https://github.com/nuniesmith/construction-calc" target="_blank" rel="noopener">GitHub</a>.
    </p>
  </section>
</main>

<style>
  main { max-width: 600px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; color: #f4f6fa; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.5; }
  .back { color: #94a3b8; text-decoration: none; font-size: 0.85rem; }
  h1 { margin: 0; font-size: 1.8rem; }
  .lead { color: #cbd5e1; font-size: 1.05rem; margin: 0; }
  .card { background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 0.6rem; padding: 0.9rem 1.1rem; }
  .card h2 { margin: 0 0 0.5rem; font-size: 1rem; text-transform: uppercase; letter-spacing: 0.05em; color: #94a3b8; }
  .card p { margin: 0 0 0.6rem; }
  .card p:last-child { margin-bottom: 0; }
  ul { margin: 0; padding-left: 1.2rem; }
  ul li { margin-bottom: 0.3rem; }
  code { background: rgba(255, 255, 255, 0.06); padding: 0.1rem 0.4rem; border-radius: 0.3rem; font-family: 'JetBrains Mono', ui-monospace, monospace; font-size: 0.9em; }
  a { color: #fcd34d; }
  .small { color: #94a3b8; font-size: 0.85rem; }
</style>
