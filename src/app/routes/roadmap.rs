//! `GET /roadmap` — roadmap SSOT hub.
//!
//! This page is the human-readable index for product state. It does not promise
//! dates; it separates what is built, partial, and planned per product and links
//! to the detailed product boards.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::inline_code;
use crate::app::shell::page;
use crate::app::site_ui;
use crate::app::{Css, Meta};

const TITLE: &str = "Roadmap — Qirava";
const DESCRIPTION: &str = "The Qirava roadmap SSOT: honest built, partial, and planned status for the DMS, Quill, the q* stdlib, and Qirava Cloud.";

fn legend() -> Node {
    el("div")
        .class("q-ui-card q-road-legend")
        .child(el("h2").class("q-h2").child(text("How to read status")))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--3")
                .child(site_ui::feature_card("Built", "Shipping today", "Present in the codebase and usable now. It can still improve, but the capability exists."))
                .child(site_ui::feature_card("Partial", "Working seam", "A real implementation path exists, with known gaps or hardening still in progress."))
                .child(site_ui::feature_card("Planned", "Designed, not shipped", "Important to the product direction, but not something users should depend on today.")),
        )
}

fn body() -> Node {
    let head = site_ui::page_head(
        "Roadmap SSOT",
        "What is complete, what is partial, what is planned",
        "The qirava website is the source of truth for product status. This page gives the product-level summary; open a product board for the detailed lane-by-lane status.",
    );

    let board = el("section")
        .class("q-section q-section--tight")
        .child(site_ui::section_head(
            "Product boards",
            "Status per product",
            "Capability state is more useful than aspirational dates. Each product has an honest board with built, partial, and planned lanes.",
            false,
        ))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--2")
                .child(site_ui::roadmap_card(
                    "Qirava DMS",
                    "qdms",
                    "Engine, workers, function registry, QQL/DDL, RBAC + governance, config-as-data, WAL, and the self-describing API are shipping; replication and cluster/KMS hardening continue.",
                    &[("Engine + workers", "built"), ("Replication", "partial"), ("KMS · cluster", "planned")],
                    "/roadmap/dms",
                ))
                .child(site_ui::roadmap_card(
                    "Quill",
                    "qquill",
                    "The view layer, native SSR, islands, per-page bundling, static export, component library, theming, and CLI are shipping; dev ergonomics and component breadth continue.",
                    &[("SSR · islands · SSG", "built"), ("Auto-routes", "partial"), ("quill dev", "planned")],
                    "/roadmap/quill",
                ))
                .child(site_ui::roadmap_card(
                    "The q* stdlib",
                    "qpkgs",
                    "The zero-dependency substrate crates are shipping. Planned work adds crypto primitives and broader standard library coverage behind first-party contracts.",
                    &[("13 crates", "built"), ("More crypto", "planned")],
                    "/roadmap/stdlib",
                ))
                .child(site_ui::roadmap_card(
                    "Qirava Cloud",
                    "qcloud",
                    "Control plane v1 is built: _cp_* catalogs, cloud.* functions, and the RBAC-gated Cloud Console. The physical infra effect is simulated today; live tenant spawning and billing are planned.",
                    &[("Control plane", "built"), ("Infra effect", "partial"), ("Live cloud", "planned")],
                    "/roadmap/cloud",
                )),
        );

    let audit = el("section")
        .class("q-section")
        .child(site_ui::section_head(
            "Code audit snapshot",
            "What the repo proves today",
            "This tracker is based on local product code, not aspirational copy. Unknowns stay out until product info is supplied.",
            false,
        ))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--2")
                .child(site_ui::status_card(
                    "Qirava DMS",
                    "Engine/QQL, WAL, worker funnel, auth/session/HMAC, Studio, API/OpenAPI, jobs, and single-leader replication seams are present in qdms.",
                    "Standalone KMS, dual-sync clustering, authenticated replication hardening, and confidential-compute seed ceremony remain planned/in progress.",
                    "/docs/dms",
                    "/roadmap/dms",
                ))
                .child(site_ui::status_card(
                    "Quill",
                    "view/style/theme/ui/design/runtime/docs/icons/cli/build/signal crates are present; Motion::Press/Lift/Tilt3d and the motion runtime are now implemented.",
                    "quill dev, route auto-discovery hardening, and broader component catalog remain planned/in progress.",
                    "/docs/quill",
                    "/roadmap/quill",
                ))
                .child(site_ui::status_card(
                    "q* stdlib",
                    "Thirteen q* crates are present: qexec, qvalue, qarray, qobject, qstring, qmath, qnumber, qconvert, qencoding, qcrypto, qregex, qtime, quuid.",
                    "Additional crypto/provider work and wider stdlib coverage remain planned.",
                    "/docs/stdlib",
                    "/roadmap/stdlib",
                ))
                .child(site_ui::status_card(
                    "Qirava Cloud",
                    "qcloud boots a control DMS, creates _cp_* catalogs, registers cloud.* functions, and serves an RBAC-gated Cloud Console.",
                    "Real DMS spawning, cgroup caps, live scaling, public signup, payment, and domain automation are simulated/planned.",
                    "/docs/cloud",
                    "/roadmap/cloud",
                )),
        );

    let ssot = el("section")
        .class("q-section")
        .child(site_ui::section_head(
            "Why this page exists",
            "No parallel roadmap, no marketing fog",
            "The roadmap is part of the website SSOT and is backed by the code audit: qdms, qquill, qpkgs, qcloud, qbrand, and qirava. Status changes should update this site instead of creating disconnected docs.",
            false,
        ))
        .child(
            el("p")
                .class("q-muted")
                .children([
                    text("Status legend: "),
                    inline_code("BUILT"),
                    text(" means usable now, "),
                    inline_code("PARTIAL"),
                    text(" means a working seam with known gaps, and "),
                    inline_code("PLANNED"),
                    text(" means designed but not yet built. No dates are promised here."),
                ]),
        );

    site_ui::page_frame("q-roadmap-hub")
        .child(head)
        .child(legend())
        .child(board)
        .child(audit)
        .child(ssot)
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let css = Css::new();
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/roadmap",
    };
    page(&meta, css, body())
}
