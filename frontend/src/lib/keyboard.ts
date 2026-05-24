/**
 * Wire up physical-keyboard support so the calculator is usable on desktop
 * without clicking. Maps common keys to engine events.
 *
 * Mappings:
 *   0-9, .         → digits / decimal
 *   + - * /        → operators (note: / can be either op or fraction; we
 *                    treat it as op only when no entry is active. Press
 *                    `Shift+/` (`?`) for the fraction slash.)
 *   ?              → fraction slash
 *   Enter, =       → equals
 *   Backspace      → backspace
 *   Delete, Escape → clear / clear-all
 *   F              → Feet
 *   I              → Inch
 *   Y              → Yards
 *   M              → Meters
 *
 * Returns a teardown to call on cleanup.
 */
import type { Calc } from './calc';

export function bindKeyboard(calc: Calc): () => void {
  function onKey(e: KeyboardEvent) {
    // Ignore when typing in form inputs (the EZ Calc forms have several).
    const target = e.target as HTMLElement | null;
    if (
      target &&
      (target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.tagName === 'SELECT' ||
        target.isContentEditable)
    ) {
      return;
    }
    // Don't hijack browser/OS shortcuts (Ctrl+R reload, Cmd+0 zoom reset,
    // Alt+Tab, etc.). Note: Shift is fine — it's how `?` and `+` are typed.
    if (e.ctrlKey || e.metaKey || e.altKey) {
      return;
    }
    const k = e.key;
    if (k >= '0' && k <= '9') {
      calc.send({ type: 'digit', value: parseInt(k, 10) });
      e.preventDefault();
    } else if (k === '.') {
      calc.send({ type: 'decimal' });
      e.preventDefault();
    } else if (k === '+') {
      calc.send({ type: 'op', op: 'add' });
      e.preventDefault();
    } else if (k === '-') {
      calc.send({ type: 'op', op: 'sub' });
      e.preventDefault();
    } else if (k === '*' || k === 'x' || k === 'X') {
      calc.send({ type: 'op', op: 'mul' });
      e.preventDefault();
    } else if (k === '/') {
      calc.send({ type: 'op', op: 'div' });
      e.preventDefault();
    } else if (k === '?') {
      calc.send({ type: 'slash' });
      e.preventDefault();
    } else if (k === 'Enter' || k === '=') {
      calc.send({ type: 'equals' });
      e.preventDefault();
    } else if (k === 'Backspace') {
      calc.send({ type: 'backspace' });
      e.preventDefault();
    } else if (k === 'Delete') {
      calc.send({ type: 'clear' });
      e.preventDefault();
    } else if (k === 'Escape') {
      calc.send({ type: 'clearAll' });
      e.preventDefault();
    } else if (k === 'f' || k === 'F') {
      calc.send({ type: 'unit', unit: 'ft' });
      e.preventDefault();
    } else if (k === 'i' || k === 'I') {
      calc.send({ type: 'unit', unit: 'in' });
      e.preventDefault();
    } else if (k === 'y' || k === 'Y') {
      calc.send({ type: 'unit', unit: 'yd' });
      e.preventDefault();
    } else if (k === 'm' || k === 'M') {
      calc.send({ type: 'unit', unit: 'm' });
      e.preventDefault();
    }
  }

  document.addEventListener('keydown', onKey);
  return () => document.removeEventListener('keydown', onKey);
}
