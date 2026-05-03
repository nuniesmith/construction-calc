<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { HELP, type HelpEntry } from './help';

  export let helpKey: string | null = null;

  let closeButton: HTMLButtonElement | null = null;
  let previousActive: HTMLElement | null = null;

  function close() {
    helpKey = null;
  }

  function handleKey(e: KeyboardEvent) {
    if (!entry) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      close();
    }
  }

  // When the overlay opens, remember what had focus, move focus into the
  // dialog, and restore it on close. Listen for Escape on the document so
  // the user doesn't have to hunt for the right element.
  onMount(() => {
    document.addEventListener('keydown', handleKey);
    return () => document.removeEventListener('keydown', handleKey);
  });

  $: entry = helpKey ? HELP[helpKey] ?? null : null;
  $: if (entry) {
    previousActive = document.activeElement as HTMLElement | null;
    tick().then(() => closeButton?.focus());
  } else if (previousActive) {
    previousActive.focus();
    previousActive = null;
  }
</script>

{#if entry}
  <!-- The backdrop is a button so screen readers and keyboards can dismiss
       the dialog the same way a click does. The visible card stops
       propagation so clicks inside don't close it. -->
  <button
    type="button"
    class="overlay"
    aria-label="Close help"
    on:click={close}
  ></button>
  <div class="card" role="dialog" aria-modal="true" aria-labelledby="help-title">
    <h3 id="help-title">{entry.title}</h3>
    <p>{entry.body}</p>
    {#if entry.formula}
      <p class="formula">{entry.formula}</p>
    {/if}
    {#if entry.example}
      <p class="example"><strong>Example:</strong> {entry.example}</p>
    {/if}
    <button class="close" bind:this={closeButton} on:click={close}>Got it</button>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    backdrop-filter: blur(2px);
    z-index: 100;
    border: 0;
    padding: 0;
    cursor: pointer;
    /* Visually hidden label, but keep the button focusable. */
    color: transparent;
  }
  .card {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: #1a2438;
    color: #f4f6fa;
    border-radius: 0.75rem;
    padding: 1.25rem 1.5rem;
    max-width: 420px;
    width: calc(100% - 2rem);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.08);
    z-index: 101;
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
