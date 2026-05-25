<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Rebar grid EZ Calc.
   *
   * For a rectangular slab with rebar in a grid at given on-center
   * spacing, returns:
   *   - bars in each direction (count + length each)
   *   - total linear feet of rebar
   *   - total weight (per common bar sizes — #3 through #6)
   *
   * Adds an overlap factor for splicing (typical: 40× bar diameter for
   * #4, which works out to a ~5% length premium on a 20' slab).
   */

  let slabLenFt = '20';
  let slabWidFt = '10';
  let ocIn = 12;
  let barSize = '#4';
  let overlapPct = 5;

  // Approximate weight per linear foot for plain US rebar.
  const weightPerFt: Record<string, number> = {
    '#3': 0.376,
    '#4': 0.668,
    '#5': 1.043,
    '#6': 1.502
  };

  $: result = compute();

  interface Result {
    barsLong: number;
    barsShort: number;
    totalLengthFt: number;
    weightLbs: number;
    valid: boolean;
  }

  function compute(): Result {
    const L = parseFloat(slabLenFt) || 0;
    const W = parseFloat(slabWidFt) || 0;
    const oc = ocIn / 12;
    if (L <= 0 || W <= 0 || oc <= 0) {
      return { barsLong: 0, barsShort: 0, totalLengthFt: 0, weightLbs: 0, valid: false };
    }
    // Bars running long: as many as fit across the width at oc spacing,
    // each one is L long. Plus end bars.
    const barsLong = Math.floor(W / oc) + 1;
    const barsShort = Math.floor(L / oc) + 1;
    const lenFt = (barsLong * L + barsShort * W) * (1 + overlapPct / 100);
    const wpf = weightPerFt[barSize] ?? 0;
    return {
      barsLong,
      barsShort,
      totalLengthFt: lenFt,
      weightLbs: lenFt * wpf,
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const summary =
      `Rebar — ${slabLenFt}'×${slabWidFt}' slab, ${barSize} @ ${ocIn}" OC,` +
      ` ${overlapPct}% overlap` +
      ` → ${result.barsLong} long × ${slabLenFt}' + ${result.barsShort} short × ${slabWidFt}'` +
      ` = ${result.totalLengthFt.toFixed(0)} ft (${result.weightLbs.toFixed(0)} lb)`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Rebar · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Rebar grid</h1>
  <p class="hint">Rebar count, linear feet, and weight for a rectangular slab on grid.</p>

  <section class="card">
    <h2>Slab</h2>
    <div class="row">
      <label>Length (ft)
        <input type="number" min="0" step="0.5" bind:value={slabLenFt} />
      </label>
      <label>Width (ft)
        <input type="number" min="0" step="0.5" bind:value={slabWidFt} />
      </label>
    </div>
  </section>

  <section class="card">
    <h2>Grid & bar</h2>
    <div class="row">
      <label>OC spacing
        <select bind:value={ocIn}>
          <option value={6}>6"</option>
          <option value={8}>8"</option>
          <option value={12}>12"</option>
          <option value={16}>16"</option>
          <option value={18}>18"</option>
          <option value={24}>24"</option>
        </select>
      </label>
      <label>Bar size
        <select bind:value={barSize}>
          <option value="#3">#3 (3/8")</option>
          <option value="#4">#4 (1/2")</option>
          <option value="#5">#5 (5/8")</option>
          <option value="#6">#6 (3/4")</option>
        </select>
      </label>
      <label>Overlap %
        <input type="number" min="0" max="20" bind:value={overlapPct} />
      </label>
    </div>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt>Bars (long direction)</dt><dd>{result.barsLong} × {slabLenFt}'</dd>
        <dt>Bars (short direction)</dt><dd>{result.barsShort} × {slabWidFt}'</dd>
        <dt class="big">Total length</dt><dd class="big">{result.totalLengthFt.toFixed(0)} ft</dd>
        <dt class="big">Weight</dt><dd class="big">{result.weightLbs.toFixed(0)} lb</dd>
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
  input, select { background: #0e1623; color: #f4f6fa; border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 0.4rem; padding: 0.4rem 0.5rem; font: inherit; min-width: 0; width: 100%; box-sizing: border-box; }
  .result dl { display: grid; grid-template-columns: 1fr auto; gap: 0.3rem 0.8rem; margin: 0; }
  .result dt { color: #94a3b8; font-size: 0.85rem; }
  .result dd { margin: 0; font-family: 'JetBrains Mono', ui-monospace, monospace; }
  .result dt.big, .result dd.big { font-size: 1.2rem; font-weight: 600; color: #fcd34d; padding-top: 0.4rem; }
  button { margin-top: 0.6rem; background: #d97706; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.4rem; cursor: pointer; font-weight: 500; }
  .saved { color: #94a3b8; font-size: 0.8rem; margin: 0.4rem 0 0; }
</style>
