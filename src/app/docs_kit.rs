//! A small docs-site layout built on `qquill-docs` primitives, rendered inside
//! the site's own shell (one consistent header/theme-toggle across the whole
//! site). It owns the LEFT SIDEBAR (sections → pages), the content column, an
//! on-page TABLE OF CONTENTS, and prev/next, plus `heading()` permalink anchors
//! and `Callout`s from `qquill-docs`. Copy-enabled code comes from the site's
//! own `copy` island (`routes::copy_code`).

use qquill_view::{el, text, Node};

/// One page in the docs nav (sidebar order = slice order; sections group by
/// first appearance).
pub struct DocRef {
    pub path: &'static str,
    pub title: &'static str,
    pub section: &'static str,
}

/// Every docs page, in sidebar order. Adding a page here lists it in the
/// sidebar and wires prev/next automatically.
pub const DOCS: &[DocRef] = &[
    DocRef { path: "/docs", title: "Overview", section: "Start here" },
    DocRef { path: "/docs/getting-started", title: "Getting started", section: "Start here" },
    DocRef { path: "/docs/concepts", title: "Core concepts", section: "Concepts" },
];

/// A heading captured for the on-page table of contents: its slug (id) + text.
pub struct Toc {
    entries: Vec<(String, String)>,
}

impl Toc {
    pub fn new() -> Self {
        Toc { entries: Vec::new() }
    }

    /// Record an H2 (`slug`, `title`) for the TOC, returning the rendered
    /// heading node (a `qquill_docs::heading` with a permalink anchor).
    pub fn h2(&mut self, title: &str) -> Node {
        let slug = qquill_docs::slugify(title);
        self.entries.push((slug.clone(), title.to_string()));
        qquill_docs::heading(2, title)
    }

    fn render(&self) -> Node {
        let mut nav = el("nav").class("q-docs__toc").attr("aria-label", "On this page");
        nav = nav.child(el("p").class("q-docs__toc-label").child(text("On this page")));
        for (slug, title) in &self.entries {
            nav = nav.child(
                el("a")
                    .attr("href", format!("#{slug}"))
                    .child(text(title.clone())),
            );
        }
        nav
    }
}

/// The LEFT SIDEBAR: sections (first-appearance order) → page links, with the
/// current page marked `aria-current="page"`.
fn sidebar(current: &str) -> Node {
    let mut nav = el("nav").class("q-docs__side").attr("aria-label", "Documentation");
    let mut seen: Vec<&'static str> = Vec::new();
    for d in DOCS {
        if !seen.contains(&d.section) {
            seen.push(d.section);
        }
    }
    for section in seen {
        let mut sec = el("div").class("q-docs__sec");
        sec = sec.child(el("p").class("q-docs__sec-label").child(text(section.to_string())));
        let mut list = el("div").class("q-docs__nav");
        for d in DOCS.iter().filter(|d| d.section == section) {
            let mut link = el("a").attr("href", d.path).child(text(d.title.to_string()));
            if d.path == current {
                link = link.attr("aria-current", "page");
            }
            list = list.child(link);
        }
        sec = sec.child(list);
        nav = nav.child(sec);
    }
    nav
}

/// Prev/next links derived from the `DOCS` order for `current`.
fn prev_next(current: &str) -> Node {
    let idx = DOCS.iter().position(|d| d.path == current);
    let prev = idx.and_then(|i| i.checked_sub(1)).and_then(|i| DOCS.get(i));
    let next = idx.and_then(|i| DOCS.get(i + 1));

    let mut row = el("nav").class("q-docs__pager").attr("aria-label", "Pagination");
    if let Some(p) = prev {
        row = row.child(
            el("a")
                .class("q-pager q-pager--prev")
                .attr("href", p.path)
                .child(el("span").class("q-pager__dir").child(text("← Previous")))
                .child(el("span").class("q-pager__title").child(text(p.title.to_string()))),
        );
    } else {
        row = row.child(el("span"));
    }
    if let Some(n) = next {
        row = row.child(
            el("a")
                .class("q-pager q-pager--next")
                .attr("href", n.path)
                .child(el("span").class("q-pager__dir").child(text("Next →")))
                .child(el("span").class("q-pager__title").child(text(n.title.to_string()))),
        );
    }
    row
}

/// Assemble the full docs `<main>`: sidebar + (h1 + content + pager) + TOC.
/// `content` is the page body the route built (interleaving `toc.h2(..)` and
/// prose); `toc` is the same accumulator, rendered into the right rail.
pub fn layout(current: &str, title: &str, lead: &str, content: Node, toc: Toc) -> Node {
    let main = el("article")
        .class("q-docs__main")
        .child(el("h1").child(text(title.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())))
        .child(content)
        .child(prev_next(current));

    el("main")
        .class("q-docs")
        .id("main")
        .child(sidebar(current))
        .child(main)
        .child(toc.render())
}

/// The docs-pager CSS (kept here next to the markup it styles).
pub fn pager_css() -> &'static str {
    "\
.q-docs__pager{display:flex;justify-content:space-between;gap:1rem;margin:3rem 0 0;padding-top:1.5rem;border-top:1px solid var(--q-color-border)}\
.q-pager{display:flex;flex-direction:column;gap:.2rem;padding:.8rem 1rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);min-width:0;transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-pager:hover{border-color:var(--q-color-brand);text-decoration:none;transform:translateY(-2px)}\
.q-pager--next{text-align:right;margin-left:auto}\
.q-pager__dir{font-size:.78rem;color:var(--q-color-muted)}\
.q-pager__title{color:var(--q-color-fg);font-weight:var(--q-font-weight-medium)}"
}
