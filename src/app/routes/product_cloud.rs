//! `GET /products/cloud` — the Qirava Cloud product page. **PLANNED.**
//!
//! Cloud in one read: a MANAGED DMS service. A customer subscribes, picks
//! resources (storage: dynamic auto-scale or fixed; mode: standalone or cluster,
//! switchable either direction non-destructively because the on-disk layout is
//! identical), and gets their OWN ISOLATED DMS — own custodian governance, own
//! dbs, own Studio. The Cloud is the control plane only (subscriptions, billing,
//! server allocation, scaling); it never touches tenant data. Vertical scaling
//! signals the running DMS to use new resources live; horizontal admits/rebalances
//! tenants and grows clusters. Content is accurate to `docs/CLOUD_MULTITENANT.md`
//! and is clearly marked PLANNED.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{
    closing, feature_section, hero, product_css, status_section, Cta, Feature, HeroStat, GITHUB_URL,
};
use crate::app::routes::{reveal, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava Cloud (Planned) — a managed DMS service";
const DESCRIPTION: &str = "Qirava Cloud (PLANNED) is a managed DMS service: subscribe, pick your \
resources and mode, and get your own isolated DMS — own custodian governance, dbs, and Studio. \
The Cloud is the control plane only; it never touches tenant data.";

/// The PLANNED banner that opens the page, so the status is unmissable.
fn planned_banner() -> Node {
    reveal(
        "cloud-banner",
        el("div")
            .class("q-cloud-banner")
            .attr("data-q-reveal", "")
            .attr("role", "note")
            .child(el("span").class("q-cloud-banner__tag").child(text("Planned")))
            .child(el("p").class("q-cloud-banner__text").child(text(
                "Qirava Cloud is the designed-but-not-yet-built managed offering. The single-tenant \
                 DMS primitives it will orchestrate — resource governance, RBAC, config-as-data, \
                 replication, the worker/function model — are already shipping in the open-core \
                 engine. This page describes the target architecture.",
            ))),
    )
}

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(cloud_css().to_string());

    let hero = hero(
        css,
        "Qirava Cloud",
        "qcloud · planned",
        "A managed DMS service — ",
        "your own isolated DMS",
        ".",
        "Subscribe, pick your resources and mode, and get a fully isolated Qirava DMS: your own \
         custodian governance, your own databases, your own Studio UI. The Cloud is the control \
         plane only — subscriptions, billing, server allocation, and scaling. It never reaches \
         into your data. The open-core managed layer; the engine stays Apache-2.0.",
        &[
            Cta { label: "Read the cloud design", href: "/architecture", solid: true },
            Cta { label: "View on GitHub", href: GITHUB_URL, solid: false },
        ],
        &[
            HeroStat { value: "1", label: "isolated DMS per tenant" },
            HeroStat { value: "2-way", label: "standalone ↔ cluster" },
            HeroStat { value: "0", label: "tenant data the Cloud sees" },
            HeroStat { value: "Planned", label: "current status" },
        ],
    );

    // What the customer chooses.
    let choose = feature_section(
        "cloud-choose",
        "gradient",
        "The subscription",
        "Pick your resources — and change your mind, non-destructively",
        "There are no fixed tiers. You choose storage and DMS mode, and either choice can change \
         at any time. The on-disk storage layout is identical across modes, so switching is \
         non-destructive in both directions.",
        &[
            Feature {
                kicker: "storage",
                title: "Dynamic or fixed",
                body: "Choose dynamic auto-scale (grows with usage, billed by what you store) or a \
                       fixed size. Dynamic storage grows without operator action; overflow is a \
                       billing event, never a hard block.",
            },
            Feature {
                kicker: "standalone ↔ cluster",
                title: "Switchable either direction",
                body: "Run standalone or as a cluster, and switch either way at any time. Because \
                       the storage layout is identical across modes, going cluster (add followers) \
                       or back to standalone (drop them) is non-destructive.",
            },
            Feature {
                kicker: "pay per unit",
                title: "Per-resource pricing",
                body: "Price is per CPU thread, GB of memory, GB of storage, and GB of bandwidth — \
                       shown live. The slider is the plan; there are no predefined fixed plans.",
            },
        ],
    );

    // Isolation + governance.
    let isolation = feature_section(
        "cloud-isolation",
        "glass",
        "Isolation",
        "Your own DMS, your own custodian, your own Studio",
        "Each tenant is one isolated DMS — its own process, WAL, _sys_* catalogs, seed, and \
         governance. It is the same custodian governance as any DMS, just one isolated instance \
         per tenant.",
        &[
            Feature {
                kicker: "one DMS per tenant",
                title: "Hard isolation",
                body: "A tenant is a separate DMS instance with its own data, WAL, governance \
                       hierarchy, and resource cap. A tenant never sees the control plane or any \
                       sibling tenant.",
            },
            Feature {
                kicker: "custodian > admin > user > guest",
                title: "The same governance",
                body: "Your DMS uses the exact custodian governance model the open-core engine \
                       ships — you are root of trust inside your own tenant, and only there. You \
                       sign into your own Studio UI.",
            },
            Feature {
                kicker: "two authority domains",
                title: "Control plane ≠ tenant authority",
                body: "The Cloud's operators govern plans, nodes, caps, billing, and placement — \
                       and never your data or governance. The two authority domains never cross.",
            },
        ],
    );

    // Control plane scope.
    let control = feature_section(
        "cloud-control",
        "flat",
        "The control plane",
        "It allocates resources — it never touches your data",
        "The Cloud is itself a Qirava DMS running a cloud app (the same way Studio is the system \
         app). It orchestrates lifecycle and billing; it adds no privileged side-door into a \
         tenant.",
        &[
            Feature {
                kicker: "subscriptions · billing",
                title: "Metering + dynamic billing",
                body: "Per-tenant usage counters (thread-seconds, GB-hours, GB stored, GB \
                       bandwidth) stream to the billing engine; the slider sets the cap and the \
                       meter sets the invoice.",
            },
            Feature {
                kicker: "allocation · placement",
                title: "Server allocation",
                body: "The control plane provisions a tenant, picks a node with headroom, and \
                       mirrors the cap into the tenant's executor budget and OS-level resource \
                       caps.",
            },
            Feature {
                kicker: "control plane only",
                title: "Never inside tenant data",
                body: "A DMS that manages other DMSes: the Cloud allocates resources and manages \
                       lifecycle, and that is all. It does not read or mutate a tenant's data — the \
                       same L1→L2→L3 gate the core enforces applies.",
            },
        ],
    );

    // Scaling, vertical + horizontal.
    let scaling = feature_section(
        "cloud-scaling",
        "flat",
        "Scaling",
        "Grow live, vertically and horizontally",
        "Vertical scaling grows one tenant's cap and signals the running DMS to use the new \
         resources without a rebuild or downtime; horizontal scaling admits and rebalances \
         tenants and grows clusters.",
        &[
            Feature {
                kicker: "vertical",
                title: "Bigger, live",
                body: "Grow a tenant's CPU/memory/storage cap, then signal the running DMS to start \
                       using the newly allocated resources live — it re-reads its cap and expands \
                       its executor budget and storage with no rebuild and no downtime.",
            },
            Feature {
                kicker: "horizontal",
                title: "More tenants, bigger clusters",
                body: "Admit more tenants, rebalance them across nodes as hardware is added, and \
                       grow a tenant's cluster (more nodes/replicas) as its data and load grow — \
                       reusing the replication promotion path.",
            },
            Feature {
                kicker: "isolation preserved",
                title: "Every move keeps the invariants",
                body: "Rebalancing moves a tenant by promoting/migrating via replication, draining \
                       the source — and every move preserves all per-tenant isolation invariants.",
            },
        ],
    );

    let status = status_section(
        "cloud-status",
        "Status",
        "Planned — and what it builds on",
        "The control plane, OS-level caps, metering, and billing are PLANNED. They orchestrate \
         single-tenant primitives that are already BUILT and tested in the open-core engine.",
        &[
            (Status::Built, "In-process resource budget",
             "qexec bounds memory, threads, and jobs per call — the cap the control plane will mirror."),
            (Status::Built, "Per-tenant DMS instance",
             "One DMS = one tenant today: own process, WAL, _sys_* catalogs, seed, and governance."),
            (Status::Built, "RBAC + governance + privileged-key gate",
             "The same custodian governance and L1→L2→L3 gate each isolated tenant will run."),
            (Status::Built, "Single-leader replication",
             "The promotion/migration path rebalance reuses; dual-master is itself still planned."),
            (Status::Planned, "Control plane (provision · place · scale · rebalance)",
             "The cloud app, _cp_* catalogs, and Cloud Console that orchestrate the fleet."),
            (Status::Planned, "OS-level caps + metering + billing",
             "cgroups/namespaces around the budget, per-tenant usage counters, and dynamic per-unit billing."),
        ],
    );

    let closing = closing(
        "cloud-closing",
        "Designed in the open",
        "Qirava Cloud is documented in the open repo so the contracts it relies on — resource \
         caps, RBAC, isolation seams — are correct in the Apache-2.0 core. Read the architecture \
         while it is being built.",
        Cta { label: "Read the architecture", href: "/architecture", solid: true },
        Cta { label: "Explore the DMS today", href: "/products/dms", solid: false },
    );

    // Build the main with the PLANNED banner inserted right after the hero.
    let mut main = el("main").class("q-main").id("main");
    main = main.child(hero);
    main = main.child(el("section").class("q-section").child(planned_banner()));
    for s in [choose, isolation, control, scaling, status, closing] {
        main = main.child(s);
    }
    main
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/products/cloud" };
    page(&meta, css, content)
}

/// Cloud-only CSS: the PLANNED banner. Token-driven; pushed once + deduped.
fn cloud_css() -> &'static str {
    "\
.q-cloud-banner{display:flex;align-items:flex-start;gap:var(--q-space-3);padding:var(--q-space-4) var(--q-space-5);border:1px solid color-mix(in srgb,var(--q-color-brand) 35%,var(--q-color-border));border-left:3px solid var(--q-color-brand);border-radius:var(--q-radius-lg);background:color-mix(in srgb,var(--q-color-brand) 7%,var(--q-color-surface))}\
.q-cloud-banner__tag{flex:0 0 auto;font-size:.68rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.25rem .6rem;border-radius:var(--q-radius-full);color:var(--q-color-on-brand);background:var(--q-color-brand);margin-top:.1rem}\
.q-cloud-banner__text{margin:0;font-size:.95rem;line-height:1.6;color:var(--q-color-fg)}"
}
