//! A PER-PRODUCT docs-site layout built on `qquill-docs` primitives, rendered
//! inside the site's own shell (one consistent header/theme-toggle across the
//! whole site).
//!
//! ## The per-product model (like Next.js / shadcn)
//!
//! Docs live at `/docs/<product>/<page>` for four products: **dms**, **quill**,
//! **stdlib**, **cloud**. Every doc page is one [`DocRef`] in the single [`DOCS`]
//! table, and each `DocRef` declares its [`Product`] + a `section` string + a
//! title.
//!
//! When a `/docs/<product>/*` page renders, the LEFT SIDEBAR shows ONLY that
//! product's sections and pages (grouped by first-appearance section order, the
//! current page marked `aria-current="page"`), and prev/next walk WITHIN that
//! product. The sidebar is also topped by a compact product switcher so a reader
//! can hop between product doc sets.
//!
//! Adding a page is one line in [`DOCS`] (plus a handler in `routes/docs.rs` and
//! a row in `PAGES`): pick the product, name the section, give it a title — it
//! appears in that product's sidebar, in order, with prev/next wired.

use qquill_view::{el, text, Node};

/// The four documentation products. Each owns an isolated sidebar + pager scope
/// and a landing page at `/docs/<slug>`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Product {
    Dms,
    Quill,
    Stdlib,
    Cloud,
}

impl Product {
    /// The URL slug (`/docs/<slug>`). Part of the per-product API for the
    /// stages that add pages; kept even while only `landing()` is wired today.
    #[allow(dead_code)]
    pub fn slug(self) -> &'static str {
        match self {
            Product::Dms => "dms",
            Product::Quill => "quill",
            Product::Stdlib => "stdlib",
            Product::Cloud => "cloud",
        }
    }

    /// The display name shown in the sidebar header + switcher.
    pub fn name(self) -> &'static str {
        match self {
            Product::Dms => "Qirava DMS",
            Product::Quill => "Quill",
            Product::Stdlib => "The q* stdlib",
            Product::Cloud => "Qirava Cloud",
        }
    }

    /// The landing path for this product's docs.
    pub fn landing(self) -> &'static str {
        match self {
            Product::Dms => "/docs/dms",
            Product::Quill => "/docs/quill",
            Product::Stdlib => "/docs/stdlib",
            Product::Cloud => "/docs/cloud",
        }
    }

    /// All four products, in switcher order.
    pub const ALL: [Product; 4] = [Product::Dms, Product::Quill, Product::Stdlib, Product::Cloud];
}

/// One page in the docs nav. Sidebar order = slice order within a product;
/// sections group by first appearance within that product.
pub struct DocRef {
    pub path: &'static str,
    pub title: &'static str,
    pub product: Product,
    pub section: &'static str,
}

