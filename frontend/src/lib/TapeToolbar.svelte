<script lang="ts">
  import { calc } from './calc';
  import { downloadText, pickTextFile, FileTooLargeError } from './download';
  import { saveTape, TapeLimitReachedError, TapeTooLargeError } from './tapeStore';

  let copied = false;
  let copiedTimer: ReturnType<typeof setTimeout> | null = null;

  // Inline message displayed above the toolbar — replaces alert() popups.
  // null = hidden; 'error' tints red; 'info' is neutral.
  let message: { kind: 'error' | 'info'; text: string } | null = null;
  let messageTimer: ReturnType<typeof setTimeout> | null = null;

  // Two-step confirm for destructive Clear — replaces window.confirm().
  let clearArmed = false;
  let clearArmedTimer: ReturnType<typeof setTimeout> | null = null;

  // Inline name-entry form for Save. `null` = hidden.
  let saveName: string | null = null;

  function flash(kind: 'error' | 'info', text: string, ms = 4000) {
    message = { kind, text };
    if (messageTimer) clearTimeout(messageTimer);
    messageTimer = setTimeout(() => (message = null), ms);
  }

  function timestampedFilename(ext: string): string {
    // YYYY-MM-DD_HH-MM filenames sort cleanly in any file manager and
    // are filesystem-safe everywhere.
    const d = new Date();
    const pad = (n: number) => String(n).padStart(2, '0');
    const stamp = `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(
      d.getDate()
    )}_${pad(d.getHours())}-${pad(d.getMinutes())}`;
    return `calc-tape_${stamp}.${ext}`;
  }

  function downloadJson() {
    downloadText(timestampedFilename('json'), calc.exportJson(), 'application/json');
  }

  function downloadMarkdown() {
    downloadText(timestampedFilename('md'), calc.exportMarkdown(), 'text/markdown');
  }

  async function copyMarkdown() {
    try {
      await navigator.clipboard.writeText(calc.exportMarkdown());
      copied = true;
      if (copiedTimer) clearTimeout(copiedTimer);
      copiedTimer = setTimeout(() => (copied = false), 1500);
    } catch {
      // Clipboard permission denied or unavailable (older Safari): fall
      // back to download.
      downloadMarkdown();
    }
  }

  async function loadJson() {
    let txt: string | null;
    try {
      txt = await pickTextFile();
    } catch (e) {
      if (e instanceof FileTooLargeError) {
        flash('error', `File is too large (${(e.size / 1024 / 1024).toFixed(1)} MB). Tapes are typically a few KB.`);
      } else {
        flash('error', `Could not read file: ${e instanceof Error ? e.message : String(e)}`);
      }
      return;
    }
    if (!txt) return;
    try {
      calc.loadJsonTape(txt);
      flash('info', 'Tape loaded.');
    } catch (e) {
      flash('error', `Could not load tape: ${e instanceof Error ? e.message : String(e)}`);
    }
  }

  function startSave() {
    saveName = `Tape ${new Date().toLocaleString()}`;
  }

  function commitSave() {
    if (saveName === null) return;
    const name = saveName;
    try {
      saveTape(name, calc.exportJson());
      flash('info', `Saved as "${name}".`);
      saveName = null;
    } catch (e) {
      if (e instanceof TapeTooLargeError) {
        flash('error', `Tape is too large to save (${(e.size / 1024).toFixed(1)} KB). Try .json export.`);
      } else if (e instanceof TapeLimitReachedError) {
        flash('error', `Hit the saved-tape limit. Delete some at /tapes.`);
      } else {
        flash('error', `Save failed: ${e instanceof Error ? e.message : String(e)}`);
      }
    }
  }

  function cancelSave() {
    saveName = null;
  }

  function clearTape() {
    if (!clearArmed) {
      // First click arms the action; the button label changes to confirm.
      // After 3 s without a second click, reset.
      clearArmed = true;
      if (clearArmedTimer) clearTimeout(clearArmedTimer);
      clearArmedTimer = setTimeout(() => (clearArmed = false), 3000);
      return;
    }
    clearArmed = false;
    if (clearArmedTimer) clearTimeout(clearArmedTimer);
    calc.clearTape();
    flash('info', 'Tape cleared.');
  }
</script>

{#if message}
  <p class="msg {message.kind}" role="status" aria-live="polite">{message.text}</p>
{/if}

{#if saveName !== null}
  <form class="save-form" on:submit|preventDefault={commitSave}>
    <input
      type="text"
      bind:value={saveName}
      placeholder="Tape name"
      aria-label="Tape name"
    />
    <button type="submit">Save</button>
    <button type="button" on:click={cancelSave}>Cancel</button>
  </form>
{/if}

<div class="toolbar">
  <button on:click={copyMarkdown} title="Copy markdown to clipboard">
    {copied ? '✓ Copied' : '📋 Copy'}
  </button>
  <button on:click={startSave} title="Save tape to the browser (see /tapes)">💾 Save</button>
  <button on:click={downloadMarkdown} title="Download as Markdown">.md</button>
  <button on:click={downloadJson} title="Download as JSON">.json</button>
  <button on:click={loadJson} title="Load tape from JSON file">Load</button>
  <button
    on:click={clearTape}
    class="danger"
    class:armed={clearArmed}
    title={clearArmed ? 'Click again to confirm' : 'Clear the tape'}
  >
    {clearArmed ? 'Confirm clear' : 'Clear'}
  </button>
</div>

<style>
  .toolbar {
    display: flex;
    gap: 0.3rem;
    flex-wrap: wrap;
  }
  button {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #cbd5e1;
    padding: 0.3rem 0.6rem;
    border-radius: 0.4rem;
    font-size: 0.78rem;
    cursor: pointer;
  }
  button:hover {
    background: rgba(255, 255, 255, 0.08);
  }
  button.danger {
    color: #fca5a5;
  }
  button.danger:hover {
    background: rgba(255, 0, 0, 0.1);
  }
  button.danger.armed {
    background: rgba(239, 68, 68, 0.18);
    border-color: #ef4444;
    color: #fee2e2;
  }
  .msg {
    margin: 0 0 0.4rem;
    padding: 0.35rem 0.6rem;
    border-radius: 0.4rem;
    font-size: 0.8rem;
  }
  .msg.error {
    background: rgba(239, 68, 68, 0.15);
    color: #fecaca;
    border: 1px solid rgba(239, 68, 68, 0.4);
  }
  .msg.info {
    background: rgba(255, 255, 255, 0.05);
    color: #cbd5e1;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  .save-form {
    display: flex;
    gap: 0.3rem;
    margin-bottom: 0.4rem;
  }
  .save-form input {
    flex: 1;
    min-width: 0;
    background: #0e1623;
    color: #f4f6fa;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 0.4rem;
    padding: 0.3rem 0.5rem;
    font: inherit;
    font-size: 0.85rem;
  }
</style>
