<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Roofing bundles EZ Calc.
   *
   * Asphalt shingles are sold by the bundle; coverage varies by product
   * but 3-tab is typically 33.3 sq ft/bundle and laminate ("architectural")
   * is usually 32-33 sq ft/bundle. 100 sq ft = 1 "square" — the unit
   * roofers price by — and 3 bundles ≈ 1 square for most products.
   *
   * Inputs: pitched roof area (footprint × pitch multiplier), or direct
   * area. Includes a starter / hip-ridge waste factor.
   */

  type Input = 'direct' | 'footprint';
  let inputKind: Input = 'footprint';

  let directSqFt = '';

  let footprintLenFt = '40';
  let footprintWidFt = '30';
  // Pitch multiplier table — for a gabled roof, the actual roof area is
  // footprint × this factor (per slope). Rounded to 2 decimal places.
  let pitchNumerator = 6; // X in X/12 pitch
  // Pitch multipliers per side, applied once because both slopes meet at
  // the ridge → total roof area = footprint × multiplier.
  function pitchMultiplier(x: number): number {
    return Math.sqrt(1 + (x / 12) ** 2);
  }

  let coveragePerBundle = 33.3;
  let wastePct = 10;

  $: result = compute();

  interface Result {
    roofSqFt: number;
    squares: number;
    bundles: number;
    valid: boolean;
  }

  function compute(): Result {
    let area: number;
    if (inputKind === 'direct') {
      area = parseFloat(directSqFt) || 0;
    } else {
      const L = parseFloat(footprintLenFt) || 0;
      const W = parseFloat(footprintWidFt) || 0;
      area = L * W * pitchMultiplier(pitchNumerator);
    }
    if (area <= 0 || coveragePerBundle <= 0) {
      return { roofSqFt: 0, squares: 0, bundles: 0, valid: false };
    }
    const waste = 1 + (wastePct || 0) / 100;
    const bundles = Math.ceil((area / coveragePerBundle) * waste);
    return {
      roofSqFt: area,
      squares: area / 100,
      bundles,
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const src =
      inputKind === 'direct'
        ? `${directSqFt} sq ft direct`
        : `${footprintLenFt}'×${footprintWidFt}' footprint, ${pitchNumerator}/12 pitch`;
    const summary =
      `Roofing — ${src}` +
      ` → ${result.roofSqFt.toFixed(0)} sq ft (${result.squares.toFixed(2)} sq)` +
      ` → ${result.bundles} bundles @ ${coveragePerBundle} sq ft each, ${wastePct}% waste`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Roofing · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Roofing bundles</h1>
  <p class="hint">Asphalt-shingle bundles for a roof, from direct area or footprint + pitch.</p>

  <section class="card">
    <h2>Input</h2>
    <label>From
      <select bind:value={inputKind}>
        <option value="footprint">Footprint × pitch</option>
        <option value="direct">Direct area (sq ft)</option>
      </select>
    </label>
  </section>

  {#if inputKind === 'footprint'}
    <section class="card">
      <h2>Footprint + pitch</h2>
      <div class="row">
        <label>Length (ft)
          <input type="number" min="0" step="1" bind:value={footprintLenFt} />
        </label>
        <label>Width (ft)
          <input type="number" min="0" step="1" bind:value={footprintWidFt} />
        </label>
        <label>Pitch (x/12)
          <input type="number" min="0" step="1" bind:value={pitchNumerator} />
        </label>
      </div>
      <p class="hint">Multiplier {pitchMultiplier(pitchNumerator).toFixed(3)}× applied to footprint area.</p>
    </section>
  {:else}
    <section class="card">
      <h2>Roof area</h2>
      <label>Total (sq ft)
        <input type="number" min="0" step="1" bind:value={directSqFt} />
      </label>
    </section>
  {/if}

  <section class="card">
    <h2>Bundle & waste</h2>
    <div class="row">
      <label>Coverage per bundle (sq ft)
        <input type="number" min="0" step="0.1" bind:value={coveragePerBundle} />
      </label>
      <label>Waste %
        <input type="number" min="0" max="50" bind:value={wastePct} />
      </label>
    </div>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt>Roof area</dt><dd>{result.roofSqFt.toFixed(0)} sq ft</dd>
        <dt>Squares</dt><dd>{result.squares.toFixed(2)} sq</dd>
        <dt class="big">Bundles</dt><dd class="big">{result.bundles}</dd>
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
