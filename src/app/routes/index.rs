//! `GET /` — the landing page. Pure server-rendered HTML, zero JavaScript.

use tqexec::FunctionResponse;
use tqquill_design::{Card, Effect, Radius, Size, Stat, Tone};
use tqquill_view::{el, text, Node};

use crate::app::routes::{code_block, inline_code, section, status_badge, CodeLine, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava — an AI-native, zero-dependency data system";
const DESCRIPTION: &str = "Qirava is an AI-native, zero-dependency data system with a Rust-native \
UI framework to build on it. Apache-2.0, security- and performance-first.";

/// The hero: headline, subhead, CTAs, and the headline metrics bar.
fn hero(css: &mut Css) -> Node {
    let cta = el("div")
        .class("q-cta-row")
        .child(
            el("a")
                .class("q-btn q-btn--solid")
                .attr("href", "/products")
                .child(text("Explore the products")),
        )
        .child(
            el("a")
                .class("q-btn q-btn--ghost")
                .attr("href", "/docs")
                .child(text("Read the docs")),
        );

    let stats = el("div")
        .class("q-statbar")
        .child(css.node(Stat::new("s-crates", "stdlib crates", "13").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-deps", "third-party deps", "0").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-checks", "auth checkpoints", "3").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-port", "one port for HTTP + WS", "7179").size(Size::Lg).render()));

    el("section")
        .class("q-hero")
        .child(el("p").class("q-eyebrow").child(text("Data system + UI framework")))
        .child(
            el("h1")
                .class("q-h1")
                .child(text("An AI-native, zero-dependency data system — and a Rust-native UI framework to build on it.")),
        )
        .child(
            el("p").class("q-lead").child(text(
                "Two pillars: Qirava DMS fuses governance, KMS, database, jobs, and \
                 replication behind one executor; Quill is the zero-dependency UI \
                 framework you build the front end with. Security- and \
                 performance-first. Apache-2.0.",
            )),
        )
        .child(cta)
        .child(stats)
}

/// One "what's here" card: an eyebrow (name + crate), a one-liner body, a status.
fn product_card(
    css: &mut Css,
    id: &str,
    name: &str,
    crate_name: &str,
    blurb: &str,
    status: Status,
    href: &str,
) -> Node {
    let eyebrow = el("div")
        .class("q-card-eyebrow")
        .child(text(name.to_string()))
        .child(el("code").child(text(crate_name.to_string())));

    let header = el("div")
        .child(eyebrow)
        .child(el("div").class("q-card-actions").child(status_badge(css, status)));

    let body = el("div")
        .child(el("p").child(text(blurb.to_string())))
        .child(
            el("p").child(
                el("a").attr("href", href.to_string()).child(text("Learn more →")),
            ),
        );

    let card = Card::new(id)
        .article()
        .header(header)
        .body(body)
        .tone(Tone::Neutral)
        .effect(Effect::Flat)
        .radius(Radius::Lg);
    css.node(card.render())
}

/// The three "what's here" pillars.
fn pillars(css: &mut Css) -> Node {
    let grid = el("div")
        .class("q-grid")
        .child(product_card(
            css,
            "p-dms",
            "Qirava DMS",
            "qdms",
            "One AI-native, zero-dep data system: a single execute primitive and one \
             function registry. Governance, KMS, database, jobs, and replication are \
             functions; a worker layer serves HTTP, WS, and native SSR/SSG/ISR on one port.",
            Status::Built,
            "/products",
        ))
        .child(product_card(
            css,
            "p-quill",
            "Quill",
            "qquill",
            "A Rust-native, zero-dependency UI framework: shadcn-like components with \
             Next.js-like authoring — native SSR, islands, and SSG/ISR — behind a ~4 KB \
             hand-written client runtime. This very site is built with it.",
            Status::Built,
            "/products",
        ))
        .child(product_card(
            css,
            "p-tq",
            "The tq* stdlib",
            "qpkgs",
            "13 zero-dependency crates: the substrate tqexec (bounded executor) and \
             tqvalue (value/ABI), plus array, object, string, math, number, convert, \
             crypto, encoding, regex, time, and uuid — shared across every product.",
            Status::Built,
            "/products",
        ));

    section(
        Some("What's here"),
        "Three products, one substrate",
        "Everything is first-party and zero-dependency. Products depend on the tq* \
         stdlib; the stdlib never depends on the products.",
        grid,
    )
}

/// The architecture pitch: the planner-is-the-only-door one-liner + checkpoints.
fn architecture() -> Node {
    let prose = el("div")
        .class("q-prose")
        .child(el("p").child(text(
            "Nothing reaches the database except through a worker, behind three \
             authorization checkpoints — and the planner is the only door to read \
             or mutate.",
        )))
        .child(
            el("ul")
                .class("q-list")
                .child(el("li").children([
                    el("strong").child(text("L1 — before-auth: ")),
                    text("the worker authenticates the caller before any function runs.".to_string()),
                ]))
                .child(el("li").children([
                    el("strong").child(text("L2 — execute scope: ")),
                    text("the executor checks the caller may invoke that function at all.".to_string()),
                ]))
                .child(el("li").children([
                    el("strong").child(text("L3 — planner: ")),
                    text("QQL-level RBAC gates the actual read/mutate at plan time.".to_string()),
                ]))
        )
        .child(el("p").class("q-muted").children([
            text("Configuration is data: roles, routes, and policies live in ".to_string()),
            inline_code("_sys_*"),
            text(" tables, and the default admin app — Qirava Studio — is itself a DMS client.".to_string()),
        ]));

    section(
        Some("Architecture"),
        "The planner is the only door",
        "One executor (tqexec) is the chokepoint. Governance/RBAC, KMS, the database, \
         workers, and replication are all functions behind it.",
        prose,
    )
}

/// A quickstart teaser: clone, build, run.
fn quickstart() -> Node {
    let code = code_block(&[
        CodeLine::Comment("# clone with submodules, build the DMS, run it"),
        CodeLine::Cmd("git clone --recursive https://github.com/qirava/qirava"),
        CodeLine::Cmd("cargo build --release -p tqdms"),
        CodeLine::Cmd("./target/release/qdms"),
        CodeLine::Comment("# UI + API on 127.0.0.1:7179 — first-run credential printed once"),
        CodeLine::Plain(""),
        CodeLine::Comment("# or scaffold a Quill app (this site is one)"),
        CodeLine::Cmd("quill new myapp && cd myapp && cargo run"),
    ]);

    section(
        Some("Get started"),
        "Up in three commands",
        "No external dependencies to install — std and first-party crates only. The \
         exported site serves with no DMS running.",
        code,
    )
}

/// The page body.
fn body(css: &mut Css) -> Node {
    el("main")
        .class("q-main")
        .id("main")
        .child(hero(css))
        .child(pillars(css))
        .child(architecture())
        .child(quickstart())
}

/// The route handler.
pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/",
    };
    page(&meta, css, content)
}
