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

// ===========================================================================
// Data-driven docs content: a `Page` is a lead + ordered sections, each a
// heading + content `Block`s. One `render_doc_body` turns that data into the
// docs `<article>` body + the on-page TOC, so a page is *content data*, not a
// bespoke render function — which is what makes ~70 pages tractable and lets
// authoring agents contribute content (not Rust). Content lives in
// `routes::docs_content`; this module owns the model + the renderer + the
// shared content primitives (ascii / table / callout / defs / example).
// ===========================================================================

/// One content block on a docs page.
pub enum Block {
    /// A paragraph of prose.
    Prose(String),
    /// A fenced code sample (the language label is decorative).
    Code { lang: String, code: String },
    /// A worked example: the request/call, its response, and (optionally) the
    /// distinct rendered output.
    Example { request: String, response: String, output: String },
    /// An ASCII diagram with a caption (flows, architecture).
    Ascii { caption: String, diagram: String },
    /// A simple table.
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    /// A callout: `warn` picks the warning accent, else the note accent.
    Callout { warn: bool, label: String, body: String },
    /// A definition list (term → description).
    Defs(Vec<(String, String)>),
    /// An ordered or unordered list.
    List { ordered: bool, items: Vec<String> },
}

/// A section: an H2 heading (captured in the TOC) + its blocks.
pub struct Section {
    pub heading: String,
    pub blocks: Vec<Block>,
}

/// A full docs page's content: the lead paragraph + ordered sections.
pub struct Page {
    pub lead: String,
    pub sections: Vec<Section>,
}

/// Render one [`Block`] to a node.
pub fn render_block(b: &Block) -> Node {
    match b {
        Block::Prose(t) => p(t),
        Block::Code { code, .. } => el("pre")
            .class("q-code")
            .child(el("code").child(text(code.clone()))),
        Block::Example { request, response, output } => example(request, response, output),
        Block::Ascii { caption, diagram } => ascii(caption, diagram),
        Block::Table { headers, rows } => {
            let h: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
            let r: Vec<Vec<&str>> = rows.iter().map(|row| row.iter().map(|c| c.as_str()).collect()).collect();
            let rr: Vec<&[&str]> = r.iter().map(|row| row.as_slice()).collect();
            table(&h, &rr)
        }
        Block::Callout { warn, label, body } => {
            callout(if *warn { "warn" } else { "note" }, label, body)
        }
        Block::Defs(rows) => {
            let r: Vec<(&str, &str)> = rows.iter().map(|(t, d)| (t.as_str(), d.as_str())).collect();
            defs(&r)
        }
        Block::List { ordered, items } => {
            let tag = if *ordered { "ol" } else { "ul" };
            let mut list = el(tag).class("q-doc-list");
            for it in items {
                list = list.child(el("li").child(text(it.clone())));
            }
            list
        }
    }
}

/// Build the docs `<article>` body + the on-page TOC from a [`Page`]'s content.
/// Each section heading becomes an H2 in the TOC; blocks render in order.
pub fn render_doc_body(page: &Page) -> (Node, Toc) {
    let mut toc = Toc::new();
    let mut body = el("div").class("q-doc-body");
    for s in &page.sections {
        body = body.child(toc.h2(&s.heading));
        for b in &s.blocks {
            body = body.child(render_block(b));
        }
    }
    (body, toc)
}

// --- shared content primitives (used by render_block and by hub/landing pages) ---

/// A paragraph of body prose.
pub fn p(s: &str) -> Node {
    el("p").class("q-doc-p").child(text(s.to_string()))
}

/// A worked example: labelled Request / Response / Output panes (panes with
/// empty content are omitted).
pub fn example(request: &str, response: &str, output: &str) -> Node {
    let pane = |label: &str, body: &str| -> Option<Node> {
        if body.trim().is_empty() {
            return None;
        }
        Some(
            el("div")
                .class("q-eg__pane")
                .child(el("p").class("q-eg__label").child(text(label.to_string())))
                .child(el("pre").class("q-eg__pre").child(el("code").child(text(body.to_string())))),
        )
    };
    let mut fig = el("div").class("q-eg");
    for n in [pane("Request", request), pane("Response", response), pane("Output", output)].into_iter().flatten() {
        fig = fig.child(n);
    }
    fig
}

/// An ASCII diagram: a monospace `<pre>` that scrolls rather than wraps.
pub fn ascii(caption: &str, diagram: &str) -> Node {
    el("figure")
        .class("q-ascii-wrap")
        .child(el("pre").class("q-ascii").child(el("code").child(text(diagram.to_string()))))
        .child(el("figcaption").class("q-ascii-cap").child(text(caption.to_string())))
}

/// A simple themed table from `headers` + `rows` (escaped cells, scrolls on narrow).
pub fn table(headers: &[&str], rows: &[&[&str]]) -> Node {
    let mut head = el("tr");
    for h in headers {
        head = head.child(el("th").child(text((*h).to_string())));
    }
    let mut tbody = el("tbody");
    for row in rows {
        let mut tr = el("tr");
        for cell in *row {
            tr = tr.child(el("td").child(text((*cell).to_string())));
        }
        tbody = tbody.child(tr);
    }
    el("div").class("q-table-wrap").child(
        el("table").class("q-table").child(el("thead").child(head)).child(tbody),
    )
}

