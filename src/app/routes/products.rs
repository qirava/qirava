//! `GET /products` — the products overview. Pure SSR, zero JavaScript.

use qexec::FunctionResponse;
use qquill_design::{Card, Effect, Radius, Tone};
use qquill_view::{el, raw, text, Node};

use crate::app::routes::product_page::ARROW_SVG;
use crate::app::routes::{inline_code, status_badge, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Products — Qirava";
const DESCRIPTION: &str = "The Qirava products: the DMS (qdms), the Quill UI framework (qquill), \
the q* stdlib (qpkgs), and the planned managed cloud (qcloud).";

/// A product section: a card carrying name + crate + status, a description, and
/// a bullet list of the concrete capabilities.
#[allow(clippy::too_many_arguments)]
fn product(
    css: &mut Css,
    id: &str,
    name: &str,
    crate_name: &str,
    status: Status,
    summary: &str,
    points: &[&str],
    learn_more: &str,
) -> Node {
    let eyebrow = el("div")
        .class("q-card-eyebrow")
        .child(text(name.to_string()))
        .child(el("code").child(text(crate_name.to_string())));

    let header = el("div")
        .child(eyebrow)
        .child(el("div").class("q-card-actions").child(status_badge(css, status)));

    let mut list = el("ul").class("q-list");
    for p in points {
        list = list.child(el("li").child(text((*p).to_string())));
    }

    let learn = el("a")
        .class("q-prod-learn")
        .attr("href", learn_more.to_string())
        .child(text(format!("Learn more about {name} ")))
        .child(raw(ARROW_SVG));

    let body = el("div")
        .child(el("p").child(text(summary.to_string())))
        .child(list)
        .child(learn);

    let card = Card::new(id)
        .article()
        .header(header)
        .body(body)
        .tone(Tone::Neutral)
        .effect(Effect::Flat)
        .radius(Radius::Lg);
    css.node(card.render())
}

/// Index-only CSS: the per-card "Learn more →" link. Token-driven; deduped.
fn index_css() -> &'static str {
    "\
.q-prod-learn{display:inline-flex;align-items:center;gap:.35rem;margin-top:1rem;font-weight:var(--q-font-weight-medium);font-size:.92rem;color:var(--q-color-brand)}\
.q-prod-learn:hover{text-decoration:none}\
.q-prod-learn .q-arr{transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-prod-learn:hover .q-arr{transform:translateX(3px)}"
}

fn body(css: &mut Css) -> Node {
    css.push(index_css().to_string());
    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text("Products")))
        .child(el("h1").class("q-h1").child(text("One ecosystem, all first-party")))
        .child(el("p").class("q-lead").child(text(
            "Three shipping products plus one in design. Every product is \
             zero-dependency — std and sibling Qirava crates only — and shares the \
             same q* substrate.",
        )));

    let dms = product(
        css,
        "prod-dms",
        "Qirava DMS",
        "qdms",
        Status::Built,
        "An AI-native, zero-dependency data system built on one execute primitive and \
         one function registry. Governance, KMS, the database, jobs, and replication \
         are all functions behind a single bounded executor.",
        &[
            "Worker layer (before → handle → after) serves HTTP, WS, and native SSR/SSG/ISR on one port (127.0.0.1:7179).",
            "Self-describing API: GET /api/spec returns native JSON; /api/spec/openapi returns OpenAPI 3.1.",
            "Vector, graph, and search built in (vector indexing via LSH today).",
            "RBAC hierarchy custodian > admin > user > guest, with custodian-gated single-use invites.",
            "Qirava Studio — the default admin app — is itself a DMS client; config is data in _sys_* tables.",
        ],
        "/products/dms",
    );

    let quill = product(
        css,
        "prod-quill",
        "Quill",
        "qquill",
        Status::Built,
        "A Rust-native, zero-dependency UI/app framework: shadcn-like styled components \
         with Next.js-like authoring. Native SSR + islands + SSG/ISR, no server-side JS.",
        &[
            "A ~4 KB hand-written client runtime, injected only on pages that actually use islands — content pages ship zero JS.",
            "Styled components (Navbar, Card, Button, Badge, Stat, Table…) over headless state machines, theme-token driven.",
            "quill new myapp → cargo run to serve → cargo run -- build for a static export.",
            "This very site is a Quill app: it dogfoods the framework end to end.",
        ],
        "/products/quill",
    );

    let stdlib = product(
        css,
        "prod-stdlib",
        "The q* stdlib",
        "qpkgs",
        Status::Built,
        "13 zero-dependency crates shared across every product. The substrate is qexec \
         (the bounded executor) and qvalue (the value/ABI); the rest are focused \
         utility crates.",
        &[
            "Substrate: qexec (bounded executor) + qvalue (value model + ABI).",
            "Utilities: array, object, string, math, number, convert, crypto, encoding, regex, time, uuid.",
            "Products depend on q*; q* never depends on the products — the dependency arrow points one way.",
        ],
        "/products/stdlib",
    );

    let cloud = product(
        css,
        "prod-cloud",
        "Qirava Cloud",
        "qcloud",
        Status::Planned,
        "The managed, multi-tenant control plane — a DMS that manages other DMSes, billed \
         per resource. Designed, not yet built.",
        &[
            "Metering + billing, OS-level resource caps, and per-tenant sandboxing.",
            "The open-core managed-cloud offering atop the Apache-2.0 core.",
        ],
        "/products/cloud",
    );

    let grid = el("div")
        .class("q-grid")
        .child(dms)
        .child(quill)
        .child(stdlib)
        .child(cloud);

    let note = el("p").class("q-muted").children([
        text("Status legend: ".to_string()),
        inline_code("BUILT"),
        text(" is shipping today, ".to_string()),
        inline_code("PLANNED"),
        text(" is designed but not yet built. See the ".to_string()),
        el("a").attr("href", "/roadmap").child(text("roadmap")),
        text(" for the full matrix.".to_string()),
    ]);

    el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(el("div").class("q-section").child(grid))
        .child(el("div").class("q-section").child(note))
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/products",
    };
    page(&meta, css, content)
}
