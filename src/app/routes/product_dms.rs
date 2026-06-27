//! `GET /products/dms` — the Qirava DMS product page.
//!
//! The DMS in one read: a single `execute` primitive and one function registry;
//! governance, KMS, the database, jobs, and replication are all functions; a
//! worker layer (before → handle → after) serves HTTP, WS, and native SSR/SSG/ISR
//! on one port; access is gated by three ordered checkpoints (L1 worker
//! before-auth, L2 execute scope, L3 the planner — the only db door); the API is
//! self-describing; vector/graph/search are built in; Studio is the admin app.
//! Apache-2.0. All content is accurate to `AGENTS.md` and the docs.

use qexec::FunctionResponse;
use qquill_view::Node;

use crate::app::routes::product_page::{
    closing, feature_section, hero, main_wrap, product_css, status_section, Cta, Feature, HeroStat,
    GITHUB_URL,
};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava DMS — one execute primitive, one function registry";
const DESCRIPTION: &str = "Qirava DMS is an AI-native, zero-dependency data system: one execute \
primitive and one function registry, with governance, KMS, database, jobs, and replication as \
functions, served over HTTP, WS, and native SSR/SSG/ISR on one port. Apache-2.0.";

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());

    let hero = hero(
        css,
        "Qirava DMS",
        "qdms",
        "One execute primitive, ",
        "one function registry",
        ".",
        "An AI-native, zero-dependency data system. Governance, KMS, the database, jobs, and \
         replication are not separate services — they are functions in one registry, reachable \
         only through execute(), only via a worker. The worker layer serves HTTP, WS, and native \
         SSR/SSG/ISR on a single port. Apache-2.0.",
        &[
            Cta { label: "Read the developer docs", href: "/docs", solid: true },
            Cta { label: "View on GitHub", href: GITHUB_URL, solid: false },
        ],
        &[
            HeroStat { value: "1", label: "execute primitive" },
            HeroStat { value: "3", label: "auth checkpoints" },
            HeroStat { value: "7179", label: "one port: HTTP·WS·SSR" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    // The core model: one primitive + one registry, functions for everything.
    let core = feature_section(
        "dms-core",
        "gradient",
        "The model",
        "Everything is a function behind one executor",
        "There is exactly one way in: execute() a function, and only through a worker. There is no \
         second entry point and no privileged side-door. Every subsystem is a function group, \
         governed by the same bounded executor.",
        &[
            Feature {
                kicker: "execute()",
                title: "One primitive, one registry",
                body: "A single execute() call dispatches against one function registry. New \
                       capability = a new function — never a new entry point. The executor bounds \
                       memory and work for every call.",
            },
            Feature {
                kicker: "functions/",
                title: "Subsystems as function groups",
                body: "Governance, KMS, the database, jobs, and replication live under functions/ \
                       as groups — not as top-level modules. src holds exactly functions/ and \
                       workers/, nothing of substance at the root.",
            },
            Feature {
                kicker: "config = data",
                title: "Configuration is data",
                body: "Routes, pages, assets, users, and grants are rows in _sys_* tables. \
                       Studio — the admin app — is itself a DMS client reading and writing that \
                       data; there is no separate config format.",
            },
        ],
    );

    // The worker layer + one port serving.
    let serving = feature_section(
        "dms-serving",
        "glass",
        "The serving layer",
        "before → handle → after, on one port",
        "Workers wrap every request in a before-chain and an after-chain around the handler, and \
         serve HTTP, WebSocket, and native rendering from the same listener — no sidecar, no \
         second runtime.",
        &[
            Feature {
                kicker: "before → handle → after",
                title: "The worker pipeline",
                body: "A worker runs before-functions (authentication, context setup), then the \
                       handler, then after-functions. Auth is just the first before-function: it \
                       writes the caller's identity into the shared context.",
            },
            Feature {
                kicker: "127.0.0.1:7179",
                title: "HTTP · WS · native render, one listener",
                body: "The same port serves request/response HTTP, WebSocket streams, and native \
                       server rendering — SSR per request, SSG at build, and ISR on a revalidate \
                       window — with no separate web tier.",
            },
            Feature {
                kicker: "SSR · SSG · ISR",
                title: "Rendering is a function too",
                body: "A page render is a registered function the worker calls; this very site is \
                       served (or statically exported) through exactly that path, so what ships \
                       and what serves are byte-identical.",
            },
        ],
    );

    // The three checkpoints — the security spine.
    let auth = feature_section(
        "dms-auth",
        "flat",
        "The access model",
        "Three checkpoints, one door to the data",
        "Every read and mutate crosses the same three gates in the same order. The planner is the \
         only door to read or mutate — no write path is allowed to skip it.",
        &[
            Feature {
                kicker: "L1 · worker before-auth",
                title: "Authenticate",
                body: "The worker's before-auth function verifies a session or an HMAC-signed key \
                       and writes the caller's identity into the shared context. Every auth/RBAC \
                       scenario extends this before-chain.",
            },
            Feature {
                kicker: "L2 · execute scope",
                title: "Authorize the function",
                body: "execute() checks the target function's declared scope — public, all-apps, \
                       system-only, or owner — against the authenticated caller before any handler \
                       runs.",
            },
            Feature {
                kicker: "L3 · the planner",
                title: "The only db door",
                body: "Database and table RBAC is enforced in the planner as app-scope ∩ \
                       principal-grant. The planner is the single door to read or mutate; nothing \
                       reaches the tables or WAL around it.",
            },
        ],
    );

    // AI-native capabilities + the self-describing API.
    let ai = feature_section(
        "dms-ai",
        "flat",
        "AI-native",
        "Vector, graph, search — and a self-describing API",
        "The data system is shaped for AI workloads out of the box, and it describes its own \
         surface so tools and agents can discover every function without a hand-written client.",
        &[
            Feature {
                kicker: "vector · graph · search",
                title: "Built-in indexing",
                body: "Vector similarity (an LSH path today), graph relationships, and search live \
                       in the engine — not as bolt-on extensions — so AI retrieval is a first-class \
                       query, not a second database.",
            },
            Feature {
                kicker: "GET /api/spec",
                title: "Self-describing surface",
                body: "/api/spec returns the live function catalog as native JSON, and \
                       /api/spec/openapi returns OpenAPI 3.1 — generated from the registry, so the \
                       spec can never drift from what executes.",
            },
            Feature {
                kicker: "Studio",
                title: "Studio is the admin app",
                body: "Qirava Studio, the default admin UI, is a Quill app that runs as just \
                       another DMS client — the same execute() path, the same auth checkpoints. No \
                       privileged backchannel.",
            },
        ],
    );

    // What's built.
    let status = status_section(
        "dms-status",
        "Status",
        "What's built today",
        "The DMS engine, the worker/serving layer, governance, and the AI retrieval path are \
         shipping. Dual-master clustering is the main in-flight item.",
        &[
            (Status::Built, "Execute primitive + function registry",
             "One bounded executor, one registry; every subsystem reachable only through execute()."),
            (Status::Built, "Worker layer on one port",
             "before → handle → after serving HTTP, WS, and native SSR/SSG/ISR from a single listener."),
            (Status::Built, "Three auth checkpoints + RBAC",
             "L1 before-auth, L2 execute scope, L3 planner; custodian > admin > user > guest with custodian-gated single-use invites."),
            (Status::Built, "Self-describing API",
             "/api/spec (native JSON) and /api/spec/openapi (OpenAPI 3.1), generated from the live catalog."),
            (Status::Built, "Vector / graph / search + Studio",
             "AI retrieval in the engine (vector via LSH today); Studio admin app runs as a DMS client."),
            (Status::Built, "Single-leader replication",
             "Single-leader (SINGLE) replication is built; dual-master (DUAL) is the planned next step."),
        ],
    );

    let closing = closing(
        "dms-closing",
        "Build on the DMS",
        "Read the getting-started guide, browse the self-describing API reference, or dive into \
         the three-checkpoint access model.",
        Cta { label: "Get started", href: "/docs/dms/getting-started", solid: true },
        Cta { label: "Read the architecture", href: "/architecture", solid: false },
    );

    main_wrap(vec![hero, core, serving, auth, ai, status, closing])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/products/dms" };
    page(&meta, css, content)
}
