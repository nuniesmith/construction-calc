/**
 * Pure rendering helpers for tape entries received from the WASM bridge.
 * Extracted from `Tape.svelte` so they can be unit-tested without spinning
 * up Svelte. The `Rational64` shape we get out of `serde_json` is `{numer,
 * denom}` (num-rational's default Serialize impl on its struct).
 */

export interface Rat {
  numer: number | string;
  denom: number | string;
}

/**
 * Render a Rational64 as a decimal with sensible precision. Uses BigInt for
 * the division so we don't lose precision for the giant denominators that
 * can come out of metric conversions (1mm = 5/127 in, so 100mm = 500/127).
 */
export function rationalApprox(r: Rat | null | undefined, precision = 4): string {
  if (!r) return '?';
  const n = BigInt(r.numer);
  const d = BigInt(r.denom);
  if (d === 0n) return 'NaN';
  const sign = n < 0n !== d < 0n ? '-' : '';
  const an = n < 0n ? -n : n;
  const ad = d < 0n ? -d : d;
  const whole = an / ad;
  let rem = an % ad;
  if (rem === 0n) return `${sign}${whole}`;
  let frac = '';
  for (let i = 0; i < precision && rem !== 0n; i++) {
    rem *= 10n;
    frac += (rem / ad).toString();
    rem %= ad;
  }
  // Trim trailing zeros
  frac = frac.replace(/0+$/, '');
  return frac ? `${sign}${whole}.${frac}` : `${sign}${whole}`;
}

export interface RenderedLength {
  kind: 'result';
  text: string;
  title: string;
}

/**
 * Render a Rational64 inches value as feet-inch-fraction at 1/16" precision —
 * matches the calculator's default display mode. The tooltip carries the
 * exact rational for users who want the canonical value.
 */
export function rationalLength(r: Rat): RenderedLength {
  const n = BigInt(r.numer);
  const d = BigInt(r.denom);
  // Round half-away-from-zero to the nearest 16th of an inch.
  const totalSixteenths = (n * 16n + (n >= 0n ? d / 2n : -(d / 2n))) / d;
  const negative = totalSixteenths < 0n;
  const abs = negative ? -totalSixteenths : totalSixteenths;
  const totalInches = abs / 16n;
  const sixteenths = Number(abs % 16n);
  const feet = totalInches / 12n;
  const inches = totalInches % 12n;

  const sign = negative ? '-' : '';
  let text: string;
  let frac = '';
  if (sixteenths !== 0) {
    // Reduce 16ths to lowest terms: gcd with 16 is 1, 2, 4, 8, or 16.
    let num = sixteenths;
    let den = 16;
    while (num % 2 === 0 && den % 2 === 0) {
      num /= 2;
      den /= 2;
    }
    frac = `${num}/${den}`;
  }

  if (feet === 0n && inches === 0n && !frac) text = '0"';
  else if (feet > 0n && inches === 0n && !frac) text = `${feet}'`;
  else if (feet === 0n && !frac) text = `${inches}"`;
  else if (feet === 0n && inches === 0n) text = `${frac}"`;
  else if (!frac) text = feet > 0n ? `${feet}' ${inches}"` : `${inches}"`;
  else if (feet === 0n) text = `${inches}-${frac}"`;
  else if (inches === 0n) text = `${feet}' ${frac}"`;
  else text = `${feet}' ${inches}-${frac}"`;

  return {
    kind: 'result',
    text: `${sign}${text}`,
    title: `Exact: ${r.numer}/${r.denom} in`,
  };
}
