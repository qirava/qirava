//! `GET /architecture` — an animated explainer of the Qirava access model and
//! submodule map.
//!
//! Two visuals, both pure CSS (theme-token colored, zero images) and both
//! wrapped in the shared `reveal` island so their parts fade/slide in on scroll:
//!
//!   1. The THREE checkpoints (L1 worker before-auth → L2 execute function
//!      scope → L3 the planner, the only db door) laid over the
//!      execute → worker → planner request flow.
//!   2. The FIVE-submodule map (qpkgs / qquill / qdms / qirava / qcloud) plus
//!      the brand crate (qbrand).
//!
//! Content is accurate to `AGENTS.md` and `docs/ARCHITECTURE_OVERVIEW.md`: the
//! one entry point (`execute`, only through a worker), auth as a before-function
//! writing identity into the shared context, the planner as the single
//! read/mutate door, and the one-way dependency arrow (products depend on tq*;
//! tq* never depends on a product; Quill does not depend on tq*).

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::{reveal, section};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Architecture — Qirava";
const DESCRIPTION: &str =
    "How Qirava enforces access: three ordered checkpoints (worker before-auth, execute \
     function scope, the planner), the execute → worker → planner request flow, and the \
     five-submodule map.";

const LEAD: &str = "Every read or mutate in Qirava passes the same three checkpoints, in the \
same order. A function is reachable only through execute(), only via a worker — there is no \
other entry point, and no write path skips the planner.";

// ---------------------------------------------------------------------------
// Checkpoint 1: the three gates over the request flow
// ---------------------------------------------------------------------------

/// One checkpoint card: its rank label (L1/L2/L3), title, the subsystem it
/// lives in, and a one-line description of what it enforces.
struct Checkpoint {
    rank: &'static str,
    title: &'static str,
    where_: &'static str,
    detail: &'static str,
}

const CHECKPOINTS: &[Checkpoint] = &[
    Checkpoint {
        rank: "L1",
        title: "Worker before-auth",
        where_: "functions/auth · before-chain",
        detail: "Authentication runs as a before-function: it verifies a session or an \
                 HMAC-signed key and writes the caller's identity into the shared context. \
                 Every auth/RBAC scenario extends this chain.",
    },
    Checkpoint {
        rank: "L2",
        title: "Execute function scope",
        where_: "execute() · per-function",
        detail: "The executor checks the target function's declared scope — public, all-apps, \
                 system-only, or owner — against the authenticated caller before any handler runs.",
    },
    Checkpoint {
        rank: "L3",
        title: "The planner",
        where_: "engine · QQL plan",
        detail: "Database and table RBAC is enforced in the planner as app-scope ∩ \
                 principal-grant. The planner is the only door to read or mutate; no write \
                 path is allowed to skip it.",
    },
];

/// Build the checkpoint flow: the request pipeline (request → worker → execute →
/// planner → tables/WAL) rendered as a stepped flow, with the three checkpoint
/// cards beneath calling out where each gate sits.
fn checkpoints(css: &mut Css) -> Node {
    css.push(diagram_css().to_string());

    // The horizontal request pipeline. Each stage is a node; gate stages are
    // tagged so CSS can badge them with their L-rank.
    let stage = |label: &str, sub: &str, gate: Option<&str>| -> Node {
        let mut n = el("div").class("q-arch-stage").attr("data-q-reveal", "");
        if let Some(g) = gate {
            n = n.attr("data-gate", g.to_string()).child(
                el("span").class("q-arch-stage__gate").child(text(g.to_string())),
            );
        }
        n.child(el("span").class("q-arch-stage__label").child(text(label.to_string())))
            .child(el("span").class("q-arch-stage__sub").child(text(sub.to_string())))
    };

    let arrow = || el("span").class("q-arch-arrow").attr("aria-hidden", "true").child(text("→"));

    let flow = el("div")
        .class("q-arch-flow")
        .attr("role", "list")
        .attr("aria-label", "Request flow")
        .child(stage("Request", "browser · SDK · peer", None))
        .child(arrow())
        .child(stage("Worker", "before → handle → after", Some("L1")))
        .child(arrow())
        .child(stage("execute()", "bounded executor", Some("L2")))
        .child(arrow())
        .child(stage("Planner", "QQL plan → execute", Some("L3")))
        .child(arrow())
        .child(stage("Tables + WAL", "storage", None));

    // The three checkpoint cards, in order.
    let mut cards = el("div").class("q-arch-gates");
    for cp in CHECKPOINTS {
        cards = cards.child(
            el("div")
                .class("q-arch-gate")
                .attr("data-q-reveal", "")
                .child(
                    el("div")
                        .class("q-arch-gate__head")
                        .child(el("span").class("q-arch-gate__rank").child(text(cp.rank.to_string())))
                        .child(el("h3").class("q-arch-gate__title").child(text(cp.title.to_string()))),
                )
                .child(el("p").class("q-arch-gate__where").child(text(cp.where_.to_string())))
                .child(el("p").class("q-arch-gate__detail").child(text(cp.detail.to_string()))),
        );
    }

    let body = el("div")
        .class("q-arch-checkpoints")
        .child(flow)
        .child(cards);

    reveal("arch-checkpoints", body)
}

