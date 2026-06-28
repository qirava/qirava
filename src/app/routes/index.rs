//! `GET /` — the clean Qirava landing page.
//!
//! The page is intentionally content-first: what Qirava is, which products exist,
//! how a human starts, and where the SSOT roadmap lives. Repeated UI comes from
//! `app::site_ui`; this route owns only content and order.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::{reveal, tilt};
use crate::app::shell::page;
use crate::app::site_ui;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava — zero-dependency data system and UI framework";
const DESCRIPTION: &str = "Qirava is an AI-native, zero-dependency data system with Quill, a Rust-native UI framework, plus first-party docs and roadmap SSOT.";

fn hero_panel() -> Node {
    el("aside")
        .class("q-hero-panel")
        .attr("aria-label", "How a Qirava request stays safe")
        .child(el("p").class("q-hero-panel__label").child(text("One safe request path")))
        .child(
            el("ol")
                .class("q-hero-flow")
                .child(
                    el("li")
                        .child(el("code").child(text("L1")))
                        .child(
                            el("div")
                                .child(el("strong").child(text("Worker before-auth")))
                                .child(el("span").child(text("Authenticate session or HMAC key and write identity into shared context."))),
                        ),
                )
                .child(
                    el("li")
                        .child(el("code").child(text("L2")))
                        .child(
                            el("div")
                                .child(el("strong").child(text("execute() scope")))
                                .child(el("span").child(text("Check public / all-apps / system-only / owner before the function runs."))),
                        ),
                )
                .child(
                    el("li")
                        .child(el("code").child(text("L3")))
                        .child(
                            el("div")
                                .child(el("strong").child(text("Planner RBAC")))
                                .child(el("span").child(text("The only door to read or mutate data: app-scope ∩ principal-grant."))),
                        ),
                ),
        )
}

fn hero() -> Node {
    el("section")
        .class("q-hero2")
        .child(
            el("div")
                .class("q-hero2__copy")
                .child(el("p").class("q-eyebrow").child(text("DMS + UI framework + docs SSOT")))
                .child(
                    el("h1")
                        .class("q-hero2__title")
                        .child(text("Build data products "))
                        .child(el("span").class("q-hero2__accent").child(text("without dependency sprawl"))),
                )
                .child(el("p").class("q-hero2__lead").child(text(
                    "Qirava is one first-party ecosystem: Qirava DMS stores and governs data, Quill renders the product UI in Rust, the q* stdlib keeps the substrate small, and this website is the source of truth for docs, architecture, and roadmap status.",
                )))
                .child(
                    el("div")
                        .class("q-cta-row")
                        .child(site_ui::action_link("Explore products", "/products", "primary"))
                        .child(site_ui::action_link("Start with docs", "/docs", "ghost"))
                        .child(site_ui::action_link("Check roadmap", "/roadmap", "plain")),
                ),
        )
        .child(hero_panel())
}

fn metrics() -> Node {
    el("section")
        .class("q-section q-section--tight")
        .attr("aria-label", "Project metrics")
        .child(
            el("div")
                .class("q-ui-grid q-ui-grid--4")
                .child(site_ui::metric(
                    "0",
                    "third-party dependencies in shipped JS and first-party crates",
                ))
                .child(site_ui::metric(
                    "3",
                    "ordered authorization checkpoints before data access",
                ))
                .child(site_ui::metric(
                    "4",
                    "product areas: DMS, Quill, q* stdlib, Cloud",
                ))
                .child(site_ui::metric(
                    "SSOT",
                    "architecture, docs, and roadmap live on this site",
                )),
        )
}

fn what_is_qirava() -> Node {
    let head = site_ui::section_head(
        "Plain English",
        "What is Qirava?",
        "A small, security-first stack for building data-backed products without assembling a database, auth layer, UI framework, and documentation system from unrelated packages.",
        false,
    );

    let grid = el("div")
        .class("q-ui-grid q-ui-grid--3")
        .child(site_ui::feature_card(
            "DMS",
            "Store and govern data",
            "The DMS is the product engine: QQL, tables, WAL, jobs, API catalog, Studio seams, and RBAC all behind one execute() path.",
        ))
        .child(site_ui::feature_card(
            "Quill",
            "Render the product UI",
            "Quill renders Rust-authored pages on the server and hydrates only the interactive components as small islands.",
        ))
        .child(site_ui::feature_card(
            "SSOT",
            "Understand what is real",
            "The qirava website explains products, docs, architecture, and built/partial/planned status in one place so readers do not guess.",
        ));

    el("section")
        .class("q-section")
        .child(reveal("home-what-head", head))
        .child(tilt("home-what-grid", reveal("home-what", grid)))
}

