/**
 * Framing material estimators — pure functions so they can be unit-tested
 * without mounting a Svelte component. The `/ez/framing` form is a thin
 * shell over these.
 *
 * All inputs are plain numbers in the units named by the parameter; callers
 * are responsible for unit entry. Counts are returned already rounded up to
 * whole pieces (you can't buy 0.4 of a sheet).
 */

/**
 * Number of sheathing panels to cover an area.
 * A standard panel is 4' × 8' = 32 sq ft. `wastePct` (e.g. 10 for 10%)
 * inflates the area before dividing, then we round up to whole panels.
 */
export function sheathingPanels(
  areaSqFt: number,
  panelSqFt: number,
  wastePct: number
): number {
  if (areaSqFt <= 0 || panelSqFt <= 0) return 0;
  const withWaste = areaSqFt * (1 + Math.max(0, wastePct) / 100);
  return Math.ceil(withWaste / panelSqFt);
}

export interface PlateResult {
  /** Total lineal feet of plate stock (wall length × number of plates). */
  totalLinFt: number;
  /** Stock boards needed at the chosen length, rounded up. */
  boards: number;
}

/**
 * Plate stock for a wall. A typical wall has 3 plates: one bottom plate and
 * a doubled top plate. `plateCount` lets you pick 1–3. `stockLenFt` is the
 * length of board you buy (8/10/12/16 ft).
 */
export function plateStock(
  wallLenFt: number,
  plateCount: number,
  stockLenFt: number
): PlateResult {
  if (wallLenFt <= 0 || plateCount <= 0) return { totalLinFt: 0, boards: 0 };
  const totalLinFt = wallLenFt * plateCount;
  const boards = stockLenFt > 0 ? Math.ceil(totalLinFt / stockLenFt) : 0;
  return { totalLinFt, boards };
}

export interface HeaderResult {
  /** Length of one header: opening width + bearing on both sides. */
  eachIn: number;
  /** Total lineal feet of header stock for all openings. */
  totalLinFt: number;
}

/**
 * Header stock for `count` openings. Each header spans the rough opening
 * plus a bearing length resting on the jack stud at each end
 * (`bearingEachIn`, typically 1.5" for 2× material).
 */
export function headerStock(
  openingWidthIn: number,
  count: number,
  bearingEachIn: number
): HeaderResult {
  if (openingWidthIn <= 0 || count <= 0) return { eachIn: 0, totalLinFt: 0 };
  const eachIn = openingWidthIn + 2 * Math.max(0, bearingEachIn);
  const totalLinFt = (eachIn * count) / 12;
  return { eachIn, totalLinFt };
}
