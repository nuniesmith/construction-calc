/**
 * Browser-side download helper. Avoids each component re-implementing
 * the blob/anchor/click dance.
 *
 * Works in any modern browser; no library deps. Returns void — fire and
 * forget.
 */
export function downloadText(filename: string, contents: string, mime = 'text/plain') {
  const blob = new Blob([contents], { type: mime });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  // Some browsers require the anchor to be in the DOM to honor the click.
  document.body.appendChild(a);
  a.click();
  // Defer cleanup so the click has time to fire.
  setTimeout(() => {
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }, 0);
}

/** Cap on imported file size: a generous 5 MB. A real tape is a few KB; */
/* anything larger is almost certainly the wrong file or hostile input. */
const MAX_IMPORT_BYTES = 5 * 1024 * 1024;

export class FileTooLargeError extends Error {
  constructor(public size: number) {
    super(`file is ${size} bytes; max ${MAX_IMPORT_BYTES}`);
    this.name = 'FileTooLargeError';
  }
}

/**
 * Prompt the user for a file and return its text contents. Used for
 * loading a previously-saved tape JSON. Returns null if they cancel.
 * Throws `FileTooLargeError` if the file exceeds the import cap.
 */
export function pickTextFile(accept = '.json,application/json'): Promise<string | null> {
  return new Promise((resolve, reject) => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = accept;
    input.onchange = async () => {
      const file = input.files?.[0];
      if (!file) {
        resolve(null);
        return;
      }
      if (file.size > MAX_IMPORT_BYTES) {
        reject(new FileTooLargeError(file.size));
        return;
      }
      try {
        const text = await file.text();
        resolve(text);
      } catch (err) {
        reject(err);
      }
    };
    // Some browsers (Safari) need this in the DOM.
    document.body.appendChild(input);
    input.click();
    setTimeout(() => document.body.removeChild(input), 0);
  });
}
