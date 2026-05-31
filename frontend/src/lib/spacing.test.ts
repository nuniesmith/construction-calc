import { describe, expect, it } from 'vitest';
import { gapLayout, onCenterLayout } from './spacing';

describe('gapLayout', () => {
  it('matches the canonical baluster case', () => {
    // 72" rail, 1.5" balusters, 4" max gap. minN = (72-4)/(1.5+4) = 12.36
    // → 13 balusters; gap = (72 - 13·1.5)/14 = 3.75" (≤ 4"). 12 would give
    // 54/13 = 4.15" > 4", so 13 is the minimum. (Identical to the existing
    // /ez/baluster form's formula.)
    const r = gapLayout(72, 1.5, 4);
    expect(r.count).toBe(13);
    expect(r.gap).toBeLessThanOrEqual(4);
    expect(r.gap).toBeCloseTo(3.75, 2);
    expect(r.onCenter).toBeCloseTo(r.gap + 1.5, 6);
  });

  it('keeps every gap within the limit', () => {
    const r = gapLayout(100, 2, 3);
    expect(r.gap).toBeLessThanOrEqual(3);
    // One fewer member would exceed the gap limit.
    const tighter = (100 - (r.count - 1) * 2) / r.count;
    expect(tighter).toBeGreaterThan(3);
  });

  it('returns zero members when the run fits under one gap', () => {
    const r = gapLayout(3, 1.5, 4);
    expect(r.count).toBe(0);
    expect(r.gap).toBe(3);
  });

  it('is empty for non-positive inputs', () => {
    expect(gapLayout(0, 1.5, 4)).toEqual({ count: 0, gap: 0, onCenter: 0 });
  });
});

describe('onCenterLayout', () => {
  it('counts both ends (joist convention)', () => {
    // 24' = 288" at 16" OC → ceil(288/16)=18 bays, 19 joists.
    const r = onCenterLayout(288, 16);
    expect(r.bays).toBe(18);
    expect(r.count).toBe(19);
    expect(r.spacing).toBe(16);
  });

  it('tightens spacing so it never exceeds the requested OC', () => {
    // 100" at 16" OC → ceil(100/16)=7 bays, spacing 100/7 ≈ 14.29".
    const r = onCenterLayout(100, 16);
    expect(r.bays).toBe(7);
    expect(r.count).toBe(8);
    expect(r.spacing).toBeCloseTo(14.2857, 3);
    expect(r.spacing).toBeLessThanOrEqual(16);
  });

  it('is empty for non-positive inputs', () => {
    expect(onCenterLayout(0, 16)).toEqual({ count: 0, spacing: 0, bays: 0 });
  });
});
