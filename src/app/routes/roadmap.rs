//! `GET /roadmap` — the honest BUILT/PARTIAL/PLANNED matrix. Pure SSR, zero JS.
//!
//! Sourced from `ARCHITECTURE_OVERVIEW.md` and `CLOUD_MULTITENANT.md §0`. No
//! dates are promised — only state.

use tqexec::FunctionResponse;
use tqquill_design::{HeaderCell, Size, Table};
use tqquill_view::{el, text, Node};

use crate::app::routes::{status_badge, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Roadmap — Qirava";
const DESCRIPTION: &str = "An honest BUILT / PARTIAL / PLANNED status matrix for Qirava: what \
works today, what is partial, and what is designed but not yet built.";

/// One roadmap row: area, capability, and status.
struct Row {
    area: &'static str,
    capability: &'static str,
    status: Status,
}

const ROWS: &[Row] = &[
    Row { area: "Workers", capability: "before → handle → after pipeline; HTTP + WS + SSR/SSG/ISR on one port", status: Status::Built },
    Row { area: "Executor", capability: "tqexec bounded executor — the single chokepoint to the engine", status: Status::Built },
    Row { area: "Function registry", capability: "one execute primitive; governance/KMS/db/jobs as registered functions", status: Status::Built },
    Row { area: "Engine + storage", capability: "the database engine, storage layer, and write-ahead log (WAL)", status: Status::Built },
    Row { area: "Governance / RBAC", capability: "QQL-level RBAC; custodian > admin > user > guest; single-use invites", status: Status::Built },
    Row { area: "Config-as-data", capability: "roles, routes, and policies live in the _sys_* tables", status: Status::Built },
    Row { area: "Vector search", capability: "vector indexing via LSH (graph + search also built in)", status: Status::Built },
    Row { area: "Replication", capability: "single-direction (single-leader) replication", status: Status::Partial },
    Row { area: "Standalone KMS", capability: "key management as an independent service", status: Status::Planned },
    Row { area: "DUAL sync", capability: "symmetric bidirectional replication", status: Status::Planned },
    Row { area: "Managed cloud", capability: "qcloud: metering, billing, OS caps, per-tenant sandbox", status: Status::Planned },
    Row { area: "TS / WinterTC workers", capability: "TypeScript / WinterTC worker runtime", status: Status::Planned },
];

/// The status matrix as a semantic table.
fn matrix(css: &mut Css) -> Node {
    let mut table = Table::new(3, ROWS.len())
        .size(Size::Md)
        .caption(text("Qirava capability status".to_string()))
        .header(HeaderCell::new(text("Area".to_string())))
        .header(HeaderCell::new(text("Capability".to_string())))
        .header(HeaderCell::new(text("Status".to_string())));

    for row in ROWS {
        table = table.row(vec![
            el("strong").child(text(row.area.to_string())),
            text(row.capability.to_string()),
            status_badge(css, row.status),
        ]);
    }

    el("div").class("q-table-wrap").child(css.node(table.render()))
}

/// The legend explaining each status.
fn legend(css: &mut Css) -> Node {
    let item = |css: &mut Css, status: Status, desc: &str| -> Node {
        el("li").children([
            status_badge(css, status),
            text(format!("  {desc}")),
        ])
    };

    el("ul")
        .class("q-list")
        .child(item(css, Status::Built, "shipping and usable today."))
        .child(item(css, Status::Partial, "works in one direction / one mode; the rest is designed."))
        .child(item(css, Status::Planned, "designed but not yet built — no dates promised."))
}

fn body(css: &mut Css) -> Node {
    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text("Roadmap")))
        .child(el("h1").class("q-h1").child(text("What's built, what's next")))
        .child(el("p").class("q-lead").child(text(
            "An honest status matrix. We track three states and promise no dates — \
             only where each capability actually stands.",
        )));

    let legend_section = el("section")
        .class("q-section")
        .child(
            el("div")
                .class("q-section__head")
                .child(el("h2").class("q-h2").child(text("Legend"))),
        )
        .child(legend(css));

    let matrix_section = el("section")
        .class("q-section")
        .child(
            el("div")
                .class("q-section__head")
                .child(el("h2").class("q-h2").child(text("Status matrix"))),
        )
        .child(matrix(css));

    el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(legend_section)
        .child(matrix_section)
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/roadmap",
    };
    page(&meta, css, content)
}
