//! The **Architecture section** — a docs-like reading experience that is the
//! single source of truth for Qirava's system design.
//!
//! It mirrors [`docs_kit`](crate::app::docs_kit), but instead of per-product
//! developer docs it carries the cross-cutting design: the access model, the
//! security/governance trust roots, the managed-cloud control plane, the
//! cluster/replication mechanism, and the embedded/sync reach. Pages live at
//! `/architecture/<page>` and share the same `.q-docs*` grid (sidebar + article
//! + on-page TOC) as the docs, so the two sections feel identical.
//!
//! Adding a page is one row in [`ARCH`] (plus a `respond` in `routes/` and a row
//! in `PAGES`): name the section, give it a title + summary — it appears in the
//! sidebar, in order, with prev/next wired and a hub card on the overview.

use qquill_view::{el, text, Node};

use crate::app::docs_kit::Toc;

/// One page in the architecture section. Sidebar order = slice order; sections
/// group by first appearance; `summary` feeds the overview hub cards.
pub struct ArchRef {
    pub path: &'static str,
    pub title: &'static str,
    pub section: &'static str,
    pub summary: &'static str,
}

/// EVERY architecture page, in sidebar order. One table, one source of truth.
pub const ARCH: &[ArchRef] = &[
    ArchRef {
        path: "/architecture",
        title: "Overview",
        section: "Start here",
        summary: "The whole system on one screen: three access checkpoints, the \
                  module map, and a map of the deep dives below.",
    },
    ArchRef {
        path: "/architecture/security",
        title: "Security & governance",
        section: "Foundations",
        summary: "Custodian M-of-N, the master seed, recovery (cloud account vs \
                  DMS data), the three checkpoints, and confidential computing — \
                  policy-flexible, transparency-mandatory.",
    },
    ArchRef {
        path: "/architecture/cloud",
        title: "Cloud control plane",
        section: "Managed cloud",
        summary: "Public signup, a resource pool you spend across many isolated \
                  DMSes, dense per-node packing, email-scoped delegation, and the \
                  envelope-only control channel that never reads tenant data.",
    },
    ArchRef {
        path: "/architecture/cluster",
        title: "Scaling, migration & upgrades",
        section: "Managed cloud",
        summary: "One replication mechanism behind every operation — vertical and \
                  horizontal scale, live migration, standalone⇄cluster, and \
                  zero-downtime CI/CD rollouts, all via a FIFO-hold cutover.",
    },
    ArchRef {
        path: "/architecture/embed",
        title: "Embedded & sync",
        section: "Reach",
        summary: "The engine as an in-process library (Tauri/mobile) and \
                  bidirectional WebSocket sync for offline-first, backup, and \
                  restore — authenticated by API key.",
    },
];

/// Look up a page's `ArchRef` by path. Part of the section's public API (mirrors
/// `docs_kit::doc_for`); kept for routes that derive their own page metadata.
#[allow(dead_code)]
pub fn arch_for(path: &str) -> Option<&'static ArchRef> {
    ARCH.iter().find(|a| a.path == path)
}

/// The LEFT SIDEBAR for the architecture section: a section header, then the
/// sections (first-appearance order) → page links, current marked.
fn sidebar(current: &str) -> Node {
    let mut nav = el("nav").class("q-docs__side").attr("aria-label", "Architecture");
    nav = nav.child(
        el("div")
            .class("q-docs__switch")
            .child(el("span").class("q-arch-side__label").child(text("Architecture"))),
    );

    let mut seen: Vec<&'static str> = Vec::new();
    for a in ARCH {
        if !seen.contains(&a.section) {
            seen.push(a.section);
        }
    }
    for section in seen {
        let mut sec = el("div").class("q-docs__sec");
        sec = sec.child(el("p").class("q-docs__sec-label").child(text(section.to_string())));
        let mut list = el("div").class("q-docs__nav");
        for a in ARCH.iter().filter(|a| a.section == section) {
            let mut link = el("a").attr("href", a.path).child(text(a.title.to_string()));
            if a.path == current {
                link = link.attr("aria-current", "page");
            }
            list = list.child(link);
        }
        sec = sec.child(list);
        nav = nav.child(sec);
    }
    nav
}

