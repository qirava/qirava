//! `GET /products/quill` — the Qirava Quill marketing page.
//!
//! Quill in one read: a Rust-native, zero-dependency UI/app framework — `view!{}`
//! components with native SSR, islands, and byte-identical static export; a small
//! hand-written client runtime and no server-side JS; `quill new` scaffolds an app
//! and `quill build` static-exports it. This very site is a Quill app. The page is
//! a focused marketing page (hero → what-it-is → features → how-it-works →
//! architecture animation → honest status → closing CTA), not deep docs. Content
//! is accurate to `AGENTS.md` and the framework docs.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{
    arch_anim, arch_anim_css, closing, feature_section, hero, main_wrap, product_css,
    status_section, ArchNode, Cta, Feature, HeroStat, GITHUB_URL,
};
use crate::app::routes::{reveal, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Quill — the Rust-native, zero-dependency UI framework";
const DESCRIPTION: &str = "Quill is a Rust-native, zero-dependency UI/app framework: view!{} \
components with native SSR, islands, and byte-identical static export behind a hand-written \
client runtime and no server-side JS. This very site is built with it.";

/// A plain prose "what it is" section: an eyebrow + heading + a couple of lead
/// paragraphs, scroll-revealed. Reuses the product-page `q-pp-head`/`q-section`
/// classes so it sits flush with the feature grids above and below it.
fn what_it_is() -> Node {
    let head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text("What it is")))
        .child(
            el("h2")
                .class("q-h2")
                .child(text("A UI framework that never leaves Rust")),
        )
        .child(el("p").class("q-lead").child(text(
            "Quill is a Rust-native, zero-dependency UI and app framework. You write components \
             in Rust with the view!{} macro; they render to correct, semantic HTML on the server \
             by default, and only the interactive pieces hydrate on the client as islands.",
        )))
        .child(el("p").class("q-lead q-qw-lead2").child(text(
            "There is no JavaScript toolchain underneath and no server-side JS. The same render \
             path serves a page live and exports it to a static dist/ — the bytes are identical \
             whether served from the engine or from a CDN.",
        )));

    el("section").class("q-section").child(reveal("quill-what", head))
}

/// One ordered step in the "how it works" section.
struct Step {
    n: &'static str,
    title: &'static str,
    body: &'static str,
}

/// A plain, concrete "how it works" section: an eyebrow + heading + lead, then a
/// short ordered list of steps explaining the layer graph, the island contract,
/// and zero-JS pages. Scroll-reveals; uses page-local `q-qw-*` classes (defined in
/// [`quill_extra_css`]) that lean on the same `--q-*` tokens as the rest of the page.
fn how_it_works() -> Node {
    let head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text("How it works")))
        .child(
            el("h2")
                .class("q-h2")
                .child(text("From a Rust component to a hydrated page")),
        )
        .child(el("p").class("q-lead").child(text(
            "The framework is a small stack of crates that depend strictly inward — products lean \
             on the layers beneath them, never the reverse — so the path from authoring to a \
             shipped page is short and predictable.",
        )));

    let steps = [
        Step {
            n: "1",
            title: "Author the tree with view!{}",
            body: "A component is a Rust function returning a node tree. view! reads the design \
                   tokens and the styled/headless component libraries — view depends on \
                   style, theme, and signal; ui and design sit on top.",
        },
        Step {
            n: "2",
            title: "Render natively on the server",
            body: "The tree renders straight to HTML — no client framework boots to show content. \
                   Styles compile to one compact CSS string at build time, so there is no runtime \
                   styling cost.",
        },
        Step {
            n: "3",
            title: "Hydrate only the islands",
            body: "An interactive component declares an island: a server-rendered fallback, its \
                   sidecar props, and a trigger (load, visible, interaction, or idle). The runtime \
                   ships only the behaviors those islands use — pages with none ship zero JS.",
        },
        Step {
            n: "4",
            title: "Serve live or export static",
            body: "The same render path answers a live request or writes a CDN-ready dist/. Because \
                   it is one path, the served HTML and the exported HTML are byte-identical.",
        },
    ];

    let mut list = el("ol").class("q-qw-steps").attr("role", "list");
    for (i, s) in steps.iter().enumerate() {
        list = list.child(
            el("li")
                .class("q-qw-step")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(el("span").class("q-qw-step__n").attr("aria-hidden", "true").child(text(s.n.to_string())))
                .child(
                    el("div")
                        .class("q-qw-step__text")
                        .child(el("h3").class("q-qw-step__title").child(text(s.title.to_string())))
                        .child(el("p").class("q-qw-step__body").child(text(s.body.to_string()))),
                ),
        );
    }

    el("section")
        .class("q-section")
        .child(reveal("quill-how-head", head))
        .child(reveal("quill-how", list))
}

