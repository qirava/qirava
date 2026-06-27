//! `GET /architecture` — the Architecture **Overview**, and the entry to the
//! architecture section (the single source of truth for Qirava's system design).
//!
//! Rendered in the shared `.q-docs` reading layout (sidebar + article + on-page
//! TOC) so it feels identical to the developer docs. The page carries:
//!
//!   1. A short framing + a hub of the four deep-dive pages
//!      (security, cloud, cluster, embed).
//!   2. The THREE checkpoints (L1 worker before-auth → L2 execute function
//!      scope → L3 the planner, the only db door) over the request flow.
//!   3. The submodule map (qpkgs / qquill / qdms / qirava / qcloud + qbrand)
//!      and the one-way dependency rule.
//!
//! Both visuals are pure CSS (theme-token colored, zero images), each wrapped in
//! the shared `reveal` island so their parts fade/slide in on scroll.

use qexec::FunctionResponse;
use qquill_view::{el, raw, text, Node};

use crate::app::arch_kit::{self, ARCH};
use crate::app::docs_kit::Toc;
use crate::app::routes::product_page::ARROW_SVG;
use crate::app::routes::reveal;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Architecture — Qirava";
const DESCRIPTION: &str =
    "The single source of truth for Qirava's design: the three access checkpoints, the module \
     map, and deep dives into security & governance, the managed-cloud control plane, the \
     cluster/replication mechanism, and the embedded/sync reach.";

const LEAD: &str = "This section is the single source of truth for how Qirava is built. It starts \
with the access model every read and mutate obeys, maps the modules, and then goes deep on the \
four subsystems that carry the most design weight.";

const INTRO: &str = "Two pillars run through everything here: security and performance. The access \
model is uniform — a function is reachable only through execute(), only via a worker, and no write \
path skips the planner — and the managed cloud is layered on top without ever weakening it: the \
cloud controls availability, the tenant controls the data.";

// ---------------------------------------------------------------------------
// Deep-dive hub: a card per architecture page (everything after the overview).
// ---------------------------------------------------------------------------

fn deep_dives() -> Node {
    let mut grid = el("div").class("q-arch-hub");
    for a in ARCH.iter().filter(|a| a.path != "/architecture") {
        grid = grid.child(
            el("a")
                .class("q-arch-hub__card")
                .attr("href", a.path)
                .child(el("p").class("q-arch-hub__sec").child(text(a.section.to_string())))
                .child(el("h3").class("q-arch-hub__title").child(text(a.title.to_string())))
                .child(el("p").class("q-arch-hub__sum").child(text(a.summary.to_string())))
                .child(
                    el("span")
                        .class("q-prod-learn")
                        .child(text("Read ".to_string()))
                        .child(raw(ARROW_SVG)),
                ),
        );
    }
    grid
}

fn hub_css() -> &'static str {
    "\
.q-arch-hub{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,15rem),1fr));gap:1rem;margin:1.25rem 0 .5rem}\
.q-arch-hub__card{display:flex;flex-direction:column;gap:.35rem;padding:1.2rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);transition:border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-arch-hub__card:hover{border-color:var(--q-color-brand);transform:translateY(-2px);text-decoration:none}\
.q-arch-hub__sec{margin:0;font-size:.72rem;text-transform:uppercase;letter-spacing:.08em;color:var(--q-color-brand);font-weight:var(--q-font-weight-bold)}\
.q-arch-hub__title{margin:0;font-size:1.08rem;color:var(--q-color-fg)}\
.q-arch-hub__sum{margin:0;font-size:.9rem;line-height:1.6;color:var(--q-color-muted)}\
.q-arch-hub__card .q-prod-learn{margin-top:auto;padding-top:.6rem;font-size:.88rem}"
}

// ---------------------------------------------------------------------------
// Checkpoint flow (L1/L2/L3 over the request pipeline).
// ---------------------------------------------------------------------------

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

