//! `GET /roadmap/cloud` — the Qirava Cloud roadmap.
//!
//! Control plane v1 is BUILT: the `_cp_*` catalogs, the `cloud.*` functions (with
//! the infra effect simulated + badged), and the RBAC-gated Cloud Console. The
//! live infra — the node agent, real DMS spawning + cgroup caps, the FIFO-hold
//! cutover trio, public signup, delegation, domain automation, and metered
//! payment — is PLANNED. Sourced from the Architecture section (the SSOT). No
//! dates promised — only state.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{hero, main_wrap, product_css, Cta, HeroStat};
use crate::app::routes::roadmap_page::{board, legend, note, roadmap_css, Item, Lane};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava Cloud roadmap — built, in progress, planned";
const DESCRIPTION: &str = "An honest status board for Qirava Cloud, the managed multi-tenant \
control plane. Control plane v1 — the _cp_* catalogs, the cloud.* functions, and the RBAC-gated \
Cloud Console — is built with the infra effect simulated; the live infra, public signup, the \
FIFO-hold cutover, delegation, and metered payment are planned.";

// Built: control plane v1 + the OSS-core primitives it orchestrates.
const BUILT: &[Item] = &[
    Item { title: "Control plane v1 — the _cp_* catalogs", detail: "Tenants, nodes, plans, subscriptions, usage, invoices, and an audit log are modeled as data in the control plane's own DMS — the same config-as-data pattern as _sys_*." },
    Item { title: "cloud.* functions", detail: "provision, scale_vertical, scale_horizontal, switch_mode, suspend, resume, terminate, and generate_invoice make real _cp_* writes today; the infra effect is simulated and badged PLANNED in the UI." },
    Item { title: "The Cloud Console (a Quill app)", detail: "An RBAC-gated console — Overview, Tenants, Plans, Nodes, Billing, Governance (custodian > admin) — built as a Quill app, the way Studio is the DMS's admin app." },
    Item { title: "Resource governance (the executor)", detail: "The bounded executor caps memory and work per call — the per-DMS cap mechanism the control plane sets and meters." },
    Item { title: "Single-leader replication + change-stream", detail: "The replication seam every cutover reuses — committed op-frames stream master → follower (single-direction today)." },
    Item { title: "The cloud is itself a DMS", detail: "The control plane runs as a DMS hosting a cloud app, reusing the worker pipeline, execute(), and Quill — only the cloud.* functions are new, not a forked engine." },
];

// In progress: the seams that exist but whose real effect is not yet wired.
const PARTIAL: &[Item] = &[
    Item { title: "Live infra effect", detail: "cloud.* writes the catalogs, but the actual DMS spawn / scale / move is simulated and badged PLANNED — the node agent that performs it is the next build." },
    Item { title: "Confidential-VM attestation seam", detail: "A tenant DMS should attest from inside a SEV-SNP / TDX VM before receiving its seed; the setup runbook exists, the in-DMS attestation handshake is being designed." },
];

// Planned: the real managed-cloud layer.
const PLANNED: &[Item] = &[
    Item { title: "Node agent + DMS control socket", detail: "The cloud's per-node hands — spawn/stop, cgroup caps, routing — driving a narrow, envelope-only lifecycle socket on each DMS that never reads tenant data." },
    Item { title: "Real DMS spawning + dense packing", detail: "Spawn isolated, hard-capped DMS processes and bin-pack many per node, so one account can run many isolated DMSes and nodes are not wasted." },
    Item { title: "FIFO-hold cutover trio", detail: "Write-forwarding + epoch-fencing + location repoint — the primitive that makes vertical/horizontal scale, live migration, and upgrades silent (no restart)." },
    Item { title: "Public signup + self-serve", detail: "Open email signup, purchase a resource pool, and self-serve create DMS instances, databases, and worker apps." },
    Item { title: "Email-scoped delegation", detail: "Grant another email a scope to manage one of your DMSes — cross-account delegation bridged into the DMS's own RBAC." },
    Item { title: "Metering → billing → payment", detail: "Meter thread / RAM-GB / storage-GB per DMS and bill per unit on an hourly / monthly / yearly cycle, with real payment." },
    Item { title: "Domain automation", detail: "A default id.qirava.in subdomain per DMS plus custom domains via a cloudflared tunnel and CF Zero Trust." },
    Item { title: "Signed-release CI/CD + rolling updates", detail: "M-of-N-signed, transparency-logged releases rolled out node-by-node with proper drain and health-gated rollback." },
];

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(roadmap_css().to_string());

    let hero = hero(
        css,
        "Cloud roadmap",
        "qcloud",
        "The managed control plane, ",
        "honestly tracked",
        ".",
        "Qirava Cloud is the managed, multi-tenant control plane — a DMS that manages other DMSes, \
         billed per resource. Control plane v1 is built: the _cp_* catalogs, the cloud.* functions \
         (infra effect simulated), and the RBAC-gated Cloud Console. The live infra, public signup, \
         the FIFO-hold cutover, delegation, and metered payment are planned. No dates — only state.",
        &[
            Cta { label: "Read the architecture", href: "/docs/cloud/architecture", solid: true },
            Cta { label: "Explore Qirava Cloud", href: "/products/cloud", solid: false },
        ],
        &[
            HeroStat { value: "v1", label: "control plane built" },
            HeroStat { value: "6", label: "building blocks shipping" },
            HeroStat { value: "8", label: "infra items planned" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    let mut board_section = board(
        "Status board",
        "Built control plane, planned infra",
        "Three lanes, no dates. The BUILT lane is control plane v1 plus the OSS-core primitives it \
         reuses; PARTIAL is the seams whose real effect is not yet wired; PLANNED is the live \
         managed-cloud layer — the node agent, real spawning, the silent cutover, signup, \
         delegation, billing, and domains.",
        ["rm-cloud-built", "rm-cloud-partial", "rm-cloud-planned"],
        [
            Lane { status: Status::Built, items: BUILT },
            Lane { status: Status::Partial, items: PARTIAL },
            Lane { status: Status::Planned, items: PLANNED },
        ],
    );
    board_section = board_section.child(legend()).child(note(vec![
        text("Sourced from the ".to_string()),
        el("a").attr("href", "/docs/cloud/architecture").child(text("Cloud control plane")),
        text(" and ".to_string()),
        el("a").attr("href", "/docs/cloud/scaling-architecture").child(text("Scaling & upgrades")),
        text(" architecture pages — the single source of truth. The managed-cloud layer is the \
              commercial open-core offering atop the Apache-2.0 core."
            .to_string()),
    ]));

    main_wrap(vec![hero, board_section])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/roadmap/cloud" };
    page(&meta, css, content)
}
