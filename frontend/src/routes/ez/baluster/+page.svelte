<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Baluster spacing EZ Calc.
   *
   * Given the inside-of-posts railing length, baluster width, and max
   * allowable gap (4" is IRC code for residential decks), compute:
   *   - number of balusters
   *   - actual gap between them (≤ max)
   *   - center-to-center spacing
   *
   * Math is local; no engine function for this yet — it's a pure
   * one-shot arithmetic problem that doesn't benefit from the
   * exact-rational engine.
   */

  let railLenIn = '72';
  let balusterWIn = '1.5';
  let maxGapIn = '4'; // IRC code: 4" sphere rule

  $: result = compute();

  interface Result {
    count: number;
    actualGapIn: number;
    spacingOnCenterIn: number;
    valid: boolean;
    error?: string;
  }

  function compute(): Result {
    const L = parseFloat(railLenIn) || 0;
    const W = parseFloat(balusterWIn) || 0;
    const maxG = parseFloat(maxGapIn) || 0;
    if (L <= 0 || W <= 0 || maxG <= 0) {
      return { count: 0, actualGapIn: 0, spacingOnCenterIn: 0, valid: false };
    }
    // The classic formula:
    //   total gaps = balusters + 1
    //   L = n*W + (n+1)*gap
    //   gap = (L - n*W) / (n+1)
    // Min n to keep gap ≤ maxG:
    //   maxG ≥ (L - n*W) / (n+1)
    //   n ≥ (L - maxG) / (W + maxG)
    const minN = (L - maxG) / (W + maxG);
    const n = Math.max(0, Math.ceil(minN));
    if (n === 0) return { count: 0, actualGapIn: L, spacingOnCenterIn: 0, valid: true };
    const actualGap = (L - n * W) / (n + 1);
    const onCenter = actualGap + W;
    return {
      count: n,
      actualGapIn: actualGap,
      spacingOnCenterIn: onCenter,
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const summary =
      `Balusters — ${railLenIn}" rail, ${balusterWIn}" balusters,` +
      ` max gap ${maxGapIn}"` +
      ` → ${result.count} balusters @ ${result.actualGapIn.toFixed(3)}" gap` +
      ` (${result.spacingOnCenterIn.toFixed(3)}" OC)`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Balusters · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Baluster spacing</h1>
  <p class="hint">Even baluster layout that satisfies a max-gap code limit (IRC: 4" sphere).</p>

  <section class="card">
    <h2>Railing</h2>
    <div class="row">
      <label>Inside-of-posts length (in)
        <input type="number" min="0" step="0.125" bind:value={railLenIn} />
      </label>
    </div>
  </section>

  <section class="card">
    <h2>Baluster + code</h2>
    <div class="row">
      <label>Baluster width (in)
        <input type="number" min="0" step="0.125" bind:value={balusterWIn} />
      </label>
      <label>Max gap (in)
        <input type="number" min="0" step="0.125" bind:value={maxGapIn} />
      </label>
    </div>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt class="big">Balusters</dt><dd class="big">{result.count}</dd>
        <dt>Actual gap</dt><dd>{result.actualGapIn.toFixed(4)}"</dd>
        <dt>On-center spacing</dt><dd>{result.spacingOnCenterIn.toFixed(4)}"</dd>
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
