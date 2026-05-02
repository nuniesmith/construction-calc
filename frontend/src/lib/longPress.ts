/**
 * Svelte action: trigger a callback after the user holds down a button
 * for `delay` ms. Used for context-help on calculator keys — same as the
 * "press and hold any function key for help" pattern from the original.
 *
 * Usage:
 *   <button use:longPress={{ delay: 500, onLongPress: () => showHelp() }} />
 *
 * Cancels on pointer move beyond a small threshold so accidental drags
 * don't fire.
 */
export interface LongPressOptions {
  delay?: number;
  onLongPress: () => void;
  /** Pixel-distance after which we consider the press a drag and cancel. */
  cancelThreshold?: number;
}

export function longPress(node: HTMLElement, options: LongPressOptions) {
  let timer: ReturnType<typeof setTimeout> | null = null;
  let startX = 0;
  let startY = 0;
  let opts = options;

  function start(e: PointerEvent) {
    startX = e.clientX;
    startY = e.clientY;
    timer = setTimeout(() => {
      timer = null;
      opts.onLongPress();
    }, opts.delay ?? 500);
  }

  function move(e: PointerEvent) {
    if (timer === null) return;
    const dx = e.clientX - startX;
    const dy = e.clientY - startY;
    const threshold = opts.cancelThreshold ?? 10;
    if (Math.hypot(dx, dy) > threshold) {
      cancel();
    }
  }

  function cancel() {
    if (timer !== null) {
      clearTimeout(timer);
      timer = null;
    }
  }

  node.addEventListener('pointerdown', start);
  node.addEventListener('pointermove', move);
  node.addEventListener('pointerup', cancel);
  node.addEventListener('pointercancel', cancel);
  node.addEventListener('pointerleave', cancel);

  return {
    update(newOptions: LongPressOptions) {
      opts = newOptions;
    },
    destroy() {
      cancel();
      node.removeEventListener('pointerdown', start);
      node.removeEventListener('pointermove', move);
      node.removeEventListener('pointerup', cancel);
      node.removeEventListener('pointercancel', cancel);
      node.removeEventListener('pointerleave', cancel);
    }
  };
}
