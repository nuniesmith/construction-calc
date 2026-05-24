/**
 * Context-help text for each key. Shown in an overlay after a long-press.
 * Keep entries terse — a sentence or two with a formula and an example.
 */

export interface HelpEntry {
  title: string;
  body: string;
  formula?: string;
  example?: string;
}

export const HELP: Record<string, HelpEntry> = {
  pitch: {
    title: 'Pitch',
    body: 'Roof slope as inches of rise per 12 inches of run. Enter a number then press Pitch — small integers are interpreted as "X/12".',
    formula: 'pitch = rise ÷ run',
    example: '6 [Pitch] = 6/12 pitch'
  },
  rise: {
    title: 'Rise',
    body: 'Vertical change. With pitch and run already entered, pressing Rise solves and shows the rise.'
  },
  run: {
    title: 'Run',
    body: 'Horizontal distance. With pitch and rise entered, pressing Run solves for the run.'
  },
  diag: {
    title: 'Diagonal',
    body: 'Common rafter length / hypotenuse. With any two of pitch, rise, run, this returns the diagonal.',
    formula: 'diag² = rise² + run²'
  },
  hipv: {
    title: 'Hip / Valley',
    body: 'Length of a regular (square) hip or valley rafter. Hip run = run × √2.'
  },
  jack: {
    title: 'Jack rafter',
    body: 'Difference in length between consecutive jack rafters at the current on-center spacing.'
  },
  sin: { title: 'Sine', body: 'sin of the displayed angle.' },
  cos: { title: 'Cosine', body: 'cos of the displayed angle.' },
  tan: { title: 'Tangent', body: 'tan of the displayed angle.' },
  asin: { title: 'Arcsine', body: 'Angle whose sine is the displayed value. Domain: −1 to 1.' },
  acos: { title: 'Arccosine', body: 'Angle whose cosine is the displayed value. Domain: −1 to 1.' },
  atan: { title: 'Arctangent', body: 'Angle whose tangent is the displayed value.' },
  sqrt: { title: 'Square root', body: 'Square root of the displayed value. Area → Length, Scalar → Scalar.' },
  square: { title: 'Square', body: 'Multiplies the displayed value by itself. Length × Length = Area.' },
  recip: { title: 'Reciprocal', body: '1 ÷ x' },
  percent: {
    title: 'Percent',
    body: 'After an operator, treats the entered number as a percent of the previous operand.',
    example: '200 + 5 % = adds 5% of 200 = 210'
  },
  // -------- Compound miter --------
  corner: {
    title: 'Corner angle',
    body: 'Wall corner viewed from above. 90° for a standard inside or outside corner. Enter a value (degrees) then press Corner.',
    example: '90 [Corner]'
  },
  spring: {
    title: 'Spring angle',
    body: 'Angle between the back of the molding and the wall. 38° and 45° are typical for crown — usually printed on the box.',
    example: '38 [Spring]'
  },
  miter: {
    title: 'Miter angle',
    body: 'Saw rotation. Press after entering both Corner and Spring to get the miter setting.'
  },
  bevel: {
    title: 'Bevel angle',
    body: 'Blade tilt off vertical. Press after entering both Corner and Spring to get the bevel setting.'
  },
  // -------- Units, control --------
  ft: { title: 'Feet', body: 'Tag the typed number as feet. Press multiple unit keys to build a compound length like 5 ft 6 in.' },
  in: { title: 'Inch', body: 'Tag the typed number as inches. Use the / key to enter a fraction like 3 / 8 [Inch].' },
  yd: { title: 'Yards', body: 'Tag the typed number as yards.' },
  m: { title: 'Meters', body: 'Tag the typed number as meters. Use the format strip above the display to render results in m.' },
  slash: {
    title: 'Fraction divider',
    body: 'Use during entry to type a fraction. After typing 3 / 8, you have 3/8. After 5, /, 3, /, 8, you have 5-3/8.'
  },
  negate: { title: 'Toggle sign', body: 'Flip the sign of the entry buffer.' },
  bs: { title: 'Backspace', body: 'Remove the last typed digit, decimal point, or fraction component.' },
  c: { title: 'Clear entry', body: 'Reset the entry buffer to zero. Pending operators and memory are preserved.' },
  ac: { title: 'All clear', body: 'Reset everything: entry, pending op, rafter state, miter state. Memory is preserved.' }
};
