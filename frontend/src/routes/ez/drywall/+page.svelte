<script lang="ts">
  import { onMount } from 'svelte';
  import { calc } from '$lib/calc';

  /**
   * Drywall EZ Calc.
   *
   * Inputs:
   *   - wall width (any length unit, parsed by user input + selected unit)
   *   - wall height
   *   - door cutout w × h × count (optional)
   *   - window cutout w × h × count (optional)
   *   - sheet size (4×8, 4×12)
   *   - waste factor (%)
   *
   * Output:
   *   - net wall area
   *   - sheets needed (rounded up after waste)
   *
   * The form drives the engine by sending a sequence of key events that
   * leaves a final result on the tape — so the user's normal "share tape"
   * workflow captures the calc.
   */

  let widthFt = '';
  let widthIn = '';
  let heightFt = '';
  let heightIn = '';

  let doorWFt = '3';
  let doorWIn = '0';
  let doorHFt = '6';
  let doorHIn = '8';
  let doorCount = 0;

  let winWFt = '3';
  let winWIn = '0';
  let winHFt = '4';
  let winHIn = '0';
  let winCount = 0;

  let sheet: '4x8' | '4x12' = '4x8';
  let wastePct = 10;

  // Live-computed result, recomputed whenever inputs change.
  $: result = compute({
    widthFt: parseInt(widthFt) || 0,
    widthIn: parseInt(widthIn) || 0,
    heightFt: parseInt(heightFt) || 0,
    heightIn: parseInt(heightIn) || 0,
    doorWFt: parseInt(doorWFt) || 0,
    doorWIn: parseInt(doorWIn) || 0,
    doorHFt: parseInt(doorHFt) || 0,
    doorHIn: parseInt(doorHIn) || 0,
    doorCount,
    winWFt: parseInt(winWFt) || 0,
    winWIn: parseInt(winWIn) || 0,
    winHFt: parseInt(winHFt) || 0,
    winHIn: parseInt(winHIn) || 0,
    winCount,
    sheet,
    wastePct
  });

  interface Inputs {
    widthFt: number; widthIn: number;
    heightFt: number; heightIn: number;
    doorWFt: number; doorWIn: number; doorHFt: number; doorHIn: number; doorCount: number;
    winWFt: number; winWIn: number; winHFt: number; winHIn: number; winCount: number;
    sheet: '4x8' | '4x12';
    wastePct: number;
  }

  interface Result {
    grossSqFt: number;
    cutoutSqFt: number;
    netSqFt: number;
    sheetsNeeded: number;
    valid: boolean;
  }

  /**
   * Pure-JS shadow of the engine math. We deliberately don't hit the WASM
   * engine for live computation — that would put a Rust round-trip on
   * every keystroke. Instead we mirror the formula here for instant
   * feedback, and only push the final answer to the tape on "Save".
   */
  function compute(i: Inputs): Result {
    const wallW = i.widthFt * 12 + i.widthIn;
    const wallH = i.heightFt * 12 + i.heightIn;
    if (wallW <= 0 || wallH <= 0) {
      return { grossSqFt: 0, cutoutSqFt: 0, netSqFt: 0, sheetsNeeded: 0, valid: false };
    }
    const grossSqIn = wallW * wallH;
    const doorSqIn = (i.doorWFt * 12 + i.doorWIn) * (i.doorHFt * 12 + i.doorHIn) * i.doorCount;
    const winSqIn = (i.winWFt * 12 + i.winWIn) * (i.winHFt * 12 + i.winHIn) * i.winCount;
    const netSqIn = Math.max(0, grossSqIn - doorSqIn - winSqIn);
    const sheetSqIn = i.sheet === '4x8' ? 48 * 96 : 48 * 144;
    const sheets = Math.ceil((netSqIn / sheetSqIn) * (1 + i.wastePct / 100));
    return {
      grossSqFt: grossSqIn / 144,
      cutoutSqFt: (doorSqIn + winSqIn) / 144,
      netSqFt: netSqIn / 144,
      sheetsNeeded: sheets,
      valid: true
    };
  }

  /**
   * Push the result to the tape as a labelled note. We don't recompute
   * via the engine here — the JS shadow above is the trusted result;
   * the tape entry is just a record so the user can copy/share it.
   */
  function saveToTape() {
    if (!result.valid) return;
    const summary = [
      `Drywall — ${widthFt}'${widthIn}" × ${heightFt}'${heightIn}"`,
      ` (${result.netSqFt.toFixed(1)} sq ft net,`,
      ` ${doorCount} door + ${winCount} window cutouts,`,
      ` ${sheet} @ ${wastePct}% waste)`,
      ` → ${result.sheetsNeeded} sheets`
    ].join('');
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }

  let saved: string | null = null;