// ---------------------------------------------------------------------------
// Checkpoint 2: the five-submodule map
// ---------------------------------------------------------------------------

/// One module tile in the submodule map.
struct Module {
    crate_name: &'static str,
    name: &'static str,
    kind: &'static str,
    blurb: &'static str,
    /// `built` | `planned` | `brand` — selects the tile accent.
    state: &'static str,
}

const MODULES: &[Module] = &[
    Module {
        crate_name: "qpkgs",
        name: "Stdlib",
        kind: "shared packages",
        blurb: "The zero-dependency tq* stdlib — qexec (the bounded executor) and qvalue (the \
                value model) plus focused utility crates. Products depend on it; it depends on \
                no product.",
        state: "built",
    },
    Module {
        crate_name: "qquill",
        name: "Quill",
        kind: "UI framework",
        blurb: "The Rust-native UI/app framework: styled components over headless state machines, \
                native SSR + islands + SSG. Its own repo, a sibling to the DMS — and it does not \
                depend on tq*.",
        state: "built",
    },
    Module {
        crate_name: "qdms",
        name: "DMS + Studio",
        kind: "data system",
        blurb: "The product: src holds exactly functions/ and workers/. Governance, KMS, the \
                database, jobs, and replication are function groups; Studio, the default admin \
                app, ships under workers/apps/.",
        state: "built",
    },
    Module {
        crate_name: "qirava",
        name: "Website",
        kind: "this site",
        blurb: "The marketing + docs site, a Quill app that dogfoods the framework end to end. \
                Serves over a worker, or exports a static dist/ that runs with no DMS.",
        state: "built",
    },
    Module {
        crate_name: "qcloud",
        name: "Cloud",
        kind: "control plane",
        blurb: "The planned managed, multi-tenant control plane — a DMS that manages other \
                DMSes, metered per resource. Designed, not yet built.",
        state: "planned",
    },
];

const BRAND_MODULE: Module = Module {
    crate_name: "qbrand",
    name: "Brand",
    kind: "single source",
    blurb: "The one source of brand assets — the ink-Q mark, lockups, and favicon. Every \
            product pulls its mark from here; no hand-copied SVGs.",
    state: "brand",
};

/// A single module tile.
fn module_tile(m: &Module) -> Node {
    el("div")
        .class("q-arch-mod")
        .attr("data-state", m.state)
        .attr("data-q-reveal", "")
        .child(
            el("div")
                .class("q-arch-mod__head")
                .child(el("code").class("q-arch-mod__crate").child(text(m.crate_name.to_string())))
                .child(el("span").class("q-arch-mod__kind").child(text(m.kind.to_string()))),
        )
        .child(el("h3").class("q-arch-mod__name").child(text(m.name.to_string())))
        .child(el("p").class("q-arch-mod__blurb").child(text(m.blurb.to_string())))
}

