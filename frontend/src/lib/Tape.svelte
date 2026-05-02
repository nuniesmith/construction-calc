<script lang="ts">
  import { calc } from './calc';
  $: snapshot = calc.snapshot;

  /**
   * The tape comes back from Rust as serde_json. Each entry is a tagged
   * variant of TapeEntry. Render Result entries simply; Rafter solutions
   * get a small multi-line block.
   */
  function renderEntry(e: any): { kind: string; text: string } {
    if ('Result' in e) {
      const v = e.Result;
      if ('Scalar' in v) return { kind: 'result', text: rationalString(v.Scalar) };
      if ('Length' in v) return { kind: 'result', text: `${rationalString(v.Length.inches)}"` };
      if ('Area' in v) return { kind: 'result', text: `${rationalString(v.Area)} sq in` };
      if ('Volume' in v) return { kind: 'result', text: `${rationalString(v.Volume)} cu in` };
      if ('Angle' in v) return { kind: 'result', text: `${rationalString(v.Angle.degrees)}°` };
      return { kind: 'result', text: JSON.stringify(v) };
    }
    if ('RafterSolution' in e) {
      const r = e.RafterSolution;
      return {
        kind: 'rafter',
        text: `Rafter — pitch ${rationalString(r.pitch_ratio, 3)}, rise ${rationalString(r.rise.inches)}", run ${rationalString(r.run.inches)}", diag ${rationalString(r.diagonal.inches)}"`
      };
    }
    if ('Note' in e) {
      return { kind: 'note', text: e.Note };
    }
    return { kind: 'unknown', text: JSON.stringify(e) };
  }

  function rationalString(r: { numer: number; denom: number } | null, precision = 4): string {
    if (!r) return '?';
    const n = r.numer;
    const d = r.denom;
    if (d === 1) return String(n);
    const f = n / d;
    return f.toFixed(precision).replace(/\.?0+$/, '');
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
        <li class={r.kind}>
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
  .idx {
    color: #475569;
    font-size: 0.75rem;
  }
  .rafter {
    color: #fcd34d;
  }
  .note {
    color: #a5f3fc;
    font-style: italic;
  }
  .empty {
    color: #64748b;
    margin: 0;
    font-style: italic;
  }
</style>
