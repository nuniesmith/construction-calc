<script lang="ts">
  import { calc, type Key } from './calc';
  import { longPress } from './longPress';

  /** Bound by parent so the help overlay can read it. */
  export let helpKey: string | null = null;

  interface ButtonSpec {
    label: string;
    /** Help id; if absent, label-as-lower is used. */
    helpId?: string;
    key: Key;
    style?: 'fn' | 'op' | 'num' | 'unit' | 'ctrl';
    span?: number;
  }

  type Page = 'rafter' | 'trig';
  let page: Page = 'rafter';

  // The first row of function keys swaps with the page tab. The rest of
  // the keypad (units, controls, math, digits, ops) stays constant —
  // that's the muscle-memory part.
  const functionPages: Record<Page, ButtonSpec[]> = {
    rafter: [
      { label: 'Pitch', helpId: 'pitch', key: { type: 'function', fun: 'pitch' }, style: 'fn' },
      { label: 'Rise', helpId: 'rise', key: { type: 'function', fun: 'rise' }, style: 'fn' },
      { label: 'Run', helpId: 'run', key: { type: 'function', fun: 'run' }, style: 'fn' },
      { label: 'Diag', helpId: 'diag', key: { type: 'function', fun: 'diag' }, style: 'fn' },
      { label: 'Hip/V', helpId: 'hipv', key: { type: 'function', fun: 'hipv' }, style: 'fn' },
      { label: 'Jack', helpId: 'jack', key: { type: 'function', fun: 'jack' }, style: 'fn' }
    ],
    trig: [
      { label: 'sin', helpId: 'sin', key: { type: 'function', fun: 'sin' }, style: 'fn' },
      { label: 'cos', helpId: 'cos', key: { type: 'function', fun: 'cos' }, style: 'fn' },
      { label: 'tan', helpId: 'tan', key: { type: 'function', fun: 'tan' }, style: 'fn' },
      { label: 'asin', helpId: 'asin', key: { type: 'function', fun: 'asin' }, style: 'fn' },
      { label: 'acos', helpId: 'acos', key: { type: 'function', fun: 'acos' }, style: 'fn' },
      { label: 'atan', helpId: 'atan', key: { type: 'function', fun: 'atan' }, style: 'fn' }
    ]
  };

  const sharedRows: ButtonSpec[] = [
    { label: 'Yds', helpId: 'yd', key: { type: 'unit', unit: 'yd' }, style: 'unit' },
    { label: 'Feet', helpId: 'ft', key: { type: 'unit', unit: 'ft' }, style: 'unit' },
    { label: 'Inch', helpId: 'in', key: { type: 'unit', unit: 'in' }, style: 'unit' },
    { label: 'mm', helpId: 'mm', key: { type: 'unit', unit: 'mm' }, style: 'unit' },
    { label: 'cm', helpId: 'cm', key: { type: 'unit', unit: 'cm' }, style: 'unit' },
    { label: 'm', helpId: 'm', key: { type: 'unit', unit: 'm' }, style: 'unit' },

    { label: 'C', helpId: 'c', key: { type: 'clear' }, style: 'ctrl' },
    { label: 'AC', helpId: 'ac', key: { type: 'clearAll' }, style: 'ctrl' },
    { label: '⌫', helpId: 'bs', key: { type: 'backspace' }, style: 'ctrl' },
    { label: '/', helpId: 'slash', key: { type: 'slash' }, style: 'op' },
    { label: '±', helpId: 'negate', key: { type: 'negate' }, style: 'op' },
    { label: '÷', key: { type: 'op', op: 'div' }, style: 'op' },

    { label: '√', helpId: 'sqrt', key: { type: 'function', fun: 'sqrt' }, style: 'fn' },
    { label: 'x²', helpId: 'square', key: { type: 'function', fun: 'square' }, style: 'fn' },
    { label: '1/x', helpId: 'recip', key: { type: 'function', fun: 'recip' }, style: 'fn' },
    { label: '%', helpId: 'percent', key: { type: 'function', fun: 'percent' }, style: 'fn' },
    { label: '×', key: { type: 'op', op: 'mul' }, style: 'op' },
    { label: '−', key: { type: 'op', op: 'sub' }, style: 'op' },

    { label: '7', key: { type: 'digit', value: 7 }, style: 'num' },
    { label: '8', key: { type: 'digit', value: 8 }, style: 'num' },
    { label: '9', key: { type: 'digit', value: 9 }, style: 'num' },
    { label: '4', key: { type: 'digit', value: 4 }, style: 'num' },
    { label: '5', key: { type: 'digit', value: 5 }, style: 'num' },
    { label: '6', key: { type: 'digit', value: 6 }, style: 'num' },

    { label: '1', key: { type: 'digit', value: 1 }, style: 'num' },
    { label: '2', key: { type: 'digit', value: 2 }, style: 'num' },
    { label: '3', key: { type: 'digit', value: 3 }, style: 'num' },
    { label: '0', key: { type: 'digit', value: 0 }, style: 'num', span: 2 },
    { label: '.', key: { type: 'decimal' }, style: 'num' },

    { label: '+', key: { type: 'op', op: 'add' }, style: 'op' },
    { label: '=', key: { type: 'equals' }, style: 'op', span: 5 }
  ];

  $: layout = [...functionPages[page], ...sharedRows];

  function press(spec: ButtonSpec) {
    calc.send(spec.key);
  }

  function showHelp(spec: ButtonSpec) {
    helpKey = spec.helpId ?? spec.label.toLowerCase();
  }
