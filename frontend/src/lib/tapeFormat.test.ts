import { describe, expect, it } from 'vitest';
import { rationalApprox, rationalLength } from './tapeFormat';

describe('rationalApprox', () => {
  it('renders integers without a decimal point', () => {
    expect(rationalApprox({ numer: 5, denom: 1 })).toBe('5');
  });

  it('renders 1/2 as 0.5', () => {
    expect(rationalApprox({ numer: 1, denom: 2 })).toBe('0.5');
  });

  it('rounds to the requested precision and trims trailing zeros', () => {
    // 1/3 → 0.3333 at default precision=4
    expect(rationalApprox({ numer: 1, denom: 3 })).toBe('0.3333');
    // Trailing zeros pruned
    expect(rationalApprox({ numer: 1, denom: 4 })).toBe('0.25');
  });

  it('handles negative values', () => {
    expect(rationalApprox({ numer: -3, denom: 4 })).toBe('-0.75');
    // Negative denominator still produces a negative result
    expect(rationalApprox({ numer: 3, denom: -4 })).toBe('-0.75');
  });

  it('returns ? for null/undefined input', () => {
    expect(rationalApprox(null)).toBe('?');
    expect(rationalApprox(undefined)).toBe('?');
  });

  it('returns NaN for division by zero', () => {
    expect(rationalApprox({ numer: 5, denom: 0 })).toBe('NaN');
  });

  it('does not lose precision on large rationals (above MAX_SAFE_INTEGER)', () => {
    // 9_007_199_254_740_993 * 10 / 7  — both numer and denom fit in i64
    // but their ratio cannot be represented exactly as a JS number.
    const big = '9007199254740993';
    expect(rationalApprox({ numer: big, denom: '1' })).toBe(big);
  });

  it('honours the precision argument', () => {
    expect(rationalApprox({ numer: 1, denom: 7 }, 2)).toBe('0.14');
    expect(rationalApprox({ numer: 1, denom: 7 }, 6)).toBe('0.142857');
  });
});

describe('rationalLength', () => {
  it('renders 0 inches', () => {
    expect(rationalLength({ numer: 0, denom: 1 }).text).toBe('0"');
  });

  it('renders whole feet without inches', () => {
    // 60" = 5'
    expect(rationalLength({ numer: 60, denom: 1 }).text).toBe("5'");
  });

  it('renders inches under a foot', () => {
    expect(rationalLength({ numer: 7, denom: 1 }).text).toBe('7"');
  });

  it('renders feet plus whole inches', () => {
    // 5'6" = 66"
    expect(rationalLength({ numer: 66, denom: 1 }).text).toBe(`5' 6"`);
  });

  it('renders inches with a 1/16 fraction', () => {
    // 7-1/2"
    expect(rationalLength({ numer: 15, denom: 2 }).text).toBe('7-1/2"');
  });

  it('reduces the 1/16 fraction to lowest terms', () => {
    // 7-3/8" = 7 + 6/16
    expect(rationalLength({ numer: 59, denom: 8 }).text).toBe('7-3/8"');
    // 7-1/4" = 7 + 4/16
    expect(rationalLength({ numer: 29, denom: 4 }).text).toBe('7-1/4"');
  });

  it('renders feet plus a fraction (no whole inches)', () => {
    // 5' 1/2" = 60.5"
    expect(rationalLength({ numer: 121, denom: 2 }).text).toBe(`5' 1/2"`);
  });

  it('renders feet plus inches plus fraction', () => {
    // 5' 6-3/8" = 66 + 3/8 = 531/8
    expect(rationalLength({ numer: 531, denom: 8 }).text).toBe(`5' 6-3/8"`);
  });

  it('handles negative lengths exactly', () => {
    // -7.5" should be -7-1/2", NOT -3-3/4 (the regression I almost shipped)
    expect(rationalLength({ numer: -15, denom: 2 }).text).toBe('-7-1/2"');
  });

  it('rounds half values correctly (1/32 rounds up to 1/16)', () => {
    // 1/32" = 0.03125", at 1/16" precision rounds up to 1/16
    expect(rationalLength({ numer: 1, denom: 32 }).text).toBe('1/16"');
  });

  it('rounds tiny values to 0', () => {
    // 1/64" rounds half-away-from-zero to 0/16
    expect(rationalLength({ numer: 1, denom: 64 }).text).toBe('0"');
  });

  it('exposes the exact rational in the tooltip', () => {
    expect(rationalLength({ numer: 15, denom: 2 }).title).toBe('Exact: 15/2 in');
  });

  it('survives large numerators that overflow Number', () => {
    // 5/127 in = 1mm. A long chain of metric conversions can build up
    // arbitrarily large numerators; using BigInt internally must keep
    // the rendering accurate.
    // 1000mm = 5000/127 in ≈ 39.37" = 3' 3-3/8"
    const r = rationalLength({ numer: '5000', denom: '127' });
    // Should render close to 3' 3-3/8" (rounded to 1/16")
    expect(r.text).toMatch(/^3'/);
  });
});
