<script lang="ts">
  // Shared head tags — PWA install affordance, OG / Twitter card so
  // the site previews cleanly when shared, theme color so mobile
  // browsers tint the address bar to match the app, and the icon
  // links so a tab favicon shows up everywhere.
</script>

<svelte:head>
  <meta name="description" content="Exact-rational construction math calculator. Feet-inch-fraction without float drift, plus rafter / stair / polygon / circle solvers and estimating forms." />
  <meta name="color-scheme" content="dark" />
  <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
  <link rel="apple-touch-icon" href="/icon-512.svg" />
  <link rel="manifest" href="/manifest.webmanifest" />

  <!-- Open Graph (Facebook, LinkedIn, iMessage previews) -->
  <meta property="og:title" content="Construction Calc" />
  <meta property="og:description" content="Exact-rational construction math: feet-inch-fraction without float drift, plus rafter / stair / polygon / circle / concrete / rebar solvers." />
  <meta property="og:image" content="/icon-512.svg" />
  <meta property="og:type" content="website" />

  <!-- Twitter / X card -->
  <meta name="twitter:card" content="summary" />
  <meta name="twitter:title" content="Construction Calc" />
  <meta name="twitter:description" content="Exact-rational construction math without float drift." />
  <meta name="twitter:image" content="/icon-512.svg" />
</svelte:head>

<slot />
