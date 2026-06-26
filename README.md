# Qirava website (`qirava-site`)

The public Qirava website — and a working example of a **Qirava Quill app**. It
dogfoods Quill: the whole site is a plain Rust binary that renders its pages with
the first-party `tqquill-*` crates and exports a static, zero-JavaScript site.

It lives beside `qdms/`, `qquill/`, and `qpkgs/` under the Qirava root and depends
on them via **relative path dependencies** (`../qdms`, `../qquill/qquill-*`,
`../qpkgs/*`). **Zero third-party dependencies** — std + Qirava crates only.

## Pages

One `PAGES` list in `src/main.rs` drives both serve and build:

| Path        | Page      | What it is                                                        |
| ----------- | --------- | ---------------------------------------------------------------- |
| `/`         | Landing   | Hero, the two pillars, the three products, architecture, quickstart |
| `/products` | Products  | Overview cards: DMS, Quill, the tq* stdlib, and the planned Cloud |
| `/docs`     | Docs      | A curated index linking the repo's docs in reading order         |
| `/roadmap`  | Roadmap   | The honest BUILT / PARTIAL / PLANNED status matrix               |

Top nav: **Products · Docs · Roadmap · GitHub** (brand wordmark links home).

## Run (serve)

```sh
cargo run
# serves on http://127.0.0.1:7179/  (override with QUILL_ADDR=127.0.0.1:8080)
```

`cargo run` opens an in-memory DMS, registers each page's render handler, and
serves server-rendered HTML over a worker — the same path the static export uses,
so what you see locally is byte-identical to what you ship.

## Build (static export)

```sh
cargo run -- build           # -> dist/
cargo run -- build out       # -> out/
```

This renders every page in-process (no database, no socket) and writes a
CDN-ready `dist/`: one HTML file per route with pretty URLs (`/products` →
`products/index.html`), the copied `public/` assets, and Cloudflare-Pages-style
`_headers` / `_redirects` / `404.html`. Every page is static SSR/SSG content with
**no islands**, so the export ships **zero JavaScript**.

## Deploy

`dist/` is a plain static site — drop it on any static host (Cloudflare Pages,
S3, nginx). It serves with no DMS running. See `qquill/QUICKSTART.md` for the
Quill app lifecycle in general.

## Layout

```
src/
  main.rs            PAGES list + serve/build modes (the scaffolder pattern)
  app/
    mod.rs           document() + the CSS accumulator + respond_html()
    theme.rs         site layout CSS (colors come from tqquill-theme tokens)
    shell.rs         the shared header nav + footer
    routes/
      mod.rs         shared helpers: status badges, sections, code blocks
      index.rs       /          landing
      products.rs    /products  products overview
      docs.rs        /docs      docs index
      roadmap.rs     /roadmap   status matrix
public/              favicon.svg, robots.txt, manifest.webmanifest (copied as-is)
```

## Editing

* **Content** — edit the page in `src/app/routes/`.
* **Look & feel** — edit `src/app/theme.rs` (the site layout) or swap the
  `tqquill-theme` token set in `src/app/mod.rs::document`.
* **A new page** — add a `respond()` in `src/app/routes/`, list it in `routes/mod.rs`,
  and add one `Page { … }` to `PAGES` in `src/main.rs`. Both serve and build pick
  it up automatically.

Licensed Apache-2.0. "Qirava" is a trademark.
