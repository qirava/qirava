//! `GET /docs` — the documentation hub — and `render_doc`, the single
//! data-driven renderer for every `/docs/<product>/<page>`.
//!
//! Each docs page is now *content data* (see [`crate::app::routes::docs_content`],
//! authored from real source) rendered through [`docs_kit::render_doc_body`] +
//! [`docs_kit::layout`], so there is one render path, not ~85 bespoke functions.

use qexec::FunctionResponse;
use qquill_view::{el, raw, text, Node};

use crate::app::docs_kit::{self, doc_for, Product, Toc};
use crate::app::routes::product_page::ARROW_SVG;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Developer docs — Qirava";
const DESCRIPTION: &str = "Qirava developer documentation: the DMS data system, the Quill UI \
framework, the q* stdlib, and Qirava Cloud — setup to usage, with real request/response examples.";

/// One product hub card linking to that product's docs landing.
fn hub_card(p: Product, summary: &str) -> Node {
    el("a")
        .class("q-dochub__card")
        .attr("href", p.landing())
        .child(el("h3").class("q-dochub__name").child(text(p.name().to_string())))
        .child(el("p").class("q-dochub__sum").child(text(summary.to_string())))
        .child(
            el("span")
                .class("q-prod-learn")
                .child(text("Read the docs "))
                .child(raw(ARROW_SVG)),
        )
}

fn hub_css() -> &'static str {
    "\
.q-dochub{max-width:72rem;margin:0 auto;padding:3rem 1.5rem 5rem}\
.q-dochub__head{max-width:48rem;margin:0 0 2.5rem}\
.q-dochub-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:1.25rem}\
@media (max-width:720px){.q-dochub-grid{grid-template-columns:1fr}}\
.q-dochub__card{display:flex;flex-direction:column;padding:1.5rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-dochub__card:hover{border-color:var(--q-color-brand);transform:translateY(-2px);text-decoration:none}\
.q-dochub__name{margin:0 0 .4rem;font-size:1.2rem;color:var(--q-color-fg)}\
.q-dochub__sum{margin:0 0 1rem;color:var(--q-color-muted);line-height:1.6}\
.q-dochub__card .q-prod-learn{margin-top:auto}"
}

/// `GET /docs` — the product chooser.
pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(hub_css().to_string());

    let head = el("div")
        .class("q-dochub__head")
        .child(el("p").class("q-eyebrow").child(text("Developer docs")))
        .child(el("h1").class("q-h1").child(text("Pick a product to start")))
        .child(el("p").class("q-lead").child(text(
            "Every product has a full manual — setup, configuration, tuning, and every feature, \
             each with real request/response examples. Choose where to begin.",
        )));

    let grid = el("div")
        .class("q-dochub-grid")
        .child(hub_card(Product::Dms, "The AI-native, zero-dependency data system: QQL (relational + search + graph + vector), the access model, the API, and Studio."))
        .child(hub_card(Product::Quill, "The Rust-native UI + app framework: the view layer, components, theming, islands, signals, and static export."))
        .child(hub_card(Product::Stdlib, "The 13 zero-dependency crates: the qexec executor + qvalue model and the utility packages, all from scratch."))
        .child(hub_card(Product::Cloud, "The managed-DMS control plane: the control-plane model, orchestration, scaling, billing, and the console."));

    let body = el("main").class("q-dochub").id("main").child(head).child(grid);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/docs" };
    page(&meta, css, body)
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
    let meta = Meta { title: &full_title, description: &desc, path };
    page(&meta, css, main)
}
