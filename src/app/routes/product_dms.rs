//! `GET /products/dms` — the Qirava DMS product page (marketing).
//!
//! A focused, human marketing page (not deep docs): a hero, a plain "what it
//! is", a features grid, a concrete "how it works", an animated architecture
//! diagram, an honest BUILT / PARTIAL / PLANNED status block, and a closing CTA
//! to `/docs/dms`. The DMS in one read: a single `execute` primitive and one
//! function registry; governance, KMS, the database, jobs, and replication are
//! all functions; a worker layer (before → handle → after) serves HTTP, WS, and
//! native SSR/SSG/ISR on one port; access is gated by three ordered checkpoints
//! (L1 worker before-auth, L2 execute scope, L3 the planner — the only db door);
//! the API is self-describing; vector/graph/search are built in; Studio is the
//! admin app. Apache-2.0. All content is accurate to `AGENTS.md` and the docs.

use qexec::FunctionResponse;
use qquill_view::Node;

use crate::app::routes::product_page::{
    arch_anim, arch_anim_css, closing, feature_section, hero, main_wrap, product_css,
    status_section, ArchNode, Cta, Feature, HeroStat, GITHUB_URL,
};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava DMS — one execute primitive, one function registry";
const DESCRIPTION: &str = "Qirava DMS is an AI-native, zero-dependency data system: one execute \
primitive and one function registry, with governance, KMS, the database, jobs, and replication as \
functions, served over HTTP, WS, and native SSR/SSG/ISR on one port. Apache-2.0.";

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(arch_anim_css().to_string());

    let hero = hero(
        css,
        "Qirava DMS",
        "qdms",
        "One execute primitive, ",
        "one function registry",
        ".",
        "An AI-native, zero-dependency data system. The database, governance, KMS, jobs, and \
         replication aren't separate services — they're functions in one registry, reachable only \
         through execute(), only via a worker. That worker serves HTTP, WebSocket, and native \
         SSR/SSG/ISR on a single port. Apache-2.0.",
        &[
            Cta {
                label: "Read the DMS docs",
                href: "/docs/dms",
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
                label: "execute primitive",
            },
            HeroStat {
                value: "3",
                label: "auth checkpoints",
            },
            HeroStat {
                value: "7179",
                label: "one port: HTTP·WS·SSR",
            },
            HeroStat {
                value: "0",
                label: "third-party deps",
            },
        ],
    );

    // What it is — plain language, the four things that make it different.
    let what = feature_section(
        "dms-what",
        "gradient",
        "What it is",
        "One engine for relational, search, graph, and vector",
        "Qirava DMS is a single data system shaped for AI workloads. You write one query language \
         and get tables, full-text search, graph relationships, and vector similarity from the same \
         engine — durable through a write-ahead log, and gated by one access model. No second \
         database, no sidecar, no third-party dependency.",
        &[
            Feature {
                kicker: "QQL",
                title: "One query language, four models",
                body: "Relational rows, full-text search, graph traversals, and vector similarity \
                       are all first-class in QQL against one engine — so AI retrieval is a normal \
                       query, not a separate stack to operate.",
            },
            Feature {
                kicker: "WAL",
                title: "Durable by default",
                body: "Every mutation lands in a write-ahead log before it's acknowledged, so a \
                       crash replays cleanly. Durability is built into the engine, not bolted on.",
            },
            Feature {
                kicker: "config = data",
                title: "The system configures itself",
                body: "Routes, pages, assets, users, and grants are just rows in _sys_* tables. \
                       Studio, the admin app, is a DMS client reading and writing that data — there \
                       is no separate config format.",
            },
        ],
    );

    // Features grid — what you actually get.
    let features = feature_section(
        "dms-features",
        "glass",
        "Features",
        "Everything a data system needs, in one binary",
        "Each capability is a function in the same registry behind the same executor. New \
         capability means a new function — never a new entry point or a new service to run.",
        &[
            Feature {
                kicker: "QQL",
                title: "Multi-model queries",
                body: "Join relational data, run search, walk a graph, and rank by vector \
                       similarity in one place. One language, one planner, one engine.",
            },
            Feature {
                kicker: "ACID · WAL",
                title: "Transactions you can trust",
                body:
                    "ACID semantics on top of a write-ahead log: commits are atomic and durable, \
                       and recovery is a replay — no separate journal service to babysit.",
            },
            Feature {
                kicker: "RBAC · signed keys",
                title: "Access control built in",
                body: "Sessions or HMAC-signed API keys identify the caller; role-based grants \
                       (custodian > admin > user > guest) decide what they can touch. No add-on \
                       auth layer.",
            },
            Feature {
                kicker: "GET /api/spec",
                title: "Self-describing API",
                body: "/api/spec returns the live function catalog as JSON, and \
                       /api/spec/openapi returns OpenAPI 3.1 — generated from the registry, so the \
                       spec can never drift from what runs.",
            },
            Feature {
                kicker: "Studio",
                title: "Admin app included",
                body: "Qirava Studio is the default admin UI — a Quill app that runs as just \
                       another DMS client, through the same execute() path and the same auth \
                       checkpoints. No privileged backchannel.",
            },
            Feature {
                kicker: "replication",
                title: "Replication for resilience",
                body: "Single-leader replication keeps a follower in step with the leader's WAL, \
                       so a standby has the data. Dual-master clustering is the next step.",
            },
        ],
    );

    // How it works — concrete, plain, the request's life and the three gates.
    let how = feature_section(
        "dms-how",
        "flat",
        "How it works",
        "A request's life: before → handle → after",
        "Every request flows through the same pipeline and crosses the same three gates in the same \
         order. Reading the list below is reading exactly what happens on a call.",
        &[
            Feature {
                kicker: "before → handle → after",
                title: "The worker pipeline",
                body: "A worker runs before-functions (authenticate, set up context), then the \
                       handler, then after-functions. Auth is simply the first before-function — it \
                       writes the caller's identity into shared context for everything downstream.",
            },
            Feature {
                kicker: "L1 → L2 → L3",
                title: "Three checkpoints, one door",
                body: "L1 the worker authenticates the caller. L2 execute() checks the target \
                       function's scope (public, all-apps, system, or owner). L3 the planner — the \
                       only door to the tables — enforces app-scope ∩ principal-grant. Nothing \
                       reaches the data around it.",
            },
            Feature {
                kicker: "_sys_* catalogs",
                title: "Config lives in the database",
                body: "Routes, pages, assets, users, and grants are rows in _sys_* catalog tables, \
                       so changing how the system behaves is just changing data — versioned, \
                       queryable, and served by the same engine that serves your app.",
            },
        ],
    );

    // Architecture animation — the three-checkpoint flow + the module map.
    let arch = arch_anim(
        "dms-arch",
        "Architecture",
        "From request to data — and the modules behind it",
        "A request passes the worker, the executor, and the planner before it ever reaches the \
         tables; the diagram lights each stage in turn. The same engine is assembled from the q* \
         module family: qpkgs (stdlib), qquill (UI), qdms (the data system), and qcloud (managed \
         control plane).",
        &[
            ArchNode {
                label: "Worker",
                sub: "L1 · authenticate",
                badge: "",
            },
            ArchNode {
                label: "execute()",
                sub: "L2 · function scope",
                badge: "",
            },
            ArchNode {
                label: "Planner",
                sub: "L3 · the only db door",
                badge: "",
            },
            ArchNode {
                label: "Engine",
                sub: "tables · WAL · indexes",
                badge: "",
            },
            ArchNode {
                label: "q* modules",
                sub: "qpkgs·qquill·qdms·qcloud",
                badge: "",
            },
        ],
    );

    // Honest status — what's built, partial, and planned.
    let status = status_section(
        "dms-status",
        "Status",
        "What's built, and what's next",
        "The engine, the worker/serving layer, the access model, and AI retrieval are shipping. \
         Clustering and a couple of subsystems are honestly still in flight.",
        &[
            (Status::Built, "Execute primitive + function registry",
             "One bounded executor, one registry; every subsystem reachable only through execute()."),
            (Status::Built, "Worker layer on one port",
             "before → handle → after serving HTTP, WS, and native SSR/SSG/ISR from a single listener."),
            (Status::Built, "Three auth checkpoints + RBAC",
             "L1 before-auth, L2 execute scope, L3 planner; custodian > admin > user > guest with custodian-gated single-use invites."),
            (Status::Built, "Self-describing API + Studio",
             "/api/spec (JSON) and /api/spec/openapi (OpenAPI 3.1) from the live catalog; Studio admin app runs as a DMS client."),
            (Status::Partial, "Vector / graph / search",
             "Graph and search are in the engine; vector similarity runs via an LSH path today, with a denser index planned."),
            (Status::Partial, "Replication",
             "Single-leader (SINGLE) replication is built and running; dual-master (DUAL) clustering is the planned next step."),
        ],
    );

    let closing = closing(
        "dms-closing",
        "Build on the DMS",
        "Start with the getting-started guide, browse the self-describing API reference, or read \
         the three-checkpoint access model end to end.",
        Cta {
            label: "Read the DMS docs",
            href: "/docs/dms",
            solid: true,
        },
        Cta {
            label: "View on GitHub",
            href: GITHUB_URL,
            solid: false,
        },
    );

    main_wrap(vec![hero, what, features, how, arch, status, closing])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/products/dms",
    };
    page(&meta, css, content)
}
