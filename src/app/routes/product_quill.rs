//! `GET /products/quill` — the Qirava Quill product page.
//!
//! Quill in one read: a Rust-native, zero-dependency UI/app framework — `view!{}`
//! components with native SSR, islands, and SSG/ISR; a ~4 KB hand-written client
//! runtime and no server-side JS; `quill new` scaffolds an app and `quill build`
//! static-exports it. This very site is a Quill app. Content is accurate to
//! `AGENTS.md` and the framework docs.

use qexec::FunctionResponse;
use qquill_view::Node;

use crate::app::routes::product_page::{
    closing, feature_section, hero, main_wrap, product_css, status_section, Cta, Feature, HeroStat,
    GITHUB_URL,
};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Quill — the Rust-native, zero-dependency UI framework";
const DESCRIPTION: &str = "Quill is a Rust-native, zero-dependency UI/app framework: view!{} \
components with native SSR, islands, and SSG/ISR behind a hand-written ~4 KB runtime and no \
server-side JS. This very site is built with it.";

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());

    let hero = hero(
        css,
        "Qirava Quill",
        "qquill",
        "A Rust-native UI framework, ",
        "zero dependencies",
        ".",
        "Write components in Rust with view!{}, render them natively on the server, and hydrate \
         only the interactive bits as islands. No server-side JavaScript, no build-tool \
         dependency chain — just a hand-written ~4 KB runtime shipped only where a page actually \
         needs it. This very site is a Quill app.",
        &[
            Cta { label: "Browse the components", href: "/components", solid: true },
            Cta { label: "View on GitHub", href: GITHUB_URL, solid: false },
        ],
        &[
            HeroStat { value: "~4KB", label: "hand-written runtime" },
            HeroStat { value: "0", label: "server-side JS" },
            HeroStat { value: "0", label: "third-party deps" },
            HeroStat { value: "SSR·SSG·ISR", label: "native rendering" },
        ],
    );

    // Authoring model.
    let authoring = feature_section(
        "quill-authoring",
        "gradient",
        "Authoring",
        "Components in Rust, rendered natively",
        "Quill brings shadcn-like styled components and Next.js-like authoring to Rust — without \
         leaving the language and without a JavaScript toolchain underneath.",
        &[
            Feature {
                kicker: "view!{}",
                title: "Declarative components",
                body: "Build the UI tree with the view!{} macro: composable, typed components over \
                       headless state machines. The same component renders on the server and, where \
                       needed, hydrates on the client.",
            },
            Feature {
                kicker: "theme tokens",
                title: "Styled, token-driven",
                body: "A library of styled components (Navbar, Card, Button, Badge, Stat, Table, \
                       Tabs…) reads --q-* design tokens, so flipping theme / density / radius \
                       restyles the whole app with no reflow.",
            },
            Feature {
                kicker: "Rust end to end",
                title: "One language, no toolchain",
                body: "There is no Webpack, no bundler config, no node_modules. The app is a Rust \
                       binary; styles compile to a compact CSS string at build time with no runtime \
                       cost.",
            },
        ],
    );

    // Rendering: SSR + islands + SSG/ISR.
    let rendering = feature_section(
        "quill-rendering",
        "glass",
        "Rendering",
        "Server-first, interactive where it matters",
        "Every page renders to HTML on the server. JavaScript ships only when a page actually uses \
         an island — content pages ship zero bytes of it.",
        &[
            Feature {
                kicker: "native SSR",
                title: "HTML by default",
                body: "Pages render to correct, semantic HTML server-side — no client framework \
                       boot to see content, and the markup is identical whether served live or \
                       exported statically.",
            },
            Feature {
                kicker: "islands",
                title: "Hydrate in place",
                body: "Interactive components are islands that hydrate on their trigger — load, \
                       visible, interaction, or idle — carrying only the behaviors a page uses, not \
                       a whole-app bundle.",
            },
            Feature {
                kicker: "SSG · ISR",
                title: "Static export + revalidate",
                body: "The same pages export to a static dist/ for any CDN (SSG), or refresh on a \
                       revalidate window (ISR). One render path, three delivery modes.",
            },
        ],
    );

    // The runtime + CLI.
    let runtime = feature_section(
        "quill-runtime",
        "flat",
        "The runtime + CLI",
        "~4 KB of hand-written JS, scaffolded by a CLI",
        "The client runtime is hand-written and zero-import — no React, no framework download — \
         and a small CLI gets you from empty directory to a deployable static site.",
        &[
            Feature {
                kicker: "~4 KB · zero-import",
                title: "The islands runtime",
                body: "A single hand-written runtime hydrates islands. It is injected only on pages \
                       that use one, and it imports nothing — it is the entire client footprint.",
            },
            Feature {
                kicker: "quill new",
                title: "Scaffold an app",
                body: "quill new myapp lays down a working Quill app: the page list, the render \
                       path, and the public assets, ready to cargo run and serve.",
            },
            Feature {
                kicker: "quill build",
                title: "Static-export it",
                body: "quill build renders every route in-process and writes a CDN-ready dist/ — \
                       HTML per route plus copied assets — that serves with no server running.",
            },
        ],
    );

    // What's built.
    let status = status_section(
        "quill-status",
        "Status",
        "What's built today",
        "Quill is shipping: components, native SSR, islands, and the static export are all in use \
         by this site, which dogfoods the framework end to end.",
        &[
            (Status::Built, "view!{} components + theme tokens",
             "Styled components over headless state machines, driven by --q-* design tokens."),
            (Status::Built, "Native SSR",
             "Server-rendered HTML by default; content pages ship zero JavaScript."),
            (Status::Built, "Islands runtime (~4 KB)",
             "Hand-written, zero-import; hydrates only the interactive components a page declares."),
            (Status::Built, "SSG static export + ISR",
             "quill build writes a CDN-ready dist/; ISR refreshes on a revalidate window."),
            (Status::Built, "quill new / quill build CLI",
             "Scaffold an app and static-export it; this very site is built and exported with it."),
        ],
    );

    let closing = closing(
        "quill-closing",
        "Build your UI with Quill",
        "Browse the component catalog with its live playground, follow the getting-started guide, \
         or read how the islands runtime hydrates in place.",
        Cta { label: "Browse components", href: "/components", solid: true },
        Cta { label: "Get started", href: "/docs/getting-started", solid: false },
    );

    main_wrap(vec![hero, authoring, rendering, runtime, status, closing])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/products/quill" };
    page(&meta, css, content)
}