fn products() -> Node {
    let head = site_ui::section_head(
        "Products",
        "One ecosystem, clear boundaries",
        "Products are distinct repos with one-way dependencies: products may depend on q* packages; q* packages never depend on product code.",
        false,
    );

    let grid = el("div")
        .class("q-ui-grid q-ui-grid--2")
        .child(site_ui::product_card(
            "Qirava DMS",
            "qdms",
            "built",
            "The data system: one execute primitive and one function registry for governance, KMS, database, jobs, replication, and workers.",
            &["HTTP + WebSocket + native SSR on one port", "Every DB read/mutate reaches the planner", "Studio is a normal DMS client, not a bypass"],
            "/products/dms",
            "Understand the DMS",
        ))
        .child(site_ui::product_card(
            "Quill",
            "qquill",
            "built",
            "The UI/app framework: Rust view authoring, native SSR, islands, component docs, and static export with a hand-written runtime.",
            &["Theme/density/radius tokens", "Scoped component demos for glass/neu/gradient", "This site dogfoods the framework"],
            "/products/quill",
            "See Quill",
        ))
        .child(site_ui::product_card(
            "The q* stdlib",
            "qpkgs",
            "built",
            "Shared zero-dependency crates: qexec, qvalue, and focused utility crates used by products without pulling product code back in.",
            &["Executor and value substrate", "First-party utility crates", "One-way dependency rule"],
            "/products/stdlib",
            "Read stdlib role",
        ))
        .child(site_ui::product_card(
            "Qirava Cloud",
            "qcloud",
            "planned",
            "The managed-cloud control plane for DMS deployments. The open-source single-tenant primitives exist; managed operations are planned.",
            &["Provisioning, placement, metering, billing", "Built on the DMS", "No promised dates until shipping"],
            "/products/cloud",
            "Check cloud plan",
        ));

    el("section")
        .class("q-section")
        .child(reveal("home-products-head", head))
        .child(tilt("home-products-tilt", reveal("home-products", grid)))
}

fn path() -> Node {
    let head = site_ui::section_head(
        "How to use it",
        "A human path from first visit to shipped app",
        "The site now teaches the system in the order a person needs: pick the product, run the DMS, learn the access model, build the UI, then check status.",
        false,
    );

    let steps = el("ol")
        .class("q-path")
        .child(site_ui::path_step("01", "Pick the product", "DMS stores and governs data. Quill builds the UI. q* crates are the substrate. Cloud is planned managed operation.", "/products", "Compare products"))
        .child(site_ui::path_step("02", "Run the DMS", "Start the server, capture bootstrap credentials, and confirm the QQL routes before building anything on top.", "/docs/dms/quick-start", "DMS quick start"))
        .child(site_ui::path_step("03", "Learn safe data access", "Understand L1 worker auth, L2 execute scope, and L3 planner RBAC before writing app data.", "/docs/dms/access-model-overview", "Access model"))
        .child(site_ui::path_step("04", "Build the UI", "Use Quill components, server rendering, islands, and theme tokens. Surface effects stay in demos, not reading pages.", "/docs/quill/components", "Browse components"))
        .child(site_ui::path_step("05", "Check what is real", "Use the roadmap SSOT to separate completed, partial, and planned work before depending on a capability.", "/roadmap", "Open roadmap"));

    el("section")
        .class("q-section")
        .child(reveal("home-path-head", head))
        .child(reveal("home-path", steps))
}

fn status() -> Node {
    let head = site_ui::section_head(
        "Roadmap SSOT",
        "Built, partial, and planned are visible from the front page",
        "No reader should have to reverse-engineer repository state. Open the product roadmap for the full board; this summary keeps the landing page honest.",
        false,
    );

    let grid = el("div")
        .class("q-ui-grid q-ui-grid--2")
        .child(site_ui::status_card("Qirava DMS", "Engine, workers, execute registry, QQL/DDL, RBAC, governance, WAL, jobs, and Studio seams.", "Cluster hardening, standalone KMS packaging, and deeper managed operations.", "/docs/dms", "/roadmap/dms"))
        .child(site_ui::status_card("Quill", "Rust view authoring, SSR, islands, component library, theme tokens, static export, and CLI scaffold.", "More components, route discovery, dev server/watch flow, and richer animation presets.", "/docs/quill", "/roadmap/quill"))
        .child(site_ui::status_card("q* stdlib", "Zero-dependency substrate crates used by products: qexec, qvalue, and utility crates.", "Additional crypto primitives behind the provider trait and broader stdlib coverage.", "/docs/stdlib", "/roadmap/stdlib"))
        .child(site_ui::status_card("Qirava Cloud", "Open-source single-tenant primitives and the managed-control-plane shape.", "Provisioning, placement, billing/metering, scale modes, audit trails, and managed operations.", "/docs/cloud", "/roadmap/cloud"));

    el("section")
        .class("q-section")
        .child(reveal("home-status-head", head))
        .child(tilt("home-status-tilt", reveal("home-status", grid)))
}

fn closing() -> Node {
    site_ui::cta_band(
        "Start with the docs, not guesswork",
        "If you want to build, follow the docs path. If you want to evaluate, use the roadmap SSOT. If you want UI details, the Quill component docs show theme, density, radius, and scoped surfaces in one place.",
        ("Read developer docs", "/docs"),
        ("View roadmap", "/roadmap"),
    )
}

fn body() -> Node {
    site_ui::page_frame("q-home")
        .child(hero())
        .child(metrics())
        .child(what_is_qirava())
        .child(products())
        .child(path())
        .child(status())
        .child(closing())
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let css = Css::new();
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/",
    };
    page(&meta, css, body())
}
