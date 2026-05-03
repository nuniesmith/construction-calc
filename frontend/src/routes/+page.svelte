<script lang="ts">
  import { onMount } from 'svelte';
  import Display from '$lib/Display.svelte';
  import Keypad from '$lib/Keypad.svelte';
  import Tape from '$lib/Tape.svelte';
  import TapeToolbar from '$lib/TapeToolbar.svelte';
  import FormatStrip from '$lib/FormatStrip.svelte';
  import HelpOverlay from '$lib/HelpOverlay.svelte';
  import { calc } from '$lib/calc';
  import { bindKeyboard } from '$lib/keyboard';

  let ready = false;
  let initError: string | null = null;
  let helpKey: string | null = null;

  onMount(() => {
    let teardown: (() => void) | null = null;
    (async () => {
      try {
        await calc.init();
        ready = true;
        teardown = bindKeyboard(calc);
      } catch (e) {
        initError = e instanceof Error ? e.message : String(e);
      }
    })();
    return () => {
      if (teardown) teardown();
    };
  });
</script>

<svelte:head><title>Construction Calc</title></svelte:head>

<main>
  <header>
    <div class="title-row">
      <h1>Construction Calc</h1>
      <a class="ez-link" href="/ez">EZ Calc →</a>
    </div>
    <p class="tag">Long-press any key for help.</p>
  </header>

  {#if initError}
    <p class="error">Failed to load WASM module: {initError}</p>
    <p class="hint">
      Build the WASM bundle first:
      <code>cd frontend &amp;&amp; npm run build:wasm</code>
    </p>
  {:else if !ready}
    <p class="loading">Loading…</p>
  {:else}
    <FormatStrip />
    <Display />
    <Tape />
    <TapeToolbar />
    <Keypad bind:helpKey />
    <HelpOverlay bind:helpKey />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    background: linear-gradient(180deg, #0a1628 0%, #060d18 100%);
    color: #f4f6fa;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
  main { max-width: 480px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; min-height: 100vh; }
  .title-row { display: flex; justify-content: space-between; align-items: baseline; gap: 0.5rem; }
  header h1 { margin: 0; font-size: 1.5rem; font-weight: 600; }
  .ez-link { color: #fcd34d; text-decoration: none; font-size: 0.85rem; padding: 0.2rem 0.5rem; border: 1px solid rgba(252, 211, 77, 0.3); border-radius: 0.3rem; }
  .ez-link:hover { background: rgba(252, 211, 77, 0.1); }
  .tag { margin: 0; color: #94a3b8; font-size: 0.8rem; }
  .loading, .error, .hint { color: #94a3b8; }
  .error { color: #ff7676; }
  code { background: rgba(255, 255, 255, 0.06); padding: 0.1rem 0.4rem; border-radius: 0.3rem; }
</style>
