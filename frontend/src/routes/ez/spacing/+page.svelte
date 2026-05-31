<script lang="ts">
  import { calc } from '$lib/calc';
  import { gapLayout, onCenterLayout, type SpacingMode } from '$lib/spacing';

  /**
   * Equal-spacing on-center divider — generalized baluster/joist/picket
   * layout. Math lives in `$lib/spacing` (unit-tested). Two modes:
   *   gap      → fewest members so every gap ≤ a max (pickets, balusters)
   *   oncenter → members at a fixed OC across the run (joists, studs)
   */

  let mode: SpacingMode = 'gap';
  let runIn = '72';
  let widthIn = '1.5';
  let maxGapIn = '4';
  let ocIn = '16';

  $: run = parseFloat(runIn) || 0;
  $: gap = gapLayout(run, parseFloat(widthIn) || 0, parseFloat(maxGapIn) || 0);
  $: oc = onCenterLayout(run, parseFloat(ocIn) || 0);

  let saved: string | null = null;
  function saveToTape() {
    let summary: string;
    if (mode === 'gap') {
      summary =
        `Spacing (max-gap) — ${runIn}" run, ${widthIn}" members, ≤${maxGapIn}" gap` +
        ` → ${gap.count} members @ ${gap.gap.toFixed(3)}" gap (${gap.onCenter.toFixed(3)}" OC)`;
    } else {
      summary =
        `Spacing (on-center) — ${runIn}" run @ ${ocIn}" OC` +
        ` → ${oc.count} members across ${oc.bays} bays @ ${oc.spacing.toFixed(3)}" actual`;
    }
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Spacing · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Equal spacing</h1>
  <p class="hint">Even layout for joists, studs, pickets, or balusters. Pick a mode.</p>

  <section class="card">
    <div class="tabs">
      <button class:active={mode === 'gap'} on:click={() => (mode = 'gap')}>Max gap</button>
      <button class:active={mode === 'oncenter'} on:click={() => (mode = 'oncenter')}>On-center</button>
    </div>
  </section>

  <section class="card">
    <h2>Run</h2>
    <div class="row">
      <label>Run length (in)
        <input type="number" min="0" step="0.125" bind:value={runIn} />
      </label>
    </div>
  </section>

  {#if mode === 'gap'}
    <section class="card">
      <h2>Member + code</h2>
      <div class="row">
        <label>Member width (in)
          <input type="number" min="0" step="0.125" bind:value={widthIn} />
        </label>
        <label>Max gap (in)
          <input type="number" min="0" step="0.125" bind:value={maxGapIn} />
        </label>
      </div>
    </section>

    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt class="big">Members</dt><dd class="big">{gap.count}</dd>
        <dt>Actual gap</dt><dd>{gap.gap.toFixed(4)}"</dd>
        <dt>On-center</dt><dd>{gap.onCenter.toFixed(4)}"</dd>
      </dl>
      <button on:click={saveToTape}>Save to tape</button>
      {#if saved}<p class="saved">Saved.</p>{/if}
    </section>
  {:else}
    <section class="card">
      <h2>Spacing</h2>
      <div class="row">
        <label>On-center spacing (in)
          <input type="number" min="0" step="0.125" bind:value={ocIn} />
        </label>
      </div>
    </section>

    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt class="big">Members</dt><dd class="big">{oc.count}</dd>
        <dt>Bays</dt><dd>{oc.bays}</dd>
        <dt>Actual spacing</dt><dd>{oc.spacing.toFixed(4)}"</dd>
      </dl>
      <button on:click={saveToTape}>Save to tape</button>
      {#if saved}<p class="saved">Saved.</p>{/if}
    </section>
  {/if}
</main>

<style>
  main { max-width: 480px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; color: #f4f6fa; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
  .back { color: #94a3b8; text-decoration: none; font-size: 0.85rem; }
  h1 { margin: 0; font-size: 1.5rem; }
  .hint { color: #94a3b8; font-size: 0.85rem; margin: 0; }
  .card { background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 0.6rem; padding: 0.75rem 1rem; }
  .card h2 { margin: 0 0 0.5rem; font-size: 0.9rem; text-transform: uppercase; letter-spacing: 0.05em; color: #94a3b8; }
  .tabs { display: flex; gap: 0.4rem; }
  .tabs button { flex: 1; background: rgba(255, 255, 255, 0.04); border: 1px solid rgba(255, 255, 255, 0.08); color: #cbd5e1; padding: 0.45rem; border-radius: 0.4rem; font-size: 0.85rem; cursor: pointer; }
  .tabs button.active { background: #3b3a8a; border-color: #3b3a8a; color: white; }
  .row { display: flex; gap: 0.6rem; flex-wrap: wrap; }
  label { display: flex; flex-direction: column; gap: 0.2rem; font-size: 0.85rem; flex: 1; min-width: 0; }
  input { background: #0e1623; color: #f4f6fa; border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 0.4rem; padding: 0.4rem 0.5rem; font: inherit; min-width: 0; width: 100%; box-sizing: border-box; }
  .result dl { display: grid; grid-template-columns: 1fr auto; gap: 0.3rem 0.8rem; margin: 0; }
  .result dt { color: #94a3b8; font-size: 0.85rem; }
  .result dd { margin: 0; font-family: 'JetBrains Mono', ui-monospace, monospace; }
  .result dt.big, .result dd.big { font-size: 1.2rem; font-weight: 600; color: #fcd34d; padding-top: 0.4rem; }
  button { margin-top: 0.6rem; background: #d97706; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.4rem; cursor: pointer; font-weight: 500; }
  .saved { color: #94a3b8; font-size: 0.8rem; margin: 0.4rem 0 0; }
</style>
