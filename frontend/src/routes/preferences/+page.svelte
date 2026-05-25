<script lang="ts">
  import { onMount } from 'svelte';
  import {
    DEFAULTS,
    loadPreferences,
    savePreferences,
    type Preferences
  } from '$lib/preferences';

  // Initialize with defaults so we can render the form before
  // localStorage is read (SSR-safe). onMount loads real values.
  let prefs: Preferences = { ...DEFAULTS };
  let saved = false;
  let savedTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    prefs = loadPreferences();
  });

  function save() {
    savePreferences(prefs);
    saved = true;
    if (savedTimer) clearTimeout(savedTimer);
    savedTimer = setTimeout(() => (saved = false), 2000);
  }

  function resetToDefaults() {
    prefs = { ...DEFAULTS };
    savePreferences(prefs);
    saved = true;
    if (savedTimer) clearTimeout(savedTimer);
    savedTimer = setTimeout(() => (saved = false), 2000);
  }
</script>

<svelte:head><title>Preferences · Construction Calc</title></svelte:head>

<main>
  <a class="back" href="/">← back</a>
  <h1>Preferences</h1>
  <p class="hint">Stored in this browser only. Changes apply next time the app loads.</p>

  <section class="card">
    <h2>Display</h2>
    <label>Default fraction resolution
      <select bind:value={prefs.defaultFractionDenom}>
        <option value={4}>1/4"</option>
        <option value={8}>1/8"</option>
        <option value={16}>1/16"</option>
      </select>
    </label>
    <label>Default length format
      <select bind:value={prefs.defaultLengthFormat}>
        <option value="feet_inch_fraction">Feet + inch + fraction</option>
        <option value="decimal_feet">Decimal feet</option>
        <option value="decimal_inches">Decimal inches</option>
        <option value="meters">Meters</option>
        <option value="yards">Yards</option>
      </select>
    </label>
  </section>

  <section class="card">
    <h2>Angles</h2>
    <label class="checkbox">
      <input type="checkbox" bind:checked={prefs.angleInDegrees} />
      Trig keys use degrees (uncheck for radians)
    </label>
  </section>

  <div class="actions">
    <button class="primary" on:click={save}>Save</button>
    <button on:click={resetToDefaults}>Reset to defaults</button>
    {#if saved}<span class="saved">Saved.</span>{/if}
  </div>
</main>

<style>
  main { max-width: 480px; margin: 0 auto; padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; color: #f4f6fa; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
  .back { color: #94a3b8; text-decoration: none; font-size: 0.85rem; }
  h1 { margin: 0; font-size: 1.5rem; }
  .hint { color: #94a3b8; font-size: 0.85rem; margin: 0; }
  .card { background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 0.6rem; padding: 0.75rem 1rem; display: flex; flex-direction: column; gap: 0.6rem; }
  .card h2 { margin: 0; font-size: 0.9rem; text-transform: uppercase; letter-spacing: 0.05em; color: #94a3b8; }
  label { display: flex; flex-direction: column; gap: 0.2rem; font-size: 0.85rem; }
  label.checkbox { flex-direction: row; align-items: center; gap: 0.5rem; }
  input, select { background: #0e1623; color: #f4f6fa; border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 0.4rem; padding: 0.4rem 0.5rem; font: inherit; min-width: 0; width: 100%; box-sizing: border-box; }
  label.checkbox input { width: auto; }
  .actions { display: flex; gap: 0.5rem; align-items: center; }
  button { background: rgba(255, 255, 255, 0.04); border: 1px solid rgba(255, 255, 255, 0.08); color: #cbd5e1; padding: 0.5rem 1rem; border-radius: 0.4rem; cursor: pointer; font: inherit; font-size: 0.9rem; }
  button.primary { background: #d97706; color: white; border: none; font-weight: 500; }
  .saved { color: #fcd34d; font-size: 0.85rem; }
</style>
