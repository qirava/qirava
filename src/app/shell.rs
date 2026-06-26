//! The shared site chrome: the sticky header nav and the footer.
//!
//! [`page`] is the one entry every route calls: it takes the page's `Meta`, its
//! `<body>` content `Node`, and the [`Css`] accumulator the page filled while
//! building that content, then wraps the content between the header and footer,
//! collects the chrome's own component CSS into the same accumulator, and hands
//! the lot to [`document`] for the full `<html>` + framed response.

use qquill_design::{NavLink, Navbar};
use qquill_view::{el, text, Node};

use crate::app::{document, respond_html, Css, Meta};

/// The top-nav links, in order. The index marked active per page comes from
/// [`page`]'s `active` argument.
const NAV: &[(&str, &str)] = &[
    ("Products", "/products"),
    ("Docs", "/docs"),
    ("Roadmap", "/roadmap"),
];

/// The real Qirava brand wordmark (the source SVGs from the brand assets),
/// linked home. Two variants ship — the dark-mode wordmark (light text) and the
/// light-mode wordmark (dark text) — and [`brand_css`] shows the right one for
/// the active `data-q-theme`, exactly as the brand usage rules require. They are
/// `<img>` (referencing the copied source files), not inlined, so their shared
/// gradient ids cannot collide.
fn brand() -> Node {
    el("a")
        .class("q-brand")
        .attr("href", "/")
        .attr("aria-label", "Qirava — home")
        .child(
            el("img")
                .class("q-brand__logo q-brand__logo--dark")
                .attr("src", "/qirava-logo-dark.svg")
                .attr("alt", "")
                .attr("aria-hidden", "true")
                .attr("width", "94")
                .attr("height", "26"),
        )
        .child(
            el("img")
                .class("q-brand__logo q-brand__logo--light")
                .attr("src", "/qirava-logo-light.svg")
                .attr("alt", "")
                .attr("aria-hidden", "true")
                .attr("width", "94")
                .attr("height", "26"),
        )
}

/// Size the brand logo and swap the dark/light wordmark by theme (dark default).
fn brand_css() -> &'static str {
    ".q-brand{display:inline-flex;align-items:center;line-height:0}\
     .q-brand__logo{height:26px;width:auto;display:block}\
     .q-brand__logo--light{display:none}\
     [data-q-theme=\"light\"] .q-brand__logo--dark{display:none}\
     [data-q-theme=\"light\"] .q-brand__logo--light{display:block}"
}

/// Build the header nav. `active_path` marks the current top-level section so
/// its link gets `aria-current="page"`. A "GitHub" link is appended after the
/// in-site links. Pushes the navbar + logo CSS into `css`.
fn header(css: &mut Css, active_path: &str) -> Node {
    let mut links: Vec<NavLink> = NAV
        .iter()
        .map(|(label, href)| NavLink::new(*label, *href))
        .collect();
    links.push(NavLink::new("GitHub", "https://github.com/qirava/qirava"));

    css.push(brand_css().to_string());
    let mut nav = Navbar::new(links).label("Primary").brand(brand());
    if let Some(i) = NAV.iter().position(|(_, href)| *href == active_path) {
        nav = nav.active(i);
    }

    el("header")
        .class("q-site-header")
        .child(css.node(nav.render()))
}

/// The site footer: license + a small link row.
fn footer() -> Node {
    let links = el("nav")
        .class("q-site-footer__links")
        .attr("aria-label", "Footer")
        .child(el("a").attr("href", "/products").child(text("Products")))
        .child(el("a").attr("href", "/docs").child(text("Docs")))
        .child(el("a").attr("href", "/roadmap").child(text("Roadmap")))
        .child(
            el("a")
                .attr("href", "https://github.com/qirava/qirava")
                .child(text("GitHub")),
        );

    el("footer").class("q-site-footer").child(
        el("div")
            .class("q-site-footer__inner")
            .child(
                el("span").child(text("Qirava — Apache-2.0 licensed. \"Qirava\" is a trademark.")),
            )
            .child(links),
    )
}

/// Assemble a full page response: skip-link, header, the page `content`, footer.
///
/// `content` is the route's `<main>`-and-below body; `css` is the accumulator it
/// filled. This adds the chrome CSS, then renders the document and frames it.
pub fn page(meta: &Meta, mut css: Css, content: Node) -> qexec::FunctionResponse {
    let header = header(&mut css, meta.path);

    let body = el("div")
        .child(
            el("a")
                .class("q-skiplink")
                .attr("href", "#main")
                .child(text("Skip to content")),
        )
        .child(header)
        .child(content)
        .child(footer());

    let head_css = css.into_css();
    let tree = document(meta, head_css, body);
    respond_html(&tree)
}
