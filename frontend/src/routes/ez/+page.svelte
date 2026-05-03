<script lang="ts">
  // List of EZ Calc forms. Add new entries here as new routes are created.
  const calcs = [
    { href: '/ez/drywall', title: 'Drywall', body: 'Sheets needed for a wall with door and window cutouts.' }
    // Future:
    // { href: '/ez/baluster',  title: 'Baluster spacing', body: '…' },
    // { href: '/ez/concrete',  title: 'Concrete volume',  body: '…' },
    // { href: '/ez/rebar',     title: 'Rebar spacing',    body: '…' },
  ];
</script>

<svelte:head><title>EZ Calc · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/">← back to calculator</a>
  <h1>EZ Calc</h1>
  <p class="hint">Quick form-based calculations for common construction tasks.</p>

  <ul>
    {#each calcs as c}
      <li>
        <a href={c.href}>
          <h2>{c.title}</h2>
          <p>{c.body}</p>
        </a>
      </li>
    {/each}
  </ul>
</main>

<style>
  main { max-width: 480px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; color: #f4f6fa; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
  .back { color: #94a3b8; text-decoration: none; font-size: 0.85rem; }
  h1 { margin: 0; font-size: 1.5rem; }
  .hint { color: #94a3b8; font-size: 0.85rem; margin: 0 0 0.5rem; }
  ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.5rem; }
  li a { display: block; background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 0.6rem; padding: 0.75rem 1rem; color: inherit; text-decoration: none; }
  li a:hover { background: rgba(255, 255, 255, 0.06); }
  h2 { margin: 0; font-size: 1rem; }
  li p { margin: 0.25rem 0 0; color: #94a3b8; font-size: 0.85rem; }
</style>