/// EVERY docs page, across all products, in sidebar order. Adding a page here
/// lists it in that product's sidebar and wires prev/next within the product
/// automatically. Keep each product's pages contiguous for readability (the
/// scoping does not require it, but the source reads better).
pub const DOCS: &[DocRef] = &[
    // ---- DMS -------------------------------------------------------------
    DocRef { path: "/docs/dms", title: "Overview", product: Product::Dms, section: "Start here" },
    DocRef { path: "/docs/dms/getting-started", title: "Installation", product: Product::Dms, section: "Get started" },
    DocRef { path: "/docs/dms/quick-start", title: "Quick start", product: Product::Dms, section: "Get started" },
    DocRef { path: "/docs/dms/concepts", title: "Core concepts", product: Product::Dms, section: "Core" },
    DocRef { path: "/docs/dms/execute-model", title: "The execute model", product: Product::Dms, section: "Core" },
    DocRef { path: "/docs/dms/workers", title: "Workers", product: Product::Dms, section: "Core" },
    DocRef { path: "/docs/dms/access-control", title: "Access control", product: Product::Dms, section: "Core" },
    DocRef { path: "/docs/dms/qql-reading", title: "Reading", product: Product::Dms, section: "QQL" },
    DocRef { path: "/docs/dms/qql-writing", title: "Writing", product: Product::Dms, section: "QQL" },
    DocRef { path: "/docs/dms/qql-search", title: "Search", product: Product::Dms, section: "QQL" },
    DocRef { path: "/docs/dms/qql-graph", title: "Graph", product: Product::Dms, section: "QQL" },
    DocRef { path: "/docs/dms/qql-vector", title: "Vector", product: Product::Dms, section: "QQL" },
    DocRef { path: "/docs/dms/api-spec", title: "The self-describing API", product: Product::Dms, section: "API" },
    DocRef { path: "/docs/dms/api-envelope", title: "Envelope & error codes", product: Product::Dms, section: "API" },
    // ---- Quill -----------------------------------------------------------
    DocRef { path: "/docs/quill", title: "Overview", product: Product::Quill, section: "Start here" },
    DocRef { path: "/docs/quill/installation", title: "Installation", product: Product::Quill, section: "Get started" },
    DocRef { path: "/docs/quill/quickstart", title: "Quickstart", product: Product::Quill, section: "Get started" },
    DocRef { path: "/docs/quill/project-structure", title: "Project structure", product: Product::Quill, section: "Get started" },
    DocRef { path: "/docs/quill/view-macro", title: "The view macro", product: Product::Quill, section: "Authoring" },
    DocRef { path: "/docs/quill/components", title: "Components", product: Product::Quill, section: "Authoring" },
    DocRef { path: "/docs/quill/theming", title: "Theming", product: Product::Quill, section: "Authoring" },
    DocRef { path: "/docs/quill/islands", title: "Islands", product: Product::Quill, section: "Interactivity" },
    DocRef { path: "/docs/quill/building-an-island", title: "Building an island", product: Product::Quill, section: "Interactivity" },
    DocRef { path: "/docs/quill/static-export", title: "Static export & deploy", product: Product::Quill, section: "Ship" },
    // ---- stdlib ----------------------------------------------------------
    DocRef { path: "/docs/stdlib", title: "Overview", product: Product::Stdlib, section: "Start here" },
    DocRef { path: "/docs/stdlib/substrate", title: "The substrate: qexec + qvalue", product: Product::Stdlib, section: "Substrate" },
    DocRef { path: "/docs/stdlib/utilities", title: "Utility crates", product: Product::Stdlib, section: "Crates" },
    DocRef { path: "/docs/stdlib/dependency-rule", title: "The dependency rule", product: Product::Stdlib, section: "Rules" },
    // ---- Cloud -----------------------------------------------------------
    DocRef { path: "/docs/cloud", title: "Overview", product: Product::Cloud, section: "Start here" },
    DocRef { path: "/docs/cloud/plans-and-resources", title: "Plans & resources", product: Product::Cloud, section: "Using Cloud" },
    DocRef { path: "/docs/cloud/scaling", title: "Scaling", product: Product::Cloud, section: "Using Cloud" },
    DocRef { path: "/docs/cloud/architecture", title: "Architecture", product: Product::Cloud, section: "How it works" },
];

/// Look up a page's `DocRef` by path (so a route can derive its own product).
pub fn doc_for(path: &str) -> Option<&'static DocRef> {
    DOCS.iter().find(|d| d.path == path)
}

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

/// The product switcher pinned to the top of the sidebar: one pill per product,
/// the active product marked. Lets a reader jump between product doc sets.
fn product_switcher(active: Product) -> Node {
    let mut row = el("div").class("q-docs__switch").attr("role", "tablist").attr("aria-label", "Documentation product");
    for p in Product::ALL {
        let mut link = el("a")
            .class("q-docs__switch-pill")
            .attr("href", p.landing())
            .child(text(p.name().to_string()));
        if p == active {
            link = link.attr("aria-current", "page");
        }
        row = row.child(link);
    }
    row
}

