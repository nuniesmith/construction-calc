<script lang="ts">
  import { calc, type Key } from './calc';

  /**
   * A small strip across the top of the display that lets the user pick
   * how lengths render. Maps directly to KeyEvent::Convert on the engine.
   */
  interface FormatChoice {
    label: string;
    key: Key;
  }

  const choices: FormatChoice[] = [
    { label: '1/16"', key: { type: 'convert', format: 'feet_inch_fraction', denom: 16 } },
    { label: '1/32"', key: { type: 'convert', format: 'feet_inch_fraction', denom: 32 } },
    { label: '1/64"', key: { type: 'convert', format: 'feet_inch_fraction', denom: 64 } },
    { label: 'dec ft', key: { type: 'convert', format: 'decimal_feet', precision: 4 } },
    { label: 'dec in', key: { type: 'convert', format: 'decimal_inches', precision: 4 } },
    { label: 'mm', key: { type: 'convert', format: 'mm', precision: 0 } },
    { label: 'cm', key: { type: 'convert', format: 'cm', precision: 2 } },
    { label: 'm', key: { type: 'convert', format: 'm', precision: 4 } },
    { label: 'yd', key: { type: 'convert', format: 'yards', precision: 4 } }
  ];

  let active = 0;
  function pick(i: number) {
    active = i;
    calc.send(choices[i].key);
  }
</script>

<div class="strip" role="tablist" aria-label="Display format">
  {#each choices as c, i}
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
