import { afterEach, describe, expect, it, vi } from 'vitest';
import { canShare, shareText } from './download';

afterEach(() => {
  vi.unstubAllGlobals();
});

describe('canShare', () => {
  it('is false when the Web Share API is absent', () => {
    vi.stubGlobal('navigator', {});
    expect(canShare()).toBe(false);
  });

  it('is true when navigator.share exists', () => {
    vi.stubGlobal('navigator', { share: () => Promise.resolve() });
    expect(canShare()).toBe(true);
  });
});

describe('shareText', () => {
  it('returns "unsupported" without the Web Share API', async () => {
    vi.stubGlobal('navigator', {});
    expect(await shareText('Title', 'body')).toBe('unsupported');
  });

  it('calls navigator.share with title + text and returns "shared"', async () => {
    const share = vi.fn().mockResolvedValue(undefined);
    vi.stubGlobal('navigator', { share });
    expect(await shareText('Title', 'body')).toBe('shared');
    expect(share).toHaveBeenCalledWith({ title: 'Title', text: 'body' });
  });

  it('treats a user-cancelled sheet (AbortError) as "cancelled"', async () => {
    const abort = new Error('user cancelled');
    abort.name = 'AbortError';
    vi.stubGlobal('navigator', { share: vi.fn().mockRejectedValue(abort) });
    expect(await shareText('Title', 'body')).toBe('cancelled');
  });

  it('rethrows non-abort errors so the caller can surface them', async () => {
    vi.stubGlobal('navigator', { share: vi.fn().mockRejectedValue(new Error('boom')) });
    await expect(shareText('Title', 'body')).rejects.toThrow('boom');
  });
});
