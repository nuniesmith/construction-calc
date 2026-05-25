<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Polygon EZ Calc.
   *
   * Solve a regular n-sided polygon from any one of {side, apothem,
   * circumradius}. Returns side / apothem / circumradius / perimeter /
   * area / interior angle.
   *
   * JS shadow of `operations::polygon::solve`. The engine uses 1/64"
   * reification on the f64 trig; the JS preview is a straight f64
   * computation since the result is shown in decimal anyway.
   */

  type InputKind = 'side' | 'apothem' | 'circumradius';

  let sides = 6;
  let inputKind: InputKind = 'side';
  let valueIn = '1'; // value in inches

  $: result = solve(sides, inputKind, parseFloat(valueIn) || 0);

  interface Result {
    side: number;
    apothem: number;
    circumradius: number;
    perimeter: number;
    areaSqIn: number;
    areaSqFt: number;
    interiorDeg: number;
    centralDeg: number;
    valid: boolean;
    error?: string;
  }

  function solve(n: number, kind: InputKind, v: number): Result {
    const blank = {
      side: 0,
      apothem: 0,
      circumradius: 0,
      perimeter: 0,
      areaSqIn: 0,
      areaSqFt: 0,
      interiorDeg: 0,
      centralDeg: 0,
      valid: false
    };
    if (!Number.isFinite(n) || n < 3) return { ...blank, error: 'polygon must have at least 3 sides' };
    if (!Number.isFinite(v) || v <= 0) return { ...blank, error: 'dimension must be positive' };

    const halfAngle = Math.PI / n;
    let side: number, apothem: number, circumradius: number;
    if (kind === 'side') {
      side = v;
      apothem = v / (2 * Math.tan(halfAngle));
      circumradius = v / (2 * Math.sin(halfAngle));
    } else if (kind === 'apothem') {
      side = 2 * v * Math.tan(halfAngle);
      apothem = v;
      circumradius = v / Math.cos(halfAngle);
    } else {
      side = 2 * v * Math.sin(halfAngle);
      apothem = v * Math.cos(halfAngle);
      circumradius = v;
    }

    const perimeter = side * n;
    const areaSqIn = 0.5 * n * side * apothem;
    return {
      side,
      apothem,
      circumradius,
      perimeter,
      areaSqIn,
      areaSqFt: areaSqIn / 144,
      interiorDeg: ((n - 2) * 180) / n,
      centralDeg: 360 / n,
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const summary =
      `Polygon — ${sides}-sided, ${inputKind} ${valueIn}"` +
      ` → side ${result.side.toFixed(3)}",` +
      ` perimeter ${result.perimeter.toFixed(3)}",` +
      ` area ${result.areaSqFt.toFixed(2)} sq ft,` +
      ` interior ${result.interiorDeg.toFixed(2)}°`;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Polygon · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Polygon</h1>
  <p class="hint">Solve a regular (equal-sided) polygon from any one dimension.</p>

  <section class="card">
    <h2>Shape</h2>
    <div class="row">
      <label>Number of sides
        <input type="number" min="3" bind:value={sides} />
      </label>
      <label>From
        <select bind:value={inputKind}>
          <option value="side">Side length</option>
          <option value="apothem">Apothem (center → side)</option>
          <option value="circumradius">Circumradius (center → vertex)</option>
        </select>
      </label>
      <label>Value (in)
        <input type="number" min="0" step="0.001" bind:value={valueIn} />
      </label>
    </div>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Derived</h2>
      <dl>
        <dt>Side length</dt><dd>{result.side.toFixed(4)}"</dd>
        <dt>Apothem</dt><dd>{result.apothem.toFixed(4)}"</dd>
        <dt>Circumradius</dt><dd>{result.circumradius.toFixed(4)}"</dd>
        <dt>Perimeter</dt><dd>{result.perimeter.toFixed(4)}"</dd>
        <dt class="big">Area</dt><dd class="big">{result.areaSqFt.toFixed(3)} sq ft</dd>
        <dt>Interior angle</dt><dd>{result.interiorDeg.toFixed(3)}°</dd>
        <dt>Central angle</dt><dd>{result.centralDeg.toFixed(3)}°</dd>
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
