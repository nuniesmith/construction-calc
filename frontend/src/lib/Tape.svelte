<script lang="ts">
  import { calc } from './calc';
  import { rationalApprox, rationalLength } from './tapeFormat';
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