/// A keyed callout. `kind` is `note` | `warn`.
pub fn callout(kind: &str, label: &str, body: &str) -> Node {
    el("aside")
        .class(format!("q-callout q-callout--{kind}"))
        .child(el("span").class("q-callout__label").child(text(label.to_string())))
        .child(el("p").class("q-callout__body").child(text(body.to_string())))
}

/// A definition list (term → description).
pub fn defs(rows: &[(&str, &str)]) -> Node {
    let mut dl = el("dl").class("q-defs");
    for (term, desc) in rows {
        dl = dl
            .child(el("dt").child(text((*term).to_string())))
            .child(el("dd").child(text((*desc).to_string())));
    }
    dl
}

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

    /// Render the captured headings into the right-rail TOC nav. Public so the
    /// architecture section (`arch_kit`) can reuse the same TOC widget.
    pub fn render(&self) -> Node {
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

/// CSS for the data-driven content blocks (prose, example panes, ascii, tables,
/// callouts, defs, lists). Theme-token only, so it flips with light/dark and
/// restyles on any size/radius switch. Pushed once per docs page, deduped.
pub fn docs_extras_css() -> &'static str {
    "\
.q-doc-body{min-width:0}\
.q-doc-p{color:var(--q-color-fg);line-height:1.7;margin:0 0 1.1rem}\
.q-doc-body h2{scroll-margin-top:5rem}\
.q-doc-list{margin:0 0 1.1rem;padding-left:1.2rem;color:var(--q-color-fg);line-height:1.7}\
.q-doc-list li{margin:.3rem 0}\
/* ---- worked example (request / response / output) ---- */\
.q-eg{display:grid;gap:.75rem;margin:1.5rem 0}\
.q-eg__pane{border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);overflow:hidden;background:var(--q-color-surface)}\
.q-eg__label{margin:0;padding:.45rem .85rem;font-size:.7rem;text-transform:uppercase;letter-spacing:.07em;font-weight:var(--q-font-weight-bold);color:var(--q-color-brand);background:color-mix(in srgb,var(--q-color-brand) 7%,transparent);border-bottom:1px solid var(--q-color-border)}\
.q-eg__pre{margin:0;padding:.85rem;overflow-x:auto;background:var(--q-color-bg);font-family:var(--q-font-mono,monospace);font-size:.82rem;line-height:1.55;color:var(--q-color-fg)}\
.q-eg__pre code{font:inherit;white-space:pre}\
/* ---- ascii diagrams ---- */\
.q-ascii-wrap{margin:1.5rem 0;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);overflow:hidden}\
.q-ascii{margin:0;padding:1rem 1.1rem;overflow-x:auto;background:var(--q-color-bg);font-family:var(--q-font-mono,monospace);font-size:.74rem;line-height:1.5;color:var(--q-color-fg)}\
.q-ascii code{font:inherit;white-space:pre;display:block;min-width:max-content}\
.q-ascii-cap{padding:.6rem 1.1rem;font-size:.82rem;color:var(--q-color-muted);border-top:1px solid var(--q-color-border)}\
/* ---- tables ---- */\
.q-table-wrap{margin:1.5rem 0;overflow-x:auto;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg)}\
.q-table{border-collapse:collapse;width:100%;font-size:.9rem;min-width:34rem}\
.q-table th,.q-table td{text-align:left;padding:.6rem .85rem;border-bottom:1px solid var(--q-color-border);vertical-align:top}\
.q-table th{background:var(--q-color-surface);color:var(--q-color-fg);font-weight:var(--q-font-weight-bold);font-size:.78rem;text-transform:uppercase;letter-spacing:.04em}\
.q-table tbody tr:last-child td{border-bottom:0}\
.q-table td:first-child{font-weight:var(--q-font-weight-medium);color:var(--q-color-fg)}\
/* ---- callouts ---- */\
.q-callout{display:flex;flex-direction:column;gap:.3rem;margin:1.5rem 0;padding:1rem 1.15rem;border:1px solid var(--q-color-border);border-left:3px solid var(--q-color-brand);border-radius:var(--q-radius-md);background:var(--q-color-surface)}\
.q-callout--warn{border-left-color:color-mix(in srgb,var(--q-color-fg) 45%,var(--q-color-brand))}\
.q-callout__label{font-size:.72rem;text-transform:uppercase;letter-spacing:.08em;font-weight:var(--q-font-weight-bold);color:var(--q-color-brand)}\
.q-callout--warn .q-callout__label{color:var(--q-color-fg)}\
.q-callout__body{margin:0;color:var(--q-color-muted);line-height:1.6;font-size:.92rem}\
/* ---- definition lists ---- */\
.q-defs{display:grid;grid-template-columns:auto 1fr;gap:.5rem 1.1rem;margin:1.25rem 0;align-items:baseline}\
@media (max-width:560px){.q-defs{grid-template-columns:1fr;gap:.15rem 0}}\
.q-defs dt{font-weight:var(--q-font-weight-bold);color:var(--q-color-fg);font-size:.92rem}\
.q-defs dd{margin:0;color:var(--q-color-muted);line-height:1.6;font-size:.92rem}"
}
