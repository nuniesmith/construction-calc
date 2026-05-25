import { sveltekit } from '@sveltejs/kit/vite';

export default {
  plugins: [sveltekit()],
  server: { fs: { allow: ['..'] } },
  test: {
    // Inject an in-memory localStorage so the preferences / saved-tapes
    // unit tests can exercise the real persistence code without a DOM env.
    setupFiles: ['src/lib/test-setup.ts']
  }
};
