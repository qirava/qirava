//! `GET /roadmap/cloud` — the Qirava Cloud roadmap.
//!
//! The managed-DMS control plane is PLANNED end to end: provisioning, placement,
//! metering, per-resource billing, OS-level caps + tenant sandboxing, autoscale/
//! rebalance, and the standalone↔cluster switch (per docs/CLOUD_MULTITENANT.md).
//! The single-tenant DMS primitives the control plane will orchestrate — resource
//! governance, RBAC, config-as-data, replication, the worker/function model — are
//! already BUILT in the OSS core. No dates promised — only state.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{hero, main_wrap, product_css, Cta, HeroStat};
use crate::app::routes::roadmap_page::{board, legend, note, roadmap_css, Item, Lane};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava Cloud roadmap — planned managed control plane";
const DESCRIPTION: &str = "An honest status board for Qirava Cloud, the managed multi-tenant \
control plane. The whole control plane — provisioning, metering, billing, OS caps, scaling, and \
the standalone↔cluster switch — is PLANNED; the single-tenant DMS primitives it orchestrates are \
already built in the open-source core.";

// The OSS-core primitives the control plane will orchestrate — these exist today
// in the DMS, which is why the cloud is "the missing top layer," not a rewrite.
const BUILT: &[Item] = &[
    Item { title: "Resource governance (the executor)", detail: "The bounded executor already caps memory and work per call — the per-tenant cap mechanism the control plane will set and meter." },
    Item { title: "RBAC + two authority domains", detail: "The custodian > admin > user > guest hierarchy and the L1→L2→L3 gate are the same boundary that keeps a tenant's domain separate from the control plane's." },
    Item { title: "Config-as-data (_sys_*)", detail: "Every tenant DMS is configured as data; the control plane's _cp_* catalogs follow the identical data-driven pattern." },
    Item { title: "Single-leader replication", detail: "The replication seam the control plane will flip a tenant from standalone to cluster mode through is built (single-direction today)." },
    Item { title: "The worker / function model", detail: "The cloud is itself a DMS running a cloud app — it reuses the worker pipeline, execute(), and Quill; only the cloud.* functions are new." },
];

const PARTIAL: &[Item] = &[
    Item { title: "Confidential-VM attestation seam", detail: "A tenant DMS must attest from inside a SEV-SNP / TDX VM before receiving its seed; the SEV-SNP setup runbook exists, the in-DMS attestation handshake is being designed." },
];

const PLANNED: &[Item] = &[
    Item { title: "Provisioning + placement", detail: "cloud.provision / place — spin up an isolated tenant DMS process (its own WAL, _sys_*, seed, custodian) and place it on a node." },
    Item { title: "Metering + per-resource billing", detail: "Meter CPU-thread, memory-GB, storage-GB, and bandwidth-GB per tenant and bill dynamically per unit — no fixed plans, a live pricing slider." },
    Item { title: "OS-level caps + tenant sandbox", detail: "cgroups / namespaces enforce each tenant's CPU/memory/storage/bandwidth cap; tenant-installed cargo modules execute inside that sandbox and cannot reach a sibling." },
    Item { title: "Autoscale + rebalance", detail: "Grow storage with usage, scale a tenant vertically, and admit/rebalance tenants across nodes as bare-metal is added — isolation invariants preserved across every move." },
    Item { title: "Standalone ↔ cluster switch", detail: "A tenant flips between a single node and a replicated cluster on demand, driven by the control plane through the replication seam." },
    Item { title: "The Cloud Console (a Quill app)", detail: "The control-plane UI — subscriptions, billing, node + tenant management — built as a Quill app, the way Studio is the DMS's admin app." },
    Item { title: "The qcloud submodule", detail: "Created when the first phase is built (no hollow skeleton in the public tree); the commercial open-core layer atop the Apache-2.0 engine." },
];

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(roadmap_css().to_string());

    let hero = hero(
        css,
        "Cloud roadmap — planned",
        "qcloud",
        "The managed control plane, ",
        "planned",
        ".",
        "Qirava Cloud is the managed, multi-tenant control plane — a DMS that manages other DMSes, \
         billed per resource. It is forward-looking by design: the whole control plane is PLANNED. \
         What is already built is the single-tenant DMS the control plane will orchestrate — \
         resource governance, RBAC, config-as-data, replication, and the worker model. No dates \
         promised — only state.",
        &[
            Cta { label: "Explore Qirava Cloud", href: "/products/cloud", solid: true },
            Cta { label: "Read the docs", href: "/docs/cloud", solid: false },
        ],
        &[
            HeroStat { value: "PLANNED", label: "control plane status" },
            HeroStat { value: "5", label: "OSS primitives ready" },
            HeroStat { value: "7", label: "control-plane items planned" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    let mut board_section = board(
        "Status board",
        "Built foundation, planned control plane",
        "Three lanes, no dates. The BUILT lane is the open-source DMS the control plane reuses; the \
         PLANNED lane is the managed-cloud layer itself — provisioning, metering, billing, OS caps, \
         scaling, and the standalone↔cluster switch. The control plane is the missing top layer, \
         not a forked engine.",
        ["rm-cloud-built", "rm-cloud-partial", "rm-cloud-planned"],
        [
            Lane { status: Status::Built, items: BUILT },
            Lane { status: Status::Partial, items: PARTIAL },
            Lane { status: Status::Planned, items: PLANNED },
        ],
    );
    board_section = board_section.child(legend()).child(note(vec![
        text("Sourced from the repo's cloud + multi-tenant design doc. The managed-cloud layer is the \
              commercial open-core offering atop the Apache-2.0 core; see "
            .to_string()),
        el("a").attr("href", "/products/cloud").child(text("the product page")),
        text(" for the open-core boundary.".to_string()),
    ]));

    main_wrap(vec![hero, board_section])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/roadmap/cloud" };
    page(&meta, css, content)
}
