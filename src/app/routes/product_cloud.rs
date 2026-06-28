//! `GET /products/cloud` — the Qirava Cloud product page.
//!
//! Cloud in one read: a managed-DMS control plane — a Qirava DMS that manages
//! OTHER DMSes. An operator provisions a tenant, the placer bin-packs it onto a
//! node, and the tenant gets its own isolated DMS (own custodian governance, own
//! databases, own Studio). The control plane is itself a DMS running a `cloud`
//! app: it persists the whole fleet through `_cp_*` catalogs and `cloud.*`
//! orchestration functions, behind a deny-by-default, RBAC-gated Console. It is
//! honest about its seam: the DATA MODEL is real and durable (provision, place,
//! scale, switch, suspend, terminate, invoice all persist); the INFRA EFFECT each
//! one would have (spawning a tenant DMS, applying cgroup caps, live-scaling,
//! taking payment) is SIMULATED and clearly badged. Content is accurate to the
//! `qcloud` submodule and `docs/CLOUD_MULTITENANT.md`.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{
    arch_anim, arch_anim_css, closing, feature_section, hero, main_wrap, product_css,
    status_section, ArchNode, Cta, Feature, HeroStat, GITHUB_URL,
};
use crate::app::routes::{reveal, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava Cloud — the managed-DMS control plane";
const DESCRIPTION: &str = "Qirava Cloud is a managed-DMS control plane: a DMS that manages other \
DMSes. Provision a tenant, the placer bin-packs it onto a node, and it gets its own isolated DMS — \
own custodian governance, databases, and Studio. The control plane never touches tenant data.";

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(arch_anim_css().to_string());
    css.push(cloud_css().to_string());

    let hero = hero(
        css,
        "Qirava Cloud",
        "qcloud",
        "A DMS that manages ",
        "other DMSes",
        ".",
        "Qirava Cloud is a managed-DMS control plane. An operator provisions a tenant, the placer \
         bin-packs it onto a node, and that tenant gets its own fully isolated DMS — its own \
         custodian governance, databases, and Studio. The control plane is itself a Qirava DMS \
         running a cloud app: it persists the whole fleet and orchestrates lifecycle, and it never \
         reads or mutates tenant data. The open-core managed layer; the engine stays Apache-2.0.",
        &[
            Cta {
                label: "Read the cloud docs",
                href: "/docs/cloud",
                solid: true,
            },
            Cta {
                label: "View on GitHub",
                href: GITHUB_URL,
                solid: false,
            },
        ],
        &[
            HeroStat {
                value: "1",
                label: "isolated DMS per tenant",
            },
            HeroStat {
                value: "7",
                label: "_cp_* control catalogs",
            },
            HeroStat {
                value: "2-way",
                label: "standalone ↔ cluster",
            },
            HeroStat {
                value: "0",
                label: "tenant data the Cloud sees",
            },
        ],
    );

    // What it is — the control-plane-vs-tenant separation, stated plainly.
    let what = feature_section(
        "cloud-what",
        "gradient",
        "What it is",
        "A control plane that allocates resources — never your data",
        "The Cloud is a Qirava DMS running a cloud app, the same way Studio is the system app. It \
         orchestrates the fleet — provisioning, placement, scaling, billing — and that is all. The \
         tenant DMS it stands up is a separate instance with its own data, WAL, governance, and \
         Studio; the two authority domains never cross.",
        &[
            Feature {
                kicker: "a DMS of DMSes",
                title: "The plane is just a DMS",
                body: "The control plane runs as a normal Qirava DMS with a cloud app on top. No \
                       new runtime, no special server — the same execute() funnel, the same auth \
                       checkpoints. It manages the fleet from inside its own catalogs.",
            },
            Feature {
                kicker: "one DMS per tenant",
                title: "Hard tenant isolation",
                body: "A provisioned tenant is its own DMS instance: own data, WAL, _sys_* \
                       catalogs, seed, custodian governance, and Studio UI. A tenant never sees \
                       the control plane or any sibling — the cap is the only thing shared.",
            },
            Feature {
                kicker: "control plane only",
                title: "It never touches tenant data",
                body: "The Cloud allocates resources and manages lifecycle; it has no privileged \
                       side-door into a tenant. The same L1→L2→L3 gate the core enforces keeps the \
                       operator out of tenant tables — by design, not by policy.",
            },
        ],
    );

    // Features grid — the operator-facing capabilities.
    let features = feature_section(
        "cloud-features",
        "glass",
        "Features",
        "Everything an operator needs to run the fleet",
        "The Console exposes the full lifecycle of a tenant as a set of governed actions. Each one \
         persists durably to the control catalogs; each one writes an audit row; each one is gated \
         by the operator's cloud role, deny-by-default.",
        &[
            Feature {
                kicker: "provision · place",
                title: "Provision + bin-pack placement",
                body: "Create a tenant and the placer runs first-fit bin-packing over the node \
                       fleet, picking the first node with cap headroom and bumping its allocation \
                       — or flagging the operator when the fleet is full.",
            },
            Feature {
                kicker: "vertical · horizontal",
                title: "Scale up and scale out",
                body: "Grow one tenant's CPU/memory/storage cap, or grow its cluster by adding \
                       replicas on distinct nodes. The new cap and replica count persist to the \
                       tenant row and the node allocations adjust.",
            },
            Feature {
                kicker: "standalone ↔ cluster",
                title: "Switch mode, non-destructively",
                body: "Flip a tenant between standalone and cluster either direction. Because the \
                       on-disk storage layout is identical across modes, the switch is \
                       non-destructive — it just adds or drops followers.",
            },
            Feature {
                kicker: "suspend · resume · terminate",
                title: "Full lifecycle control",
                body: "Suspend a tenant on non-payment, resume it in place, or terminate it — \
                       releasing its node allocation. Each transition is one durable status change \
                       on the tenant row.",
            },
            Feature {
                kicker: "metering · invoices",
                title: "Per-unit billing",
                body: "Usage counters (thread-seconds, GB-hours, GB stored, GB bandwidth) feed a \
                       per-unit plan — priced per thread-hour, GB-hour, GB-month, and GB transfer \
                       — and generate durable invoices. No fixed tiers.",
            },
            Feature {
                kicker: "audit · RBAC console",
                title: "Audited, role-gated Console",
                body: "Every mutation writes a _cp_audit row (operator, action, tenant, node, \
                       result). The Console renders only the screens the signed-in cloud role can \
                       reach, and each action re-checks the role server-side.",
            },
        ],
    );

    // How it works — concrete and plain.
    let how = feature_section(
        "cloud-how",
        "flat",
        "How it works",
        "Persisted in catalogs, run through one funnel, gated by role",
        "There is no magic and no hidden service. The control plane keeps its entire state in a \
         handful of catalogs, mutates it only through one set of functions, and gates those \
         functions on the operator's cloud role.",
        &[
            Feature {
                kicker: "two authority domains",
                title: "Control plane ≠ tenant",
                body: "Cloud governance (the operators of the fleet) is a completely separate \
                       authority from any tenant's custodian. A cloud admin governs plans, nodes, \
                       caps, and billing — and never a tenant's data or governance.",
            },
            Feature {
                kicker: "_cp_* catalogs",
                title: "The whole fleet is rows",
                body: "_cp_tenants, _cp_nodes, _cp_plans, _cp_subscriptions, _cp_usage, \
                       _cp_invoices, and _cp_audit hold the entire control-plane state. There is \
                       no out-of-band config — placement and billing read these tables.",
            },
            Feature {
                kicker: "cloud.* funnel",
                title: "One orchestration funnel",
                body: "Every change runs through a cloud.* function — provision, scale_vertical, \
                       scale_horizontal, switch_mode, suspend, resume, terminate, generate_invoice \
                       — reached via execute(). One write path, durable through the WAL.",
            },
            Feature {
                kicker: "deny-by-default",
                title: "Role gates, re-checked server-side",
                body: "The Console only shows what your cloud role permits, but the gate that \
                       matters is in the function: each cloud.* re-checks ctx.role and refuses if \
                       it is below the required rank. The UI is a convenience, not the boundary.",
            },
            Feature {
                kicker: "first-fit",
                title: "Bin-packing placement",
                body: "The placer walks nodes in a stable order and takes the first with enough \
                       thread/memory/storage headroom, then records the allocation delta. \
                       Deterministic, so placement is testable and repeatable.",
            },
            Feature {
                kicker: "real vs simulated",
                title: "Durable data, simulated infra",
                body: "Each cloud.* function persists the INTENDED new state and returns it; the \
                       physical INFRA EFFECT — spawning the tenant DMS, applying OS caps, taking \
                       payment — is simulated and badged. The data model is real today.",
            },
        ],
    );

    // Architecture animation — the control-plane-over-tenants diagram, honest badges.
    let arch = arch_anim(
        "cloud-arch",
        "Architecture",
        "The control plane over the tenant fleet",
        "An operator action enters the Console, runs through the cloud.* funnel, persists to the \
         _cp_* catalogs, and the placer assigns a node — at which point a tenant DMS would be \
         stood up. The persistence is real (BUILT); the physical infra step is SIMULATED.",
        &[
            ArchNode {
                label: "Console",
                sub: "RBAC-gated, deny-by-default",
                badge: "BUILT",
            },
            ArchNode {
                label: "cloud.* funnel",
                sub: "execute() · audited",
                badge: "BUILT",
            },
            ArchNode {
                label: "_cp_* catalogs",
                sub: "durable · WAL-backed",
                badge: "BUILT",
            },
            ArchNode {
                label: "placer",
                sub: "first-fit bin-pack",
                badge: "BUILT",
            },
            ArchNode {
                label: "tenant DMS",
                sub: "spawn · cap · serve",
                badge: "SIMULATED",
            },
        ],
    );

    // Honest status — BUILT data model, SIMULATED infra, PLANNED real provisioning.
    let status = status_section(
        "cloud-status",
        "Status",
        "What's built, what's simulated, what's planned",
        "The control-plane DATA MODEL is built and tested — provision, place, scale, switch, \
         suspend, terminate, and invoice all persist durably through one funnel. The physical \
         infra each action implies is SIMULATED; real tenant provisioning is the planned next step.",
        &[
            (Status::Built, "Control plane + _cp_* catalogs",
             "A real Qirava DMS running the cloud app; seven _cp_* catalogs hold the whole fleet, created and seeded idempotently."),
            (Status::Built, "cloud.* orchestration funnel",
             "provision · scale_vertical · scale_horizontal · switch_mode · suspend · resume · terminate · generate_invoice — each persists and audits."),
            (Status::Built, "RBAC-gated Console + audit trail",
             "Deny-by-default Console: it renders only reachable screens and each action re-checks ctx.role; every mutation writes a _cp_audit row."),
            (Status::Built, "First-fit bin-packing placement",
             "Deterministic placement over _cp_nodes by cap headroom, with allocation deltas recorded on the chosen node."),
            (Status::Partial, "Infra effects (SIMULATED)",
             "Spawning the tenant DMS, applying cgroup/OS caps, live-scaling, and taking payment are simulated against the data model and clearly badged."),
            (Status::Planned, "Real tenant provisioning",
             "Booting an actual isolated tenant DMS/CVM, wiring OS-level caps around the executor budget, and live billing replace the simulated effects."),
        ],
    );

    let closing = closing(
        "cloud-closing",
        "Run the fleet from the docs",
        "Qirava Cloud is documented in the open repo — the control catalogs, the cloud.* funnel, \
         the placement model, and the real-vs-simulated seam. Read how the managed layer works and \
         where it is honest about what is not yet real.",
        Cta {
            label: "Read the cloud docs",
            href: "/docs/cloud",
            solid: true,
        },
        Cta {
            label: "Explore the DMS today",
            href: "/products/dms",
            solid: false,
        },
    );

    // The honest banner sits right after the hero so the seam is unmissable.
    let mut sections = vec![
        hero,
        el("section").class("q-section").child(honest_banner()),
    ];
    sections.extend([what, features, how, arch, status, closing]);
    main_wrap(sections)
}

