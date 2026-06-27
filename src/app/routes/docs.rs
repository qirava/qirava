//! The PER-PRODUCT docs site.
//!
//! * `GET /docs` — the docs INDEX hub: cards linking the four product doc sets
//!   (no sidebar; it is a chooser, not a doc page).
//! * `GET /docs/<product>` — each product's docs LANDING (Overview), inside a
//!   sidebar scoped to that product.
//! * `GET /docs/<product>/<page>` — a doc page within a product's scope.
//!
//! Every doc page builds its body, then calls [`layout`] with its [`Product`] +
//! path so the sidebar/pager scope to that product (the page is declared once in
//! `docs_kit::DOCS`). Code blocks carry a working copy button (the `copy`
//! island).

use qexec::FunctionResponse;
use qquill_docs::Callout;
use qquill_view::{el, raw, text, Node};

use crate::app::docs_kit::{doc_for, layout, pager_css, Product, Toc};
use crate::app::routes::product_page::ARROW_SVG;
use crate::app::routes::{copy_code, CodeLine};
use crate::app::shell::page;
use crate::app::{Css, Meta};

/// Pull in the `qquill-docs` content-primitive CSS (`.qq-heading`, `.qq-callout`,
/// …) plus our pager/switcher CSS. The DocShell-only rules in that sheet are
/// inert here (no matching markup); the `.qq-*` classes don't collide.
fn docs_css(css: &mut Css) {
    css.push(qquill_docs::layout_css().to_css());
    css.push(pager_css().to_string());
}

/// A prose paragraph (escaped).
fn p(s: &str) -> Node {
    el("p").child(text(s.to_string()))
}

/// A bullet list from `(strong, rest)` pairs.
fn bullets(items: &[(&str, &str)]) -> Node {
    let mut ul = el("ul").class("q-list");
    for (strong, rest) in items {
        ul = ul.child(el("li").children([
            el("strong").child(text(strong.to_string())),
            text(rest.to_string()),
        ]));
    }
    ul
}

/// Render a product doc page: look up its `Product` from `docs_kit::DOCS` by
/// `path`, wrap `content`/`toc` in the product-scoped [`layout`], frame the page.
fn render_doc(
    path: &'static str,
    title: &'static str,
    lead: &'static str,
    meta_title: &'static str,
    meta_desc: &'static str,
    css: Css,
    content: Node,
    toc: Toc,
) -> FunctionResponse {
    let product = doc_for(path).map(|d| d.product).unwrap_or(Product::Dms);
    let body = layout(product, path, title, lead, content, toc);
    let meta = Meta { title: meta_title, description: meta_desc, path };
    page(&meta, css, body)
}

// ===========================================================================
// /docs — the docs INDEX hub (chooser cards, no sidebar)
// ===========================================================================

/// One product hub card on the `/docs` index.
fn hub_card(product: Product, summary: &str, points: &[&str]) -> Node {
    let mut list = el("ul").class("q-list");
    for pt in points {
        list = list.child(el("li").child(text((*pt).to_string())));
    }
    let learn = el("a")
        .class("q-prod-learn")
        .attr("href", product.landing())
        .child(text(format!("Open {} docs ", product.name())))
        .child(raw(ARROW_SVG));

    el("article")
        .class("q-hub-card")
        .child(el("h2").class("q-hub-card__title").child(text(product.name().to_string())))
        .child(el("p").class("q-hub-card__sum").child(text(summary.to_string())))
        .child(list)
        .child(learn)
}

fn hub_css() -> &'static str {
    "\
.q-docs-hub{max-width:72rem;margin:0 auto;padding:3rem 1.5rem 5rem}\
.q-docs-hub__head{max-width:46rem;margin:0 0 2.5rem}\
.q-hub-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:1.25rem}\
@media (max-width:720px){.q-hub-grid{grid-template-columns:1fr}}\
.q-hub-card{display:flex;flex-direction:column;padding:1.5rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-hub-card:hover{border-color:var(--q-color-brand);transform:translateY(-2px)}\
.q-hub-card__title{font-size:1.2rem;margin:0 0 .4rem}\
.q-hub-card__sum{color:var(--q-color-muted);margin:0 0 1rem;line-height:1.6}\
.q-prod-learn{display:inline-flex;align-items:center;gap:.35rem;margin-top:auto;padding-top:1rem;font-weight:var(--q-font-weight-medium);font-size:.92rem;color:var(--q-color-brand)}\
.q-prod-learn:hover{text-decoration:none}\
.q-prod-learn .q-arr{transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-prod-learn:hover .q-arr{transform:translateX(3px)}"
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(hub_css().to_string());

    let head = el("div")
        .class("q-docs-hub__head")
        .child(el("p").class("q-eyebrow").child(text("Documentation")))
        .child(el("h1").class("q-h1").child(text("Pick a product to dive into")))
        .child(el("p").class("q-lead").child(text(
            "Each product has its own documentation set, with a sidebar scoped to that \
             product — like reading the Next.js or shadcn docs. Choose one to begin; you \
             can switch products from the sidebar at any time.",
        )));

    let grid = el("div")
        .class("q-hub-grid")
        .child(hub_card(
            Product::Dms,
            "The AI-native, zero-dependency data system. Install, the mental model, and the \
             three authorization checkpoints.",
            &["Getting started", "Core concepts: execute → worker → planner", "The L1/L2/L3 access model"],
        ))
        .child(hub_card(
            Product::Quill,
            "The Rust-native UI/app framework that powers this very site. SSR + islands, \
             zero server-side JavaScript.",
            &["Overview and authoring model", "Islands and per-page bundling", "Styled components + theming"],
        ))
        .child(hub_card(
            Product::Stdlib,
            "The 13 zero-dependency q* crates shared across every product — the qexec executor \
             and qvalue model plus focused utilities.",
            &["The substrate: qexec + qvalue", "Utility crate catalog", "The one-way dependency arrow"],
        ))
        .child(hub_card(
            Product::Cloud,
            "The planned managed, multi-tenant control plane — a DMS that manages other DMSes. \
             Designed, not yet built.",
            &["What Qirava Cloud will be", "Open-core model", "Status and roadmap"],
        ));

    let body = el("main")
        .class("q-docs-hub")
        .id("main")
        .child(head)
        .child(grid);

    let meta = Meta {
        title: "Documentation — Qirava",
        description: "Qirava documentation, organized per product: the DMS data system, the Quill \
                      UI framework, the q* stdlib, and Qirava Cloud.",
        path: "/docs",
    };
    page(&meta, css, body)
}

// ===========================================================================
// /docs/dms — DMS Overview (landing)
// ===========================================================================

