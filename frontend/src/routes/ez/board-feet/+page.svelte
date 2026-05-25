<script lang="ts">
  import { calc } from '$lib/calc';

  /**
   * Board Feet EZ Calc.
   *
   * 1 board foot = 144 cubic inches (1" × 12" × 12"). Lumber prices
   * are usually quoted per board foot, so this is the unit estimators
   * need most often.
   *
   * Mirrors the engine math in `operations::board_feet::board_feet` so
   * the live preview doesn't require a WASM round-trip per keystroke.
   * The final answer is pushed to the tape on Save for sharing.
   */

  // Inputs in inches; nominal lumber sizes are quoted in inches already.
  let thicknessIn = '1.5'; // a "2x4" is actually 1.5 × 3.5
  let widthIn = '3.5';
  let lengthFt = '8';
  let quantity = 1;
  let pricePerBf = '';

  $: result = compute(
    parseFloat(thicknessIn) || 0,
    parseFloat(widthIn) || 0,
    parseFloat(lengthFt) || 0,
    quantity,
    parseFloat(pricePerBf) || 0
  );

  interface Result {
    bfEach: number;
    bfTotal: number;
    totalCost: number | null;
    valid: boolean;
  }

  function compute(t: number, w: number, lenFt: number, qty: number, price: number): Result {
    if (t <= 0 || w <= 0 || lenFt <= 0 || qty <= 0) {
      return { bfEach: 0, bfTotal: 0, totalCost: null, valid: false };
    }
    const lenIn = lenFt * 12;
    const bfEach = (t * w * lenIn) / 144;
    const bfTotal = bfEach * qty;
    return {
      bfEach,
      bfTotal,
      totalCost: price > 0 ? bfTotal * price : null,
      valid: true
    };
  }

  let saved: string | null = null;
  function saveToTape() {
    if (!result.valid) return;
    const cost = result.totalCost !== null ? ` @ $${pricePerBf}/bf = $${result.totalCost.toFixed(2)}` : '';
    const summary =
      `Board feet — ${quantity}× (${thicknessIn}" × ${widthIn}" × ${lengthFt}')` +
      ` = ${result.bfTotal.toFixed(2)} bf` +
      cost;
    calc.send({ type: 'note', text: summary });
    saved = summary;
  }
</script>

<svelte:head><title>Board feet · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/ez">← back</a>
  <h1>Board feet</h1>
  <p class="hint">Total board feet for a count of lumber pieces, with optional total cost.</p>

  <section class="card">
    <h2>Piece dimensions</h2>
    <div class="row">
      <label>Thickness (in)
        <input type="number" min="0" step="0.125" bind:value={thicknessIn} />
      </label>
      <label>Width (in)
        <input type="number" min="0" step="0.125" bind:value={widthIn} />
      </label>
      <label>Length (ft)
        <input type="number" min="0" step="1" bind:value={lengthFt} />
      </label>
    </div>
  </section>

  <section class="card">
    <h2>Quantity & price</h2>
    <div class="row">
      <label>Quantity
        <input type="number" min="0" bind:value={quantity} />
      </label>
      <label>Price ($/bf, optional)
        <input type="number" min="0" step="0.01" bind:value={pricePerBf} placeholder="0.00" />
      </label>
    </div>
  </section>

  {#if result.valid}
    <section class="card result">
      <h2>Result</h2>
      <dl>
        <dt>Board feet each</dt><dd>{result.bfEach.toFixed(3)} bf</dd>
        <dt class="big">Total board feet</dt><dd class="big">{result.bfTotal.toFixed(2)} bf</dd>
        {#if result.totalCost !== null}
          <dt class="big">Total cost</dt><dd class="big">${result.totalCost.toFixed(2)}</dd>
        {/if}
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
