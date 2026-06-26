//! `GET /docs` — the docs index (a curated link hub). Pure SSR, zero JavaScript.
//!
//! v1 is an index, not a re-host: it mirrors the repo's `docs/README.md` reading
//! order and links out to the GitHub repo for the full pages. The full content
//! migration is a later pass.

use qexec::FunctionResponse;
use qquill_design::{Card, Effect, Radius, Tone};
use qquill_view::{el, text, Node};

use crate::app::routes::{code_block, status_badge, CodeLine, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Docs — Qirava";
const DESCRIPTION: &str = "Documentation index for Qirava: architecture, security & governance, \
clustering/replication, the managed cloud, and how to get started.";

const REPO_DOCS: &str = "https://github.com/qirava/qirava/tree/main/docs";

/// One doc-area entry: title, a one-line summary, a build-status, and a link.
struct Doc {
    title: &'static str,
    summary: &'static str,
    status: Status,
    href: &'static str,
}

const DOCS: &[Doc] = &[
    Doc {
        title: "Architecture Overview",
        summary: "Start here: the one-executor model, the function registry, workers, and the read/mutate path.",
        status: Status::Built,
        href: "ARCHITECTURE_OVERVIEW.md",
    },
    Doc {
        title: "Architecture",
        summary: "The deep dive: engine, storage, WAL, the planner, and config-as-data in the _sys_* tables.",
        status: Status::Built,
        href: "ARCHITECTURE.md",
    },
    Doc {
        title: "Security & Governance",
        summary: "The three authorization checkpoints (L1/L2/L3), the RBAC hierarchy, sessions, and signed keys.",
        status: Status::Built,
        href: "SECURITY_GOVERNANCE.md",
    },
    Doc {
        title: "Cluster & Replication",
        summary: "Single-leader replication today; the symmetric DUAL sync path is designed and pending.",
        status: Status::Partial,
        href: "CLUSTER_REPLICATION.md",
    },
    Doc {
        title: "Cloud Multitenant",
        summary: "The managed control plane: metering, billing, OS caps, and per-tenant sandboxing.",
        status: Status::Planned,
        href: "CLOUD_MULTITENANT.md",
    },
    Doc {
        title: "Structure & Roadmap",
        summary: "Repo layout, the submodule tree, and the honest BUILT/PARTIAL/PLANNED roadmap.",
        status: Status::Built,
        href: "STRUCTURE.md",
    },
];

fn doc_card(css: &mut Css, doc: &Doc) -> Node {
    let header = el("div")
        .child(
            el("div")
                .class("q-card-eyebrow")
                .child(text(doc.title.to_string())),
        )
        .child(
            el("div")
                .class("q-card-actions")
                .child(status_badge(css, doc.status)),
        );

    let body = el("div")
        .child(el("p").child(text(doc.summary.to_string())))
        .child(
            el("p").child(
                el("a")
                    .attr("href", format!("{REPO_DOCS}/{}", doc.href))
                    .child(text("Read on GitHub →")),
            ),
        );

    let card = Card::new(format!("doc-{}", doc.href))
        .article()
        .header(header)
        .body(body)
        .tone(Tone::Neutral)
        .effect(Effect::Flat)
        .radius(Radius::Lg);
    css.node(card.render())
}

/// The "Get started" block.
fn get_started() -> Node {
    let code = code_block(&[
        CodeLine::Comment("# clone with submodules and build the DMS"),
        CodeLine::Cmd("git clone --recursive https://github.com/qirava/qirava"),
        CodeLine::Cmd("cargo build --release -p qdms"),
        CodeLine::Cmd("./target/release/qdms"),
        CodeLine::Comment("# Studio (UI) + API on 127.0.0.1:7179"),
        CodeLine::Comment("# the first-run bootstrap credential is printed once — save it"),
        CodeLine::Plain(""),
        CodeLine::Comment("# build a front end with Quill"),
        CodeLine::Cmd("quill new myapp && cd myapp && cargo run"),
    ]);

    el("section")
        .class("q-section")
        .child(
            el("div")
                .class("q-section__head")
                .child(el("p").class("q-eyebrow").child(text("Get started")))
                .child(el("h2").class("q-h2").child(text("Clone, build, run")))
                .child(el("p").class("q-lead").child(text(
                    "Clone with --recursive so the submodules come along. Everything \
                     builds with std and first-party crates — no external dependencies.",
                ))),
        )
        .child(code)
}

fn body(css: &mut Css) -> Node {
    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text("Documentation")))
        .child(el("h1").class("q-h1").child(text("Docs index")))
        .child(el("p").class("q-lead").child(text(
            "A curated hub mirroring the repository's reading order. v1 links out to \
             the full pages on GitHub; the in-site content migration is a later pass.",
        )));

    let mut grid = el("div").class("q-grid");
    for doc in DOCS {
        grid = grid.child(doc_card(css, doc));
    }

    let reading = el("section")
        .class("q-section")
        .child(
            el("div")
                .class("q-section__head")
                .child(el("p").class("q-eyebrow").child(text("Reading order")))
                .child(el("h2").class("q-h2").child(text("Where to begin"))),
        )
        .child(grid);

    el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(reading)
        .child(get_started())
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/docs",
    };
    page(&meta, css, content)
}
