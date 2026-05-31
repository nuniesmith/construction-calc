/**
 * Equal-spacing on-center divider — the generalized form of the baluster
 * layout, usable for joists, pickets, studs, balusters, etc.
 *
 * Two layout conventions, because both come up on a job site:
 *
 *  - `gap`  : fit N members of width W into a run so the *gaps between and
 *             around them* are equal and no larger than a code limit
 *             (the baluster / picket problem). End gaps included.
 *  - `oncenter` : place members at a fixed on-center spacing across a run,
 *             counting both end members (the joist / stud problem).
 *
 * Pure functions, unit-tested. Lengths are plain numbers in a single unit
 * (the form uses inches); the result echoes that unit.
 */

export type SpacingMode = 'gap' | 'oncenter';

export interface GapLayout {
  /** Number of members that fit. */
  count: number;
  /** Equal gap between/around members (≤ the requested max). */
  gap: number;
  /** Center-to-center spacing = gap + member width. */
  onCenter: number;
}

/**
 * Max-gap layout: the fewest members so every gap (including the two end
 * gaps) is ≤ `maxGap`.
 *
 *   run = n·width + (n+1)·gap   ⇒   gap = (run − n·width) / (n + 1)
 *   gap ≤ maxGap                ⇒   n ≥ (run − maxGap) / (width + maxGap)
 */
export function gapLayout(run: number, width: number, maxGap: number): GapLayout {
  if (run <= 0 || width <= 0 || maxGap <= 0) {
    return { count: 0, gap: 0, onCenter: 0 };
  }
  const minN = (run - maxGap) / (width + maxGap);
  const count = Math.max(0, Math.ceil(minN));
  if (count === 0) return { count: 0, gap: run, onCenter: 0 };
  const gap = (run - count * width) / (count + 1);
  return { count, gap, onCenter: gap + width };
}

export interface OnCenterLayout {
  /** Members placed, counting both ends. */
  count: number;
  /** The actual on-center spacing used (≤ the requested spacing). */
  spacing: number;
  /** Number of bays (gaps) between members = count − 1. */
  bays: number;
}

/**
 * Fixed on-center layout across a run, including a member at each end.
 * `ceil(run / oc) + 1` is the standard framer's count (the `+1` is the
 * closing end member). The actual spacing is the run divided evenly across
 * the bays, so it never exceeds the requested `oc`.
 */
export function onCenterLayout(run: number, oc: number): OnCenterLayout {
  if (run <= 0 || oc <= 0) return { count: 0, spacing: 0, bays: 0 };
  const bays = Math.ceil(run / oc);
  const count = bays + 1;
  const spacing = run / bays;
  return { count, spacing, bays };
}