/// The five-submodule map plus the brand crate, and the one-way dependency rule.
fn submodules(css: &mut Css) -> Node {
    css.push(diagram_css().to_string());

    let mut grid = el("div").class("q-arch-modgrid");
    for m in MODULES {
        grid = grid.child(module_tile(m));
    }
    grid = grid.child(module_tile(&BRAND_MODULE));

    let rule = el("p")
        .class("q-arch-rule")
        .attr("data-q-reveal", "")
        .children([
            el("span").class("q-arch-rule__arrow").attr("aria-hidden", "true").child(text("→")),
            text(" Direction rule: products (DMS, Quill) depend on tq*; tq* never depends on a \
                   product, and Quill does not depend on tq*. The dependency arrow points one way."
                .to_string()),
        ]);

    let body = el("div").class("q-arch-modwrap").child(grid).child(rule);
    reveal("arch-submodules", body)
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

fn intro() -> Node {
    el("div")
        .child(el("p").class("q-eyebrow").child(text("Architecture".to_string())))
        .child(el("h1").class("q-h1").child(text("Three checkpoints, one door".to_string())))
        .child(el("p").class("q-lead").child(text(LEAD.to_string())))
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();

    let gates = section(
        Some("The access model"),
        "Three ordered checkpoints",
        "A request crosses each gate in turn: the worker authenticates (L1), execute() enforces \
         the function's scope (L2), and the planner enforces table-level RBAC (L3). Skip none.",
        checkpoints(&mut css),
    );

    let map = section(
        Some("The codebase"),
        "Five submodules, one ecosystem",
        "qroot composes five products plus the brand crate. Each is its own repo; products share \
         the tq* substrate, and the dependency arrow only ever points toward it.",
        submodules(&mut css),
    );

    let content = el("main")
        .class("q-main")
        .id("main")
        .child(intro())
        .child(gates)
        .child(map);

    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/architecture" };
    page(&meta, css, content)
}

// ---------------------------------------------------------------------------
// Diagram CSS (theme-token colored; pushed once, deduped by the Css accumulator)
// ---------------------------------------------------------------------------

/// Companion CSS for both diagrams. Uses only theme tokens so it flips with the
/// light/dark theme automatically. The `[data-q-reveal]` start/end states are
/// owned by the reveal island's own sheet; these rules only style the visuals.
fn diagram_css() -> &'static str {
    "\
.q-arch-checkpoints{margin-top:.5rem}\
.q-arch-flow{display:flex;flex-wrap:wrap;align-items:stretch;gap:.5rem;padding:1.25rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-xl);background:var(--q-color-surface)}\
.q-arch-stage{position:relative;flex:1 1 8.5rem;display:flex;flex-direction:column;gap:.2rem;padding:.85rem .9rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-bg)}\
.q-arch-stage[data-gate]{border-color:color-mix(in srgb,var(--q-color-brand) 55%,var(--q-color-border));box-shadow:0 0 0 1px color-mix(in srgb,var(--q-color-brand) 22%,transparent) inset}\
.q-arch-stage__gate{position:absolute;top:-.65rem;left:.85rem;font-size:.7rem;font-weight:var(--q-font-weight-bold);letter-spacing:.04em;color:var(--q-color-on-brand);background:var(--q-color-brand);border-radius:var(--q-radius-full);padding:.1rem .5rem;line-height:1.3}\
.q-arch-stage__label{font-weight:var(--q-font-weight-bold);font-size:.98rem;color:var(--q-color-fg)}\
.q-arch-stage__sub{font-size:.8rem;color:var(--q-color-muted)}\
.q-arch-arrow{display:flex;align-items:center;color:var(--q-color-muted);font-size:1.1rem;flex:0 0 auto}\
@media (max-width:720px){.q-arch-flow{flex-direction:column}.q-arch-arrow{transform:rotate(90deg);justify-content:center}}\
.q-arch-gates{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,16rem),1fr));gap:1rem;margin-top:1.5rem}\
.q-arch-gate{display:flex;flex-direction:column;gap:.4rem;padding:1.1rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface)}\
.q-arch-gate__head{display:flex;align-items:center;gap:.6rem}\
.q-arch-gate__rank{font-weight:var(--q-font-weight-bold);font-size:.78rem;color:var(--q-color-on-brand);background:var(--q-color-brand);border-radius:var(--q-radius-md);padding:.15rem .45rem;line-height:1.3}\
.q-arch-gate__title{margin:0;font-size:1.05rem;color:var(--q-color-fg)}\
.q-arch-gate__where{margin:0;font-size:.8rem;font-family:var(--q-font-mono,monospace);color:var(--q-color-brand)}\
.q-arch-gate__detail{margin:0;font-size:.92rem;color:var(--q-color-muted)}\
.q-arch-modwrap{margin-top:.5rem}\
.q-arch-modgrid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,16rem),1fr));gap:1rem}\
.q-arch-mod{display:flex;flex-direction:column;gap:.45rem;padding:1.1rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);border-top:3px solid var(--q-color-brand)}\
.q-arch-mod[data-state=\"planned\"]{border-top-color:var(--q-color-muted);border-top-style:dashed}\
.q-arch-mod[data-state=\"brand\"]{border-top-color:color-mix(in srgb,var(--q-color-brand) 60%,var(--q-color-fg))}\
.q-arch-mod__head{display:flex;align-items:baseline;justify-content:space-between;gap:.6rem;flex-wrap:wrap}\
.q-arch-mod__crate{font-size:.92rem;color:var(--q-color-fg);font-weight:var(--q-font-weight-bold)}\
.q-arch-mod__kind{font-size:.74rem;text-transform:uppercase;letter-spacing:.07em;color:var(--q-color-muted)}\
.q-arch-mod__name{margin:0;font-size:1.1rem;color:var(--q-color-fg)}\
.q-arch-mod__blurb{margin:0;font-size:.92rem;color:var(--q-color-muted)}\
.q-arch-rule{display:flex;gap:.5rem;align-items:flex-start;margin-top:1.5rem;padding:.95rem 1.1rem;border:1px solid var(--q-color-border);border-left:3px solid var(--q-color-brand);border-radius:var(--q-radius-md);background:var(--q-color-surface);color:var(--q-color-muted);font-size:.92rem}\
.q-arch-rule__arrow{color:var(--q-color-brand);font-weight:var(--q-font-weight-bold)}"
}
