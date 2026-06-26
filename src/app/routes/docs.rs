//! `GET /docs`, `/docs/getting-started`, `/docs/concepts` — a real docs layout
//! (left sidebar → pages, content column, on-page TOC, prev/next), built on
//! `qquill-docs` primitives and rendered inside the site shell. Code blocks
//! carry a working copy button (the `copy` island).

use qexec::FunctionResponse;
use qquill_docs::Callout;
use qquill_view::{el, text, Node};

use crate::app::docs_kit::{layout, pager_css, Toc};
use crate::app::routes::{copy_code, CodeLine};
use crate::app::shell::page;
use crate::app::{Css, Meta};

/// Pull in the `qquill-docs` content-primitive CSS (`.qq-heading`, `.qq-callout`,
/// …) plus our pager CSS. The DocShell-only rules in that sheet are inert here
/// (no matching markup); the `.qq-*` classes don't collide with our layout.
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

// ---------------------------------------------------------------------------
// /docs — Overview
// ---------------------------------------------------------------------------

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    docs_css(&mut css);
    let mut toc = Toc::new();

    let content = el("div")
        .child(toc.h2("What Qirava is"))
        .child(p(
            "Qirava is two things that fit together: an AI-native, zero-dependency data system \
             (the DMS), and a Rust-native UI framework (Quill) to build interfaces on top of it. \
             Both are Apache-2.0 and built from std plus first-party crates only.",
        ))
        .child(Callout::tip(p(
            "Everything on this site — including this page and the interactive component \
             playground — is a Quill app, dogfooding the framework.",
        )).render())
        .child(toc.h2("How the docs are organized"))
        .child(bullets(&[
            ("Start here — ", "install, build, and run; then the mental model in one page."),
            ("Concepts — ", "the three authorization checkpoints and the execute → worker → planner path."),
        ]))
        .child(p(
            "Use the left sidebar to move between pages, the list on the right to jump within a \
             page, and the prev/next links at the bottom to read in order.",
        ))
        .child(toc.h2("Design principles"))
        .child(bullets(&[
            ("Zero dependencies — ", "std and first-party crates only; the sole exception is cryptography, kept behind a trait."),
            ("Security-first — ", "every read or mutate is authorized; there is no bypass path to the database."),
            ("Performance-first — ", "one bounded executor governs all work; hot paths are benchmarked before and after changes."),
        ]));

    let body = layout(
        "/docs",
        "Documentation",
        "The mental model, an honest map of what's built, and how to get running in three commands.",
        content,
        toc,
    );

    let meta = Meta {
        title: "Docs — Qirava",
        description: "Qirava documentation: the data system + Quill UI framework, the auth model, \
                      and getting started.",
        path: "/docs",
    };
    page(&meta, css, body)
}

// ---------------------------------------------------------------------------
// /docs/getting-started
// ---------------------------------------------------------------------------

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

    let body = layout(
        "/docs/getting-started",
        "Getting started",
        "Clone, build, and run the data system — then scaffold a Quill front end. No external \
         dependencies to install.",
        content,
        toc,
    );

    let meta = Meta {
        title: "Getting started — Qirava docs",
        description: "Install, build, and run the Qirava DMS, then scaffold a Quill app. Std and \
                      first-party crates only.",
        path: "/docs/getting-started",
    };
    page(&meta, css, body)
}

// ---------------------------------------------------------------------------
// /docs/concepts
// ---------------------------------------------------------------------------

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

    let body = layout(
        "/docs/concepts",
        "Core concepts",
        "The execute → worker → planner path and the three authorization checkpoints that gate \
         every read and mutate.",
        content,
        toc,
    );

    let meta = Meta {
        title: "Core concepts — Qirava docs",
        description: "The Qirava access model: execute → worker → planner, and the three \
                      authorization checkpoints (L1/L2/L3).",
        path: "/docs/concepts",
    };
    page(&meta, css, body)
}