</script>

<div class="page-tabs">
  <button class:active={page === 'rafter'} on:click={() => (page = 'rafter')}>Rafter</button>
  <button class:active={page === 'trig'} on:click={() => (page = 'trig')}>Trig</button>
</div>

<div class="keypad">
  {#each layout as spec (spec.label + (spec.helpId ?? ''))}
    <button
      class="key {spec.style ?? 'num'}"
      style:grid-column="span {spec.span ?? 1}"
      on:click={() => press(spec)}
      use:longPress={{ onLongPress: () => showHelp(spec) }}
    >
      {spec.label}
    </button>
  {/each}
</div>

<style>
  .page-tabs {
    display: flex;
    gap: 0.4rem;
    padding: 0 0.6rem;
  }
  .page-tabs button {
    flex: 1;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #cbd5e1;
    padding: 0.4rem;
    border-radius: 0.4rem;
    font-size: 0.85rem;
    cursor: pointer;
  }
  .page-tabs .active {
    background: #3b3a8a;
    border-color: #3b3a8a;
    color: white;
  }
  .keypad {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 0.4rem;
    padding: 0.6rem;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 0.75rem;
  }
  .key {
    min-height: 3rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 0.6rem;
    background: linear-gradient(180deg, #2a3340 0%, #1c2330 100%);
    color: #f4f6fa;
    font-size: 1.05rem;
    font-weight: 500;
    cursor: pointer;
    transition: transform 0.05s ease, background 0.1s ease;
    user-select: none;
    -webkit-user-select: none;
    /* iOS: suppress the long-press callout so our handler runs cleanly. */
    -webkit-touch-callout: none;
  }
  .key:active {
    transform: translateY(1px);
    background: linear-gradient(180deg, #1c2330 0%, #2a3340 100%);
  }
  .key.fn { background: linear-gradient(180deg, #3b3a8a 0%, #2a2a64 100%); }
  .key.unit { background: linear-gradient(180deg, #485a76 0%, #2f3e55 100%); }
  .key.op { background: linear-gradient(180deg, #d97706 0%, #a55906 100%); }
  .key.ctrl { background: linear-gradient(180deg, #7a1f1f 0%, #581515 100%); }
</style>
