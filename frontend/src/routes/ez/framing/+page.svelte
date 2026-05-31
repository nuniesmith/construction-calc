<script lang="ts">
  import { calc } from '$lib/calc';
  import { sheathingPanels, plateStock, headerStock } from '$lib/framing';

  /**
   * Framing materials EZ Calc — sheathing panels, wall plates, and
   * headers. Math lives in `$lib/framing` (unit-tested); this is a thin
   * form over it.
   */

  // -- Sheathing --
  let wallLenFt = '40';
  let wallHtFt = '8';
  let panelSqFt = 32; // 4×8 sheet
  let sheathWastePct = 10;
  $: areaSqFt = (parseFloat(wallLenFt) || 0) * (parseFloat(wallHtFt) || 0);
  $: panels = sheathingPanels(areaSqFt, panelSqFt, sheathWastePct);

  // -- Plates --
  let plateCount = 3; // bottom + double top
  let stockLenFt = 16;
  $: plates = plateStock(parseFloat(wallLenFt) || 0, plateCount, stockLenFt);

  // -- Headers --
  let openingWidthIn = '36';
  let openingCount = 2;
  let bearingEachIn = 1.5;
  $: headers = headerStock(parseFloat(openingWidthIn) || 0, openingCount, bearingEachIn);

  let saved: string | null = null;
  function saveToTape() {
    const summary =
      `Framing — ${wallLenFt}'×${wallHtFt}' wall:` +
      ` ${panels} sheathing panels (${sheathWastePct}% waste),` +
      ` ${plates.boards} plate boards (${plateCount}×, ${plates.totalLinFt} lin ft),` +
      ` ${openingCount} headers @ ${headers.eachIn}" (${headers.totalLinFt.toFixed(2)} lin ft)`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Framing · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Framing materials</h1>
  <p class="hint">Sheathing panels, wall plates, and headers for a stud wall.</p>

  <section class="card">
    <h2>Wall</h2>
    <div class="row">
      <label>Length (ft)
        <input type="number" min="0" step="0.5" bind:value={wallLenFt} />
      </label>
      <label>Height (ft)
        <input type="number" min="0" step="0.5" bind:value={wallHtFt} />
      </label>
    </div>
  </section>

  <section class="card">
    <h2>Sheathing</h2>
    <div class="row">
      <label>Panel size
        <select bind:value={panelSqFt}>
          <option value={32}>4×8 (32 sq ft)</option>
          <option value={36}>4×9 (36 sq ft)</option>
          <option value={40}>4×10 (40 sq ft)</option>
        </select>
      </label>
      <label>Waste %
        <input type="number" min="0" step="1" bind:value={sheathWastePct} />
      </label>
    </div>
    <p class="out">{areaSqFt.toFixed(0)} sq ft → <strong>{panels}</strong> panels</p>
  </section>

  <section class="card">
    <h2>Plates</h2>
    <div class="row">
      <label>Plates (bottom + top)
        <select bind:value={plateCount}>
          <option value={1}>1 (bottom only)</option>
          <option value={2}>2 (single top)</option>
          <option value={3}>3 (double top)</option>
        </select>
      </label>
      <label>Stock length
        <select bind:value={stockLenFt}>
          <option value={8}>8 ft</option>
          <option value={10}>10 ft</option>
          <option value={12}>12 ft</option>
          <option value={16}>16 ft</option>
        </select>
      </label>
    </div>
    <p class="out">{plates.totalLinFt} lin ft → <strong>{plates.boards}</strong> boards</p>
  </section>

  <section class="card">
    <h2>Headers</h2>
    <div class="row">
      <label>Opening width (in)
        <input type="number" min="0" step="0.5" bind:value={openingWidthIn} />
      </label>
      <label>Openings
        <input type="number" min="0" step="1" bind:value={openingCount} />
      </label>
      <label>Bearing each end (in)
        <input type="number" min="0" step="0.25" bind:value={bearingEachIn} />
      </label>
    </div>
    <p class="out">{headers.eachIn}" each → <strong>{headers.totalLinFt.toFixed(2)}</strong> lin ft total</p>
  </section>

  <section class="card result">
    <button on:click={saveToTape}>Save to tape</button>
    {#if saved}<p class="saved">Saved.</p>{/if}
  </section>
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
  .out { margin: 0.5rem 0 0; font-size: 0.9rem; color: #cbd5e1; font-family: 'JetBrains Mono', ui-monospace, monospace; }
  .out strong { color: #fcd34d; font-size: 1.1rem; }
  .result { display: flex; flex-direction: column; }
  button { background: #d97706; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.4rem; cursor: pointer; font-weight: 500; }
  .saved { color: #94a3b8; font-size: 0.8rem; margin: 0.4rem 0 0; }
</style>
