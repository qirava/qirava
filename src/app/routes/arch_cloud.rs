//! `GET /architecture/cloud` — the managed-cloud control plane.
//!
//! Public signup, a resource pool spent across many isolated DMSes, dense
//! per-node packing, email-scoped cross-account delegation, the two access
//! layers, and the envelope-only control channel (node agent + control socket +
//! trust boundary) that lets the cloud manage a DMS without ever reading it.

use qexec::FunctionResponse;
use qquill_view::{el, Node};

use crate::app::arch_kit::{self, ascii, callout, defs, p, table};
use crate::app::docs_kit::Toc;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Cloud control plane — Qirava architecture";
const DESCRIPTION: &str =
    "Qirava Cloud is a managed-DMS service: public signup, a resource pool you spend across many \
     isolated DMSes, dense per-node packing, email-scoped delegation, and an envelope-only control \
     channel that manages each DMS without ever reading tenant data.";

const LEAD: &str = "Qirava Cloud sells managed DMS. One rule makes it secure: the cloud controls \
the envelope (availability — place, scale, move, upgrade, drain), and the tenant controls the data \
(the seed). The cloud can move the box; it can never read inside it.";

const ENTITIES: &str = "\
 ACCOUNT  (email + password — anyone signs up publicly)
   ├─ Subscription: a POOL bought by the unit → N threads · M GB RAM · S GB storage
   │                priced $/unit, metered hourly | monthly | yearly
   ├─ owns ──▶ DMS instances [1 .. many]      ← one account runs MANY isolated DMSes
   └─ can be DELEGATED other accounts' DMSes (manage them, within a granted scope)

 DMS INSTANCE  (an isolated process = the SAME self-hosted DMS, control_owner=managed)
   ├─ cap: threads · RAM · storage   (drawn from the owner's pool; cloud-enforced)
   ├─ domain: id.qirava.in (default)  [+ custom domain via cloudflared tunnel]
   ├─ mode: standalone | cluster   (tenant's choice, switchable)
   ├─ inside: the tenant's own dbs · workers · KMS · governance  (they control it)
   └─ managers: the OWNER account  +  any DELEGATED emails (each with a scope/role)

 NODE (bare metal)  →  PACKS MANY capped DMS processes (bin-packed — no wasted nodes)
 CONTROL PLANE (the cloud's own 3+ node cluster DMS): _cp_* catalogs + placer +
   meter + billing + rollout orchestrator + domain automation";

const PACKING: &str = "\
 NODE-1  (e.g. 32 thr · 128 GB · 4 TB)  ── packs MANY capped DMS processes ──
 ┌DMS acctA·p1 2thr·4GB┐ ┌DMS acctA·p2 1·2┐ ┌DMS acctB 1·2┐ ┌DMS acctC 4·8┐ …
 │ :p1  a1.qirava.in    │ │ :p2 a2.qirava.in│ │ :p3 …       │ │ :p4 …        │
 └──────────────────────┘ └─────────────────┘ └─────────────┘ └──────────────┘
   acctA runs MULTIPLE isolated DMSes; many accounts co-reside; each hard-capped.
   The cloud bin-packs until the pool is full, then uses the next node.";

const LAYERS: &str = "\
 LAYER A — CLOUD / ACCOUNT IAM   (who may MANAGE a DMS instance)
   owner account + delegated emails.   _cp_delegations{ dms_id, grantee_email, scope }
   e.g.  acctA grants  bob@x.com : manage  on  DMS-7
         → Bob (a signed-up account) administers DMS-7 from his own login.

 LAYER B — DMS-INTERNAL RBAC    (who may touch the DATA inside a DMS)
   the tenant's own custodian > admin > user > guest (L1/L2/L3, invite-only).

 BRIDGE: a Layer-A delegation makes the cloud issue that email an invite/identity
   INTO the DMS at the granted role → cloud-sharing maps to a real DMS account.";

const CONTROL: &str = "\
 ┌──────────── CLOUD CONTROL PLANE (3+ node cluster DMS) ────────────┐
 │ _cp_* · placer · meter · billing · ROLLOUT orchestrator · domains │
 └───────────────────────────┬───────────────────────────────────────┘
        authenticated, OUT-OF-BAND control envelopes (signed + mTLS)
                              │   ← never carries tenant data
     ┌─────────────────────────┼─────────────────────────┐
     ▼                         ▼                         ▼
 ┌ NODE-1 ───────────┐   ┌ NODE-2 ───────────┐   ┌ NODE-3 ─── … ─┐
 │ NODE AGENT (cloud)│   │ NODE AGENT        │   │ NODE AGENT    │
 │  spawn/stop·cgroup│   │                   │   │               │
 │  cap·storage·route│   │                   │   │               │
 │   │ localhost     │   │                   │   │               │
 │   ▼ control socket│   │                   │   │               │
 │ ┌DMS (capped)   ┐ │   │ ┌DMS┐ ┌DMS┐       │   │ ┌DMS┐ ┌DMS┐   │
 │ │seed=RAM/TEE    │ │   │ └───┘ └───┘       │   │ └───┘ └───┘   │
 │ │data=CIPHERTEXT │ │   │                   │   │               │
 │ └────────────────┘ │   │                   │   │               │
 └────────────────────┘   └───────────────────┘   └───────────────┘
   The node agent is the cloud's hands; it holds NO seed, so it manages
   processes + ciphertext, never plaintext.";

fn body() -> Node {
    let mut toc = Toc::new();
    let mut c = el("div");

    // --- the model ---
    c = c
        .child(toc.h2("The entity model"))
        .child(p(
            "Everything the cloud does is one of five entities and the relationships between them. \
             Accounts buy a resource pool and spend it across many isolated DMS instances; nodes \
             pack many DMSes; the control plane places, meters, and bills them.",
        ))
        .child(ascii("The cloud entity model", ENTITIES));

    // --- signup & billing ---
    c = c
        .child(toc.h2("Signup, subscription, billing"))
        .child(p(
            "Anyone can sign up publicly with an email. An account buys a resource pool — measured \
             in threads, GB of RAM, and GB of storage — and is metered and billed per unit, on an \
             hourly, monthly, or yearly cycle. Account auth adds a passkey + seed phrase plus a \
             compulsory email OTP (see Security & governance).",
        ))
        .child(defs(&[
            ("Public signup", "self-serve account creation by email; no sales gate."),
            ("Resource pool", "the units you purchase, drawn down by each DMS you create."),
            ("Metered billing", "per-unit pricing (thread · GB RAM · GB storage), hourly / monthly / yearly."),
            ("Self-serve", "create DMS instances, then create databases and worker apps inside them."),
        ]));

    // --- multi-DMS & packing ---
    c = c
        .child(toc.h2("Many isolated DMSes, densely packed"))
        .child(p(
            "A single account can create MANY DMS instances, each fully isolated — the same as if \
             they had stood up separate self-hosted DMSes. Each instance gets a resource cap drawn \
             from the account's pool, a default subdomain (id.qirava.in), and an optional custom \
             domain via a cloudflared tunnel.",
        ))
        .child(p(
            "Nodes are not wasted on one tenant: a node packs many capped DMS processes, \
             bin-packed until its pool is full. Isolation comes from the per-process cap (executor \
             budget + cgroup) and the per-tenant seed — not from giving each tenant a whole node.",
        ))
        .child(ascii("One node packs many capped DMS processes", PACKING));

    // --- delegation & layers ---
    c = c
        .child(toc.h2("Delegation, and the two access layers"))
        .child(p(
            "An account can grant another email a scope to manage one of its DMSes — cross-account \
             delegation. This lives at the cloud layer and is kept distinct from the DMS's own \
             internal RBAC; a delegation bridges the two by issuing the grantee an identity inside \
             the DMS at the granted role.",
        ))
        .child(ascii("Cloud IAM (manage the box) vs DMS RBAC (touch the data)", LAYERS));

    // --- control channel ---
    c = c
        .child(toc.h2("The control channel"))
        .child(p(
            "Each node runs a node agent (cloud authority) that spawns DMS processes, sets cgroup \
             caps, and manages routing. Each DMS exposes a localhost-only control socket that \
             accepts a NARROW lifecycle vocabulary — never data.",
        ))
        .child(ascii("Control plane → node agent → DMS control socket", CONTROL))
        .child(table(
            &["Group", "Verbs", "Reads data?"],
            &[
                &["Resource", "apply-cap(threads, ram, storage) → executor re-reads budget live", "no"],
                &["Replica", "add-follower(peer, token) · ship-snapshot · stream-wal", "no (ciphertext)"],
                &["Cutover", "drain(grace) · fifo-hold · promote(epoch) · step-down · repoint", "no"],
                &["Lifecycle", "start · stop · checkpoint · health · version · metrics", "no"],
            ],
        ))
        .child(callout(
            "warn",
            "Trust boundary",
            "None of the control verbs read or write records, unlock the seed, or bypass L1/L2/L3. \
             The cloud owns availability; the tenant owns confidentiality. A compromised control \
             plane can disrupt uptime — it can never read your data. On a TEE node the seed is \
             sealed and attestation-released, so even the cloud host cannot read guest RAM; on a \
             non-TEE node, the posture report discloses that the operator can technically reach it.",
        ))
        .child(callout(
            "note",
            "Status",
            "Control plane v1 is BUILT: the _cp_* catalogs, the cloud.* functions (provision, \
             scale, switch-mode, suspend/resume, terminate, invoice), and the RBAC-gated Cloud \
             Console — with the infra effect simulated and badged. The node agent, the DMS control \
             socket, real DMS spawning + cgroup caps, public signup, and domain automation are \
             designed and on the roadmap.",
        ));

    arch_kit::layout("/architecture/cloud", "Cloud control plane", LEAD, c, toc)
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(arch_kit::arch_css().to_string());
    css.push(crate::app::docs_kit::pager_css().to_string());
    let content = body();
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/architecture/cloud" };
    page(&meta, css, content)
}
