//! `GET /roadmap` — the roadmap INDEX hub.
//!
//! A chooser, mirroring the `/docs` hub: one card per product linking to that
//! product's roadmap (`/roadmap/{dms,quill,stdlib,cloud}`). The per-product
//! roadmaps carry the honest BUILT / PARTIAL / PLANNED boards. Pure SSR, zero JS.

use qexec::FunctionResponse;
use qquill_view::{el, raw, text, Node};

use crate::app::routes::product_page::ARROW_SVG;
use crate::app::routes::{inline_code, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Roadmap — Qirava";
const DESCRIPTION: &str = "The Qirava roadmap, organized per product: an honest BUILT / PARTIAL / \
PLANNED status board for the DMS, Quill, the q* stdlib, and the planned managed cloud. No dates \
promised — only state.";

/// One product hub card: name, a summary, a status snapshot, and a "View" link
/// to that product's roadmap. `snapshot` is the (built, partial, planned) tags
/// shown as chips.
fn hub_card(
    name: &str,
    crate_name: &str,
    href: &str,
    summary: &str,
    snapshot: &[(Status, &str)],
) -> Node {
    let title = el("div")
        .class("q-rm-hub-card__title")
        .child(el("span").child(text(name.to_string())))
        .child(el("code").class("q-rm-hub-card__crate").child(text(crate_name.to_string())));

    let mut chips = el("div").class("q-rm-hub-card__chips");
    for (status, label) in snapshot {
        let chip_mod = match status {
            Status::Built => "is-built",
            Status::Partial => "is-partial",
            Status::Planned => "is-planned",
        };
        chips = chips.child(
            el("span")
                .class(format!("q-rm-hub-card__chip {chip_mod}"))
                .child(text((*label).to_string())),
        );
    }

    let learn = el("a")
        .class("q-prod-learn")
        .attr("href", href.to_string())
        .child(text(format!("View the {name} roadmap ")))
        .child(raw(ARROW_SVG));

    el("article")
        .class("q-rm-hub-card")
        .child(title)
        .child(el("p").class("q-rm-hub-card__sum").child(text(summary.to_string())))
        .child(chips)
        .child(learn)
}

fn hub_css() -> &'static str {
    "\
.q-rm-hub{max-width:72rem;margin:0 auto;padding:3rem 1.5rem 5rem}\
.q-rm-hub__head{max-width:48rem;margin:0 0 2.5rem}\
.q-rm-hub-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:1.25rem}\
@media (max-width:720px){.q-rm-hub-grid{grid-template-columns:1fr}}\
.q-rm-hub-card{display:flex;flex-direction:column;padding:1.5rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-rm-hub-card:hover{border-color:var(--q-color-brand);transform:translateY(-2px)}\
.q-rm-hub-card__title{display:flex;align-items:baseline;gap:.6rem;font-size:1.2rem;font-weight:var(--q-font-weight-bold);margin:0 0 .4rem}\
.q-rm-hub-card__crate{font-family:var(--q-font-mono);font-size:.78rem;color:var(--q-color-muted);font-weight:var(--q-font-weight-normal)}\
.q-rm-hub-card__sum{color:var(--q-color-muted);margin:0 0 1rem;line-height:1.6}\
.q-rm-hub-card__chips{display:flex;flex-wrap:wrap;gap:.4rem;margin:0 0 .25rem}\
.q-rm-hub-card__chip{font-size:.66rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.2rem .5rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted)}\
.q-rm-hub-card__chip.is-built{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 45%,transparent);background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
.q-rm-hub-card__chip.is-partial{color:var(--q-color-fg);border-color:color-mix(in srgb,var(--q-color-fg) 30%,transparent);background:color-mix(in srgb,var(--q-color-fg) 7%,transparent)}\
.q-prod-learn{display:inline-flex;align-items:center;gap:.35rem;margin-top:auto;padding-top:1rem;font-weight:var(--q-font-weight-medium);font-size:.92rem;color:var(--q-color-brand)}\
.q-prod-learn:hover{text-decoration:none}\
.q-prod-learn .q-arr{transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-prod-learn:hover .q-arr{transform:translateX(3px)}\
.q-rm-hub__legend{margin:2.5rem 0 0;color:var(--q-color-muted);font-size:.92rem}"
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(hub_css().to_string());

    let head = el("div")
        .class("q-rm-hub__head")
        .child(el("p").class("q-eyebrow").child(text("Roadmap")))
        .child(el("h1").class("q-h1").child(text("Pick a product to see what's next")))
        .child(el("p").class("q-lead").child(text(
            "Each product has its own roadmap — an honest BUILT / PARTIAL / PLANNED board sourced \
             from the repository and the architecture docs. We track three states and promise no \
             dates: only where each capability actually stands.",
        )));

    let grid = el("div")
        .class("q-rm-hub-grid")
        .child(hub_card(
            "Qirava DMS",
            "qdms",
            "/roadmap/dms",
            "The engine, workers, function registry, QQL/DDL, RBAC + governance, config-as-data, \
             WAL, and the self-describing API are shipping. Single-leader replication is in \
             progress; standalone KMS and dual-sync clustering are planned.",
            &[(Status::Built, "Engine + workers + RBAC"), (Status::Partial, "Replication"), (Status::Planned, "KMS · cluster")],
        ))
        .child(hub_card(
            "Quill",
            "qquill",
            "/roadmap/quill",
            "The view layer, native SSR, islands, per-page bundling, static export, the component \
             library, theming, and the quill CLI are shipping. Folder-route auto-discovery, a quill \
             dev server, and more components are next.",
            &[(Status::Built, "SSR · islands · SSG · CLI"), (Status::Partial, "Auto-routes"), (Status::Planned, "quill dev")],
        ))
        .child(hub_card(
            "The q* stdlib",
            "qpkgs",
            "/roadmap/stdlib",
            "The 13 zero-dependency crates — qexec and qvalue plus the focused utilities — are \
             shipping. The planned work is the cryptographic primitives the security model needs, \
             behind the Crypto provider trait.",
            &[(Status::Built, "13 crates"), (Status::Planned, "More crypto")],
        ))
        .child(hub_card(
            "Qirava Cloud",
            "qcloud",
            "/roadmap/cloud",
            "The managed, multi-tenant control plane — a DMS that manages other DMSes. The whole \
             control plane (provisioning, metering, billing, OS caps, scaling) is planned; the \
             single-tenant primitives it orchestrates are already built.",
            &[(Status::Built, "OSS primitives"), (Status::Planned, "Control plane")],
        ));

    let legend = el("p").class("q-rm-hub__legend").children([
        text("Status legend: ".to_string()),
        inline_code("BUILT"),
        text(" is shipping today, ".to_string()),
        inline_code("PARTIAL"),
        text(" has a working seam with deferred parts, and ".to_string()),
        inline_code("PLANNED"),
        text(" is designed but not yet built. Open a product roadmap for the full board.".to_string()),
    ]);

    let body = el("main")
        .class("q-rm-hub")
        .id("main")
        .child(head)
        .child(grid)
        .child(legend);

    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/roadmap" };
    page(&meta, css, body)
}
