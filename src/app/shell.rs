//! The shared site chrome: a modern sticky header (real qbrand mark + nav +
//! theme-toggle island) and the footer.
//!
//! [`page`] is the one entry every marketing/route page calls. It wraps the
//! page's `<body>` content between the header and footer and hands the lot to
//! [`document`] for the full `<html>` + framed response.
//!
//! Brand: the nav mark is the **inlined** qbrand ink-Q recolored to
//! `currentColor` (the only sanctioned recolor), so it inherits the link's text
//! color and flips light/dark with the theme automatically — no second asset,
//! no `<img>` swap, no gradient-id collisions.

use qquill_view::{el, island, raw, text, Node, Trigger};

use crate::app::{document, respond_html, Css, Meta};

/// The top-nav links, in order.
const NAV: &[(&str, &str)] = &[
    ("Docs", "/docs"),
    ("Components", "/components"),
    ("Products", "/products"),
    ("Roadmap", "/roadmap"),
];

const GITHUB_URL: &str = "https://github.com/qirava/qirava";

/// The real Qirava brand mark — the qbrand ink-Q, inlined and recolored to
/// `currentColor`, plus the wordmark as live text. The mark's color comes from
/// the brand link's `color: var(--q-color-fg)` so it is dark on light and light
/// on dark with no asset swap.
fn brand() -> Node {
    let mark = raw(qbrand::recolor_currentcolor(qbrand::ICON_SVG));
    el("a")
        .class("q-brand")
        .attr("href", "/")
        .attr("aria-label", "Qirava — home")
        .child(el("span").class("q-brand__mark").child(mark))
        .child(el("span").class("q-brand__word").child(text("qirava")))
}

/// The theme-toggle control, shipped as a `theme` island. The SSR fallback is a
/// real, labelled `<button>`; the island wires the click to flip `data-q-theme`
/// on `<html>` and persist the choice. With JS off it is inert but present.
fn theme_toggle() -> Node {
    let fallback = el("button")
        .class("q-theme-toggle")
        .attr("type", "button")
        .attr("data-q-part", "toggle")
        .attr("aria-label", "Toggle color theme")
        .attr("title", "Toggle color theme")
        .child(el("span").class("q-theme-toggle__sun").attr("aria-hidden", "true").child(raw(SUN_SVG)))
        .child(el("span").class("q-theme-toggle__moon").attr("aria-hidden", "true").child(raw(MOON_SVG)));
    island("theme-toggle", "theme", Trigger::Load, "{}", fallback)
}

/// Inline glyphs for the toggle (decorative; one shows per theme via CSS).
const SUN_SVG: &str = "<svg viewBox=\"0 0 24 24\" width=\"18\" height=\"18\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\"><circle cx=\"12\" cy=\"12\" r=\"4\"/><path d=\"M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M19.1 4.9l-1.4 1.4M6.3 17.7l-1.4 1.4\"/></svg>";
const MOON_SVG: &str = "<svg viewBox=\"0 0 24 24\" width=\"18\" height=\"18\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z\"/></svg>";

/// Build the header: brand left, nav + theme toggle right. `active_path` marks
/// the current top-level section (prefix match) for `aria-current="page"`.
fn header(active_path: &str) -> Node {
    let mut nav = el("nav").class("q-nav").attr("aria-label", "Primary");
    for (label, href) in NAV {
        let active = active_path == *href
            || (*href != "/" && active_path.starts_with(href));
        let mut link = el("a").class("q-nav__link").attr("href", *href).child(text(*label));
        if active {
            link = link.attr("aria-current", "page");
        }
        nav = nav.child(link);
    }
    nav = nav.child(
        el("a")
            .class("q-nav__link q-nav__link--external")
            .attr("href", GITHUB_URL)
            .attr("rel", "noopener")
            .child(text("GitHub"))
            .child(raw(EXTERNAL_SVG)),
    );

    el("header").class("q-site-header").child(
        el("div")
            .class("q-site-header__inner")
            .child(brand())
            .child(el("div").class("q-nav-wrap").child(nav).child(theme_toggle())),
    )
}

const EXTERNAL_SVG: &str = "<svg class=\"q-ext\" viewBox=\"0 0 24 24\" width=\"13\" height=\"13\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M7 17 17 7M9 7h8v8\"/></svg>";