</script>

<svelte:head><title>Drywall · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/">← back</a>
  <h1>Drywall</h1>
  <p class="hint">Sheets needed for a wall, with door and window deductions.</p>

  <section class="card">
    <h2>Wall</h2>
    <div class="row">
      <label>Width
        <span class="ftIn">
          <input type="number" min="0" bind:value={widthFt} placeholder="ft" />
          <input type="number" min="0" max="11" bind:value={widthIn} placeholder="in" />
        </span>
      </label>
      <label>Height
        <span class="ftIn">
          <input type="number" min="0" bind:value={heightFt} placeholder="ft" />
          <input type="number" min="0" max="11" bind:value={heightIn} placeholder="in" />
        </span>
      </label>
    </div>
  </section>

  <section class="card">
    <h2>Door cutouts</h2>
    <div class="row">
      <label>Width
        <span class="ftIn">
          <input type="number" min="0" bind:value={doorWFt} placeholder="ft" />
          <input type="number" min="0" max="11" bind:value={doorWIn} placeholder="in" />
        </span>
      </label>
      <label>Height
        <span class="ftIn">
          <input type="number" min="0" bind:value={doorHFt} placeholder="ft" />
          <input type="number" min="0" max="11" bind:value={doorHIn} placeholder="in" />
        </span>
      </label>
      <label>Count <input type="number" min="0" bind:value={doorCount} /></label>
    </div>
  </section>

  <section class="card">
    <h2>Window cutouts</h2>
    <div class="row">
      <label>Width
        <span class="ftIn">
          <input type="number" min="0" bind:value={winWFt} placeholder="ft" />
          <input type="number" min="0" max="11" bind:value={winWIn} placeholder="in" />
        </span>
      </label>
      <label>Height
        <span class="ftIn">
          <input type="number" min="0" bind:value={winHFt} placeholder="ft" />
          <input type="number" min="0" max="11" bind:value={winHIn} placeholder="in" />
        </span>
      </label>
      <label>Count <input type="number" min="0" bind:value={winCount} /></label>
    </div>
  </section>

  <section class="card">
    <h2>Sheets</h2>
    <div class="row">
      <label>Size
        <select bind:value={sheet}>
          <option value="4x8">4' × 8'</option>
          <option value="4x12">4' × 12'</option>
        </select>
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
        <dt>Gross area</dt><dd>{result.grossSqFt.toFixed(1)} sq ft</dd>
        <dt>Cutouts</dt><dd>{result.cutoutSqFt.toFixed(1)} sq ft</dd>
        <dt>Net area</dt><dd>{result.netSqFt.toFixed(1)} sq ft</dd>
        <dt class="big">Sheets needed</dt><dd class="big">{result.sheetsNeeded}</dd>
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
  .ftIn { display: flex; gap: 0.3rem; }
  .ftIn input { flex: 1; }
  .result dl { display: grid; grid-template-columns: 1fr auto; gap: 0.3rem 0.8rem; margin: 0; }
  .result dt { color: #94a3b8; font-size: 0.85rem; }
  .result dd { margin: 0; font-family: 'JetBrains Mono', ui-monospace, monospace; }
  .result dt.big, .result dd.big { font-size: 1.2rem; font-weight: 600; color: #fcd34d; padding-top: 0.4rem; }
  button { margin-top: 0.6rem; background: #d97706; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.4rem; cursor: pointer; font-weight: 500; }
  .saved { color: #94a3b8; font-size: 0.8rem; margin: 0.4rem 0 0; }
</style>
