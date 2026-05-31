import { describe, expect, it } from 'vitest';
import { headerStock, plateStock, sheathingPanels } from './framing';

describe('sheathingPanels', () => {
  it('rounds up to whole 4×8 panels', () => {
    // 480 sq ft / 32 = 15 exactly, no waste.
    expect(sheathingPanels(480, 32, 0)).toBe(15);
    // 481 sq ft needs a 16th panel.
    expect(sheathingPanels(481, 32, 0)).toBe(16);
  });

  it('applies a waste percentage before dividing', () => {
    // 320 sq ft + 10% = 352 → 11 panels.
    expect(sheathingPanels(320, 32, 10)).toBe(11);
  });

  it('is zero for non-positive inputs', () => {
    expect(sheathingPanels(0, 32, 0)).toBe(0);
    expect(sheathingPanels(100, 0, 0)).toBe(0);
  });
});

describe('plateStock', () => {
  it('multiplies wall length by the plate count', () => {
    // 40' wall, 3 plates (bottom + double top) = 120 lin ft.
    const r = plateStock(40, 3, 16);
    expect(r.totalLinFt).toBe(120);
    // 120 / 16 = 7.5 → 8 boards.
    expect(r.boards).toBe(8);
  });

  it('rounds boards up to whole sticks', () => {
    // 24 lin ft / 10 = 2.4 → 3 boards.
    expect(plateStock(12, 2, 10).boards).toBe(3);
  });

  it('is zero for non-positive inputs', () => {
    expect(plateStock(0, 3, 16)).toEqual({ totalLinFt: 0, boards: 0 });
  });
});

describe('headerStock', () => {
  it('adds bearing on both ends of each opening', () => {
    // 36" opening + 1.5" bearing × 2 = 39" each.
    const r = headerStock(36, 1, 1.5);
    expect(r.eachIn).toBe(39);
    expect(r.totalLinFt).toBeCloseTo(39 / 12, 6);
  });

  it('totals across multiple openings', () => {
    // 3 openings × 39" = 117" = 9.75 lin ft.
    const r = headerStock(36, 3, 1.5);
    expect(r.totalLinFt).toBeCloseTo(9.75, 6);
  });

  it('is zero for non-positive inputs', () => {
    expect(headerStock(0, 2, 1.5)).toEqual({ eachIn: 0, totalLinFt: 0 });
  });
});
