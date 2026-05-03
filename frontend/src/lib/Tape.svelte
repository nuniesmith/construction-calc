<script lang="ts">
  import { calc } from './calc';
  $: snapshot = calc.snapshot;

  /**
   * Tape entries come back from the WASM bridge as `serde_json` values
   * matching the Rust `TapeEntry` enum. Length values inside contain a
   * raw `Rational64` numer/denom pair — we render them as approximate
   * decimal inches with a tooltip showing the exact fraction. For full
   * fidelity, the user can copy/export markdown from the toolbar; this
   * tape is a glanceable summary, not the canonical record.
   */
  function renderEntry(e: any): { kind: string; text: string; title?: string } {
    if ('Result' in e) {
      const v = e.Result;
      if ('Scalar' in v) return { kind: 'result', text: rationalApprox(v.Scalar) };
      if ('Length' in v) return rationalLength(v.Length.inches);
      if ('Area' in v) return { kind: 'result', text: `${rationalApprox(v.Area)} sq in` };
      if ('Volume' in v) return { kind: 'result', text: `${rationalApprox(v.Volume)} cu in` };
      if ('Angle' in v) return { kind: 'result', text: `${rationalApprox(v.Angle.degrees)}°` };
      return { kind: 'result', text: JSON.stringify(v) };
    }
    if ('RafterSolution' in e) {
      const r = e.RafterSolution;
      const pitch12 = (r.pitch_ratio.numer / r.pitch_ratio.denom) * 12;
      const rise = rationalLength(r.rise.inches);
      const run = rationalLength(r.run.inches);
      const diag = rationalLength(r.diagonal.inches);
      return {
        kind: 'rafter',
        text: `Rafter: ${pitch12.toFixed(2)}/12 — rise ${rise.text}, run ${run.text}, diag ${diag.text}`,
        title: 'Rafter solution'
      };
    }
    if ('Note' in e) {
      return { kind: 'note', text: e.Note };
    }
    return { kind: 'unknown', text: JSON.stringify(e) };
  }

  /**
   * Render a Rational64 as a decimal with sensible precision. Uses
   * BigInt for the division so we don't lose precision for the giant
   * denominators that can come out of metric conversions
   * (1mm = 5/127 in, so 100mm = 500/127 — fine — but a long chain of
   * conversions can build up much larger numerators/denominators that
   * exceed Number.MAX_SAFE_INTEGER).
   */
  function rationalApprox(r: { numer: number | string; denom: number | string }, precision = 4): string {
    if (!r) return '?';
    const n = BigInt(r.numer);
    const d = BigInt(r.denom);
    if (d === 0n) return 'NaN';
    const sign = (n < 0n) !== (d < 0n) ? '-' : '';
    const an = n < 0n ? -n : n;
    const ad = d < 0n ? -d : d;
    const whole = an / ad;
    let rem = an % ad;
    if (rem === 0n) return `${sign}${whole}`;
    let frac = '';
    for (let i = 0; i < precision && rem !== 0n; i++) {
      rem *= 10n;
      frac += (rem / ad).toString();
      rem %= ad;
    }
    // Trim trailing zeros
    frac = frac.replace(/0+$/, '');
    return frac ? `${sign}${whole}.${frac}` : `${sign}${whole}`;
  }

  /**
   * Render a Rational64 inches value as feet-inch-fraction at 1/16"
   * precision — matches the calculator's default display mode.
   */
  function rationalLength(r: { numer: number | string; denom: number | string }): {
    kind: string;
    text: string;
    title: string;
  } {
    const n = BigInt(r.numer);
    const d = BigInt(r.denom);
    const totalSixteenths = (n * 16n + (n >= 0n ? d / 2n : -(d / 2n))) / d;
    const negative = totalSixteenths < 0n;
    const abs = negative ? -totalSixteenths : totalSixteenths;
    const totalInches = abs / 16n;
    const sixteenths = Number(abs % 16n);
    const feet = totalInches / 12n;
    const inches = totalInches % 12n;

    const sign = negative ? '-' : '';
    let text: string;
    let frac = '';
    if (sixteenths !== 0) {
      // Reduce 16ths to lowest terms: gcd with 16 is 1, 2, 4, 8, or 16.
      let num = sixteenths;
      let den = 16;
      while (num % 2 === 0 && den % 2 === 0) {
        num /= 2;
        den /= 2;
      }
      frac = `${num}/${den}`;
    }

    if (feet === 0n && inches === 0n && !frac) text = '0"';
    else if (feet > 0n && inches === 0n && !frac) text = `${feet}'`;
    else if (feet === 0n && !frac) text = `${inches}"`;
    else if (feet === 0n && inches === 0n) text = `${frac}"`;
    else if (!frac) text = feet > 0n ? `${feet}' ${inches}"` : `${inches}"`;
    else if (feet === 0n) text = `${inches}-${frac}"`;
    else if (inches === 0n) text = `${feet}' ${frac}"`;
    else text = `${feet}' ${inches}-${frac}"`;

    // Tooltip with the exact rational for the curious.
    const title = `Exact: ${r.numer}/${r.denom} in`;
    return { kind: 'result', text: `${sign}${text}`, title };
  }

  $: entries = (($snapshot.tape as any)?.entries as any[]) ?? [];
</script>

<aside class="tape">
  <header>
    <span>Tape</span>
    <span class="count">{entries.length}</span>
  </header>
  {#if entries.length === 0}
    <p class="empty">Results will appear here.</p>
  {:else}
    <ol>
      {#each entries as e, i}
        {@const r = renderEntry(e)}
        <li class={r.kind} title={r.title ?? ''}>
          <span class="idx">{i + 1}</span>
          <span class="text">{r.text}</span>
        </li>
      {/each}
    </ol>
  {/if}
</aside>

<style>
  .tape {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 0.6rem;
    padding: 0.6rem 0.8rem;
    color: #cbd5e1;
    font-size: 0.85rem;
    max-height: 180px;
    overflow-y: auto;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #94a3b8;
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin-bottom: 0.4rem;
  }
  .count {
    background: rgba(255, 255, 255, 0.08);
    padding: 0.05rem 0.4rem;
    border-radius: 999px;
    font-size: 0.7rem;
  }
  ol {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-family: 'JetBrains Mono', ui-monospace, monospace;
  }
  li {
    display: grid;
    grid-template-columns: 1.5rem 1fr;
    gap: 0.4rem;
    align-items: baseline;
  }
  .idx { color: #475569; font-size: 0.75rem; }
  .rafter { color: #fcd34d; }
  .note { color: #a5f3fc; font-style: italic; }
  .empty { color: #64748b; margin: 0; font-style: italic; }
</style>
