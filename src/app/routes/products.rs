//! `GET /products` — product overview.
//!
//! This route is content/data only. Cards, grids, chips, buttons, and spacing are
//! centralized in `app::site_ui` so the product hub follows the same design
//! system as home, docs, and roadmap.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::inline_code;
use crate::app::shell::page;
use crate::app::site_ui;
use crate::app::{Css, Meta};

const TITLE: &str = "Products — Qirava";
const DESCRIPTION: &str = "The Qirava products: Qirava DMS, Quill, the q* stdlib, and the planned Qirava Cloud control plane.";

fn body() -> Node {
    let head = site_ui::page_head(
        "Products",
        "Four product areas, one clean dependency direction",
        "Qirava is not one giant bucket. The DMS, Quill, q* stdlib, website, brand, and cloud control plane are distinct products/repos with clear boundaries.",
    );

    let overview = el("section")
        .class("q-section q-section--tight")
        .child(site_ui::section_head(
            "Product map",
            "What each repo is for",
            "Use this as the quick product comparison before choosing docs or roadmap pages.",
            false,
        ))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--2")
                .child(site_ui::product_card(
                    "Qirava DMS",
                    "qdms",
                    "built",
                    "The AI-native data system. Governance, KMS, database, jobs, and replication are functions in one registry, reached only through execute() via a worker.",
                    &[
                        "HTTP, WebSocket, and native SSR/SSG/ISR on one port",
                        "Three checkpoints: L1 before-auth, L2 execute scope, L3 planner RBAC",
                        "Self-describing API and Studio admin seams",
                    ],
                    "/products/dms",
                    "Product page",
                ))
                .child(site_ui::product_card(
                    "Quill",
                    "qquill",
                    "built",
                    "The Rust-native UI/app framework used by this site: server-rendered pages, island hydration, component docs, theme tokens, and static export.",
                    &[
                        "No server-side JavaScript toolchain",
                        "Headless state + styled token-driven components",
                        "Theme, density, and radius apply through shared --q-* tokens",
                    ],
                    "/products/quill",
                    "Product page",
                ))
                .child(site_ui::product_card(
                    "The q* stdlib",
                    "qpkgs",
                    "built",
                    "Shared zero-dependency crates used by products: qexec, qvalue, and focused utility crates. It is not a dumping ground for product code.",
                    &[
                        "Products depend on q*; q* never depends on products",
                        "Executor + value model substrate",
                        "First-party utilities only",
                    ],
                    "/products/stdlib",
                    "Product page",
                ))
                .child(site_ui::product_card(
                    "Qirava Cloud",
                    "qcloud",
                    "planned",
                    "The managed-cloud control plane for DMS deployments. The product direction is designed; live managed infrastructure remains planned.",
                    &[
                        "Provisioning, placement, metering, billing, and scaling",
                        "Control plane v1 shape, live infra planned",
                        "Built on the open-source DMS primitives",
                    ],
                    "/products/cloud",
                    "Product page",
                )),
        );

    let boundaries = el("section")
        .class("q-section")
        .child(site_ui::section_head(
            "Boundaries",
            "Packages are not products",
            "The clean design system is not only visual: the product architecture has to stay legible too.",
            false,
        ))
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--3")
                .child(site_ui::feature_card("qpkgs", "Shared packages only", "qpkgs holds q* stdlib crates usable by any product. It is not where DMS, Quill, or Cloud product code goes."))
                .child(site_ui::feature_card("qquill", "Quill is a product", "Quill is its own UI/app framework product, not a shared-functions folder moved into qpkgs."))
                .child(site_ui::feature_card("qirava", "Website is SSOT", "Products, docs, architecture, and roadmap status are explained here so humans have one canonical place to read.")),
        );

    let note = el("section")
        .class("q-section")
        .child(el("p").class("q-muted").children([
            text("Status legend: "),
            inline_code("BUILT"),
            text(" is shipping today, "),
            inline_code("PARTIAL"),
            text(" has a working seam with deferred parts, and "),
            inline_code("PLANNED"),
            text(" is designed but not yet built. See the "),
            el("a").attr("href", "/roadmap").child(text("roadmap")),
            text(" for the full matrix."),
        ]));

    site_ui::page_frame("q-products-hub")
        .child(head)
        .child(overview)
        .child(boundaries)
        .child(note)
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let css = Css::new();
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/products",
    };
    page(&meta, css, body())
}