pub fn respond_dms(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(toc.h2("What the DMS is"))
        .child(p(
            "Qirava DMS is an AI-native, zero-dependency data system built on one execute \
             primitive and one function registry. Governance/RBAC, KMS, the database, jobs, and \
             replication are all functions behind a single bounded executor — std and first-party \
             crates only.",
        ))
        .child(Callout::tip(p(
            "Everything on this site is a Quill app talking to the same data substrate. The DMS \
             and Quill fit together, but each has its own docs — this set is the DMS.",
        )).render())
        .child(toc.h2("How these docs are organized"))
        .child(bullets(&[
            ("Start here — ", "install, build, and run; then the mental model in one page."),
            ("Concepts — ", "the three authorization checkpoints and the execute → worker → planner path."),
        ]))
        .child(p(
            "Use the left sidebar to move between DMS pages, the product switcher above it to hop \
             to another product, the list on the right to jump within a page, and prev/next at the \
             bottom to read in order.",
        ))
        .child(toc.h2("Design principles"))
        .child(bullets(&[
            ("Zero dependencies — ", "std and first-party crates only; the sole exception is cryptography, kept behind a trait."),
            ("Security-first — ", "every read or mutate is authorized; there is no bypass path to the database."),
            ("Performance-first — ", "one bounded executor governs all work; hot paths are benchmarked before and after changes."),
        ]));

    render_doc(
        "/docs/dms",
        "Qirava DMS",
        "The mental model, an honest map of what's built, and how to get running in three commands.",
        "Qirava DMS docs",
        "Qirava DMS documentation: the data system, the auth model, and getting started.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/getting-started
// ===========================================================================

pub fn respond_getting_started(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(toc.h2("Prerequisites"))
        .child(p(
            "A recent stable Rust toolchain and git. There is nothing else to install — Qirava has \
             no external dependencies, so the whole tree builds with cargo alone.",
        ))
        .child(toc.h2("Clone and run the DMS"))
        .child(p(
            "Clone with --recursive so the submodules (qdms, qpkgs, qquill) come along, then build \
             and run the data system:",
        ))
        .child(copy_code("gs-dms", &[
            CodeLine::Comment("# clone with submodules, build the DMS, run it"),
            CodeLine::Cmd("git clone --recursive https://github.com/qirava/qirava"),
            CodeLine::Cmd("cargo build --release -p qdms"),
            CodeLine::Cmd("./target/release/qdms"),
            CodeLine::Comment("# Studio (UI) + API on 127.0.0.1:7179"),
        ]))
        .child(Callout::note(p(
            "On first run the bootstrap credential is printed to the console exactly once — save it. \
             Onboarding is custodian-gated: new users join via a single-use invite as a guest, and \
             grants (not the invite) confer power.",
        )).render())
        .child(toc.h2("Build a front end with Quill"))
        .child(p(
            "Quill apps are plain Rust binaries that render HTML on the server and ship islands only \
             where a page needs interactivity. Scaffold one and run it:",
        ))
        .child(copy_code("gs-quill", &[
            CodeLine::Comment("# scaffold a Quill app (this very site is one)"),
            CodeLine::Cmd("quill new myapp && cd myapp"),
            CodeLine::Cmd("cargo run"),
            CodeLine::Comment("# serves SSR HTML; `cargo run -- build` exports a static dist/"),
        ]))
        .child(toc.h2("What ships to the browser"))
        .child(p(
            "A page with no islands ships zero JavaScript. A page that uses islands ships a per-page \
             bundle: the ~4 KB signal/hydration core plus only the behaviors that page actually \
             uses — nothing more.",
        ));

    render_doc(
        "/docs/dms/getting-started",
        "Installation",
        "Clone, build, and run the data system — then scaffold a Quill front end. No external \
         dependencies to install.",
        "Installation — Qirava DMS docs",
        "Install, build, and run the Qirava DMS, then scaffold a Quill app. Std and first-party \
         crates only.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/quick-start
// ===========================================================================

pub fn respond_quick_start(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "You have the DMS running on 127.0.0.1:7179 (see Installation). This page logs in \
             with the bootstrap credentials, runs a first query over HTTP, and shows where the \
             admin UI lives.",
        ))
        .child(toc.h2("Capture the bootstrap credentials"))
        .child(p(
            "On first run the launcher prints two secrets to the console, each exactly once — \
             save them before anything else:",
        ))
        .child(copy_code("qs-boot", &[
            CodeLine::Plain("  BOOTSTRAP API KEY (shown once): <api-key>"),
            CodeLine::Plain("  BOOTSTRAP CUSTODIAN (shown once): user=<name> password=<pass>"),
        ]))
        .child(bullets(&[
            ("The API key — ", "an admin key for the /api/qql endpoint, minted only when require_api_key is on and no keys exist yet."),
            ("The custodian — ", "the root of trust for governance: it can promote admins, create custodians, and define roles. The login for Qirava Studio."),
        ]))
        .child(Callout::warn(p(
            "Both are printed once and never again. If you lose them, the simplest recovery is a \
             fresh data_dir. Treat them like root credentials.",
        )).render())
        .child(toc.h2("Run your first query"))
        .child(p(
            "QQL is the query language; it reads SQL-like. POST the statement as the raw request \
             body to /api/qql, with the admin API key in the X-API-Key header:",
        ))
        .child(copy_code("qs-query", &[
            CodeLine::Comment("# insert a row, then read it back"),
            CodeLine::Cmd("curl -X POST http://127.0.0.1:7179/api/qql \\"),
            CodeLine::Plain("     -H \"X-API-Key: <api-key>\" \\"),
            CodeLine::Plain("     --data 'INSERT INTO people { id: 1, name: \"Ada\" }'"),
            CodeLine::Cmd("curl -X POST http://127.0.0.1:7179/api/qql \\"),
            CodeLine::Plain("     -H \"X-API-Key: <api-key>\" \\"),
            CodeLine::Plain("     --data 'SELECT name FROM people'"),
        ]))
        .child(p(
            "Every response comes back in the same envelope: a success carries data, an error \
             carries a stable code and message. There is also a read-only GET form for quick \
             checks:",
        ))
        .child(copy_code("qs-get", &[
            CodeLine::Cmd("curl 'http://127.0.0.1:7179/api/qql?q=SELECT+count(*)+FROM+people' \\"),
            CodeLine::Plain("     -H \"X-API-Key: <api-key>\""),
        ]))
        .child(Callout::note(p(
            "If require_api_key is off (the default in a fresh dms.config can be either; the UI \
             routes always stay public), the /api/qql endpoint accepts requests without a key — \
             turn it on for anything beyond local experimentation.",
        )).render())
        .child(toc.h2("Log in to Qirava Studio"))
        .child(p(
            "Studio is the built-in admin app, served on the same port at /studio/login. Sign in \
             with the bootstrap custodian username and password. Studio is a normal DMS client — \
             every action it takes is re-checked through the same three authorization checkpoints; \
             there is no privileged backdoor.",
        ))
        .child(copy_code("qs-studio", &[
            CodeLine::Comment("# open the admin app in a browser"),
            CodeLine::Plain("http://127.0.0.1:7179/studio/login"),
        ]))
        .child(toc.h2("Where to next"))
        .child(bullets(&[
            ("Core concepts — ", "the execute -> worker -> planner path and why it is the whole security model."),
            ("QQL: Reading / Writing — ", "the query surface in depth, with copy-pasteable statements."),
            ("The self-describing API — ", "GET /api/spec returns the live catalog of every callable function."),
        ]));

    render_doc(
        "/docs/dms/quick-start",
        "Quick start",
        "First login, first query: capture the bootstrap secrets, run QQL over HTTP, and open \
         Qirava Studio.",
        "Quick start — Qirava DMS docs",
        "First login and first query against a running Qirava DMS: bootstrap credentials, QQL \
         over /api/qql, and Qirava Studio.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/execute-model
// ===========================================================================

pub fn respond_execute_model(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "The DMS is, at its core, two things: one execute primitive and one function \
             registry. Everything else — governance, KMS, the database, jobs, replication — is a \
             function in that registry, reachable only through execute, and only ever from a \
             worker.",
        ))
        .child(toc.h2("One executor, one registry"))
        .child(p(
            "Functions register under stable string keys (for example qql.read.key or \
             auth.login). To run one you call execute(key, input). There is no other entry \
             point — no direct function calls that skip the executor, and nothing of substance \
             lives at the crate root. The qdms source tree has exactly two folders: functions/ \
             and workers/.",
        ))
        .child(Callout::tip(p(
            "Because every unit of work flows through one bounded executor, the security and \
             performance properties are auditable: there is exactly one place to look.",
        )).render())
        .child(toc.h2("Functions are grouped, not scattered"))
        .child(p(
            "Functions live in groups under functions/: read, write, search, graph, vector, \
             inline, planner, auth, db, api, repl, docs. Governance and RBAC are the auth group; \
             the database is the db group; there is no top-level governance/ or auth/ module.",
        ))
        .child(toc.h2("Function scope"))
        .child(p(
            "Every function declares a scope that the executor enforces before it runs — this is \
             checkpoint L2. The four scopes:",
        ))
        .child(bullets(&[
            ("public — ", "callable from outside through a worker (e.g. the qql.* roots and auth.login)."),
            ("internal_global — ", "a shared helper any function may call, but not reachable from outside."),
            ("internal_scoped — ", "a helper callable only by a specific owning root function."),
            ("internal_private — ", "a helper callable only by its single owner; the tightest scope."),
        ]))
        .child(p(
            "A request from the network can only ever invoke a public function. Read and write \
             logic is split into small helpers at tighter scopes, so the externally reachable \
             surface stays minimal and explicit.",
        ))
        .child(toc.h2("Roots and helpers"))
        .child(p(
            "A public function is a root: it orchestrates a chain of internal helpers to do its \
             work. For example the qql.read.filter_sort_asc root drives a plan helper and a \
             normalize helper that only it can call. The catalog at GET /api/spec lists every \
             root with its real scope, inputs, outputs, and error codes.",
        ));

    render_doc(
        "/docs/dms/execute-model",
        "The execute model",
        "One execute primitive and one function registry: how every unit of work — db, auth, \
         jobs — is a scoped function reachable only through the executor.",
        "The execute model — Qirava DMS docs",
        "The Qirava execute model: one executor, one function registry, function scopes \
         (public/internal), and roots vs helpers.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/workers
// ===========================================================================

pub fn respond_workers(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "A worker is the only thing that calls execute. It fronts a network port, maps an \
             incoming request to a function key, runs the authorization before-chain, executes \
             the function, and runs the after-chain. Workers are config-as-data: their routes \
             and chains live in _sys_* tables and hot-reload on write.",
        ))
        .child(toc.h2("before -> handle -> after"))
        .child(p(
            "Every request a worker serves runs the same three phases:",
        ))
        .child(copy_code("w-chain", &[
            CodeLine::Plain("before:  auth (L1) — authenticate, write identity into ctx"),
            CodeLine::Plain("handle:  execute(key, input) — the function chain (L2 + L3)"),
            CodeLine::Plain("after:   hooks — e.g. bump the session idle window"),
        ]))
        .child(p(
            "Auth is a before-function: it authenticates the caller and writes their identity \
             into the shared context before the handler runs. Every auth/RBAC scenario extends \
             that before-chain — you never add a special code path.",
        ))
        .child(toc.h2("One port, HTTP and WebSocket"))
        .child(p(
            "The system worker binds one address and serves both the API and the UI on it: \
             POST /api/qql (QQL in the body), GET /api/qql?q=... (read-only), POST \
             /api/qql/batch (a JSON array of statements), the docs site, and Qirava Studio. A \
             route can also upgrade to a WebSocket, bound to the same key it routes to.",
        ))
        .child(Callout::note(p(
            "The UI and the API share one port. The /api/qql routes can require an API key while \
             the UI routes stay public — gating is per-route, expressed as data.",
        )).render())
        .child(toc.h2("SSR, SSG, and ISR"))
        .child(p(
            "Workers render pages as well as serve data. A page handler returns HTML; the worker \
             can serve it three ways:",
        ))
        .child(bullets(&[
            ("SSR — ", "render on each request, for fully dynamic pages."),
            ("SSG — ", "prerender once into _sys_pages with a strong content-hash etag, served immutable with a 304 on If-None-Match."),
            ("ISR — ", "serve a cached render and revalidate it in the background after it goes stale."),
        ]))
        .child(p(
            "This very docs site is served by the DMS this way: each page is prerendered (SSG) \
             into _sys_pages and bound to a _sys_routes row, so it serves with zero render cost \
             at request time.",
        ))
        .child(toc.h2("Trusted vs deployed workers"))
        .child(p(
            "The system worker is trusted (its code is in the binary). User workers are deployed \
             as data into the _sys_* tables and loaded at boot; a write to those tables \
             hot-reloads the worker definitions with no restart. The executor's resource budget \
             still bounds every worker — serving never escapes the DMS limits.",
        ));

    render_doc(
        "/docs/dms/workers",
        "Workers",
        "The before -> handle -> after request funnel: HTTP/WebSocket on one port, SSR/SSG/ISR \
         rendering, and routes expressed as data.",
        "Workers — Qirava DMS docs",
        "Qirava workers: the before/handle/after chain, one-port HTTP and WebSocket, SSR/SSG/ISR \
         rendering, and config-as-data routing.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/access-control
// ===========================================================================

pub fn respond_access_control(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Authorization in the DMS is three checkpoints, always in order. Together they make \
             one guarantee: nothing reads or mutates the database except through the planner, \
             after passing every gate before it.",
        ))
        .child(toc.h2("The three checkpoints"))
        .child(bullets(&[
            ("L1 — worker before-auth: ", "the worker authenticates the caller before any function runs — a session token or an HMAC-signed API key. It writes the caller's identity into the shared context."),
            ("L2 — execute scope: ", "the executor checks the caller may invoke that function at all — public / internal_global / internal_scoped / internal_private. A network request can only reach a public function."),
            ("L3 — the planner: ", "QQL-level RBAC at plan time — the effective grant is app-scope intersect principal-grant. This is the ONLY door to read or mutate."),
        ]))
        .child(Callout::warn(p(
            "Never add a write path that skips L3. The planner is the single chokepoint for every \
             read and mutate; bypassing it breaks the security model. Reads and writes are \
             deny-by-default.",
        )).render())
        .child(toc.h2("RBAC roles"))
        .child(p(
            "Human identities carry a role, ordered custodian > admin > user > guest:",
        ))
        .child(bullets(&[
            ("custodian — ", "the root of trust. Can promote admins, create other custodians, and define roles. Bootstrapped once on first install."),
            ("admin — ", "manages users and grants, but only within their own grants. Can mint and grant API keys."),
            ("user — ", "an ordinary authenticated principal, limited to its db grants."),
            ("guest — ", "the role a new invite onboards as; holds no power until granted."),
        ]))
        .child(p(
            "Custodian-only governance functions (promote_admin, create_custodian, define_role) \
             deny with forbidden on the role gate; admin-or-custodian functions (create_user, \
             set_db_grant) re-check the caller's role on every call.",
        ))
        .child(toc.h2("Invites and onboarding"))
        .child(p(
            "Onboarding is custodian-gated and closed by default. A new person joins via a \
             single-use invite and onboards as a guest. The invite gets them in the door; it \
             confers no power — grants do. Admins manage users only within the grants they \
             themselves hold.",
        ))
        .child(toc.h2("Sessions vs signed API keys"))
        .child(p(
            "There are two ways to authenticate at L1, for two audiences:",
        ))
        .child(bullets(&[
            ("Sessions (humans) — ", "auth.login exchanges username + password (PBKDF2) for a random session token; only its SHA-256 hash is stored. auth.session validates the bearer token on each request; an after-hook extends a 30-minute idle window. Used by Qirava Studio."),
            ("Signed API keys (services) — ", "a stateless request signed with HMAC: the key id, timestamp, nonce, and signature over a canonical string. auth.signed checks the timestamp skew and nonce replay, then injects the key's scope and table grants. The plaintext key is shown once at mint time; only its hash is stored."),
        ]))
        .child(p(
            "Per-statement table RBAC (auth.check_table_grant) then allows or denies each QQL \
             statement deny-by-default against the key's scope and table grants — admin keys \
             bypass. The /api/qql/batch route enforces this per statement in the array.",
        ));

    render_doc(
        "/docs/dms/access-control",
        "Access control",
        "The three authorization checkpoints (L1 before-auth, L2 execute scope, L3 the planner), \
         the RBAC role ladder, invites, and sessions vs signed API keys.",
        "Access control — Qirava DMS docs",
        "Qirava access control: the L1/L2/L3 checkpoints, the custodian>admin>user>guest role \
         ladder, invites, and sessions vs HMAC-signed API keys.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/qql-reading
// ===========================================================================

pub fn respond_qql_reading(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "QQL is the query language; it reads SQL-like and is sent as the request body to \
             /api/qql. Read statements are planned (L3) and then executed by the read group's \
             roots — by key, filter, sort, limit, join, and aggregate.",
        ))
        .child(toc.h2("Filter and sort"))
        .child(p(
            "A SELECT with a WHERE predicate filters rows; ORDER BY sorts them. The read group \
             exposes filter, sort, and a combined filter + ascending-sort + limit path.",
        ))
        .child(copy_code("r-filter", &[
            CodeLine::Plain("SELECT id, name FROM people WHERE active = true"),
            CodeLine::Plain("SELECT name FROM people ORDER BY name ASC"),
        ]))
        .child(toc.h2("Limit"))
        .child(p(
            "Cap the number of rows returned. The limit root returns at most `limit` matched \
             rows.",
        ))
        .child(copy_code("r-limit", &[
            CodeLine::Plain("SELECT * FROM people WHERE active = true ORDER BY name ASC LIMIT 20"),
        ]))
        .child(toc.h2("Join"))
        .child(p(
            "Join matched rows against a related table; the join root returns the combined rows.",
        ))
        .child(copy_code("r-join", &[
            CodeLine::Plain("SELECT people.name, orders.total"),
            CodeLine::Plain("FROM people JOIN orders ON orders.person_id = people.id"),
        ]))
        .child(toc.h2("Aggregate"))
        .child(p(
            "Aggregate over matched rows — counts, sums, and grouped results. The aggregate root \
             returns groups; there is also a join-then-aggregate root.",
        ))
        .child(copy_code("r-agg", &[
            CodeLine::Plain("SELECT count(*) FROM people"),
            CodeLine::Plain("SELECT person_id, sum(total) FROM orders GROUP BY person_id"),
        ]))
        .child(Callout::note(p(
            "Every read root takes a QQL query (with optional bound params) and returns rows plus \
             a count — except the aggregate roots, which return groups. The exact input/output \
             shape of each root is in the live catalog at GET /api/spec.",
        )).render());

    render_doc(
        "/docs/dms/qql-reading",
        "Reading",
        "Read data with QQL: filter, sort, limit, join, and aggregate — each a planned, \
         authorized read root.",
        "QQL: Reading — Qirava DMS docs",
        "Reading data with QQL: filter, sort, limit, join, and aggregate over the Qirava read \
         roots.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/qql-writing
// ===========================================================================

pub fn respond_qql_writing(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Write statements are planned (L3), staged, and committed through a write-ahead log. \
             Write roots are FIFO and reserve resources before running, so a mutate is durable \
             and the hot path stays bounded. Each returns whether it committed and how many rows \
             it affected.",
        ))
        .child(toc.h2("Insert"))
        .child(p("Add new rows. A record literal uses `{ field: value }` syntax."))
        .child(copy_code("w-insert", &[
            CodeLine::Plain("INSERT INTO people { id: 1, name: \"Ada\", active: true }"),
        ]))
        .child(toc.h2("Update"))
        .child(p("Update the rows matching a filter, setting new field values."))
        .child(copy_code("w-update", &[
            CodeLine::Plain("UPDATE people SET active = false WHERE id = 1"),
        ]))
        .child(toc.h2("Upsert"))
        .child(p(
            "Insert-or-update by primary key: existing keys are updated, new keys are inserted.",
        ))
        .child(copy_code("w-upsert", &[
            CodeLine::Plain("UPSERT INTO people { id: 1, name: \"Ada Lovelace\", active: true }"),
        ]))
        .child(toc.h2("Delete"))
        .child(p("Delete the rows matching a filter."))
        .child(copy_code("w-delete", &[
            CodeLine::Plain("DELETE FROM people WHERE active = false"),
        ]))
        .child(Callout::warn(p(
            "Writes are deny-by-default at L3 and per-statement table RBAC. A non-admin API key \
             must hold a write or read_write grant on the target table, or the statement is \
             denied with forbidden before it reaches the WAL.",
        )).render())
        .child(toc.h2("Batches"))
        .child(p(
            "POST a JSON array of QQL statements to /api/qql/batch to run several in one request; \
             each statement is authorized independently per the deny-by-default table grant.",
        ))
        .child(copy_code("w-batch", &[
            CodeLine::Cmd("curl -X POST http://127.0.0.1:7179/api/qql/batch \\"),
            CodeLine::Plain("     -H \"X-API-Key: <api-key>\" \\"),
            CodeLine::Plain("     --data '[\"INSERT INTO people { name: \\\"Ada\\\" }\",\"INSERT INTO people { name: \\\"Bob\\\" }\"]'"),
        ]));

    render_doc(
        "/docs/dms/qql-writing",
        "Writing",
        "Mutate data with QQL: insert, update, upsert, and delete — planned, WAL-committed, and \
         deny-by-default authorized.",
        "QQL: Writing — Qirava DMS docs",
        "Writing data with QQL: insert, update, upsert, delete, and batches over the Qirava \
         FIFO write roots.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/qql-search
// ===========================================================================

pub fn respond_qql_search(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Full-text search scores rows against a text term and returns ranked hits. The \
             search group has two roots: score a term, and score-then-filter the hits.",
        ))
        .child(toc.h2("Score a full-text query"))
        .child(p(
            "The text root plans the query, scores matching rows against the term, and returns \
             ranked hits plus a count.",
        ))
        .child(copy_code("s-text", &[
            CodeLine::Plain("SELECT * FROM articles SEARCH \"distributed systems\""),
        ]))
        .child(toc.h2("Search with a post-filter"))
        .child(p(
            "The filter root scores the full-text hits, then applies a predicate filter to the \
             ranked results before returning them.",
        ))
        .child(copy_code("s-filter", &[
            CodeLine::Plain("SELECT * FROM articles SEARCH \"distributed systems\""),
            CodeLine::Plain("WHERE published = true"),
        ]))
        .child(Callout::note(p(
            "Both search roots return hits plus a count; the filter root takes an optional \
             predicate. See GET /api/spec for the exact field shapes.",
        )).render());

    render_doc(
        "/docs/dms/qql-search",
        "Search",
        "Full-text search with QQL: score a term and rank the hits, optionally post-filtering the \
         ranked results.",
        "QQL: Search — Qirava DMS docs",
        "Full-text search with QQL: scoring a term and post-filtering ranked hits over the \
         Qirava search roots.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/qql-graph
// ===========================================================================

pub fn respond_qql_graph(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Graph queries resolve a start node and walk edges. The graph group has three roots: \
             resolve a node, traverse adjacent edges, and resolve a path between two nodes.",
        ))
        .child(toc.h2("Resolve a node"))
        .child(p("Resolve the start node a query identifies, returning the matched node."))
        .child(copy_code("g-node", &[
            CodeLine::Plain("MATCH (p:person { id: 1 }) RETURN p"),
        ]))
        .child(toc.h2("Traverse edges"))
        .child(p(
            "Resolve the start node, then traverse adjacent edges to a depth, returning the \
             reached nodes plus a count. Depth is optional.",
        ))
        .child(copy_code("g-traverse", &[
            CodeLine::Plain("MATCH (p:person { id: 1 })-[:follows]->(f) RETURN f"),
        ]))
        .child(toc.h2("Resolve a path"))
        .child(p(
            "Resolve the start node, traverse edges, and return a path to a target node, plus a \
             count of the steps.",
        ))
        .child(copy_code("g-path", &[
            CodeLine::Plain("MATCH path = (a:person { id: 1 })-[:follows*]->(b:person { id: 9 })"),
            CodeLine::Plain("RETURN path"),
        ]))
        .child(Callout::note(p(
            "The node root returns a node; traverse returns nodes + count; path returns a path + \
             count. The live catalog at GET /api/spec documents each root's exact inputs and \
             outputs.",
        )).render());

    render_doc(
        "/docs/dms/qql-graph",
        "Graph",
        "Graph queries with QQL: resolve a node, traverse adjacent edges, and resolve a path \
         between two nodes.",
        "QQL: Graph — Qirava DMS docs",
        "Graph queries with QQL: node resolution, edge traversal, and pathfinding over the \
         Qirava graph roots.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/qql-vector
// ===========================================================================

pub fn respond_qql_vector(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Vector search returns the approximate nearest neighbors to a query vector. The \
             vector group has two roots: KNN, and filtered KNN.",
        ))
        .child(toc.h2("Nearest-neighbor search"))
        .child(p(
            "The KNN root plans the query and returns the approximate k nearest neighbors to the \
             request vector, plus a count. k is optional.",
        ))
        .child(copy_code("v-knn", &[
            CodeLine::Plain("SELECT id FROM docs NEAREST [0.12, 0.04, 0.98] LIMIT 10"),
        ]))
        .child(toc.h2("Filtered nearest-neighbor search"))
        .child(p(
            "The filtered root computes the nearest neighbors, then applies a predicate filter to \
             the candidates before returning them — useful for combining semantic similarity with \
             structured constraints.",
        ))
        .child(copy_code("v-filtered", &[
            CodeLine::Plain("SELECT id FROM docs NEAREST [0.12, 0.04, 0.98] LIMIT 10"),
            CodeLine::Plain("WHERE lang = \"en\""),
        ]))
        .child(Callout::note(p(
            "Both vector roots take a query, a vector (list of floats), and an optional k; the \
             filtered root adds an optional predicate. Both return hits plus a count. See GET \
             /api/spec for the exact shapes.",
        )).render());

    render_doc(
        "/docs/dms/qql-vector",
        "Vector",
        "Vector search with QQL: approximate nearest-neighbor search, and filtered KNN that \
         combines similarity with a predicate.",
        "QQL: Vector — Qirava DMS docs",
        "Vector search with QQL: approximate nearest-neighbor and filtered KNN over the Qirava \
         vector roots.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/api-spec
// ===========================================================================

pub fn respond_api_spec(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "The DMS describes its own API. Two endpoints serve a machine-readable catalog of \
             every callable function, built once over the live registry and cached — zero \
             per-request cost. The catalog documents the public surface only: keys, scopes, \
             inputs, outputs, and error codes. It never reads row data or reflects a request \
             body.",
        ))
        .child(toc.h2("GET /api/spec"))
        .child(p(
            "The native catalog. It JOINs each function's static metadata with the live registry \
             to report each root's real scope, origin, and FIFO flag, grouped by category \
             (read, write, search, graph, vector, inline, auth, planner).",
        ))
        .child(copy_code("spec-get", &[
            CodeLine::Cmd("curl http://127.0.0.1:7179/api/spec"),
        ]))
        .child(p("The document's top-level shape:"))
        .child(copy_code("spec-shape", &[
            CodeLine::Plain("{"),
            CodeLine::Plain("  \"qirava_api_version\": \"1\","),
            CodeLine::Plain("  \"envelope\": { \"ok\": {...}, \"error\": {...} },"),
            CodeLine::Plain("  \"error_codes\": [ { \"code\": \"...\", \"http\": 200 }, ... ],"),
            CodeLine::Plain("  \"categories\": ["),
            CodeLine::Plain("    { \"name\": \"read\", \"functions\": ["),
            CodeLine::Plain("      { \"key\": \"qql.read.key\", \"scope\": \"public\","),
            CodeLine::Plain("        \"origin\": \"builtin\", \"fifo\": false,"),
            CodeLine::Plain("        \"summary\": \"...\", \"description\": \"...\","),
            CodeLine::Plain("        \"input\": [...], \"output\": [...],"),
            CodeLine::Plain("        \"error_codes\": [\"bad_request\", ...] } ] } ]"),
            CodeLine::Plain("}"),
        ]))
        .child(toc.h2("GET /api/spec/openapi"))
        .child(p(
            "An OpenAPI 3.1 projection of the same surface, modeling the real HTTP routes (for \
             example /api/qql) with the error-code enum derived from the same single source. \
             Feed it to any OpenAPI-aware tool — a client generator, a docs viewer, a test \
             harness.",
        ))
        .child(copy_code("spec-openapi", &[
            CodeLine::Cmd("curl http://127.0.0.1:7179/api/spec/openapi"),
        ]))
        .child(Callout::tip(p(
            "Because the catalog is generated from the live registry, it can never drift from \
             what the DMS actually does. It is the source of truth for the function surface — \
             prefer it over any hand-written list.",
        )).render());

    render_doc(
        "/docs/dms/api-spec",
        "The self-describing API",
        "Two endpoints — GET /api/spec and GET /api/spec/openapi — serve a live, machine-readable \
         catalog of every callable function.",
        "The self-describing API — Qirava DMS docs",
        "The Qirava self-describing API: GET /api/spec and GET /api/spec/openapi serve a live \
         catalog of every callable function.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/api-envelope
// ===========================================================================

pub fn respond_api_envelope(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Every API response uses one uniform envelope, and every failure maps to one of \
             eight stable error codes. Both the envelope shape and the code table are published \
             in the self-describing catalog at GET /api/spec.",
        ))
        .child(toc.h2("The response envelope"))
        .child(p(
            "A success carries data and a null error; a failure carries a {code, message} error \
             and null data. Both carry a root block with timing and a request id.",
        ))
        .child(copy_code("env-ok", &[
            CodeLine::Comment("// success"),
            CodeLine::Plain("{ \"error\": null,"),
            CodeLine::Plain("  \"data\": <value>,"),
            CodeLine::Plain("  \"root\": { \"took_us\": <int>, \"request_id\": <string> } }"),
            CodeLine::Comment("// error"),
            CodeLine::Plain("{ \"error\": { \"code\": <stable_code>, \"message\": <string> },"),
            CodeLine::Plain("  \"data\": null,"),
            CodeLine::Plain("  \"root\": { \"took_us\": <int>, \"request_id\": <string> } }"),
        ]))
        .child(toc.h2("The eight error codes"))
        .child(p(
            "Each stable code maps to one HTTP status. This table is the single source of truth, \
             emitted from the executor's response-code mapping:",
        ))
        .child(bullets(&[
            ("ok (200) — ", "the request succeeded; data is present."),
            ("workers_busy (503) — ", "the worker/executor pool is saturated; retry later."),
            ("not_found (404) — ", "the requested public (or internal) function does not exist."),
            ("forbidden (403) — ", "access denied: failed auth, insufficient role, or a denied grant at L1/L2/L3."),
            ("bad_request (400) — ", "invalid input: a missing or malformed field, or an unplannable query."),
            ("rate_limited (429) — ", "a resource reservation was denied (e.g. a FIFO write under load)."),
            ("function_failed (422) — ", "the function ran but a step failed."),
            ("internal (500) — ", "an internal error or a panicked function."),
        ]))
        .child(Callout::note(p(
            "These eight codes are stable wire contract. Several internal executor variants \
             collapse onto one code (for example both function-not-found variants map to \
             not_found), so eight distinct codes cover every outcome.",
        )).render());

    render_doc(
        "/docs/dms/api-envelope",
        "Envelope & error codes",
        "The uniform response envelope (data or {code, message}) and the eight stable error \
         codes, each mapped to an HTTP status.",
        "Envelope & error codes — Qirava DMS docs",
        "The Qirava response envelope and the eight stable error codes (ok, workers_busy, \
         not_found, forbidden, bad_request, rate_limited, function_failed, internal).",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/dms/concepts
// ===========================================================================

pub fn respond_concepts(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Qirava's access model rests on one rule: nothing reaches the database except through a \
             worker, behind three authorization checkpoints — and the planner is the only door to \
             read or mutate.",
        ))
        .child(toc.h2("execute → worker → planner"))
        .child(p(
            "A function is reachable only via execute(), and only through a worker. There is no \
             other entry point. The flow for any request is:",
        ))
        .child(copy_code("c-flow", &[
            CodeLine::Plain("request → worker (L1 before-auth)"),
            CodeLine::Plain("        → execute() (L2 function scope)"),
            CodeLine::Plain("        → planner   (L3 db/table RBAC)"),
            CodeLine::Plain("        → read / mutate"),
        ]))
        .child(p(
            "Auth is a before-function: it authenticates the caller and writes their identity into \
             the shared context before any function runs. Every auth/RBAC scenario extends that \
             before-chain.",
        ))
        .child(toc.h2("The three checkpoints"))
        .child(bullets(&[
            ("L1 — before-auth: ", "the worker authenticates the caller (session or HMAC-signed key) before any function executes."),
            ("L2 — execute scope: ", "the executor checks the caller may invoke that function at all (public | all-apps | system-only | owner)."),
            ("L3 — the planner: ", "QQL-level RBAC = app-scope ∩ principal-grant gates the actual read or mutate at plan time. This is the only write path."),
        ]))
        .child(Callout::warn(p(
            "Never add a write path that skips L3. The planner is the single chokepoint for every \
             read and mutate — bypassing it breaks the security model.",
        )).render())
        .child(toc.h2("Configuration is data"))
        .child(p(
            "Roles, routes, and policies live in _sys_* tables, not in code. The default admin app — \
             Qirava Studio — is itself a DMS client with no special backdoor; it goes through the \
             same three checkpoints as any caller.",
        ))
        .child(toc.h2("One executor governs all work"))
        .child(p(
            "Governance/RBAC, KMS, the database, the workers, and replication are all functions \
             behind a single bounded executor (qexec). That one chokepoint is what makes the \
             security and performance guarantees auditable: there is exactly one place work flows \
             through.",
        ));

    render_doc(
        "/docs/dms/concepts",
        "Core concepts",
        "The execute → worker → planner path and the three authorization checkpoints that gate \
         every read and mutate.",
        "Core concepts — Qirava DMS docs",
        "The Qirava access model: execute → worker → planner, and the three authorization \
         checkpoints (L1/L2/L3).",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill — Quill Overview (landing seed)
// ===========================================================================

pub fn respond_quill(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(toc.h2("What Quill is"))
        .child(p(
            "Quill is a Rust-native, zero-dependency UI/app framework: shadcn-like styled components \
             with Next.js-like authoring. It does native SSR plus islands and SSG/ISR, with no \
             server-side JavaScript. This very site is a Quill app, dogfooding the framework end to \
             end.",
        ))
        .child(Callout::tip(p(
            "Quill does not depend on the DMS or the q* stdlib. It is its own product — these are \
             its docs; the DMS has a separate set.",
        )).render())
        .child(toc.h2("The authoring model"))
        .child(p(
            "A Quill app is a plain Rust binary. You write pages that render HTML on the server, and \
             opt into interactivity per-component via islands. Scaffold, serve, and export:",
        ))
        .child(copy_code("quill-cli", &[
            CodeLine::Cmd("quill new myapp && cd myapp"),
            CodeLine::Cmd("cargo run"),
            CodeLine::Comment("# serves SSR HTML on a worker"),
            CodeLine::Cmd("cargo run -- build"),
            CodeLine::Comment("# exports a static dist/ that serves with no DMS"),
        ]))
        .child(toc.h2("What ships to the browser"))
        .child(bullets(&[
            ("Zero JS by default — ", "a page with no islands ships no JavaScript at all."),
            ("Per-page bundles — ", "a page with islands ships the ~4 KB signal/hydration core plus only the behaviors that page uses."),
            ("Styled components — ", "Navbar, Card, Button, Badge, Stat, Table and more, theme-token driven, over headless state machines."),
        ]));

    render_doc(
        "/docs/quill",
        "Quill",
        "The Rust-native UI/app framework: SSR + islands, zero server-side JavaScript, and a \
         per-page client bundle that ships only what a page uses.",
        "Quill docs",
        "Quill documentation: the Rust-native UI/app framework — SSR, islands, and styled \
         components.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/installation
// ===========================================================================

pub fn respond_quill_installation(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Quill is a set of Rust crates, not a downloadable framework runtime. There is nothing \
             to npm-install: an app depends on the qquill-* crates by path and builds with cargo \
             alone. The only prerequisite is a recent stable Rust toolchain and git.",
        ))
        .child(toc.h2("Get the source"))
        .child(p(
            "Quill lives in the Qirava monorepo as a sibling of the DMS. Clone it with --recursive \
             so the submodules come along; the qquill-* crates are under qquill/.",
        ))
        .child(copy_code("qi-clone", &[
            CodeLine::Comment("# clone the workspace with its submodules"),
            CodeLine::Cmd("git clone --recursive https://github.com/qirava/qirava"),
            CodeLine::Cmd("cd qirava"),
        ]))
        .child(toc.h2("Build the quill CLI"))
        .child(p(
            "The scaffolder lives at qquill/qquill-cli. Build it once, then put the binary on your \
             PATH (or call it by path). It bakes in your checkout location at build time, so the \
             apps it generates already point their path dependencies at the right place — no manual \
             editing.",
        ))
        .child(copy_code("qi-cli", &[
            CodeLine::Comment("# build the `quill` binary once"),
            CodeLine::Cmd("cargo build -p qquill-cli"),
            CodeLine::Comment("# -> qquill/qquill-cli/target/debug/quill — add it to PATH"),
        ]))
        .child(Callout::note(p(
            "Quill depends on neither the DMS nor the q* stdlib — every qquill-* crate is std-only \
             and depends only on other Quill crates. Zero third-party dependencies; the shipped \
             client runtime is hand-written.",
        )).render())
        .child(toc.h2("The crates"))
        .child(p(
            "An app rarely touches every crate, but it helps to know the shape. qquill-view is the \
             root (the Node tree + the view!{} macro + the HTML renderer); the rest layer on top:",
        ))
        .child(bullets(&[
            ("qquill-view — ", "the view core: the Node tree, the view!{} macro, the streaming HTML renderer. The root crate."),
            ("qquill-style — ", "the CSS compiler: a co-located style block becomes a deterministic, escaped CSS string for the SSR head."),
            ("qquill-theme — ", "the design-token system: typed tokens emitted as --q-* custom properties, with light/dark/contrast modes."),
            ("qquill-ui / qquill-design — ", "the styled component library and the design system that composes icons + style + theme + ui + view."),
            ("qquill-signal / qquill-runtime — ", "the reactive signal core and the hand-written, zero-import client runtime that hydrates islands."),
            ("qquill-build — ", "the static-export and per-page bundling helpers; qquill-docs is the docs-site primitives this page is built with."),
        ]));

    render_doc(
        "/docs/quill/installation",
        "Installation",
        "Quill is Rust crates, not a runtime download — clone the workspace and build the quill \
         CLI. No third-party dependencies to install.",
        "Installation — Quill docs",
        "Install Quill: clone the Qirava workspace and build the quill CLI. Std and first-party \
         crates only; zero third-party dependencies.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/quickstart
// ===========================================================================

pub fn respond_quill_quickstart(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "With the quill CLI built (see Installation), you can go from an empty directory to a \
             running, server-rendered app in two commands. This page scaffolds an app, runs it, and \
             shows what ships to the browser.",
        ))
        .child(toc.h2("Scaffold an app"))
        .child(p(
            "From the root of your Qirava checkout (the directory holding qdms/ and qquill/), run \
             quill new with an app name:",
        ))
        .child(copy_code("qq-new", &[
            CodeLine::Cmd("quill new myapp"),
            CodeLine::Comment("# Created Quill app `myapp`."),
            CodeLine::Comment("# Next steps: cd myapp && cargo run"),
        ]))
        .child(toc.h2("Run it"))
        .child(p(
            "Build and serve. The first build compiles the engine and the Quill crates; after that \
             the app boots and prints the URLs it serves.",
        ))
        .child(copy_code("qq-run", &[
            CodeLine::Cmd("cd myapp && cargo run"),
            CodeLine::Plain("myapp serving:"),
            CodeLine::Plain("  http://127.0.0.1:7179/"),
            CodeLine::Plain("  http://127.0.0.1:7179/counter"),
        ]))
        .child(p(
            "Open http://127.0.0.1:7179/. The home page is pure server-rendered HTML — view source \
             and there is no <script> tag at all. Open /counter and the button works: the server \
             rendered a correct static fallback, and the runtime hydrated just that one component.",
        ))
        .child(Callout::tip(p(
            "The starter runs in memory (Qdb::new()), so there is nothing to clean up. To persist \
             across restarts, swap main.rs to Qdb::open(path).",
        )).render())
        .child(toc.h2("What ships to the browser"))
        .child(p(
            "Quill is server-first. A page with no islands ships zero JavaScript; a page that uses \
             an island ships a per-page bundle — the signal/hydration core plus only the behaviors \
             that page declares, not a whole-app runtime.",
        ))
        .child(bullets(&[
            ("Zero JS by default — ", "the home page above ships no script at all."),
            ("Per-page bundles — ", "/counter ships the hand-written runtime carrying only the counter behavior."),
            ("Identical bytes — ", "served HTML and exported HTML go through the one render path, so they are byte-for-byte the same."),
        ]))
        .child(toc.h2("Where to next"))
        .child(bullets(&[
            ("Project structure — ", "what quill new generated: the PAGES list, the routes, and the render path."),
            ("The view macro — ", "author the UI as a Node tree with view!{}."),
            ("Islands — ", "add interactivity that hydrates in place on its trigger."),
        ]));

    render_doc(
        "/docs/quill/quickstart",
        "Quickstart",
        "From empty directory to a running app in two commands: quill new myapp, cargo run, open \
         http://127.0.0.1:7179.",
        "Quickstart — Quill docs",
        "Quill quickstart: scaffold an app with quill new, run it with cargo run, and open it on \
         127.0.0.1:7179.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/project-structure
// ===========================================================================

pub fn respond_quill_project_structure(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "A Quill app is a plain Rust binary. quill new lays down a small, legible tree: a boot \
             file that owns the route list, one module per page, and a theme. There is no bundler \
             config and no node_modules.",
        ))
        .child(toc.h2("The generated tree"))
        .child(copy_code("ps-tree", &[
            CodeLine::Plain("myapp/"),
            CodeLine::Plain("├── Cargo.toml          # path deps into your Qirava checkout (zero third-party)"),
            CodeLine::Plain("└── src/"),
            CodeLine::Plain("    ├── main.rs         # boot: open db, register the PAGES list, serve"),
            CodeLine::Plain("    └── app/"),
            CodeLine::Plain("        ├── mod.rs      # document() shell + respond_html() (the island rule)"),
            CodeLine::Plain("        ├── theme.rs    # design tokens + base CSS — re-skin here"),
            CodeLine::Plain("        └── routes/"),
            CodeLine::Plain("            ├── mod.rs    # the route-module list"),
            CodeLine::Plain("            ├── index.rs  # GET /        — a static, zero-JS page"),
            CodeLine::Plain("            └── counter.rs# GET /counter — one hydrating island"),
        ]))
        .child(toc.h2("PAGES: the one source of truth"))
        .child(p(
            "main.rs holds the PAGES array — the single list of every route. Each entry is an id, a \
             URL path, and the handler that renders it. Both serve and build walk this one list, so \
             what is served and what is exported can never drift.",
        ))
        .child(copy_code("ps-pages", &[
            CodeLine::Plain("const PAGES: &[Page] = &["),
            CodeLine::Plain("    Page { id: \"index\",   path: \"/\",        handler: app::routes::index::respond },"),
            CodeLine::Plain("    Page { id: \"counter\", path: \"/counter\", handler: app::routes::counter::respond },"),
            CodeLine::Plain("];"),
        ]))
        .child(toc.h2("A route module"))
        .child(p(
            "Each page is one module under app/routes/, exporting pub fn respond(_input: &[u8]) -> \
             FunctionResponse. It builds a Node tree and returns it through the app's render path.",
        ))
        .child(copy_code("ps-route", &[
            CodeLine::Plain("use qexec::FunctionResponse;"),
            CodeLine::Plain("use qquill_view::{view, Node};"),
            CodeLine::Plain("use crate::app::{document, respond_html};"),
            CodeLine::Plain(""),
            CodeLine::Plain("fn page() -> Node {"),
            CodeLine::Plain("    view! { main { h1 { \"About\" } p .lead { \"Added by hand.\" } } }"),
            CodeLine::Plain("}"),
            CodeLine::Plain(""),
            CodeLine::Plain("pub fn respond(_input: &[u8]) -> FunctionResponse {"),
            CodeLine::Plain("    respond_html(&document(\"About\", Node::Fragment(vec![]), page()))"),
            CodeLine::Plain("}"),
        ]))
        .child(toc.h2("Adding a page"))
        .child(p(
            "Three edits, then restart cargo run: write the route module, declare it in \
             routes/mod.rs, and add one line to PAGES. That is the entire contract — the route is \
             then live and (because it is in PAGES) exported too.",
        ))
        .child(Callout::note(p(
            "app/mod.rs holds document() (wraps a body in the full HTML shell with the theme \
             inlined) and respond_html() (renders to bytes and — only if the tree contains an \
             island — injects the props sidecar plus the one runtime <script>). That conditional \
             injection is the island rule: no island, no JavaScript.",
        )).render());

    render_doc(
        "/docs/quill/project-structure",
        "Project structure",
        "What quill new generates: a plain Rust binary whose PAGES list is the one source of truth \
         for routes, one module per page, and a theme.",
        "Project structure — Quill docs",
        "The structure of a Quill app: the PAGES list in main.rs, one route module per page, the \
         document()/respond_html() shell, and the theme.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/view-macro
// ===========================================================================

pub fn respond_quill_view_macro(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Every Quill UI is a Node tree. You can build it with the declarative view!{} macro or \
             with the chainable builder API (el / text / raw / fragment) — they produce the same \
             tree, and the same renderer streams it to HTML on the server.",
        ))
        .child(toc.h2("The Node tree"))
        .child(p(
            "A Node is one of: an Element (a tag with attributes and children), Text (escaped), Raw \
             (trusted, not escaped), a Fragment (a list of nodes), an Island (an interactive \
             subtree, see Islands), or a Slot. The renderer walks the tree and emits correct, \
             semantic HTML, escaping text with a hand-written escaper.",
        ))
        .child(toc.h2("The view! macro"))
        .child(p(
            "view!{} reads like markup: a tag, optional .class and attribute shorthands, then a \
             braced child block. Text is a string literal; expressions interpolate.",
        ))
        .child(copy_code("vm-view", &[
            CodeLine::Plain("use qquill_view::{view, Node};"),
            CodeLine::Plain(""),
            CodeLine::Plain("fn card(title: &str) -> Node {"),
            CodeLine::Plain("    view! {"),
            CodeLine::Plain("        article .card {"),
            CodeLine::Plain("            h2 { (title) }"),
            CodeLine::Plain("            p .muted { \"A server-rendered card.\" }"),
            CodeLine::Plain("        }"),
            CodeLine::Plain("    }"),
            CodeLine::Plain("}"),
        ]))
        .child(toc.h2("The builder API"))
        .child(p(
            "The same tree, built by chaining. el(tag) starts an element; .class / .attr set \
             attributes; .child / .children append; text() and raw() make leaves. This is what the \
             docs site itself uses for fine-grained control.",
        ))
        .child(copy_code("vm-builder", &[
            CodeLine::Plain("use qquill_view::{el, text, Node};"),
            CodeLine::Plain(""),
            CodeLine::Plain("fn card(title: &str) -> Node {"),
            CodeLine::Plain("    el(\"article\").class(\"card\")"),
            CodeLine::Plain("        .child(el(\"h2\").child(text(title.to_string())))"),
            CodeLine::Plain("        .child(el(\"p\").class(\"muted\").child(text(\"A server-rendered card.\")))"),
            CodeLine::Plain("}"),
        ]))
        .child(Callout::warn(p(
            "text() escapes its content; raw() does NOT — it emits the string verbatim. Only ever \
             pass raw() trusted, author-controlled markup (e.g. an inline SVG), never user input.",
        )).render())
        .child(toc.h2("Rendering"))
        .child(p(
            "render(&node) returns the HTML string. In an app you rarely call it directly — the \
             generated respond_html() renders the document tree for you and frames the HTTP \
             response. The same render path runs whether the page is served live or exported, so \
             the bytes are identical.",
        ));

    render_doc(
        "/docs/quill/view-macro",
        "The view macro",
        "Every UI is a Node tree. Build it with the declarative view!{} macro or the chainable \
         builder API — both stream to escaped, semantic HTML on the server.",
        "The view macro — Quill docs",
        "The Quill view layer: the Node tree, the view!{} macro and the el/text/raw builder API, \
         and the server-side HTML renderer.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/components
// ===========================================================================

pub fn respond_quill_components(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Quill ships a library of styled components — built on the view core, driven by --q-* \
             design tokens, layered over headless state machines. Static components render to plain \
             HTML; interactive ones expose an .island() method that turns the same component into a \
             hydrating island.",
        ))
        .child(Callout::tip(el("span").children([
            text("Every component has a live page with its rendered demo and source. Browse the "),
            el("a").attr("href", "/components").child(text("component showcase")),
            text(" — the interactive ones (Dialog, Menu, Tooltip…) run a real island right on the page."),
        ])).render())
        .child(toc.h2("Static components"))
        .child(p(
            "Most components are pure SSR: construct them and drop them into a tree, and they render \
             to semantic, token-styled HTML with zero JavaScript. Navbar, Card, Button, Badge, \
             Stat, Table, List, Divider, Breadcrumb, and more.",
        ))
        .child(copy_code("co-static", &[
            CodeLine::Plain("use qquill_design::Button;"),
            CodeLine::Plain(""),
            CodeLine::Plain("// a styled button — renders to HTML, ships no JS"),
            CodeLine::Plain("Button::new(\"Save changes\")"),
        ]))
        .child(toc.h2("Interactive components"))
        .child(p(
            "Components whose behavior needs the client (Dialog, Menu, Tooltip, Tabs, Checkbox, \
             Switch, Accordion) render a correct static fallback for SSR, then expose .island(id) \
             to hydrate that subtree in place. The headless state machine is shared between the \
             server fallback and the client behavior.",
        ))
        .child(copy_code("co-island", &[
            CodeLine::Plain("use qquill_design::{Dialog, Effect};"),
            CodeLine::Plain("use qquill_view::el; use qquill_view::text;"),
            CodeLine::Plain(""),
            CodeLine::Plain("let body = el(\"p\").child(text(\"This surface is focus-trapped while open.\"));"),
            CodeLine::Plain("Dialog::modal(\"demo-dialog\", \"Delete project?\", body)"),
            CodeLine::Plain("    .effect(Effect::Elevated)"),
            CodeLine::Plain("    .island(\"demo-dialog-island\", \"Open dialog\")"),
        ]))
        .child(toc.h2("Variants, tones, and effects"))
        .child(p(
            "Components compose typed enums rather than ad-hoc strings. Tone (Brand, Neutral, \
             Danger…) drives color; Effect (Flat, Glass, Neumorphic, Gradient) drives the surface \
             treatment. Because they all read the same --q-* tokens, restyling is a token change, \
             not a per-component edit — see Theming.",
        ));

    render_doc(
        "/docs/quill/components",
        "Components",
        "The styled component library: token-driven, over headless state machines. Static \
         components render to plain HTML; interactive ones hydrate with .island().",
        "Components — Quill docs",
        "Quill components: token-driven styled components over headless state machines, with .island() \
         for interactive ones. See the live showcase.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/theming
// ===========================================================================

pub fn respond_quill_theming(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Quill's look is driven entirely by design tokens. qquill-theme emits typed tokens as \
             --q-* CSS custom properties; every component reads those variables, so re-skinning the \
             whole app, flipping to dark mode, or changing density is a token change — no \
             per-component edits, no reflow.",
        ))
        .child(toc.h2("Tokens are --q-* variables"))
        .child(p(
            "Color, type scale, spacing, radius, shadows, and motion are all tokens. They land in \
             the document as custom properties, and components reference them — for example a card's \
             background is var(--q-color-surface), its border var(--q-color-border).",
        ))
        .child(copy_code("th-tokens", &[
            CodeLine::Comment("/* a few of the emitted --q-* custom properties */"),
            CodeLine::Plain("--q-color-brand: ...;   --q-color-surface: ...;"),
            CodeLine::Plain("--q-color-fg: ...;      --q-color-border: ...;"),
            CodeLine::Plain("--q-radius-lg: ...;     --q-font-weight-medium: ...;"),
        ]))
        .child(toc.h2("Dark mode and modes"))
        .child(p(
            "The theme system ships light, dark, and high-contrast modes. The tokens re-point per \
             mode, and a tiny no-flicker boot script sets the mode before first paint so there is no \
             flash of the wrong theme. The site header's theme toggle drives exactly this.",
        ))
        .child(toc.h2("Surface styles"))
        .child(p(
            "Beyond color, a surface can carry a visual treatment via the data-q-surface attribute. \
             Each mode re-points a small set of --q-surf-* variables, so the same element markup \
             gets a different finish:",
        ))
        .child(bullets(&[
            ("flat — ", "the default: a solid surface fill with a hairline border and no shadow."),
            ("glass — ", "glassmorphism: a translucent fill plus backdrop blur (with a solid fallback where backdrop-filter is unsupported)."),
            ("neu — ", "neumorphism: paired light/dark shadows for a soft extruded surface, with an inset state when pressed."),
            ("gradient — ", "a brand gradient fill with on-brand foreground text."),
        ]))
        .child(copy_code("th-surface", &[
            CodeLine::Comment("<!-- drop a surface style onto any element -->"),
            CodeLine::Plain("<article data-q-surface=\"glass\"> … </article>"),
        ]))
        .child(Callout::note(p(
            "Styles compile to a compact CSS string at build time (via qquill-style) and inline \
             into the SSR <head> — there is no runtime style cost and no flash of unstyled content.",
        )).render());

    render_doc(
        "/docs/quill/theming",
        "Theming",
        "The whole look is design tokens: qquill-theme emits --q-* custom properties, with \
         light/dark/contrast modes and flat/glass/neu/gradient surfaces.",
        "Theming — Quill docs",
        "Theming in Quill: --q-* design tokens, dark mode with a no-flicker boot, and the \
         flat/glass/neu/gradient surface styles.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/islands
// ===========================================================================

pub fn respond_quill_islands(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "An island is a self-contained interactive component. The server renders a correct \
             static fallback (so it works with JavaScript disabled), and the client runtime \
             hydrates just that subtree on its declared trigger. A page with no islands ships no \
             runtime at all; a page with one ships only that behavior.",
        ))
        .child(toc.h2("Signals"))
        .child(p(
            "State inside an island is held in signals. The signal core (signal / computed / \
             effect) tracks dependencies fine-grained, batches updates via a microtask flush, and \
             is glitch-free: an effect re-runs only when a signal it actually read changes. An \
             effect is how a signal value gets written back into the DOM.",
        ))
        .child(toc.h2("The runtime"))
        .child(p(
            "The client runtime is a single hand-written, zero-import JavaScript bundle — there is \
             no React, no framework download. It is injected only on pages that contain at least \
             one island, and it is the entire client footprint. Its pieces:",
        ))
        .child(bullets(&[
            ("the signal core — ", "fine-grained, batched, glitch-free reactivity with effect cleanup."),
            ("the triggers — ", "OnLoad, OnVisible (IntersectionObserver), OnInteraction (first pointer/focus/key), and OnIdle (requestIdleCallback)."),
            ("the registry — ", "a behavior registry keyed by data-q-kind; register(kind, factory) wires a behavior."),
            ("hydration — ", "scans [data-q-island], reads each island's props sidecar, and hydrates it in place on its trigger."),
        ]))
        .child(toc.h2("Hydration triggers"))
        .child(p(
            "Each island declares WHEN to hydrate. The trigger renders to a data-q-trigger \
             attribute the runtime matches on:",
        ))
        .child(bullets(&[
            ("Load — ", "hydrate as soon as the runtime boots."),
            ("Visible — ", "hydrate when the island scrolls into view (IntersectionObserver)."),
            ("Interaction — ", "hydrate on the first pointer, focus, or key event on the island."),
            ("Idle — ", "hydrate when the main thread is idle."),
        ]))
        .child(toc.h2("The SSR/client contract"))
        .child(p(
            "An island carries an instance id (unique per page), a behavior kind (the data-q-kind \
             the runtime looks up), a trigger, a props JSON string, and the static fallback. At \
             render time Quill emits the fallback plus a <script type=\"application/qq\"> props \
             sidecar; on hydration the matching behavior re-instantiates from those props and binds \
             to the existing markup via data-q-bind / data-q-action attributes.",
        ))
        .child(Callout::tip(p(
            "Because the props are serialized into the page and the behavior re-instantiates from \
             them, the static fallback and the hydrated component agree — there is no double-render \
             flash. Next: build one end to end.",
        )).render());

    render_doc(
        "/docs/quill/islands",
        "Islands",
        "Interactive components that hydrate in place: signals for state, a hand-written zero-import \
         runtime, and four hydration triggers — injected only where a page needs them.",
        "Islands — Quill docs",
        "Quill islands: the signal core, the hand-written client runtime, the load/visible/interaction/idle \
         triggers, and the SSR-to-client hydration contract.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/building-an-island
// ===========================================================================

pub fn respond_quill_building_an_island(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "This walks through the reference island — a counter — end to end: the static fallback \
             the server renders, the props it carries, and how it ships. It is the pattern every \
             island follows.",
        ))
        .child(toc.h2("Render the static fallback"))
        .child(p(
            "First build the HTML the server renders — it must work with JavaScript disabled. The \
             data-q-bind and data-q-action attributes are the contract the SSR markup and the \
             client behavior share: data-q-bind marks the node a signal writes to, data-q-action \
             marks an element that triggers behavior.",
        ))
        .child(copy_code("bi-fallback", &[
            CodeLine::Plain("use qquill_view::{el, text, Node};"),
            CodeLine::Plain(""),
            CodeLine::Plain("let fallback = el(\"div\").class(\"counter\")"),
            CodeLine::Plain("    .child(el(\"output\").attr(\"data-q-bind\", \"count\")"),
            CodeLine::Plain("        .child(text(start.to_string())))"),
            CodeLine::Plain("    .child(el(\"button\").attr(\"type\", \"button\")"),
            CodeLine::Plain("        .attr(\"data-q-action\", \"inc\").child(text(\"+1\")));"),
        ]))
        .child(toc.h2("Wrap it as an island"))
        .child(p(
            "Call island(instance_id, kind, trigger, props, fallback). instance_id is unique per \
             page; kind is the data-q-kind the runtime's behavior registry matches; props is a JSON \
             string the behavior re-instantiates from on hydration.",
        ))
        .child(copy_code("bi-island", &[
            CodeLine::Plain("use qquill_view::{island, Trigger};"),
            CodeLine::Plain(""),
            CodeLine::Plain("let props = format!(\"{{\\\"start\\\":{start}}}\");"),
            CodeLine::Plain("// (instance id, behavior kind, trigger, props JSON, fallback)"),
            CodeLine::Plain("island(\"counter-1\", \"counter\", Trigger::Load, props, fallback)"),
        ]))
        .child(p(
            "Many shipped components build this for you: an interactive component's .island(id) \
             method constructs the fallback, kind, and props internally — you only pass the \
             instance id (and any per-instance label).",
        ))
        .child(toc.h2("The client behavior"))
        .child(p(
            "A behavior is registered by kind with register(kind, factory). The counter behavior is \
             a pure-signal proof: a signal holds the count, the data-q-action button increments it, \
             and an effect writes the value into the existing data-q-bind <output>. The reference \
             behaviors (counter, dialog, menu, tooltip, …) ship with the runtime.",
        ))
        .child(toc.h2("Ship it"))
        .child(p(
            "Drop the island into a page body and return it through respond_html as usual. Because \
             the tree contains an island, respond_html automatically appends the props sidecar and \
             the single runtime <script> — carrying only the behaviors this page uses. No island, \
             no script.",
        ))
        .child(Callout::warn(p(
            "Keep each instance_id unique within a page, and make sure the kind matches a \
             registered behavior — those two strings are how hydration finds and wires the right \
             component.",
        )).render());

    render_doc(
        "/docs/quill/building-an-island",
        "Building an island",
        "The counter, end to end: render the static fallback, wrap it with island(), and ship it — \
         respond_html injects the props sidecar and runtime only because the tree has an island.",
        "Building an island — Quill docs",
        "Build a Quill island end to end: the static fallback, the island() call with kind and \
         props, the client behavior, and automatic per-page runtime injection.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/quill/static-export
// ===========================================================================

pub fn respond_quill_static_export(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "A Quill app serves the same HTML whether a DMS is running or not — every page goes \
             through the one render path. So you can render every route once, at build time, and \
             write the result to disk as plain files. The output needs no database, no worker, no \
             socket: just a CDN.",
        ))
        .child(toc.h2("Build a static site"))
        .child(p(
            "From inside the app directory, quill build renders every page in-process and writes a \
             CDN-ready dist/. It is a thin wrapper around cargo run --release -- build; pass an \
             output dir to override the default.",
        ))
        .child(copy_code("se-build", &[
            CodeLine::Cmd("quill build"),
            CodeLine::Comment("# -> ./dist  (or: cargo run --release -- build)"),
            CodeLine::Cmd("quill build public_site"),
            CodeLine::Comment("# -> ./public_site"),
        ]))
        .child(p("You see exactly what it wrote:"))
        .child(copy_code("se-out", &[
            CodeLine::Plain("Exported 2 page(s) to dist/"),
            CodeLine::Plain("  dist/index.html"),
            CodeLine::Plain("  dist/counter/index.html"),
            CodeLine::Plain("Copied 1 asset(s) from public/"),
            CodeLine::Plain("Deploy dist/ to Cloudflare Pages — it serves with no DMS running."),
        ]))
        .child(toc.h2("What's in dist/"))
        .child(p(
            "Routes map to pretty URLs: / becomes index.html, /counter becomes counter/index.html. \
             Each index.html is the complete document; everything under public/ is copied verbatim. \
             The bytes on disk are byte-identical to what the live server sends.",
        ))
        .child(bullets(&[
            ("_headers — ", "one stanza per page with its Cache-Control, plus an immutable year-long rule for content-hashed /assets/*. Cloudflare Pages reads it verbatim."),
            ("404.html — ", "served automatically for any unmatched path."),
            ("_redirects — ", "a documented placeholder; a pure static export needs no rules, but you can append your own."),
        ]))
        .child(Callout::note(p(
            "Dynamic routes (/users/:id, /files/*rest) are skipped — a static file tree has no \
             params — and reported as skipped dynamic route. Serve and build walk the same PAGES \
             list, so they can never drift.",
        )).render())
        .child(toc.h2("Deploy to Cloudflare Pages"))
        .child(p(
            "dist/ is a self-contained static site. Direct upload builds locally and uploads the \
             directory as-is; a Git-connected build runs the build command on Cloudflare and serves \
             the output directory.",
        ))
        .child(copy_code("se-deploy", &[
            CodeLine::Comment("# direct upload with Wrangler"),
            CodeLine::Cmd("quill build"),
            CodeLine::Cmd("npx wrangler pages deploy dist"),
        ]))
        .child(p(
            "For a Git-connected build, set the Build command to cargo run --release -- build and \
             the Build output directory to dist. Quill writes the Pages control files \
             (_headers, _redirects, 404.html) for you, so no extra config is needed. There is no \
             server process — the CDN serves the plain files Quill exported.",
        ));

    render_doc(
        "/docs/quill/static-export",
        "Static export & deploy",
        "quill build renders every route to a CDN-ready dist/ — pretty URLs, _headers, and a 404, \
         byte-identical to the live server. Deploy it to Cloudflare Pages with no DMS running.",
        "Static export & deploy — Quill docs",
        "Static-export a Quill app with quill build to a CDN-ready dist/, then deploy to Cloudflare \
         Pages (direct upload or Git-connected) with no server running.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/stdlib — q* stdlib Overview (landing seed)
// ===========================================================================

pub fn respond_stdlib(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(toc.h2("What the q* stdlib is"))
        .child(p(
            "The q* stdlib is a set of zero-dependency crates shared across every Qirava product. \
             The substrate is qexec (the bounded executor) and qvalue (the value model + ABI); the \
             rest are focused utility crates.",
        ))
        .child(toc.h2("The substrate"))
        .child(bullets(&[
            ("qexec — ", "the bounded executor every product runs work through; the one chokepoint."),
            ("qvalue — ", "the value model and ABI shared across functions and workers."),
        ]))
        .child(toc.h2("Utility crates"))
        .child(p(
            "Alongside the substrate are focused, dependency-free utilities: array, object, string, \
             math, number, convert, crypto, encoding, regex, time, and uuid.",
        ))
        .child(Callout::note(p(
            "Direction rule: products (DMS, Quill) depend on q*; q* never depends on a product. \
             The dependency arrow points one way.",
        )).render());

    render_doc(
        "/docs/stdlib",
        "The q* stdlib",
        "The zero-dependency crates shared across every product: the qexec executor and qvalue \
         model, plus focused utilities.",
        "The q* stdlib docs",
        "Documentation for the q* stdlib: the qexec executor, the qvalue model, and the utility \
         crates shared across every Qirava product.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/stdlib/substrate
// ===========================================================================

pub fn respond_stdlib_substrate(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Two crates underpin everything else in the stdlib. qexec is the bounded executor and \
             function runtime every product runs work through; qvalue is the shared value model and \
             binary ABI that functions and workers speak. Every other q* crate depends on exactly \
             these two — and nothing more.",
        ))
        .child(toc.h2("qexec — the bounded executor"))
        .child(p(
            "qexec is a thread-backed worker pool plus a scoped function registry. It derives its \
             resource budgets from the device's real capacities (a default 80% safety budget over \
             the raw CPU/memory/storage totals), caps load per worker, and gives FIFO routes \
             affinity so ordered work stays ordered. It is the one chokepoint all work flows \
             through — which is what makes the security and performance properties auditable.",
        ))
        .child(bullets(&[
            ("The worker pool — ", "thread-backed, with per-worker load caps and resource budgets derived from device capacities (default 80% of the raw totals; it never exceeds physical capacity)."),
            ("The function registry — ", "scoped, holding built-in and user-defined functions; any function package can register against it WITHOUT depending on a database."),
            ("execute + chain — ", "the single entry point that runs a registered function (and the before/after chains), catching panics so a failing function cannot take down a worker."),
            ("FIFO affinity — ", "ordered routes get a dedicated lane so sequential work (e.g. writes) preserves order under concurrency."),
        ]))
        .child(p(
            "Device capacities are raw totals, not budgets: qexec applies the safety percentage on \
             top. It can scan the host itself (CPU threads always resolve; memory and storage are \
             detected where the platform supports it and left unbounded otherwise, so it degrades \
             gracefully instead of guessing), or take user-supplied values.",
        ))
        .child(Callout::tip(p(
            "Because qexec registers functions without knowing about any database, the same executor \
             serves the DMS, this Quill site's render handlers, and the q* utility functions — one \
             runtime, many products.",
        )).render())
        .child(toc.h2("qvalue — the value model + ABI"))
        .child(p(
            "qvalue is the shared structured-data contract. Functions and workers exchange args and \
             results as qvalue Records, encoded with qvalue's binary codec, so every package speaks \
             the same shapes without depending on the DB. It has two modules:",
        ))
        .child(bullets(&[
            ("model — ", "the dynamic Value/Record type and its binary codec (encode_record / decode_record). Value covers Null, Bool, Int, Float, Str, Bytes, Timestamp, List, Map, plus the exact-number variants."),
            ("bignum — ", "arbitrary-precision BigInt and BigDecimal, so exact arithmetic (the number.* crate) never rounds to a fixed range."),
        ]))
        .child(toc.h2("The call convention"))
        .child(p(
            "Every utility crate follows one convention built on qvalue: positional args arrive as \
             a Record keyed \"0\", \"1\", \"2\", … and the result comes back as a Record with a \
             single \"result\" field. That uniformity is what lets QQL call any of them inline as \
             string.upper(...), math.sqrt(...), or number.add(...).",
        ))
        .child(copy_code("sub-conv", &[
            CodeLine::Comment("// the shared convention (qvalue Records, qexec FunctionResponse)"),
            CodeLine::Plain("input : Record { \"0\": <Value>, \"1\": <Value>, ... }   // positional args"),
            CodeLine::Plain("output: Record { \"result\": <Value> }                   // single result"),
        ]));

    render_doc(
        "/docs/stdlib/substrate",
        "The substrate: qexec + qvalue",
        "qexec is the bounded executor + function registry every product runs work through; qvalue \
         is the value model and binary ABI that functions and workers speak.",
        "The substrate: qexec + qvalue — q* stdlib docs",
        "The q* stdlib substrate: qexec (the bounded executor and scoped function registry) and \
         qvalue (the shared Value/Record model, binary codec, and bignum).",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/stdlib/utilities
// ===========================================================================

pub fn respond_stdlib_utilities(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "On top of the substrate sit eleven focused utility crates. Each is an isolated function \
             package: it registers a family of public built-ins (array.*, string.*, …) on a qexec \
             runtime, speaks the shared qvalue call convention, and depends only on qexec + qvalue. \
             Because they register as ordinary functions, QQL can call any of them inline.",
        ))
        .child(toc.h2("Data structures"))
        .child(bullets(&[
            ("qarray — ", "array.* over a Value::List: len, first, last, reverse, and more — the list toolbox."),
            ("qobject — ", "object.* over a Value::Map: keys, values, get, merge — the map counterpart to qarray."),
            ("qstring — ", "string.* text ops: upper, lower, trim, reverse, len, and friends, all Unicode-aware."),
        ]))
        .child(toc.h2("Numbers"))
        .child(p(
            "Two number crates, a deliberate split between speed and exactness:",
        ))
        .child(bullets(&[
            ("qmath — ", "math.* fast f64 math (sqrt, pow, trig, …). Fast and fixed-range."),
            ("qnumber — ", "number.* EXACT arbitrary-precision arithmetic over qvalue's BigInt/BigDecimal — any-length integers, decimals, scientific input. Where math.* rounds, number.* never does."),
            ("qconvert — ", "convert.* cross-type casts (to string / int / float / bool …): exact where it can be via BigDecimal, lenient where it must be."),
        ]))
        .child(Callout::tip(p(
            "Reach for qmath when speed matters and f64 precision is fine; reach for qnumber when \
             you cannot afford to round — money, large integers, exact decimals.",
        )).render())
        .child(toc.h2("Bytes & matching"))
        .child(bullets(&[
            ("qcrypto — ", "crypto.* hashing written from scratch: SHA-256 and HMAC-SHA-256, returned as lowercase hex. (SHA-1 is included ONLY for the RFC 6455 WebSocket handshake — it is broken and must never be used for signatures.)"),
            ("qencoding — ", "encoding.* base64 and hex codecs, implemented from scratch (RFC 4648). String/Bytes in, encoded Str out, and back."),
            ("qregex — ", "regex.* a compact regex engine from scratch: recursive-descent parser → bytecode → backtracking VM. Classes, anchors, alternation, groups, greedy/lazy and counted quantifiers. test, find, replace."),
        ]))
        .child(toc.h2("Time & identity"))
        .child(bullets(&[
            ("qtime — ", "time.* clock + arithmetic: now, now_ms, add_secs, returning integer timestamps."),
            ("quuid — ", "uuid.* id generation from scratch: v4 (random) and v7 (time-ordered, ideal for DB keys), with entropy from /dev/urandom and a time-seeded fallback."),
        ]))
        .child(Callout::note(p(
            "qcrypto, qencoding, qregex, and quuid are all implemented from scratch — zero \
             third-party crates. That is the project's hard rule: std and first-party only.",
        )).render());

    render_doc(
        "/docs/stdlib/utilities",
        "Utility crates",
        "Eleven focused, dependency-free packages — array, object, string, math, number, convert, \
         crypto, encoding, regex, time, uuid — each registering inline-callable built-ins.",
        "Utility crates — q* stdlib docs",
        "The q* stdlib utility crates: qarray, qobject, qstring, qmath, qnumber, qconvert, qcrypto, \
         qencoding, qregex, qtime, quuid — each an isolated, zero-dependency function package.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/stdlib/dependency-rule
// ===========================================================================

pub fn respond_stdlib_dependency_rule(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(p(
            "Two rules keep the stdlib clean and the dependency graph legible: the arrow points one \
             way, and cryptography hides behind a trait. Both exist so the core stays portable and \
             nothing accidentally couples a shared crate to a product.",
        ))
        .child(toc.h2("The one-way arrow"))
        .child(p(
            "Products depend on q*; q* NEVER depends on a product. The DMS and other consumers pull \
             in the stdlib crates — but no q* crate may reach back into the DMS, into Quill, or into \
             a product's code. Within the stdlib the graph is just as strict: every utility crate \
             depends only on qexec + qvalue, never on another utility crate.",
        ))
        .child(copy_code("dr-arrow", &[
            CodeLine::Plain("products (DMS, Quill, …)"),
            CodeLine::Plain("        │  depend on"),
            CodeLine::Plain("        ▼"),
            CodeLine::Plain("q* utility crates  ──►  qexec + qvalue   (and nothing else)"),
            CodeLine::Comment("# the arrow never points back up"),
        ]))
        .child(Callout::warn(p(
            "Do not put product code in a q* crate, and do not make a q* crate depend on a product. \
             A shared crate that knows about a product is no longer shared — it has become product \
             code in the wrong place.",
        )).render())
        .child(toc.h2("Crypto behind a trait"))
        .child(p(
            "Zero third-party dependencies has exactly one exception: cryptography. The crypto \
             primitives the stdlib needs (SHA-256, HMAC-SHA-256) are implemented from scratch in \
             qcrypto, and where a swappable backend is wanted it is kept behind a Crypto trait. That \
             seam means a future hardware or audited backend can drop in WITHOUT a lock-in — the \
             rest of the code depends on the trait, not a vendor.",
        ))
        .child(bullets(&[
            ("From scratch — ", "qcrypto's SHA-256 and HMAC-SHA-256 are hand-written (no ring, no openssl); SHA-1 exists only for the mandated WebSocket handshake."),
            ("Behind a trait — ", "the abstraction point keeps the option open to swap in an audited or hardware-accelerated backend later, without rewriting callers."),
            ("Everything else — ", "std and first-party crates only. Shipped JavaScript is hand-written and zero-import too."),
        ]))
        .child(Callout::note(p(
            "These are project-wide invariants, recorded in AGENTS.md. They are not stylistic \
             preferences — they are the contract that keeps the two pillars (security and \
             performance) auditable.",
        )).render());

    render_doc(
        "/docs/stdlib/dependency-rule",
        "The dependency rule",
        "Two invariants: the dependency arrow points one way (products → q*, never back), and \
         cryptography is the lone third-party exception, kept behind a trait.",
        "The dependency rule — q* stdlib docs",
        "The q* stdlib invariants: the one-way dependency arrow (products depend on q*, q* never \
         depends on a product) and crypto kept behind a trait as the sole dependency exception.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/cloud — Qirava Cloud Overview (landing seed)
// ===========================================================================

pub fn respond_cloud(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(Callout::warn(p(
            "Qirava Cloud is designed, not yet built. These docs describe the intended shape; \
             expect them to firm up as the control plane lands.",
        )).render())
        .child(toc.h2("What Qirava Cloud will be"))
        .child(p(
            "Qirava Cloud is the planned managed-DMS service: you subscribe, pick your resources, \
             and get your OWN isolated DMS — your own custodian governance, your own databases, your \
             own Studio UI. The Cloud is only the control plane (subscriptions, billing, server \
             allocation, scaling); the tenant owns and governs the DMS inside it.",
        ))
        .child(Callout::tip(p(
            "The model in one line: \"a DMS that manages other DMSes.\" The control plane is itself \
             a Qirava DMS running a cloud app — the same way Studio is the DMS's own admin app. It \
             does not fork the engine; it reuses it.",
        )).render())
        .child(toc.h2("One isolated DMS per tenant"))
        .child(p(
            "Each tenant is a completely separate DMS instance — its own process, its own \
             write-ahead log, its own _sys_* tables, its own seed, and its own custodian > admin > \
             user > guest hierarchy. A tenant never sees the control plane or any sibling tenant; \
             the control plane never reaches into a tenant's data. It allocates resources and \
             manages lifecycle only.",
        ))
        .child(bullets(&[
            ("Same governance everywhere — ", "a tenant DMS uses the identical custodian governance as any self-hosted DMS; there is no special multi-tenant schema."),
            ("Two authority domains — ", "the Cloud's operators govern plans, nodes, caps, and billing; a tenant's custodian governs its data and users. They never cross."),
            ("Your own Studio — ", "tenants sign in to their own Studio UI, exactly like a standalone install."),
        ]))
        .child(toc.h2("The open-core model"))
        .child(p(
            "The Apache-2.0 core is the whole data system — engine, packages, and Studio. Qirava \
             Cloud is the commercial managed layer atop it. The core never depends on the cloud, so \
             you can self-host the DMS indefinitely without it. The control plane is documented in \
             the open repo so the contracts it relies on (resource caps, RBAC, isolation seams) are \
             designed correctly in the core, not bolted on.",
        ))
        .child(toc.h2("Status"))
        .child(p(
            "Planned. What the control plane will orchestrate is already built and tested in the \
             core — the in-process resource budget, RBAC and governance, config-as-data, \
             replication, the worker/function model. The missing top layer (metering, billing, \
             OS-level caps, placement, sandboxing) is the planned work.",
        ));

    render_doc(
        "/docs/cloud",
        "Qirava Cloud",
        "The planned managed, multi-tenant control plane — a DMS that manages other DMSes. \
         Designed, not yet built.",
        "Qirava Cloud docs",
        "Qirava Cloud documentation: the planned managed, multi-tenant control plane atop the \
         Apache-2.0 core.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/cloud/plans-and-resources
// ===========================================================================

pub fn respond_cloud_plans(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(Callout::warn(p(
            "Planned. This page describes the intended subscription model; the slider, billing, and \
             mode switching are not yet built.",
        )).render())
        .child(p(
            "Qirava Cloud has no fixed tier catalog. Instead of picking a plan from a list, you \
             choose your resources directly — the pricing slider is the plan. You pay per unit you \
             consume (CPU thread, GB memory, GB storage, GB bandwidth), with a live quote as you \
             move the slider.",
        ))
        .child(toc.h2("Storage: dynamic or fixed"))
        .child(p(
            "When you provision a tenant you pick how its storage behaves. Both are first-class; \
             neither is a downgrade of the other:",
        ))
        .child(bullets(&[
            ("Dynamic auto-scale — ", "storage grows with usage and is billed by what you actually store (GB-month). Overflow is billed, not blocked — your tenant never hits a wall mid-write."),
            ("Fixed size — ", "a set storage cap you provision up front, for predictable cost. A minimal tenant can even run disk-only with a tiny footprint and still be high-throughput."),
        ]))
        .child(toc.h2("Mode: standalone or cluster"))
        .child(p(
            "A tenant runs in one of two modes, and — crucially — you can switch EITHER DIRECTION at \
             any time. The on-disk storage layout is identical across both modes, so the switch is \
             non-destructive in both directions: there is no dump-and-restore, no data loss.",
        ))
        .child(bullets(&[
            ("Standalone — ", "a single DMS instance: one process, one WAL. The default; everything a single-node install gives you."),
            ("Cluster — ", "the same tenant with followers replicating the master, placed on distinct nodes for availability."),
        ]))
        .child(copy_code("cp-switch", &[
            CodeLine::Comment("# the switch is non-destructive in BOTH directions"),
            CodeLine::Plain("standalone  ──add followers──►  cluster"),
            CodeLine::Plain("cluster     ──drop followers─►  standalone"),
            CodeLine::Comment("# identical on-disk layout means no migration, no data loss"),
        ]))
        .child(Callout::note(p(
            "Bidirectional, non-destructive standalone ↔ cluster is a hard requirement of the \
             design — you are never locked into the mode you started with.",
        )).render())
        .child(toc.h2("What you pay for"))
        .child(p(
            "Pricing is per unit, metered, with no fixed plan. The dimensions:",
        ))
        .child(bullets(&[
            ("CPU thread — ", "per thread-hour, against your slider-set cap."),
            ("Memory — ", "per GB-hour, against your slider-set cap."),
            ("Storage — ", "per GB-month, usage-based when dynamic; overflow is billed, not blocked."),
            ("Bandwidth — ", "per GB per month, metered egress/ingress."),
        ]));

    render_doc(
        "/docs/cloud/plans-and-resources",
        "Plans & resources",
        "No fixed tiers — the pricing slider is the plan. Choose storage (dynamic auto-scale or \
         fixed) and mode (standalone or cluster, switchable both directions non-destructively).",
        "Plans & resources — Qirava Cloud docs",
        "Qirava Cloud plans and resources: per-unit pricing via a slider, dynamic or fixed storage, \
         and standalone/cluster mode switchable in both directions non-destructively.",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/cloud/scaling
// ===========================================================================

pub fn respond_cloud_scaling(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(Callout::warn(p(
            "Planned. The scaling behaviors below describe the intended control plane; the placer, \
             autoscaler, and rebalancer are not yet built.",
        )).render())
        .child(p(
            "Qirava Cloud scales in two dimensions. Vertical scaling grows a single tenant on its \
             node; horizontal scaling grows the fleet — more tenants, rebalanced across nodes, and \
             bigger clusters. Both preserve every isolation invariant on every move.",
        ))
        .child(toc.h2("Vertical: grow a tenant live"))
        .child(p(
            "When you raise a tenant's CPU, memory, or storage cap, the control plane signals the \
             RUNNING DMS to start using the newly allocated resources live. The DMS re-reads its cap \
             and expands its qexec budget (and storage) in place — no rebuild, no restart, no \
             downtime.",
        ))
        .child(copy_code("sc-vertical", &[
            CodeLine::Plain("slider ↑  →  control plane raises the tenant's cap"),
            CodeLine::Plain("          →  signal the running DMS"),
            CodeLine::Plain("          →  DMS re-reads cap, expands qexec budget LIVE"),
            CodeLine::Comment("# no rebuild, no restart, no downtime"),
        ]))
        .child(Callout::tip(p(
            "The in-process resource budget this relies on is already built and tested in the core — \
             qexec derives and enforces the cap today. The planned piece is the live signal that \
             tells a running tenant its cap changed.",
        )).render())
        .child(toc.h2("Horizontal: grow the fleet"))
        .child(p(
            "Horizontal scaling is about the fleet, not one tenant's box. It has three moves:",
        ))
        .child(bullets(&[
            ("Admit more tenants — ", "the placer bin-packs a new tenant onto a node with headroom for its cap (threads, memory, storage, projected bandwidth)."),
            ("Rebalance — ", "as nodes are added or drained, tenants move across nodes — via the replication path (promote a follower on the target, drain the source) — always preserving isolation."),
            ("Grow a cluster — ", "a tenant in cluster mode gains more nodes/replicas as its data and load grow; cluster members are placed on distinct nodes."),
        ]))
        .child(toc.h2("Fleet auto-scale"))
        .child(p(
            "The fleet itself auto-scales: when aggregate headroom across the nodes drops below a \
             threshold, a node is added and the rebalancer spreads tenants onto it. Every rebalance \
             move must preserve all the isolation invariants — separate process, WAL, _sys_*, seed, \
             and the operator-vs-tenant authority split — so growing the fleet never weakens a \
             tenant's boundary.",
        ));

    render_doc(
        "/docs/cloud/scaling",
        "Scaling",
        "Two dimensions: vertical signals the running DMS to use newly allocated resources live; \
         horizontal admits more tenants, rebalances across nodes, and grows clusters.",
        "Scaling — Qirava Cloud docs",
        "Qirava Cloud scaling: vertical (signal the running DMS to expand its cap live) and \
         horizontal (more tenants, rebalance across nodes, grow a cluster).",
        css,
        content,
        toc,
    )
}

// ===========================================================================
// /docs/cloud/architecture
// ===========================================================================

pub fn respond_cloud_architecture(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(Callout::warn(p(
            "Planned. This is the target architecture for the control plane; it is designed in the \
             open so the OSS core's contracts are correct, but it is not yet built.",
        )).render())
        .child(p(
            "The control plane is a CONTROL PLANE ONLY: it provisions, places, scales, rebalances, \
             meters, and bills tenant DMSes — and it never touches tenant data. It is itself a \
             Qirava DMS running a cloud app, so it reuses the engine, the governance hierarchy, and \
             the worker/function model; it adds only the cloud.* orchestration functions, the _cp_* \
             catalogs, and the Cloud Console UI.",
        ))
        .child(toc.h2("A DMS that manages other DMSes"))
        .child(p(
            "The cloud manager is not a separate special service. It is a Qirava DMS — the same \
             engine a tenant runs — loaded with a cloud app, the way Studio is the DMS's own admin \
             app. The control DMS holds a \"cloud\" database; each tenant is its own DMS process \
             elsewhere.",
        ))
        .child(copy_code("ca-shape", &[
            CodeLine::Plain("CONTROL DMS  = qdms engine + cloud app"),
            CodeLine::Plain("  data db:   \"cloud\" → _cp_tenants / _cp_nodes / _cp_usage / ..."),
            CodeLine::Plain("  functions: cloud.provision · place · meter · bill · rebalance · suspend"),
            CodeLine::Plain("  UI:        the Cloud Console (a Quill app, like Studio)"),
            CodeLine::Plain("        │ orchestrates: provision / move / set-cap / meter"),
            CodeLine::Plain("        ▼"),
            CodeLine::Plain("  Tenant A DMS    Tenant B DMS    Tenant C DMS (master → follower)"),
            CodeLine::Comment("# each tenant = its OWN process: own WAL · _sys_* · seed · custodian"),
        ]))
        .child(toc.h2("Two authority domains, never crossing"))
        .child(p(
            "The cloud's custodian/admin govern plans, nodes, customers, caps, billing, placement, \
             and migration — and give a tenant only its resource cap plus hosting and metering. A \
             tenant's own custodian/admin/user/guest govern its data, users, RBAC, and mode. The \
             control plane never sees inside a tenant's data or governance; a tenant never sees the \
             control plane or sibling tenants. It is the same L1 → L2 → L3 gate the core already \
             enforces.",
        ))
        .child(Callout::note(p(
            "Control plane ≠ tenant authority. The operator domain is entirely separate from every \
             tenant's custodian hierarchy — that separation is a core security invariant, not a \
             cloud add-on.",
        )).render())
        .child(toc.h2("The _cp_* catalogs"))
        .child(p(
            "The control plane keeps its own catalogs in the control DMS's store — distinct from any \
             tenant's _sys_* tables. They are how it tracks the fleet without ever reading tenant \
             data:",
        ))
        .child(bullets(&[
            ("_cp_tenants — ", "each tenant's owner, plan (standalone/cluster), resource cap, status, and node."),
            ("_cp_nodes — ", "each bare-metal node's capacity, what is allocated, and its status."),
            ("_cp_usage — ", "per-tenant metered counters: thread-seconds, memory GB-hours, storage GB, bandwidth GB."),
            ("_cp_invoices — ", "billing periods, line items, totals."),
            ("_cp_audit — ", "every operator action (provision/scale/rebalance/suspend/terminate) and its result."),
        ]))
        .child(toc.h2("Confidential VM (when the hardware allows)"))
        .child(p(
            "A DMS cannot build its own secure VM, so the confidential VM is set up by the infra \
             layer BEFORE any tenant DMS exists; the tenant DMS then boots inside it and ATTESTS — \
             proving its launch measurement to the seed authority to receive its seed into RAM only, \
             never on disk. Where the hardware lacks SEV-SNP, the DMS honestly reports the \
             software-hardened tier instead; it cannot claim attestation it does not have.",
        ));

    render_doc(
        "/docs/cloud/architecture",
        "Architecture",
        "The control plane is a DMS that manages other DMSes — it provisions, places, scales, and \
         meters tenant DMSes via the _cp_* catalogs, and never touches tenant data.",
        "Architecture — Qirava Cloud docs",
        "Qirava Cloud architecture: a control plane that is itself a DMS running a cloud app, the \
         _cp_* catalogs, two non-crossing authority domains, and confidential-VM attestation.",
        css,
        content,
        toc,
    )
}
