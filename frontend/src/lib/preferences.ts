/**
 * User preferences, persisted to localStorage.
 *
 * The schema is versioned via the storage key so a future breaking
 * change (`v1` → `v2`) just falls back to defaults instead of crashing
 * on an old object shape. iOS will mirror this on UserDefaults using
 * the same Preferences shape.
 */

import type { Key } from './calc';

const STORAGE_KEY = 'cc.preferences.v1';

export type FractionDenom = 4 | 8 | 16;

export type DefaultLengthFormat =
  | 'feet_inch_fraction'
  | 'decimal_feet'
  | 'decimal_inches'
  | 'meters'
  | 'yards';

export interface Preferences {
  /** Rounding resolution for the feet-inch-fraction display. */
  defaultFractionDenom: FractionDenom;
  /** Which length format the calculator opens in. */
  defaultLengthFormat: DefaultLengthFormat;
  /** Trig keys interpret/return angles in degrees when true, radians when false. */
  angleInDegrees: boolean;
}

export const DEFAULTS: Preferences = {
  defaultFractionDenom: 16,
  defaultLengthFormat: 'feet_inch_fraction',
  angleInDegrees: true
};

export function loadPreferences(): Preferences {
  if (typeof localStorage === 'undefined') return { ...DEFAULTS };
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...DEFAULTS };
    const parsed = JSON.parse(raw);
    return {
      defaultFractionDenom: coerceDenom(parsed.defaultFractionDenom),
      defaultLengthFormat: coerceFormat(parsed.defaultLengthFormat),
      angleInDegrees: typeof parsed.angleInDegrees === 'boolean' ? parsed.angleInDegrees : true
    };
  } catch {
    // Corrupt JSON — fall back rather than wedge the app forever.
    return { ...DEFAULTS };
  }
}

export function savePreferences(p: Preferences): void {
  if (typeof localStorage === 'undefined') return;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(p));
}

function coerceDenom(v: unknown): FractionDenom {
  return v === 4 || v === 8 || v === 16 ? v : DEFAULTS.defaultFractionDenom;
}

function coerceFormat(v: unknown): DefaultLengthFormat {
  const allowed: DefaultLengthFormat[] = [
    'feet_inch_fraction',
    'decimal_feet',
    'decimal_inches',
    'meters',
    'yards'
  ];
  return allowed.includes(v as DefaultLengthFormat)
    ? (v as DefaultLengthFormat)
    : DEFAULTS.defaultLengthFormat;
}

/**
 * Translate the chosen format into the `Convert` KeyEvent the engine
 * accepts. Pulled out of `+page.svelte` so the iOS app can reuse it
 * when it ships.
 */
export function preferencesToConvertKey(p: Preferences): Key {
  switch (p.defaultLengthFormat) {
    case 'feet_inch_fraction':
      return {
        type: 'convert',
        format: 'feet_inch_fraction',
        denom: p.defaultFractionDenom
      };
    case 'decimal_feet':
      return { type: 'convert', format: 'decimal_feet', precision: 4 };
    case 'decimal_inches':
      return { type: 'convert', format: 'decimal_inches', precision: 4 };
    case 'meters':
      return { type: 'convert', format: 'm', precision: 4 };
    case 'yards':
      return { type: 'convert', format: 'yards', precision: 4 };
  }
}

/**
 * Translate the angle preference into the engine `SetAngleMode` event.
 * Sent alongside `preferencesToConvertKey` at launch so trig keys honour
 * the stored degrees/radians choice.
 */
export function preferencesToAngleModeKey(p: Preferences): Key {
  return { type: 'setAngleMode', degrees: p.angleInDegrees };
}
