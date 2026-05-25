/**
 * Vitest setup shim.
 *
 * Provides a minimal in-memory `localStorage` on `globalThis` so the
 * preferences and tape-store unit tests can run without spinning up
 * jsdom / happy-dom. The shim implements just the surface the modules
 * under test actually touch — `getItem`, `setItem`, `removeItem`,
 * `clear`. If we later need a real DOM, swap this for
 * `environment: 'happy-dom'` in vite.config.js.
 */

class InMemoryStorage {
  private store = new Map<string, string>();

  getItem(key: string): string | null {
    return this.store.has(key) ? (this.store.get(key) as string) : null;
  }

  setItem(key: string, value: string): void {
    this.store.set(key, value);
  }

  removeItem(key: string): void {
    this.store.delete(key);
  }

  clear(): void {
    this.store.clear();
  }
}

if (typeof globalThis.localStorage === 'undefined') {
  (globalThis as unknown as { localStorage: Storage }).localStorage =
    new InMemoryStorage() as unknown as Storage;
}

// `globalThis.crypto.getRandomValues` is built into Node 19+, which is
// well below our floor (CI uses Node 20). No shim needed — if you're
// on an older runtime, tapeStore's Math.random fallback kicks in.