/// The site footer: brand mark, license, and grouped link columns.
fn footer() -> Node {
    let col = |title: &str, links: &[(&'static str, &'static str)]| {
        let mut c = el("div")
            .class("q-foot__col")
            .child(el("p").class("q-foot__title").child(text(title.to_string())));
        for (label, href) in links {
            c = c.child(el("a").attr("href", *href).child(text(*label)));
        }
        c
    };

    let mark = raw(qbrand::recolor_currentcolor(qbrand::ICON_SVG));
    let brand_block = el("div")
        .class("q-foot__brand")
        .child(el("span").class("q-brand__mark").child(mark))
        .child(el("p").class("q-foot__tag").child(text(
            "An AI-native, zero-dependency data system — and the Rust-native UI framework to build on it.",
        )));

    el("footer").class("q-site-footer").child(
        el("div")
            .class("q-site-footer__inner")
            .child(
                el("div")
                    .class("q-foot__top")
                    .child(brand_block)
                    .child(col("Product", &[("Docs", "/docs"), ("Components", "/components"), ("Products", "/products")]))
                    .child(col("Project", &[("Roadmap", "/roadmap"), ("GitHub", GITHUB_URL), ("Getting started", "/docs/getting-started")])),
            )
            .child(
                el("p")
                    .class("q-foot__legal")
                    .child(text("Qirava — Apache-2.0 licensed. \"Qirava\" is a trademark. Built with Quill, dogfooding itself.")),
            ),
    )
}

/// Assemble a full page response: skip-link, header, the page `content`, footer.
pub fn page(meta: &Meta, mut css: Css, content: Node) -> qexec::FunctionResponse {
    css.push(chrome_css().to_string());
    let body = el("div")
        .class("q-app")
        .child(
            el("a")
                .class("q-skiplink")
                .attr("href", "#main")
                .child(text("Skip to content")),
        )
        .child(header(meta.path))
        .child(content)
        .child(footer());

    let head_css = css.into_css();
    let tree = document(meta, head_css, body);
    respond_html(&tree)
}

/// Chrome-specific CSS (header/brand/nav/theme-toggle/footer). Layout-level
/// rules (`.q-main`, hero, etc.) live in `theme::layout_css`.
fn chrome_css() -> &'static str {
    "\
.q-site-header{position:sticky;top:0;z-index:20;backdrop-filter:saturate(1.4) blur(10px);background:color-mix(in srgb,var(--q-color-bg) 78%,transparent);border-bottom:1px solid var(--q-color-border)}\
.q-brand{display:inline-flex;align-items:center;gap:.55rem;line-height:0;color:var(--q-color-fg)}\
.q-brand:hover{text-decoration:none}\
.q-brand__mark{display:inline-flex;width:26px;height:26px}\
.q-brand__mark svg{width:100%;height:100%;display:block}\
.q-brand__word{font-weight:var(--q-font-weight-bold);letter-spacing:-.02em;font-size:1.1rem}\
.q-site-header__inner{max-width:78rem;margin:0 auto;display:flex;align-items:center;justify-content:space-between;gap:1rem;padding:.85rem 1.5rem}\
.q-nav-wrap{display:flex;align-items:center;gap:1.25rem}\
.q-nav{display:flex;align-items:center;gap:.25rem}\
.q-nav__link{display:inline-flex;align-items:center;gap:.25rem;color:var(--q-color-muted);font-weight:var(--q-font-weight-medium);font-size:.95rem;padding:.4rem .65rem;border-radius:var(--q-radius-md);transition:color var(--q-duration-fast) var(--q-ease-out),background-color var(--q-duration-fast) var(--q-ease-out)}\
.q-nav__link:hover{color:var(--q-color-fg);background-color:var(--q-color-surface);text-decoration:none}\
.q-nav__link[aria-current=\"page\"]{color:var(--q-color-brand)}\
.q-ext{opacity:.6}\
@media (max-width:720px){.q-nav{display:none}}\
.q-theme-toggle{display:inline-flex;align-items:center;justify-content:center;width:38px;height:38px;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);background:var(--q-color-surface);color:var(--q-color-fg);cursor:pointer;transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-theme-toggle:hover{border-color:var(--q-color-brand);transform:translateY(-1px)}\
.q-theme-toggle:focus-visible{outline:2px solid var(--q-color-brand);outline-offset:2px}\
.q-theme-toggle__sun{display:none}\
.q-theme-toggle__moon{display:inline-flex}\
[data-q-theme=\"light\"] .q-theme-toggle__sun{display:inline-flex}\
[data-q-theme=\"light\"] .q-theme-toggle__moon{display:none}\
.q-foot__top{display:grid;grid-template-columns:2fr 1fr 1fr;gap:2rem;padding:0 0 1.75rem;border-bottom:1px solid var(--q-color-border)}\
@media (max-width:720px){.q-foot__top{grid-template-columns:1fr}}\
.q-foot__brand .q-brand__mark{width:30px;height:30px;color:var(--q-color-fg)}\
.q-foot__tag{color:var(--q-color-muted);max-width:34ch;margin:.75rem 0 0;font-size:.92rem}\
.q-foot__col{display:flex;flex-direction:column;gap:.55rem}\
.q-foot__title{color:var(--q-color-fg);font-weight:var(--q-font-weight-bold);margin:0 0 .25rem;font-size:.85rem;text-transform:uppercase;letter-spacing:.08em}\
.q-foot__col a{color:var(--q-color-muted);font-size:.92rem}\
.q-foot__col a:hover{color:var(--q-color-fg)}\
.q-foot__legal{color:var(--q-color-muted);font-size:.85rem;margin:1.5rem 0 0}"
}
