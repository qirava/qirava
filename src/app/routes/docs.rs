//! `GET /docs` — the documentation hub — and `render_doc`, the single
//! data-driven renderer for every `/docs/<product>/<page>`.
//!
//! Each docs page is now *content data* (see [`crate::app::routes::docs_content`],
//! authored from real source) rendered through [`docs_kit::render_doc_body`] +
//! [`docs_kit::layout`], so there is one render path, not ~85 bespoke functions.

use qexec::FunctionResponse;
use qquill_view::{el, Node};

use crate::app::docs_kit::{self, doc_for, Product, Toc};
use crate::app::shell::page;
use crate::app::site_ui;
use crate::app::{Css, Meta};

const TITLE: &str = "Developer docs — Qirava";
const DESCRIPTION: &str = "Qirava developer documentation: the DMS data system, the Quill UI \
framework, the q* stdlib, and Qirava Cloud — setup to usage, with real request/response examples.";

fn body() -> Node {
    let head = site_ui::page_head(
        "Developer docs",
        "Learn Qirava in the order humans need",
        "Start from the product path, then dive into the per-product manuals. The docs explain what to run, how access works, how to build UI, and what is already complete.",
    );

    let start = el("section")
        .class("q-section q-section--tight")
        .child(site_ui::section_head(
            "Start path",
            "From zero to a safe app",
            "If you are new, do not begin with an API catalog. Follow this path first.",
            false,
        ))
        .child(
            el("ol")
                .class("q-path")
                .child(site_ui::path_step("01", "Run the DMS", "Start the server, capture bootstrap credentials, and confirm QQL routes on the local worker.", "/docs/dms/quick-start", "Open quick start"))
                .child(site_ui::path_step("02", "Learn the access model", "Understand L1 before-auth, L2 execute scope, and L3 planner RBAC before mutating data.", "/docs/dms/access-model-overview", "Read access model"))
                .child(site_ui::path_step("03", "Build with Quill", "Use the component catalog, theme tokens, islands, and static export without hardcoded UI drift.", "/docs/quill/components", "Browse components"))
                .child(site_ui::path_step("04", "Verify roadmap state", "Check built, partial, and planned capability status before relying on a feature.", "/roadmap", "Open roadmap")),
        );

    let products = el("section")
        .class("q-section")
        .child(site_ui::section_head(
            "Manuals",
            "Pick the product you are working with",
            "Each manual is product-specific. The website stays the SSOT; route files render authored content through the shared docs kit.",
            false,
        ))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--2")
                .child(site_ui::link_card("Qirava DMS", "qdms", "Install, run, query, secure, and operate the data system: QQL, RBAC, workers, API catalog, Studio, jobs, and replication.", "/docs/dms", "Read DMS docs"))
                .child(site_ui::link_card("Quill", "qquill", "Build UI in Rust: views, styled/headless components, theme controls, scoped demos, islands, static export, and CLI.", "/docs/quill", "Read Quill docs"))
                .child(site_ui::link_card("The q* stdlib", "qpkgs", "Understand qexec, qvalue, and the zero-dependency utility crates that products can depend on without pulling product code.", "/docs/stdlib", "Read stdlib docs"))
                .child(site_ui::link_card("Qirava Cloud", "qcloud", "Learn the managed-control-plane design and what is planned versus what exists in the open-source primitives today.", "/docs/cloud", "Read cloud docs")),
        );

    let explain = el("section")
        .class("q-section")
        .child(site_ui::section_head(
            "Docs promise",
            "No hidden second source of truth",
            "Architecture, product docs, and roadmap/status live here. Operator runbooks can live elsewhere, but product truth should not fork into parallel files.",
            false,
        ))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--3")
                .child(site_ui::feature_card("Architecture", "Design SSOT", "Security and performance rules are explained on the site so reviewers can evaluate changes against one published model."))
                .child(site_ui::feature_card("Components", "Theme-reactive demos", "Quill docs keep density, radius, and scoped surface demos together so components prove they follow tokens."))
                .child(site_ui::feature_card("Roadmap", "Built / partial / planned", "Status boards say what ships today, what has a working seam, and what remains planned — no vague future marketing.")),
        );

    site_ui::page_frame("q-doc-hub")
        .child(head)
        .child(start)
        .child(products)
        .child(explain)
}

/// `GET /docs` — the product chooser and learning path.
pub fn respond(_input: &[u8]) -> FunctionResponse {
    let css = Css::new();
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/docs",
    };
    page(&meta, css, body())
}

/// Render any `/docs/<product>/<page>` from its authored content data. The product
/// (for the scoped sidebar) comes from the DOCS table; the body + TOC come from
/// the content data. Falls back to a placeholder if a page has no content yet.
pub fn render_doc(path: &str) -> FunctionResponse {
    let dref = doc_for(path);
    let (product, title): (Product, &str) = match dref {
        Some(d) => (d.product, d.title),
        None => (Product::Dms, "Documentation"),
    };

    let mut css = Css::new();
    css.push(docs_kit::docs_extras_css().to_string());
    css.push(docs_kit::pager_css().to_string());

    // Hand-authored (verified) content wins; otherwise the generated content.
    let page_opt = crate::app::routes::docs_authored::content(path)
        .or_else(|| crate::app::routes::docs_content::content(path));
    let (body, toc, lead) = match page_opt {
        Some(p) => {
            let (b, t) = docs_kit::render_doc_body(&p);
            (b, t, p.lead)
        }
        None => {
            let t = Toc::new();
            let b = el("div").child(docs_kit::p("This page is being written."));
            (b, t, String::new())
        }
    };

    let main = docs_kit::layout(product, path, title, &lead, body, toc);

    let full_title = format!("{title} — Qirava docs");
    let mut desc: String = lead.chars().take(155).collect();
    if desc.is_empty() {
        desc = format!("{} documentation — {title}.", product.name());
    }
    let meta = Meta {
        title: &full_title,
        description: &desc,
        path,
    };
    page(&meta, css, main)
}
