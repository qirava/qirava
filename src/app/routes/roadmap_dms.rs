//! `GET /roadmap/dms` — the Qirava DMS roadmap.
//!
//! An honest BUILT / PARTIAL / PLANNED board for the data system. Sourced from
//! the repo: the engine + workers + function registry + QQL/DDL + RBAC/governance
//! + config-as-data + WAL + self-describing API + Studio are BUILT; single-leader
//! replication is PARTIAL (the master→follower op-frame stream ships; epoch
//! fencing, write-forwarding, and an authenticated replication link are pending);
//! a standalone KMS, dual-sync clustering, and the M-of-N custodian seed ceremony
//! are PLANNED. No dates are promised — only state.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{hero, main_wrap, product_css, Cta, HeroStat};
use crate::app::routes::roadmap_page::{board, legend, note, roadmap_css, Item, Lane};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava DMS roadmap — built, in progress, planned";
const DESCRIPTION: &str = "An honest status board for Qirava DMS: the engine, workers, function \
registry, QQL/DDL, RBAC + governance, WAL, and self-describing API are shipping; single-leader \
replication is in progress; standalone KMS, dual-sync clustering, and the M-of-N custodian seed \
ceremony are planned.";

const BUILT: &[Item] = &[
    Item { title: "Engine: indexes, planner, streaming", detail: "Filter/range/OR/AND/composite/join, sort-via-index, TTL sweep, and a plan cache — the planner is the only door to read or mutate." },
    Item { title: "Vector (LSH), graph, and search", detail: "Vector ANN via LSH, graph traversal, and search are in the engine — AI retrieval is a first-class query, not a second database." },
    Item { title: "QQL + DDL", detail: "The query language plus CREATE TABLE / CREATE INDEX, recorded as data in _sys_tables / _sys_indexes." },
    Item { title: "Worker pipeline: before → handle → after", detail: "Every request flows through a before-chain and after-chain around the handler; auth is just the first before-function." },
    Item { title: "One port: HTTP · WS · SSR/SSG/ISR", detail: "From-scratch HTTP/1.1 (CORS, ETag, cache, TCP_NODELAY), WebSocket CDC fan-out, and native rendering on a single listener." },
    Item { title: "Per-function scheduler", detail: "A bounded executor governs every call; jobs run as registered functions with overlap control." },
    Item { title: "Function registry + execute()", detail: "One execute primitive dispatches one registry (static + dynamic); new capability is a new function, never a new entry point." },
    Item { title: "RBAC L1/L2/L3 + governance", detail: "L1 before-auth, L2 execute scope, L3 the planner; custodian > admin > user > guest, deny-by-default, live authority re-read per request." },
    Item { title: "Auth: sessions + HMAC keys + invites", detail: "Session and HMAC-signed-key surfaces; governance catalogs are write-denied on the QQL surface; custodian-gated single-use invites onboard as guest." },
    Item { title: "Config-as-data (_sys_*)", detail: "Routes, workers, functions, pages, assets, users, and grants are rows in _sys_* tables, with live hot-reload." },
    Item { title: "Write-ahead log (WAL)", detail: "WAL-before-apply for mutations; recovery replays only committed transactions and truncates a torn tail." },
    Item { title: "Self-describing API + OpenAPI", detail: "GET /api/spec returns the live function catalog as JSON; /api/spec/openapi returns OpenAPI 3.1, generated from the registry." },
    Item { title: "Studio — the admin app", detail: "Qirava Studio runs as a DMS client over the same execute() path and the same auth checkpoints — no privileged backchannel." },
];

const PARTIAL: &[Item] = &[
    Item { title: "Single-leader replication", detail: "Master → follower committed op-frames ship over a length-prefixed TCP transport; config-as-data and the WAL replicate through the same stream." },
    Item { title: "Epoch fencing", detail: "Per-record {shard_id, epoch, hlc} tags and a promotion-bumped term — needed for multi-node ordering and fencing — are designed, not yet wired." },
    Item { title: "Write-forwarding + semi-sync acks", detail: "Follower last_applied is tracked but not yet fed back into the master's commit gate; write-forwarding from a follower is pending." },
    Item { title: "Authenticated replication link", detail: "The replication socket assumes a trusted LAN today; reusing the HMAC-signed-key machinery to authenticate followers is the next hardening step." },
];

const PLANNED: &[Item] = &[
    Item { title: "Standalone KMS", detail: "Key management as an independent service: the master seed plus per-key encryption-at-rest behind the Crypto provider trait." },
    Item { title: "Dual-sync cluster", detail: "Symmetric, per-shard roles (a node master for some shards, follower for others) with quorum, failover, and promotion." },
    Item { title: "M-of-N custodian seed ceremony", detail: "Shamir-split master seed, multi-authenticator custodians (FIDO2 PRF + BIP39), engaged only at boot/governance — never per query." },
    Item { title: "Confidential-compute attestation", detail: "The DMS boots inside a SEV-SNP / TDX VM and attests before receiving its seed; a startup security-tier banner states the residual risk." },
    Item { title: "TS / WinterTC worker runtime", detail: "A TypeScript / WinterTC worker runtime alongside the native function model." },
];

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(roadmap_css().to_string());

    let hero = hero(
        css,
        "DMS roadmap",
        "qdms",
        "The data system, ",
        "honestly tracked",
        ".",
        "What ships today, what is in progress, and what is designed but not yet built. The engine, \
         the worker/serving layer, the access model, and the AI retrieval path are shipping; \
         single-leader replication is in progress; standalone KMS, dual-sync clustering, and the \
         custodian seed ceremony are planned. No dates promised — only state.",
        &[
            Cta { label: "Explore the DMS", href: "/products/dms", solid: true },
            Cta { label: "Read the architecture", href: "/architecture", solid: false },
        ],
        &[
            HeroStat { value: "13", label: "capabilities shipping" },
            HeroStat { value: "4", label: "in progress" },
            HeroStat { value: "5", label: "planned" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    let mut board_section = board(
        "Status board",
        "Built, in progress, and planned",
        "Three lanes, no dates. Each item is sourced from the repository and the architecture docs \
         — the BUILT lane is present and usable now; PARTIAL has a working seam with deferred parts; \
         PLANNED is designed but not yet built.",
        ["rm-dms-built", "rm-dms-partial", "rm-dms-planned"],
        [
            Lane { status: Status::Built, items: BUILT },
            Lane { status: Status::Partial, items: PARTIAL },
            Lane { status: Status::Planned, items: PLANNED },
        ],
    );
    board_section = board_section.child(legend()).child(note(vec![
        text("Sourced from ".to_string()),
        el("a").attr("href", "/architecture").child(text("the architecture")),
        text(", the DMS docs, and the cluster/replication + security-governance design docs in the repo."
            .to_string()),
    ]));

    main_wrap(vec![hero, board_section])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/roadmap/dms" };
    page(&meta, css, content)
}
