<script lang="ts">
  import { HELP, type HelpEntry } from './help';

  export let helpKey: string | null = null;

  function close() {
    helpKey = null;
  }

  $: entry = helpKey ? HELP[helpKey] ?? null : null;
</script>

{#if entry}
  <div class="overlay" role="dialog" aria-modal="true" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()}>
    <div class="card" on:click|stopPropagation>
      <h3>{entry.title}</h3>
      <p>{entry.body}</p>
      {#if entry.formula}
        <p class="formula">{entry.formula}</p>
      {/if}
      {#if entry.example}
        <p class="example"><strong>Example:</strong> {entry.example}</p>
      {/if}
      <button class="close" on:click={close}>Got it</button>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    backdrop-filter: blur(2px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
  }
  .card {
    background: #1a2438;
    color: #f4f6fa;
    border-radius: 0.75rem;
    padding: 1.25rem 1.5rem;
    max-width: 420px;
    width: 100%;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }
  h3 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
  }
  p {
    margin: 0 0 0.6rem;
    line-height: 1.45;
    font-size: 0.95rem;
  }
  .formula {
    background: rgba(255, 255, 255, 0.05);
    padding: 0.4rem 0.6rem;
    border-radius: 0.4rem;
    font-family: 'JetBrains Mono', ui-monospace, monospace;
    font-size: 0.9rem;
  }
  .example {
    color: #94a3b8;
  }
  .close {
    margin-top: 0.6rem;
    background: #d97706;
    border: none;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.4rem;
    cursor: pointer;
    font-weight: 500;
  }
</style>
