<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Studs on-center EZ Calc.
   *
   * Wraps `operations::materials::studs_on_center` — count the studs
   * needed for a wall at a given on-center spacing, including both
   * end studs. Adds standard extras for corners and openings since
   * those are usually doubled or tripled.
   */

  let wallLenFt = '12';
  let ocIn = 16;
  let cornerCount = 0;
  let openingCount = 0;

  $: result = compute();

  interface Result {
    studsField: number;
    extras: number;
    total: number;
    valid: boolean;
  }

  function compute(): Result {
    const lengthIn = (parseFloat(wallLenFt) || 0) * 12;
    if (lengthIn <= 0 || ocIn <= 0) return { studsField: 0, extras: 0, total: 0, valid: false };
    // Mirrors the engine: ceil(length/OC) + 1 for both ends.
    const field = Math.ceil(lengthIn / ocIn) + 1;
    // Industry rule of thumb: 2 extra studs per corner (the post + a backer),
    // 2 extra per rough opening (king + jack/trimmer).
    const extras = cornerCount * 2 + openingCount * 2;
    return { studsField: field, extras, total: field + extras, valid: true };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const summary =
      `Studs — ${wallLenFt}' wall @ ${ocIn}" OC` +
      ` → ${result.studsField} field` +
      ` + ${result.extras} extras (${cornerCount} corners, ${openingCount} openings)` +
      ` = ${result.total} total`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Studs · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Studs on-center</h1>
  <p class="hint">Stud count for a wall at standard on-center spacing, with corner and opening extras.</p>

  <section class="card">
    <h2>Wall</h2>
    <div class="row">
      <label>Length (ft)
        <input type="number" min="0" step="0.5" bind:value={wallLenFt} />
      </label>
      <label>On-center spacing
        <select bind:value={ocIn}>
          <option value={12}>12"</option>
          <option value={16}>16"</option>
          <option value={19.2}>19.2"</option>
          <option value={24}>24"</option>
        </select>
      </label>
    </div>
  </section>

  <section class="card">
    <h2>Extras</h2>
    <div class="row">
      <label>Corners (2 studs each)
        <input type="number" min="0" bind:value={cornerCount} />
      </label>
      <label>Openings (2 studs each)
        <input type="number" min="0" bind:value={openingCount} />
      </label>
    </div>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt>Field studs</dt><dd>{result.studsField}</dd>
        <dt>Extras</dt><dd>{result.extras}</dd>
        <dt class="big">Total studs</dt><dd class="big">{result.total}</dd>
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
