import { defineConfig } from 'astro/config';

export default defineConfig({
  site: 'https://voidflare.app',
  // /privacy is served from privacy.html, so the address in the sitemap answers without a redirect.
  build: { format: 'file' },
  // The page script must stay a file of its own so the CSP can say script-src 'self' (no inline scripts).
  vite: { build: { assetsInlineLimit: 0 } },
});
