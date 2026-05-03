<script lang="ts">
  import { calc } from './calc';
  import { downloadText, pickTextFile } from './download';

  let copied = false;
  let copiedTimer: ReturnType<typeof setTimeout> | null = null;

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
    const txt = await pickTextFile();
    if (!txt) return;
    try {
      calc.loadJsonTape(txt);
    } catch (e) {
      alert(`Could not load tape: ${e instanceof Error ? e.message : String(e)}`);
    }
  }

  function clearTape() {
    if (confirm('Clear the tape? This cannot be undone.')) {
      calc.clearTape();
    }
  }
</script>

<div class="toolbar">
  <button on:click={copyMarkdown} title="Copy markdown to clipboard">
    {copied ? '✓ Copied' : '📋 Copy'}
  </button>
  <button on:click={downloadMarkdown} title="Download as Markdown">.md</button>
  <button on:click={downloadJson} title="Download as JSON">.json</button>
  <button on:click={loadJson} title="Load tape from JSON file">Load</button>
  <button on:click={clearTape} class="danger" title="Clear the tape">Clear</button>
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
</style>
