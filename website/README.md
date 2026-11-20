# OpenWren website

The static site for [openwren.app](https://openwren.app), built with [Astro](https://astro.build). It is plain HTML and CSS; only the Stats page uses JavaScript. No cookies, no analytics, no third-party scripts or fonts (Inter is bundled from `@fontsource-variable/inter`).

## Run and build

Needs Node.js 22 or later and pnpm. From the project root:

| Command | What it does |
|---|---|
| `pnpm --dir website install` | Install dependencies |
| `pnpm --dir website dev` | Run the site with live reload at http://localhost:4321 |
| `pnpm --dir website build` | Build the site into `website/dist/` |
| `pnpm --dir website preview` | Serve the built `dist/` at http://localhost:4321 (add `--port 4400` for another port) |

## Pages

| Address | Source |
|---|---|
| `/` | `src/pages/index.astro` |
| `/download/` | `src/pages/download.astro` |
| `/privacy/` | `src/pages/privacy.md` |
| `/security/` | `src/pages/security.md` |
| `/docs/` | `src/pages/docs.md` |
| `/stats/` | `src/pages/stats.astro` |
| `/search/` | `src/pages/search.astro` |
| `/benchmarks/` | `src/pages/benchmarks.md` (a copy of `docs/benchmarks.md`; update both) |
| `/windows/`, `/mac/`, `/linux/` | `src/pages/windows.astro`, `mac.astro`, `linux.astro` |
| `/compare/` | `src/pages/compare.astro` |
| `/outlook-alternative/`, `/thunderbird-alternative/` | `src/pages/outlook-alternative.astro`, `thunderbird-alternative.astro` |
| `/outlook-search-not-working/` | `src/pages/outlook-search-not-working.astro` |
| not found | `src/pages/404.astro` |

The layout, header and footer are in `src/layouts/Base.astro`, which also writes each page's canonical address, link-preview tags and structured data; the sitemap is written by `@astrojs/sitemap` when the site is built. The list of things OpenWren does not do is in `src/components/Limits.astro`, shared by the comparison pages.

The colours and type are in `src/styles/global.css`. The repository address is set once in `src/site.ts`.

## Stats page

The page fetches `GET {STATS_URL}/api/daily?days=90` in the browser. `STATS_URL` is `https://stats.openwren.app` unless the environment variable `PUBLIC_STATS_URL` is set at build time. The Worker must allow cross-origin requests from `https://openwren.app` (an `Access-Control-Allow-Origin` header), or the page shows "The numbers could not be loaded".

### Checking it without the Worker

Sample data is in `src/fixtures/daily.json`. When `PUBLIC_STATS_URL` starts with `/fixture`, the build adds two local files that follow the Worker's contract (`src/pages/[...fixture].ts`); a normal build contains neither.

| Command | Shows |
|---|---|
| `pnpm --dir website build:fixture` then `preview` | The dashboard with the sample data, re-dated so its last day is today |
| `PUBLIC_STATS_URL=/fixture-empty pnpm --dir website build` then `preview` | The "No data yet" message |
| `PUBLIC_STATS_URL=/missing pnpm --dir website build` then `preview` | The "could not be loaded" message |

The same values work with `dev`, for example `PUBLIC_STATS_URL=/fixture pnpm --dir website dev`. Run a normal `build` again before deploying.

## Screenshots

Screenshots are in `src/assets/screens/` and are shown with `src/components/Screenshot.astro`, which converts them to WebP in the sizes the page needs. While a file is missing, the page shows a neutral frame with the logo instead. The demo video and its poster stay in `public/screens/`.

`public/og.png` (1200 × 630, from `docs/cover.png`) is the picture shown when a link to the site is shared. The logo `public/openwren-logo.svg` is a copy of `design/openwren-logo.svg`; `favicon-32.png` and `apple-touch-icon.png` are made from it.

## Deploying

The site is served at `openwren.app` as static files (`wrangler.toml`). After a change:

```bash
pnpm build && pnpm dlx wrangler deploy
```

Leave `PUBLIC_STATS_URL` unset to use `https://stats.openwren.app`.