/// Prev/next within the architecture section (reuses the docs pager styling).
fn prev_next(current: &str) -> Node {
    let idx = ARCH.iter().position(|a| a.path == current);
    let prev = idx.and_then(|i| i.checked_sub(1)).and_then(|i| ARCH.get(i));
    let next = idx.and_then(|i| ARCH.get(i + 1));

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

/// Assemble the architecture `<main>`: scoped sidebar + (crumb + h1 + lead +
/// content + pager) + on-page TOC. `current` MUST exist in [`ARCH`].
pub fn layout(current: &str, title: &str, lead: &str, content: Node, toc: Toc) -> Node {
    let main = el("article")
        .class("q-docs__main")
        .child(el("p").class("q-docs__crumb").child(text("Architecture")))
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

// ---------------------------------------------------------------------------
// Content building blocks (used by every architecture page)
// ---------------------------------------------------------------------------

/// An ASCII diagram: a monospace `<pre>` that scrolls horizontally on small
/// screens rather than wrapping. The text is escaped (built via `text`), so
/// `<`, `>`, and `&` in diagrams are safe. `caption` labels it for a11y + print.
pub fn ascii(caption: &str, diagram: &str) -> Node {
    el("figure")
        .class("q-ascii-wrap")
        .child(el("pre").class("q-ascii").child(el("code").child(text(diagram.to_string()))))
        .child(el("figcaption").class("q-ascii-cap").child(text(caption.to_string())))
}

/// A simple themed table from `headers` + `rows` (each row a slice of cells).
/// Cells are plain text (escaped). Scrolls horizontally on narrow screens.
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
        el("table")
            .class("q-table")
            .child(el("thead").child(head))
            .child(tbody),
    )
}

/// A keyed callout note (e.g. "Status", "Why it matters"). `body` is inline text.
pub fn callout(kind: &str, label: &str, body: &str) -> Node {
    el("aside")
        .class(format!("q-callout q-callout--{kind}"))
        .child(el("span").class("q-callout__label").child(text(label.to_string())))
        .child(el("p").class("q-callout__body").child(text(body.to_string())))
}

/// A definition row: a bold term and its description, for compact key/value lists.
pub fn defs(rows: &[(&str, &str)]) -> Node {
    let mut dl = el("dl").class("q-defs");
    for (term, desc) in rows {
        dl = dl
            .child(el("dt").child(text((*term).to_string())))
            .child(el("dd").child(text((*desc).to_string())));
    }
    dl
}

/// A paragraph of body prose.
pub fn p(s: &str) -> Node {
    el("p").class("q-arch-p").child(text(s.to_string()))
}

// ---------------------------------------------------------------------------
// CSS — ASCII blocks, tables, callouts, defs, sidebar label. Theme-token only,
// so it flips with light/dark and restyles on any size/radius switch.
// ---------------------------------------------------------------------------

/// The architecture-section CSS. Pushed once per page, deduped by the Css set.
pub fn arch_css() -> &'static str {
    "\
.q-arch-side__label{font-size:.78rem;text-transform:uppercase;letter-spacing:.1em;color:var(--q-color-brand);font-weight:var(--q-font-weight-bold)}\
.q-arch-p{color:var(--q-color-fg);line-height:1.7;margin:0 0 1.1rem}\
.q-docs__main h2{scroll-margin-top:5rem}\
/* ---- ASCII diagrams ---- */\
.q-ascii-wrap{margin:1.5rem 0;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);overflow:hidden}\
.q-ascii{margin:0;padding:1rem 1.1rem;overflow-x:auto;background:var(--q-color-bg);font-family:var(--q-font-mono,monospace);font-size:.74rem;line-height:1.5;color:var(--q-color-fg);-webkit-overflow-scrolling:touch}\
.q-ascii code{font:inherit;white-space:pre;display:block;min-width:max-content}\
.q-ascii-cap{padding:.6rem 1.25rem;font-size:.82rem;color:var(--q-color-muted);border-top:1px solid var(--q-color-border)}\
/* ---- tables ---- */\
.q-table-wrap{margin:1.5rem 0;overflow-x:auto;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg)}\
.q-table{border-collapse:collapse;width:100%;font-size:.9rem;min-width:34rem}\
.q-table th,.q-table td{text-align:left;padding:.6rem .85rem;border-bottom:1px solid var(--q-color-border);vertical-align:top}\
.q-table th{background:var(--q-color-surface);color:var(--q-color-fg);font-weight:var(--q-font-weight-bold);font-size:.78rem;text-transform:uppercase;letter-spacing:.04em}\
.q-table tbody tr:last-child td{border-bottom:0}\
.q-table tbody tr:hover{background:color-mix(in srgb,var(--q-color-brand) 5%,transparent)}\
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
.q-defs dd{margin:0;color:var(--q-color-muted);line-height:1.6;font-size:.92rem}\
@media (max-width:560px){.q-defs dd{margin:0 0 .5rem}}"
}