/// A short banner directly under the hero making the real-vs-simulated seam
/// impossible to miss. Token-driven; the reveal island animates it in.
fn honest_banner() -> Node {
    reveal(
        "cloud-banner",
        el("div")
            .class("q-cloud-banner")
            .attr("data-q-reveal", "")
            .attr("role", "note")
            .child(
                el("span")
                    .class("q-cloud-banner__tag")
                    .child(text("Honest")),
            )
            .child(el("p").class("q-cloud-banner__text").child(text(
                "The control-plane data model is real and durable today: provisioning, placement, \
                 scaling, lifecycle, and billing all persist through one audited funnel. The \
                 physical infra each action implies — spawning a tenant DMS, applying OS caps, \
                 taking payment — is SIMULATED and clearly badged. This page says exactly which is \
                 which.",
            ))),
    )
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/products/cloud",
    };
    page(&meta, css, content)
}

/// Cloud-only CSS: the honest banner. Token-driven; pushed once + deduped.
fn cloud_css() -> &'static str {
    "\
.q-cloud-banner{display:flex;align-items:flex-start;gap:var(--q-space-3);padding:var(--q-space-4) var(--q-space-5);border:1px solid color-mix(in srgb,var(--q-color-brand) 35%,var(--q-surface-border,var(--q-color-border)));border-left:3px solid var(--q-color-brand);border-radius:var(--q-radius-lg);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none)}\
.q-cloud-banner__tag{flex:0 0 auto;font-size:.68rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.25rem .6rem;border-radius:var(--q-radius-full);color:var(--q-color-on-brand);background:var(--q-color-brand);margin-top:.1rem}\
.q-cloud-banner__text{margin:0;font-size:.95rem;line-height:1.6;color:var(--q-color-fg)}"
}
