import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import {
  DEFAULTS,
  loadPreferences,
  preferencesToAngleModeKey,
  preferencesToConvertKey,
  savePreferences
} from './preferences';

// vitest's jsdom env gives us a real localStorage. We clear it between
// tests so each one starts from a known-empty state.
beforeEach(() => {
  if (typeof localStorage !== 'undefined') localStorage.clear();
});
afterEach(() => {
  if (typeof localStorage !== 'undefined') localStorage.clear();
});

describe('preferences', () => {
  it('returns defaults when storage is empty', () => {
    expect(loadPreferences()).toEqual(DEFAULTS);
  });

  it('round-trips through save + load', () => {
    const p = { defaultFractionDenom: 8 as const, defaultLengthFormat: 'meters' as const, angleInDegrees: false };
    savePreferences(p);
    expect(loadPreferences()).toEqual(p);
  });

  it('falls back to defaults on corrupt JSON', () => {
    localStorage.setItem('cc.preferences.v1', '{not valid json');
    expect(loadPreferences()).toEqual(DEFAULTS);
  });

  it('coerces an invalid denom back to default', () => {
    localStorage.setItem(
      'cc.preferences.v1',
      JSON.stringify({ defaultFractionDenom: 32, defaultLengthFormat: 'feet_inch_fraction', angleInDegrees: true })
    );
    const p = loadPreferences();
    expect(p.defaultFractionDenom).toBe(16);
  });

  it('coerces an unknown format back to default', () => {
    localStorage.setItem(
      'cc.preferences.v1',
      JSON.stringify({ defaultFractionDenom: 16, defaultLengthFormat: 'parsecs', angleInDegrees: true })
    );
    const p = loadPreferences();
    expect(p.defaultLengthFormat).toBe('feet_inch_fraction');
  });
});

describe('preferencesToConvertKey', () => {
  it('produces a FeetInchFraction key with the configured denom', () => {
    const k = preferencesToConvertKey({ ...DEFAULTS, defaultFractionDenom: 4 });
    expect(k).toEqual({ type: 'convert', format: 'feet_inch_fraction', denom: 4 });
  });

  it('produces a meters key when meters chosen', () => {
    const k = preferencesToConvertKey({ ...DEFAULTS, defaultLengthFormat: 'meters' });
    expect(k).toEqual({ type: 'convert', format: 'm', precision: 4 });
  });

  it('produces a yards key when yards chosen', () => {
    const k = preferencesToConvertKey({ ...DEFAULTS, defaultLengthFormat: 'yards' });
    expect(k).toEqual({ type: 'convert', format: 'yards', precision: 4 });
  });
});

describe('preferencesToAngleModeKey', () => {
  it('maps degrees to a setAngleMode key', () => {
    const k = preferencesToAngleModeKey({ ...DEFAULTS, angleInDegrees: true });
    expect(k).toEqual({ type: 'setAngleMode', degrees: true });
  });

  it('maps radians to a setAngleMode key', () => {
    const k = preferencesToAngleModeKey({ ...DEFAULTS, angleInDegrees: false });
    expect(k).toEqual({ type: 'setAngleMode', degrees: false });
  });
});
