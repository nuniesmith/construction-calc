<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Concrete EZ Calc.
   *
   * Three shapes: slab (rectangular), column (cylinder), cone. Output in
   * cubic yards (the unit suppliers deliver in). Adds a waste-factor
   * multiplier — typical 5-10% for slabs, more for tight pours.
   *
   * Mirrors `operations::concrete::{slab_volume, column_volume,
   * cone_volume}` in JS for live preview. 1 cy = 46_656 cu in.
   */

  type Shape = 'slab' | 'column' | 'cone';
  const CU_IN_PER_CY = 46_656;

  let shape: Shape = 'slab';

  // Slab inputs
  let slabLenFt = '10';
  let slabWidFt = '10';
  let slabThickIn = '4';

  // Column / cone inputs
  let colDiamIn = '12';
  let colHeightFt = '8';

  let wastePct = 10;

  $: result = compute();

  interface Result {
    rawCy: number;
    withWasteCy: number;
    label: string;
    valid: boolean;
    error?: string;
  }

  function compute(): Result {
    const blank = (e?: string): Result => ({
      rawCy: 0,
      withWasteCy: 0,
      label: '',
      valid: false,
      error: e
    });
    const waste = 1 + (wastePct || 0) / 100;
    if (shape === 'slab') {
      const L = (parseFloat(slabLenFt) || 0) * 12;
      const W = (parseFloat(slabWidFt) || 0) * 12;
      const T = parseFloat(slabThickIn) || 0;
      if (L <= 0 || W <= 0 || T <= 0) return blank('every dimension must be > 0');
      const cuIn = L * W * T;
      const cy = cuIn / CU_IN_PER_CY;
      return {
        rawCy: cy,
        withWasteCy: cy * waste,
        label: `slab ${slabLenFt}' × ${slabWidFt}' × ${slabThickIn}"`,
        valid: true
      };
    }
    const D = parseFloat(colDiamIn) || 0;
    const H = (parseFloat(colHeightFt) || 0) * 12;
    if (D <= 0 || H <= 0) return blank('diameter and height must be > 0');
    const r = D / 2;
    let cuIn = Math.PI * r * r * H;
    if (shape === 'cone') cuIn /= 3;
    const cy = cuIn / CU_IN_PER_CY;
    return {
      rawCy: cy,
      withWasteCy: cy * waste,
      label: `${shape} Ø${colDiamIn}" × ${colHeightFt}'`,
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const summary =
      `Concrete — ${result.label}` +
      ` → ${result.rawCy.toFixed(2)} cy raw` +
      ` (${result.withWasteCy.toFixed(2)} cy w/ ${wastePct}% waste)`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Concrete · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Concrete volume</h1>
  <p class="hint">Cubic yards needed for a slab, column, or cone — the units suppliers deliver in.</p>

  <section class="card">
    <h2>Shape</h2>
    <label>Type
      <select bind:value={shape}>
        <option value="slab">Slab / pad / footing (rectangular)</option>
        <option value="column">Column / sonotube (cylinder)</option>
        <option value="cone">Cone / bell footing</option>
      </select>
    </label>
  </section>

  {#if shape === 'slab'}
    <section class="card">
      <h2>Slab dimensions</h2>
      <div class="row">
        <label>Length (ft)
          <input type="number" min="0" step="0.5" bind:value={slabLenFt} />
        </label>
        <label>Width (ft)
          <input type="number" min="0" step="0.5" bind:value={slabWidFt} />
        </label>
        <label>Thickness (in)
          <input type="number" min="0" step="0.5" bind:value={slabThickIn} />
        </label>
      </div>
    </section>
  {:else}
    <section class="card">
      <h2>{shape === 'column' ? 'Column' : 'Cone'} dimensions</h2>
      <div class="row">
        <label>Diameter (in)
          <input type="number" min="0" step="0.5" bind:value={colDiamIn} />
        </label>
        <label>Height (ft)
          <input type="number" min="0" step="0.5" bind:value={colHeightFt} />
        </label>
      </div>
    </section>
  {/if}

  <section class="card">
    <h2>Waste factor</h2>
    <label>Overage %
      <input type="number" min="0" max="50" bind:value={wastePct} />
    </label>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt>Raw volume</dt><dd>{result.rawCy.toFixed(3)} cy</dd>
        <dt class="big">Order (w/ waste)</dt><dd class="big">{result.withWasteCy.toFixed(2)} cy</dd>
      </dl>
      <button on:click={saveToTape}>Save to tape</button>
      {#if saved}<p class="saved">Saved.</p>{/if}
    </section>
  {:else if result.error}
    <p class="error">{result.error}</p>
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
  .error { color: #ff7676; font-size: 0.85rem; margin: 0; }
  button { margin-top: 0.6rem; background: #d97706; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.4rem; cursor: pointer; font-weight: 500; }
  .saved { color: #94a3b8; font-size: 0.8rem; margin: 0.4rem 0 0; }
</style>
