<script lang="ts">
  import { calc, type Key } from './calc';

  /**
   * A small strip across the top of the display that lets the user pick how
   * the current result renders. It is *dimension-aware*: it maps directly to
   * KeyEvent::Convert for a length, KeyEvent::ConvertArea for an area, and
   * KeyEvent::ConvertVolume for a volume — switching the button set to match
   * whatever the engine currently holds (reported via `snapshot.dimension`).
   *
   * The length fraction buttons (1/4, 1/8, 1/16) double as rounding controls:
   * they set the resolution at which sums of whole numbers and fractions are
   * rendered. The internal value stays exact.
   */
  interface FormatChoice {
    label: string;
    key: Key;
  }

  const lengthChoices: FormatChoice[] = [
    { label: '1/4"', key: { type: 'convert', format: 'feet_inch_fraction', denom: 4 } },
    { label: '1/8"', key: { type: 'convert', format: 'feet_inch_fraction', denom: 8 } },
    { label: '1/16"', key: { type: 'convert', format: 'feet_inch_fraction', denom: 16 } },
    { label: 'dec ft', key: { type: 'convert', format: 'decimal_feet', precision: 4 } },
    { label: 'dec in', key: { type: 'convert', format: 'decimal_inches', precision: 4 } },
    { label: 'm', key: { type: 'convert', format: 'm', precision: 4 } },
    { label: 'yd', key: { type: 'convert', format: 'yards', precision: 4 } }
  ];

  const areaChoices: FormatChoice[] = [
    { label: 'in²', key: { type: 'convertArea', format: 'sq_in', precision: 0 } },
    { label: 'ft²', key: { type: 'convertArea', format: 'sq_ft', precision: 2 } },
    { label: 'yd²', key: { type: 'convertArea', format: 'sq_yd', precision: 2 } },
    { label: 'm²', key: { type: 'convertArea', format: 'sq_m', precision: 3 } },
    { label: 'acre', key: { type: 'convertArea', format: 'acres', precision: 4 } }
  ];

  const volumeChoices: FormatChoice[] = [
    { label: 'in³', key: { type: 'convertVolume', format: 'cu_in', precision: 0 } },
    { label: 'ft³', key: { type: 'convertVolume', format: 'cu_ft', precision: 2 } },
    { label: 'yd³', key: { type: 'convertVolume', format: 'cu_yd', precision: 2 } },
    { label: 'm³', key: { type: 'convertVolume', format: 'cu_m', precision: 3 } },
    { label: 'gal', key: { type: 'convertVolume', format: 'gallons', precision: 2 } },
    { label: 'L', key: { type: 'convertVolume', format: 'liters', precision: 2 } }
  ];

  const snapshot = calc.snapshot;

  // Scalar / angle have no unit conversion, so we keep showing the length
  // strip (a length convert is a harmless no-op on a scalar display).
  $: choices =
    $snapshot.dimension === 'area'
      ? areaChoices
      : $snapshot.dimension === 'volume'
        ? volumeChoices
        : lengthChoices;

  // The index highlighted by default per dimension — matches the engine's
  // default Mode: length opens at 1/16" (index 2), area at ft² and volume at
  // ft³ (index 1 of their sets).
  const defaultActive: Record<string, number> = { area: 1, volume: 1 };

  let active = 2;
  let shownDimension = $snapshot.dimension;
  // When the result's dimension changes, reset the highlight to that set's
  // default rather than carrying a stale index across sets. We only move the
  // highlight here — the engine already renders in its default for the new
  // dimension, so there's nothing to re-send.
  $: if ($snapshot.dimension !== shownDimension) {
    shownDimension = $snapshot.dimension;
    active = defaultActive[$snapshot.dimension] ?? 2;
  }

  function pick(i: number) {
    active = i;
    calc.send(choices[i].key);
  }
</script>

<div class="strip" role="tablist" aria-label="Display format">
  {#each choices as c, i (c.label)}
    <button
      class:active={active === i}
      role="tab"
      aria-selected={active === i}
      on:click={() => pick(i)}
    >
      {c.label}
    </button>
  {/each}
</div>

<style>
  .strip {
    display: flex;
    gap: 0.25rem;
    overflow-x: auto;
    padding: 0.25rem;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 0.5rem;
    -webkit-overflow-scrolling: touch;
  }
  button {
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #cbd5e1;
    padding: 0.3rem 0.65rem;
    border-radius: 0.4rem;
    font-size: 0.8rem;
    cursor: pointer;
    flex-shrink: 0;
  }
  .active {
    background: #d97706;
    border-color: #d97706;
    color: white;
  }
</style>