/// Page-local CSS for the prose "what it is" lead spacing and the "how it works"
/// step list. Token-only (flips with theme/size/radius) and motion-neutral; the
/// scroll-reveal entrance is handled by the shared `reveal` island. Pushed once;
/// the accumulator dedupes it.
fn quill_extra_css() -> &'static str {
    "\
.q-qw-lead2{margin-top:var(--q-space-4);margin-bottom:0}\
.q-qw-steps{list-style:none;margin:0;padding:0;display:grid;gap:var(--q-space-3)}\
.q-qw-step{display:flex;align-items:flex-start;gap:var(--q-space-4);padding:var(--q-space-5);border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface)}\
.q-qw-step__n{flex:0 0 auto;display:inline-flex;align-items:center;justify-content:center;width:2rem;height:2rem;border-radius:var(--q-radius-full);font-family:var(--q-font-mono);font-weight:var(--q-font-weight-bold);font-size:.9rem;color:var(--q-color-on-brand);background:var(--q-color-brand)}\
.q-qw-step__text{display:flex;flex-direction:column;gap:.25rem;min-width:0}\
.q-qw-step__title{margin:0;font-size:1.06rem;font-weight:var(--q-font-weight-bold);letter-spacing:-.01em}\
.q-qw-step__body{margin:0;font-size:.94rem;line-height:1.65;color:var(--q-color-muted)}\
"
}

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(arch_anim_css().to_string());
    css.push(quill_extra_css().to_string());

    let hero = hero(
        css,
        "Qirava Quill",
        "qquill",
        "A Rust-native UI framework, ",
        "zero dependencies",
        ".",
        "Write components in Rust with view!{}, render them natively on the server, and hydrate \
         only the interactive bits as islands. No server-side JavaScript, no build-tool \
         dependency chain — just a hand-written runtime shipped only where a page actually needs \
         it. This very site is a Quill app.",
        &[
            Cta { label: "Read the docs", href: "/docs/quill", solid: true },
            Cta { label: "View on GitHub", href: GITHUB_URL, solid: false },
        ],
        &[
            HeroStat { value: "view!{}", label: "components in Rust" },
            HeroStat { value: "0", label: "server-side JS" },
            HeroStat { value: "0", label: "third-party deps" },
            HeroStat { value: "SSR·SSG", label: "byte-identical" },
        ],
    );

    let what = what_it_is();

    // The feature grid — the six pillars from the product spec.
    let features = feature_section(
        "quill-features",
        "gradient",
        "Features",
        "Everything you need to ship a UI in Rust",
        "Authoring, components, theming, interactivity, and delivery — one framework, one \
         language, no toolchain to assemble.",
        &[
            Feature {
                kicker: "view!{}",
                title: "Declarative components",
                body: "Build the UI tree with the view!{} macro: composable, typed components that \
                       render the same on the server and, where needed, hydrate on the client.",
            },
            Feature {
                kicker: "headless + styled",
                title: "Two component libraries",
                body: "Headless state machines (qquill-ui) carry the behavior and ARIA contract; \
                       styled components (qquill-design) layer token-driven looks on top — use \
                       either layer.",
            },
            Feature {
                kicker: "design tokens",
                title: "Theming that never reflows",
                body: "Components read --q-* design tokens, so flipping theme, density, or radius \
                       restyles the whole app with no reflow and no second stylesheet.",
            },
            Feature {
                kicker: "islands",
                title: "Hand-written runtime",
                body: "Interactive components hydrate as islands on their trigger. The client \
                       runtime is hand-written and imports nothing — it is the entire client \
                       footprint, shipped only where used.",
            },
            Feature {
                kicker: "quill build",
                title: "Static export (SSG)",
                body: "One command renders every route in-process and writes a CDN-ready dist/ — \
                       HTML per route plus copied assets — that serves with no server running.",
            },
            Feature {
                kicker: "quill new",
                title: "CLI scaffold",
                body: "quill new myapp lays down a working app: the page list, the render path, \
                       and the public assets, ready to cargo run and serve.",
            },
        ],
    );

    let how = how_it_works();

    // The architecture animation — the inward dependency graph + island hydration.
    let arch = arch_anim(
        "quill-arch",
        "Architecture",
        "The layer graph, animated",
        "Crates depend strictly inward: authoring sits on the substrate, components sit on \
         authoring, and a page hydrates left-to-right as its islands come alive. Reduced-motion \
         renders the same diagram, static.",
        &[
            ArchNode { label: "view!{}", sub: "authoring", badge: "" },
            ArchNode { label: "style · theme · signal", sub: "substrate", badge: "" },
            ArchNode { label: "ui (headless)", sub: "state + ARIA", badge: "" },
            ArchNode { label: "design (styled)", sub: "tokens", badge: "" },
            ArchNode { label: "runtime", sub: "island hydrate", badge: "" },
        ],
    );

    // Honest status — BUILT / PARTIAL / PLANNED.
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
            (Status::Built, "Islands runtime",
             "Hand-written, zero-import; hydrates only the interactive components a page declares."),
            (Status::Built, "Static export (SSG)",
             "quill build writes a CDN-ready dist/ whose HTML is byte-identical to the live serve."),
            (Status::Built, "quill new / quill build CLI",
             "Scaffold an app and static-export it; this very site is built and exported with it."),
            (Status::Partial, "ISR revalidation",
             "Incremental static regeneration on a revalidate window is in progress on top of the SSG path."),
        ],
    );

    let closing = closing(
        "quill-closing",
        "Build your UI with Quill",
        "Read the getting-started guide, browse the component catalog with its live playground, \
         or follow how the islands runtime hydrates in place.",
        Cta { label: "Read the docs", href: "/docs/quill", solid: true },
        Cta { label: "Browse components", href: "/docs/quill/components", solid: false },
    );

    main_wrap(vec![hero, what, features, how, arch, status, closing])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/products/quill" };
    page(&meta, css, content)
}
