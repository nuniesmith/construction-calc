<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { calc } from '$lib/calc';
  import {
    listTapes,
    deleteTape,
    getTape,
    clearAllTapes,
    type SavedTape
  } from '$lib/tapeStore';

  let tapes: SavedTape[] = [];
  let message: string | null = null;
  let messageKind: 'info' | 'error' = 'info';

  // Two-step confirm before nuking everything.
  let nukeArmed = false;
  let nukeTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    tapes = listTapes();
  });

  function refresh() {
    tapes = listTapes();
  }

  function flash(kind: 'info' | 'error', text: string) {
    messageKind = kind;
    message = text;
    setTimeout(() => (message = null), 3000);
  }

  async function restoreTape(id: string) {
    const t = getTape(id);
    if (!t) {
      flash('error', 'Tape not found.');
      return;
    }
    try {
      // The calc store may not be initialized if the user came straight
      // here without visiting /. init() is idempotent.
      await calc.init();
      calc.loadJsonTape(t.json);
      flash('info', `Loaded "${t.name}".`);
      // Bounce back to the calculator so the loaded tape is visible.
      goto('/');
    } catch (e) {
      flash('error', `Load failed: ${e instanceof Error ? e.message : String(e)}`);
    }
  }

  function removeTape(id: string) {
    deleteTape(id);
    refresh();
  }

  function nukeAll() {
    if (!nukeArmed) {
      nukeArmed = true;
      if (nukeTimer) clearTimeout(nukeTimer);
      nukeTimer = setTimeout(() => (nukeArmed = false), 3000);
      return;
    }
    nukeArmed = false;
    if (nukeTimer) clearTimeout(nukeTimer);
    clearAllTapes();
    refresh();
    flash('info', 'All saved tapes deleted.');
  }

  function fmt(ts: number): string {
    return new Date(ts).toLocaleString();
  }
</script>

<svelte:head><title>Saved tapes · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/">← back</a>
  <h1>Saved tapes</h1>
  <p class="hint">Tapes saved in this browser. Use 💾 Save on the main view to add one.</p>

  {#if message}
    <p class="msg {messageKind}" role="status" aria-live="polite">{message}</p>
  {/if}

  {#if tapes.length === 0}
    <p class="empty">No saved tapes yet.</p>
  {:else}
    <ul>
      {#each tapes as t (t.id)}
        <li>
          <div class="row">
            <div class="meta">
              <h2>{t.name}</h2>
              <p class="when">{fmt(t.createdAt)}</p>
            </div>
            <div class="actions">
              <button class="primary" on:click={() => restoreTape(t.id)}>Load</button>
              <button class="danger" on:click={() => removeTape(t.id)}>Delete</button>
            </div>
          </div>
        </li>
      {/each}
    </ul>

    <button class="nuke" class:armed={nukeArmed} on:click={nukeAll}>
      {nukeArmed ? 'Confirm delete all' : 'Delete all'}
    </button>
  {/if}
</main>

<style>
  main { max-width: 480px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; color: #f4f6fa; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
  .back { color: #94a3b8; text-decoration: none; font-size: 0.85rem; }
  h1 { margin: 0; font-size: 1.5rem; }
  .hint, .empty { color: #94a3b8; font-size: 0.85rem; margin: 0; }
  ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.4rem; }
  li { background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 0.6rem; padding: 0.6rem 0.75rem; }
  .row { display: flex; justify-content: space-between; align-items: center; gap: 0.6rem; }
  .meta { min-width: 0; flex: 1; }
  .meta h2 { margin: 0; font-size: 0.95rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .when { margin: 0.15rem 0 0; color: #94a3b8; font-size: 0.75rem; }
  .actions { display: flex; gap: 0.3rem; flex-shrink: 0; }
  button { background: rgba(255, 255, 255, 0.04); border: 1px solid rgba(255, 255, 255, 0.08); color: #cbd5e1; padding: 0.35rem 0.7rem; border-radius: 0.4rem; cursor: pointer; font: inherit; font-size: 0.8rem; }
  button.primary { background: #d97706; color: white; border: none; font-weight: 500; }
  button.danger { color: #fca5a5; }
  button.danger:hover { background: rgba(255, 0, 0, 0.1); }
  button.nuke { color: #fca5a5; margin-top: 0.6rem; align-self: flex-start; }
  button.nuke.armed { background: rgba(239, 68, 68, 0.18); border-color: #ef4444; color: #fee2e2; }
  .msg { padding: 0.35rem 0.6rem; border-radius: 0.4rem; font-size: 0.8rem; margin: 0; }
  .msg.error { background: rgba(239, 68, 68, 0.15); color: #fecaca; border: 1px solid rgba(239, 68, 68, 0.4); }
  .msg.info { background: rgba(255, 255, 255, 0.05); color: #cbd5e1; border: 1px solid rgba(255, 255, 255, 0.1); }
</style>
