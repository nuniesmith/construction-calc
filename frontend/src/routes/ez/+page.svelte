<script lang="ts">
  // List of EZ Calc forms grouped by trade so the page scans
  // quickly as it grows. Add new entries in the appropriate group.
  interface Calc {
    href: string;
    title: string;
    body: string;
  }
  interface Group {
    title: string;
    calcs: Calc[];
  }

  const groups: Group[] = [
    {
      title: 'Framing',
      calcs: [
        { href: '/ez/board-feet', title: 'Board feet', body: 'Total board feet for a count of lumber pieces, with optional cost.' },
        { href: '/ez/studs', title: 'Studs', body: 'Field stud count for a wall at OC spacing, with corner and opening extras.' },
        { href: '/ez/baluster', title: 'Balusters', body: 'Even baluster layout that satisfies the IRC 4" sphere code.' }
      ]
    },
    {
      title: 'Concrete & masonry',
      calcs: [
        { href: '/ez/concrete', title: 'Concrete', body: 'Cubic yards for a slab, column, or cone with optional waste factor.' },
        { href: '/ez/rebar', title: 'Rebar', body: 'Bar count, linear feet, and weight for a grid in a slab.' }
      ]
    },
    {
      title: 'Finishes & roofing',
      calcs: [
        { href: '/ez/drywall', title: 'Drywall', body: 'Sheets needed for a wall with door and window cutouts.' },
        { href: '/ez/roofing', title: 'Roofing', body: 'Asphalt bundles from direct area or footprint × pitch multiplier.' }
      ]
    },
    {
      title: 'Geometry',
      calcs: [
        { href: '/ez/polygon', title: 'Polygon', body: 'Regular n-sided polygon from any one of side / apothem / circumradius.' },
        { href: '/ez/circle', title: 'Circle / Arc', body: 'Circle from radius / diameter / circumference / area, plus arc length and chord.' }
      ]
    }
  ];
</script>

<svelte:head><title>EZ Calc · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/">← back to calculator</a>
  <h1>EZ Calc</h1>
  <p class="hint">Quick form-based calculations for common construction tasks.</p>

  {#each groups as g}
    <section>
      <h2 class="group">{g.title}</h2>
      <ul>
        {#each g.calcs as c}
          <li>
            <a href={c.href}>
              <h3>{c.title}</h3>
              <p>{c.body}</p>
            </a>
          </li>
        {/each}
      </ul>
    </section>
  {/each}
</main>

<style>
  main { max-width: 480px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; color: #f4f6fa; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
  .back { color: #94a3b8; text-decoration: none; font-size: 0.85rem; }
  h1 { margin: 0; font-size: 1.5rem; }
  .hint { color: #94a3b8; font-size: 0.85rem; margin: 0 0 0.5rem; }
  section { display: flex; flex-direction: column; gap: 0.4rem; }
  h2.group { margin: 0.5rem 0 0; font-size: 0.78rem; text-transform: uppercase; letter-spacing: 0.08em; color: #fcd34d; font-weight: 600; }
  ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.4rem; }
  li a { display: block; background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 0.6rem; padding: 0.6rem 0.9rem; color: inherit; text-decoration: none; }
  li a:hover { background: rgba(255, 255, 255, 0.06); }
  h3 { margin: 0; font-size: 0.95rem; }
  li p { margin: 0.2rem 0 0; color: #94a3b8; font-size: 0.8rem; }
</style>
