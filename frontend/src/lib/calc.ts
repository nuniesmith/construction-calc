/**
 * Thin TypeScript wrapper around the WASM calculator.
 *
 * Why this layer exists:
 *  - hides the JSON-encoded event protocol from Svelte components
 *  - lazy-initializes the WASM module exactly once
 *  - exposes a Svelte store with reactive `display` + `tape`
 *
 * Add new key kinds here when you add them on the Rust side.
 */

import { writable, type Writable } from 'svelte/store';

export type Op = 'add' | 'sub' | 'mul' | 'div';
export type Unit = 'ft' | 'in' | 'yd' | 'mm' | 'cm' | 'm';
export type FunctionKey =
  | 'pitch'
  | 'rise'
  | 'run'
  | 'diag'
  | 'hipv'
  | 'jack'
  | 'sin'
  | 'cos'
  | 'tan'
  | 'asin'
  | 'acos'
  | 'atan'
  | 'sqrt'
  | 'square'
  | 'recip'
  | 'percent';

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
  | { type: 'memory'; op: 'store' | 'recall' | 'add' | 'clear' | 'clear_all'; slot?: number }
  | { type: 'backspace' }
  | { type: 'clear' }
  | { type: 'clearAll' };

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
      // wasm-pack web target requires explicit init.
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
    tape: [],
    error: null
  });

  async init() {
    const mod = await loadModule();
    this.inner = new mod.WasmCalculator();
    this.snapshot.set({ display: this.inner.displayString(), tape: [], error: null });
  }

  send(key: Key) {
    if (!this.inner) throw new Error('Calc not initialized; call init() first');
    const snap = this.inner.handle(key) as Snapshot;
    this.snapshot.set(snap);
  }
}

export const calc = new Calc();
