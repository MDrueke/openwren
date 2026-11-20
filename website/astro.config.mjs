import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';

// Static output (the default). No adapters; the only integration writes the sitemap.
export default defineConfig({
  site: 'https://openwren.app',
  integrations: [
    // The sample-data routes of the Stats page and the 404 page are not for search engines.
    sitemap({ filter: (page) => !page.includes('/fixture') && !page.includes('/404') }),
  ],
  // Keep line breaks in the HTML: compressing them removes the space before a link that starts a line.
  compressHTML: false,
  markdown: {
    // Code blocks are styled by the site's own CSS, so they follow light and dark.
    syntaxHighlight: false,
  },
});