/// The LEFT SIDEBAR, SCOPED to `product`: a product switcher, then that
/// product's sections (first-appearance order) → page links, with the current
/// page marked `aria-current="page"`.
fn sidebar(product: Product, current: &str) -> Node {
    let mut nav = el("nav").class("q-docs__side").attr("aria-label", "Documentation");
    nav = nav.child(product_switcher(product));

    // Sections in first-appearance order, restricted to this product.
    let mut seen: Vec<&'static str> = Vec::new();
    for d in DOCS.iter().filter(|d| d.product == product) {
        if !seen.contains(&d.section) {
            seen.push(d.section);
        }
    }
    for section in seen {
        let mut sec = el("div").class("q-docs__sec");
        sec = sec.child(el("p").class("q-docs__sec-label").child(text(section.to_string())));
        let mut list = el("div").class("q-docs__nav");
        for d in DOCS.iter().filter(|d| d.product == product && d.section == section) {
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

/// Prev/next links derived from the `DOCS` order, SCOPED to `product` (the walk
/// never crosses a product boundary).
fn prev_next(product: Product, current: &str) -> Node {
    let pages: Vec<&DocRef> = DOCS.iter().filter(|d| d.product == product).collect();
    let idx = pages.iter().position(|d| d.path == current);
    let prev = idx.and_then(|i| i.checked_sub(1)).and_then(|i| pages.get(i));
    let next = idx.and_then(|i| pages.get(i + 1));

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

/// Assemble the full docs `<main>` for a page that belongs to `product`:
/// product-scoped sidebar + (h1 + content + product-scoped pager) + TOC.
///
/// `current` is the page path; it MUST exist in [`DOCS`] under `product`.
/// `content` is the page body the route built (interleaving `toc.h2(..)` and
/// prose); `toc` is the same accumulator, rendered into the right rail.
pub fn layout(product: Product, current: &str, title: &str, lead: &str, content: Node, toc: Toc) -> Node {
    let main = el("article")
        .class("q-docs__main")
        .child(el("p").class("q-docs__crumb").child(text(format!("{} docs", product.name()))))
        .child(el("h1").child(text(title.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())))
        .child(content)
        .child(prev_next(product, current));

    el("main")
        .class("q-docs")
        .id("main")
        .child(sidebar(product, current))
        .child(main)
        .child(toc.render())
}

/// The docs-pager + per-product chrome CSS (kept here next to the markup it
/// styles): the prev/next pager, the product switcher pills, and the breadcrumb.
pub fn pager_css() -> &'static str {
    "\
.q-docs__crumb{font-size:.8rem;text-transform:uppercase;letter-spacing:var(--q-tracking-wide,.06em);color:var(--q-color-brand);font-weight:var(--q-font-weight-bold);margin:0 0 .6rem}\
.q-docs__switch{display:flex;flex-wrap:wrap;gap:.3rem;margin:0 0 1.5rem;padding:0 0 1.25rem;border-bottom:1px solid var(--q-color-border)}\
.q-docs__switch-pill{font-size:.82rem;font-weight:var(--q-font-weight-medium);color:var(--q-color-muted);padding:.3rem .6rem;border-radius:var(--q-radius-md);border:1px solid transparent;transition:color var(--q-duration-fast) var(--q-ease-out),background-color var(--q-duration-fast) var(--q-ease-out),border-color var(--q-duration-fast) var(--q-ease-out)}\
.q-docs__switch-pill:hover{color:var(--q-color-fg);background:var(--q-color-surface);text-decoration:none}\
.q-docs__switch-pill[aria-current=\"page\"]{color:var(--q-color-on-brand);background:var(--q-color-brand);border-color:var(--q-color-brand)}\
.q-docs__pager{display:flex;justify-content:space-between;gap:1rem;margin:3rem 0 0;padding-top:1.5rem;border-top:1px solid var(--q-color-border)}\
.q-pager{display:flex;flex-direction:column;gap:.2rem;padding:.8rem 1rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);min-width:0;transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-pager:hover{border-color:var(--q-color-brand);text-decoration:none;transform:translateY(-2px)}\
.q-pager--next{text-align:right;margin-left:auto}\
.q-pager__dir{font-size:.78rem;color:var(--q-color-muted)}\
.q-pager__title{color:var(--q-color-fg);font-weight:var(--q-font-weight-medium)}"
}
