import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import {
  clearAllTapes,
  deleteTape,
  getTape,
  listTapes,
  saveTape,
  TapeTooLargeError
} from './tapeStore';

beforeEach(() => {
  if (typeof localStorage !== 'undefined') localStorage.clear();
});
afterEach(() => {
  if (typeof localStorage !== 'undefined') localStorage.clear();
});

describe('tapeStore', () => {
  it('starts empty', () => {
    expect(listTapes()).toEqual([]);
  });

  it('saves and retrieves a tape', () => {
    const t = saveTape('Test', '{"entries":[]}');
    expect(t.name).toBe('Test');
    expect(t.json).toBe('{"entries":[]}');
    expect(t.id).toMatch(/^[0-9a-f]{16}$/);

    const got = getTape(t.id);
    expect(got).toEqual(t);
  });

  it('gives an automatic name when blank', () => {
    const t = saveTape('   ', '{"entries":[]}');
    expect(t.name.startsWith('Tape ')).toBe(true);
  });

  it('lists tapes newest first', async () => {
    const a = saveTape('first', '{"entries":[]}');
    // Sleep enough that Date.now() advances at least 1ms even on
    // very fast hardware.
    await new Promise((r) => setTimeout(r, 2));
    const b = saveTape('second', '{"entries":[]}');
    const list = listTapes();
    expect(list[0].id).toBe(b.id);
    expect(list[1].id).toBe(a.id);
  });

  it('deletes a single tape', () => {
    const a = saveTape('a', '{"entries":[]}');
    saveTape('b', '{"entries":[]}');
    deleteTape(a.id);
    expect(listTapes().map((t) => t.name)).toEqual(['b']);
  });

  it('clears every tape', () => {
    saveTape('a', '{"entries":[]}');
    saveTape('b', '{"entries":[]}');
    clearAllTapes();
    expect(listTapes()).toEqual([]);
  });

  it('rejects tapes larger than the size cap', () => {
    const huge = 'x'.repeat(2_000_000);
    expect(() => saveTape('huge', huge)).toThrow(TapeTooLargeError);
  });

  it('survives corrupt storage by returning empty', () => {
    localStorage.setItem('cc.tapes.v1', 'not json');
    expect(listTapes()).toEqual([]);
  });

  it('ignores entries with the wrong shape', () => {
    localStorage.setItem(
      'cc.tapes.v1',
      JSON.stringify([
        { id: 'a', name: 'good', json: '{}', createdAt: 1 },
        { id: 'b', name: 'bad' } // missing fields
      ])
    );
    const list = listTapes();
    expect(list).toHaveLength(1);
    expect(list[0].name).toBe('good');
  });
});
