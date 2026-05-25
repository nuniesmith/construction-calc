<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Circle / Arc EZ Calc.
   *
   * Top half: full circle from any one of {radius, diameter,
   * circumference, area}.
   *
   * Bottom half: circular arc from radius + central angle, giving arc
   * length, chord, sagitta, and segment area.
   *
   * Mirrors `operations::circle::solve_circle` and `solve_arc`.
   */

  type CircleKind = 'radius' | 'diameter' | 'circumference' | 'area';

  let kind: CircleKind = 'radius';
  let valueIn = '1';
  let arcAngleDeg = '90';

  $: circle = solveCircle(kind, parseFloat(valueIn) || 0);
  $: arc = circle.valid ? solveArc(circle.radius, parseFloat(arcAngleDeg) || 0) : null;

  interface CircleResult {
    radius: number;
    diameter: number;
    circumference: number;
    areaSqIn: number;
    areaSqFt: number;
    valid: boolean;
    error?: string;
  }

  interface ArcResult {
    arcLength: number;
    chord: number;
    sagitta: number;
    segmentSqIn: number;
    valid: boolean;
    error?: string;
  }

  function solveCircle(k: CircleKind, v: number): CircleResult {
    const blank = { radius: 0, diameter: 0, circumference: 0, areaSqIn: 0, areaSqFt: 0, valid: false };
    if (!Number.isFinite(v) || v <= 0) return { ...blank, error: 'dimension must be positive' };
    let radius: number;
    if (k === 'radius') radius = v;
    else if (k === 'diameter') radius = v / 2;
    else if (k === 'circumference') radius = v / (2 * Math.PI);
    else radius = Math.sqrt(v / Math.PI);
    const areaSqIn = Math.PI * radius * radius;
    return {
      radius,
      diameter: 2 * radius,
      circumference: 2 * Math.PI * radius,
      areaSqIn,
      areaSqFt: areaSqIn / 144,
      valid: true
    };
  }

  function solveArc(radius: number, angleDeg: number): ArcResult {
    const blank = { arcLength: 0, chord: 0, sagitta: 0, segmentSqIn: 0, valid: false };
    if (!Number.isFinite(angleDeg) || angleDeg <= 0) {
      return { ...blank, error: 'angle must be positive' };
    }
    const theta = (angleDeg * Math.PI) / 180;
    return {
      arcLength: radius * theta,
      chord: 2 * radius * Math.sin(theta / 2),
      sagitta: radius * (1 - Math.cos(theta / 2)),
      segmentSqIn: 0.5 * radius * radius * (theta - Math.sin(theta)),
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!circle.valid) return;
    const arcBit = arc?.valid
      ? `; arc ${arcAngleDeg}° → length ${arc.arcLength.toFixed(3)}", chord ${arc.chord.toFixed(3)}"`
      : '';
    const summary =
      `Circle — ${kind} ${valueIn}"` +
      ` → r ${circle.radius.toFixed(3)}",` +
      ` d ${circle.diameter.toFixed(3)}",` +
      ` C ${circle.circumference.toFixed(3)}",` +
      ` area ${circle.areaSqFt.toFixed(3)} sq ft` +
      arcBit;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Circle · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Circle / Arc</h1>
  <p class="hint">Solve a circle from any one dimension, plus an optional arc by central angle.</p>

  <section class="card">
    <h2>Circle from</h2>
    <div class="row">
      <label>Quantity
        <select bind:value={kind}>
          <option value="radius">Radius</option>
          <option value="diameter">Diameter</option>
          <option value="circumference">Circumference</option>
          <option value="area">Area (sq in)</option>
        </select>
      </label>
      <label>Value
        <input type="number" min="0" step="0.001" bind:value={valueIn} />
      </label>
    </div>
  </section>

  {#if circle.valid}
    <section class="card result">
      <h2>Circle</h2>
      <dl>
        <dt>Radius</dt><dd>{circle.radius.toFixed(4)}"</dd>
        <dt>Diameter</dt><dd>{circle.diameter.toFixed(4)}"</dd>
        <dt>Circumference</dt><dd>{circle.circumference.toFixed(4)}"</dd>
        <dt class="big">Area</dt><dd class="big">{circle.areaSqFt.toFixed(3)} sq ft</dd>
      </dl>
    </section>

    <section class="card">
      <h2>Arc (optional)</h2>
      <label>Central angle (degrees)
        <input type="number" min="0" max="360" step="0.1" bind:value={arcAngleDeg} />
      </label>
    </section>

    {#if arc?.valid}
      <section class="card result">
        <h2>Arc result</h2>
        <dl>
          <dt>Arc length</dt><dd>{arc.arcLength.toFixed(4)}"</dd>
          <dt>Chord</dt><dd>{arc.chord.toFixed(4)}"</dd>
          <dt>Sagitta</dt><dd>{arc.sagitta.toFixed(4)}"</dd>
          <dt>Segment area</dt><dd>{(arc.segmentSqIn / 144).toFixed(4)} sq ft</dd>
        </dl>
      </section>
    {:else if arc?.error}
      <p class="error">{arc.error}</p>
    {/if}

    <button on:click={saveToTape}>Save to tape</button>
    {#if saved}<p class="saved">Saved.</p>{/if}
  {:else if circle.error}
    <p class="error">{circle.error}</p>
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
