/**
 * Persisted saved-tapes store.
 *
 * Backed by localStorage on web. The iOS app will mirror this on
 * UserDefaults (or CloudKit for sync). Same `SavedTape` shape and
 * same JSON body so a tape exported from the web can be imported on
 * iOS without translation.
 *
 * Tapes are stored as `{id, name, json, createdAt}`. `json` is the
 * raw `Calculator::export_json()` output — the engine knows how to
 * round-trip it via `load_json_tape`.
 */

const STORAGE_KEY = 'cc.tapes.v1';

// Soft cap on the per-tape payload to keep localStorage healthy. The
// browser quota is ~5 MB; a calc tape with hundreds of entries is
// still only ~50 KB, so 1 MB is comfortable.
const MAX_TAPE_BYTES = 1_000_000;

// Soft cap on tape count. After 200, the list view gets unwieldy and
// users should be exporting + deleting old work.
const MAX_TAPES = 200;

export class TapeTooLargeError extends Error {
  constructor(public size: number) {
    super(`tape JSON is ${size} bytes (limit ${MAX_TAPE_BYTES})`);
  }
}

export class TapeLimitReachedError extends Error {
  constructor() {
    super(`reached the ${MAX_TAPES}-tape limit; delete some first`);
  }
}

export interface SavedTape {
  id: string;
  name: string;
  json: string;
  createdAt: number;
}

function readAll(): SavedTape[] {
  if (typeof localStorage === 'undefined') return [];
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.filter(isSavedTape);
  } catch {
    return [];
  }
}

function isSavedTape(v: unknown): v is SavedTape {
  if (typeof v !== 'object' || v === null) return false;
  const t = v as Record<string, unknown>;
  return (
    typeof t.id === 'string' &&
    typeof t.name === 'string' &&
    typeof t.json === 'string' &&
    typeof t.createdAt === 'number'
  );
}

function writeAll(tapes: SavedTape[]): void {
  if (typeof localStorage === 'undefined') return;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(tapes));
}

/** Newest first. */
export function listTapes(): SavedTape[] {
  return readAll().sort((a, b) => b.createdAt - a.createdAt);
}

export function saveTape(name: string, json: string): SavedTape {
  if (json.length > MAX_TAPE_BYTES) {
    throw new TapeTooLargeError(json.length);
  }
  const existing = readAll();
  if (existing.length >= MAX_TAPES) {
    throw new TapeLimitReachedError();
  }
  const tape: SavedTape = {
    id: makeId(),
    name: name.trim() || `Tape ${new Date().toLocaleString()}`,
    json,
    createdAt: Date.now()
  };
  existing.push(tape);
  writeAll(existing);
  return tape;
}

export function deleteTape(id: string): void {
  writeAll(readAll().filter((t) => t.id !== id));
}

export function getTape(id: string): SavedTape | undefined {
  return readAll().find((t) => t.id === id);
}

/** Wipe every saved tape. Used by the "delete all" affordance on /tapes. */
export function clearAllTapes(): void {
  writeAll([]);
}

function makeId(): string {
  // 16 random hex chars is plenty for non-cryptographic uniqueness
  // and survives JSON round trips trivially.
  const bytes = new Uint8Array(8);
  if (typeof crypto !== 'undefined' && crypto.getRandomValues) {
    crypto.getRandomValues(bytes);
  } else {
    for (let i = 0; i < bytes.length; i++) bytes[i] = Math.floor(Math.random() * 256);
  }
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
}