fn checkpoints(css: &mut Css) -> Node {
    css.push(diagram_css().to_string());

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

    let body = el("div").class("q-arch-checkpoints").child(flow).child(cards);
    reveal("arch-checkpoints", body)
}

// ---------------------------------------------------------------------------
// Submodule map.
// ---------------------------------------------------------------------------

struct Module {
    crate_name: &'static str,
    name: &'static str,
    kind: &'static str,
    blurb: &'static str,
    state: &'static str,
}

const MODULES: &[Module] = &[
    Module {
        crate_name: "qpkgs",
        name: "Stdlib",
        kind: "shared packages",
        blurb: "The zero-dependency q* stdlib — qexec (the bounded executor) and qvalue (the \
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
                depend on q*.",
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
        blurb: "The marketing + docs + architecture site, a Quill app that dogfoods the framework \
                end to end. Serves over a worker, or exports a static dist/ that runs with no DMS.",
        state: "built",
    },
    Module {
        crate_name: "qcloud",
        name: "Cloud",
        kind: "control plane",
        blurb: "The managed, multi-tenant control plane — a DMS that manages other DMSes, metered \
                per resource. Control plane v1 is built; the live infra effect is in progress.",
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
            text(" Direction rule: products (DMS, Quill, the site, the cloud) depend on q*; q* \
                   never depends on a product, and Quill does not depend on q*. The dependency \
                   arrow points one way."
                .to_string()),
        ]);

    let body = el("div").class("q-arch-modwrap").child(grid).child(rule);
    reveal("arch-submodules", body)
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    css.push(arch_kit::arch_css().to_string());
    css.push(crate::app::docs_kit::pager_css().to_string());
    css.push(hub_css().to_string());

    let mut toc = Toc::new();

    let content = el("div")
        .child(arch_kit::p(INTRO))
        .child(toc.h2("The deep dives"))
        .child(arch_kit::p(
            "Each page below is self-contained and shows honest status — what is built, what is \
             a working seam, and what is designed but not yet built.",
        ))
        .child(deep_dives())
        .child(toc.h2("The access model: three checkpoints"))
        .child(arch_kit::p(
            "Every read or mutate crosses the same three gates, in the same order: the worker \
             authenticates (L1), execute() enforces the function's scope (L2), and the planner \
             enforces table-level RBAC (L3). Skip none.",
        ))
        .child(checkpoints(&mut css))
        .child(toc.h2("The module map"))
        .child(arch_kit::p(
            "qroot composes five products plus the brand crate. Each is its own repository; \
             products share the q* substrate, and the dependency arrow only ever points toward it.",
        ))
        .child(submodules(&mut css));

    let layout = arch_kit::layout("/architecture", "System architecture", LEAD, content, toc);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/architecture" };
    page(&meta, css, layout)
}

// ---------------------------------------------------------------------------
// Diagram CSS for the two overview visuals (theme-token colored).
// ---------------------------------------------------------------------------

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
.q-arch-gates{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,15rem),1fr));gap:1rem;margin-top:1.5rem}\
.q-arch-gate{display:flex;flex-direction:column;gap:.4rem;padding:1.1rem;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface)}\
.q-arch-gate__head{display:flex;align-items:center;gap:.6rem}\
.q-arch-gate__rank{font-weight:var(--q-font-weight-bold);font-size:.78rem;color:var(--q-color-on-brand);background:var(--q-color-brand);border-radius:var(--q-radius-md);padding:.15rem .45rem;line-height:1.3}\
.q-arch-gate__title{margin:0;font-size:1.05rem;color:var(--q-color-fg)}\
.q-arch-gate__where{margin:0;font-size:.8rem;font-family:var(--q-font-mono,monospace);color:var(--q-color-brand)}\
.q-arch-gate__detail{margin:0;font-size:.92rem;color:var(--q-color-muted)}\
.q-arch-modwrap{margin-top:.5rem}\
.q-arch-modgrid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,15rem),1fr));gap:1rem}\
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
