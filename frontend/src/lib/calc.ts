/**
 * Thin TypeScript wrapper around the WASM calculator.
 *
 * Why this layer exists:
 *  - hides the JSON-encoded event protocol from Svelte components
 *  - lazy-initializes the WASM module exactly once
 *  - exposes a Svelte store with reactive `display` + `tape`
 *  - centralizes export/import so components don't reach into wasm directly
 */

import { writable, type Writable } from 'svelte/store';

export type Op = 'add' | 'sub' | 'mul' | 'div';
export type Unit = 'ft' | 'in' | 'yd' | 'm';
export type FunctionKey =
  // Rafter family
  | 'pitch' | 'rise' | 'run' | 'diag' | 'hipv' | 'jack'
  // Trig
  | 'sin' | 'cos' | 'tan' | 'asin' | 'acos' | 'atan'
  // Math
  | 'sqrt' | 'square' | 'recip' | 'percent'
  // Compound miter
  | 'corner' | 'spring' | 'miter' | 'bevel';

export type Key =
  | { type: 'digit'; value: number }
  | { type: 'decimal' }
  | { type: 'slash' }
  | { type: 'negate' }
  | { type: 'op'; op: Op }
  | { type: 'equals' }
  | { type: 'unit'; unit: Unit }
  | { type: 'function'; fun: FunctionKey }
  | { type: 'convert'; format: string; denom?: number; precision?: number }
  | {
      type: 'memory';
      op: 'store' | 'recall' | 'add' | 'clear' | 'clear_all';
      slot?: number;
    }
  | { type: 'backspace' }
  | { type: 'clear' }
  | { type: 'clearAll' }
  | { type: 'note'; text: string };

export interface Snapshot {
  display: string;
  tape: unknown;
  error: string | null;
}

let modPromise: Promise<typeof import('$lib/wasm/calc_wasm.js')> | null = null;
type WasmCalculator = InstanceType<
  Awaited<ReturnType<typeof loadModule>>['WasmCalculator']
>;

async function loadModule() {
  if (!modPromise) {
    modPromise = (async () => {
      const mod = await import('$lib/wasm/calc_wasm.js');
      await mod.default();
      return mod;
    })();
  }
  return modPromise;
}

export class Calc {
  private inner: WasmCalculator | null = null;
  public readonly snapshot: Writable<Snapshot> = writable({
    display: '0',
    tape: { entries: [] },
    error: null
  });

  async init() {
    const mod = await loadModule();
    this.inner = new mod.WasmCalculator();
    this.snapshot.set({
      display: this.inner.displayString(),
      tape: { entries: [] },
      error: null
    });
  }

  send(key: Key) {
    if (!this.inner) throw new Error('Calc not initialized; call init() first');
    const snap = this.inner.handle(key) as Snapshot;
    this.snapshot.set(snap);
  }

  exportMarkdown(): string {
    if (!this.inner) throw new Error('Calc not initialized');
    return this.inner.exportMarkdown();
  }

  exportJson(): string {
    if (!this.inner) throw new Error('Calc not initialized');
    return this.inner.exportJson();
  }

  loadJsonTape(json: string) {
    if (!this.inner) throw new Error('Calc not initialized');
    this.inner.loadJsonTape(json);
    this.send({ type: 'clear' });
  }

  clearTape() {
    if (!this.inner) throw new Error('Calc not initialized');
    this.inner.clearTape();
    this.send({ type: 'clear' });
  }
}

export const calc = new Calc();
