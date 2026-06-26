//! `GET /` — the landing page. An advanced, animated hero + product cards + a
//! live component teaser. Ships the `reveal` (scroll-reveal) and `tabs`
//! (interactive) islands; everything is correct with JS off.

use qexec::FunctionResponse;
use qquill_design::{Card, Effect, Radius, Size, Stat, Tabs, Tone};
use qquill_view::{el, text, Node};

use crate::app::routes::{reveal, section, status_badge, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava — an AI-native, zero-dependency data system";
const DESCRIPTION: &str = "Qirava is an AI-native, zero-dependency data system with a Rust-native \
UI framework to build on it. Apache-2.0, security- and performance-first.";

/// The animated hero: an accent-gradient headline (rises in on load), the two
/// pillars, primary CTAs, and a metrics bar. The decorative `q-hero__glow`
/// drifts behind it; `prefers-reduced-motion` neutralizes all of it.
fn hero(css: &mut Css) -> Node {
    let cta = el("div")
        .class("q-cta-row")
        .child(
            el("a")
                .class("q-btn q-btn--solid")
                .attr("href", "/components")
                .child(text("Explore components")),
        )
        .child(
            el("a")
                .class("q-btn q-btn--ghost")
                .attr("href", "/docs")
                .child(text("Read the docs")),
        );

    let pillars = el("div")
        .class("q-pillars")
        .child(
            el("div")
                .class("q-pillar")
                .child(el("p").class("q-pillar__k").child(text("qdms")))
                .child(el("h3").child(text("The data system")))
                .child(el("p").child(text(
                    "Governance, KMS, database, jobs, and replication — all functions behind one \
                     bounded executor, served over HTTP + WS + SSR on a single port.",
                ))),
        )
        .child(
            el("div")
                .class("q-pillar")
                .child(el("p").class("q-pillar__k").child(text("qquill")))
                .child(el("h3").child(text("The UI framework")))
                .child(el("p").child(text(
                    "Rust-native, zero-dependency UI: shadcn-like components, Next.js-like \
                     authoring, native SSR + islands behind a hand-written ~4 KB runtime.",
                ))),
        );

    let stats = el("div")
        .class("q-statbar")
        .child(css.node(Stat::new("s-crates", "stdlib crates", "13").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-deps", "third-party deps", "0").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-checks", "auth checkpoints", "3").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-port", "port for HTTP + WS + SSR", "7179").size(Size::Lg).render()));

    el("section")
        .class("q-hero")
        .child(el("div").class("q-hero__glow").attr("aria-hidden", "true"))
        .child(el("p").class("q-eyebrow q-hero__in d1").child(text("Data system + UI framework")))
        .child(
            el("h1").class("q-h1").children([
                el("span").class("q-hero__in d2").child(text("An AI-native, zero-dependency ")),
                el("span").class("q-hero__in d2 q-accent").child(text("data system")),
                el("span").class("q-hero__in d3").child(text(" — and a Rust-native UI framework to build on it.")),
            ]),
        )
        .child(
            el("p").class("q-lead q-hero__in d3").child(text(
                "Two pillars: Qirava DMS fuses governance, KMS, database, jobs, and replication \
                 behind one executor; Quill is the zero-dependency UI framework you build the \
                 front end with. Security- and performance-first. Apache-2.0.",
            )),
        )
        .child(el("div").class("q-hero__in d4").child(cta))
        .child(pillars)
        .child(stats)
}

/// One product card: name + crate, a blurb, a status, and a "Learn more" link.
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
        .child(el("p").child(el("a").attr("href", href.to_string()).child(text("Learn more →"))));

    let card = Card::new(id)
        .article()
        .header(header)
        .body(body)
        .tone(Tone::Neutral)
        .effect(Effect::Flat)
        .radius(Radius::Lg);
    css.node(card.render())
}

/// The four product cards (each `[data-q-reveal]`, staggered), wrapped in one
/// scroll-reveal island.
fn products(css: &mut Css) -> Node {
    let mut grid = el("div").class("q-grid");
    let cards = [
        ("p-dms", "Qirava DMS", "qdms",
         "One AI-native, zero-dep data system: a single execute primitive and one function \
          registry. Governance, KMS, database, jobs, and replication are functions; a worker \
          layer serves HTTP, WS, and native SSR/SSG/ISR on one port.",
         Status::Built, "/products"),
        ("p-quill", "Quill", "qquill",
         "A Rust-native, zero-dependency UI framework: shadcn-like components with Next.js-like \
          authoring — native SSR, islands, and SSG/ISR — behind a ~4 KB hand-written runtime. \
          This very site is built with it.",
         Status::Built, "/components"),
        ("p-tq", "The tq* stdlib", "qpkgs",
         "13 zero-dependency crates: the substrate qexec (bounded executor) and qvalue \
          (value/ABI), plus array, object, string, math, number, convert, crypto, encoding, \
          regex, time, and uuid — shared across every product.",
         Status::Built, "/docs/concepts"),
        ("p-cloud", "Qirava Cloud", "—",
         "A managed control plane for the DMS — confidential compute (SEV-SNP), custodian-gated \
          key management, and single-leader replication, operated for you. Open-core; the engine \
          stays Apache-2.0.",
         Status::Planned, "/roadmap"),
    ];
    for (i, (id, name, cr, blurb, status, href)) in cards.iter().enumerate() {
        let card = product_card(css, id, name, cr, blurb, *status, href);
        grid = grid.child(
            el("div")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(card),
        );
    }

    section(
        Some("What's here"),
        "Three products, one substrate",
        "Everything is first-party and zero-dependency. Products depend on the tq* stdlib; the \
         stdlib never depends on the products.",
        reveal("reveal-products", grid),
    )
}

/// A live, interactive teaser: a real `Tabs` island showing the same component
/// the showcase documents, proving the islands runtime on the landing page.
fn teaser(css: &mut Css) -> Node {
    let tab_panel = |title: &str, body: &str| -> Node {
        el("div")
            .child(el("h3").class("q-h2").child(text(title.to_string())))
            .child(el("p").class("q-muted").child(text(body.to_string())))
    };

    let tabs = Tabs::new(
        "teaser-tabs",
        0,
        vec![
            ("SSR".to_string(), tab_panel(
                "Server-rendered by default",
                "Every page renders to HTML on the server. Zero JavaScript ships unless a page uses an island.",
            )),
            ("Islands".to_string(), tab_panel(
                "Interactive where it matters",
                "Islands hydrate in place on their trigger — load, visible, interaction, or idle — \
                 carrying only the behaviors a page actually uses.",
            )),
            ("Themed".to_string(), tab_panel(
                "One token system, light & dark",
                "Every color is a --q-* token. Flip data-q-theme and the whole UI restyles with no \
                 reflow — try the toggle in the header.",
            )),
        ],
    );

    let chrome = el("div")
        .class("q-teaser__chrome")
        .attr("aria-hidden", "true")
        .child(el("span"))
        .child(el("span"))
        .child(el("span"));

    let teaser = el("div")
        .class("q-teaser")
        .child(chrome)
        .child(css.node(tabs.island("teaser-tabs-island")));

    section(
        Some("Live, not a screenshot"),
        "Interactive, server-first, hydrated in place",
        "This tab strip is a real Quill island on this page — click it. The showcase has a full \
         interactive playground for every component.",
        teaser,
    )
}

/// The page body.
fn body(css: &mut Css) -> Node {
    el("main")
        .class("q-main")
        .id("main")
        .child(hero(css))
        .child(products(css))
        .child(teaser(css))
}

/// The route handler.
pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/" };
    page(&meta, css, content)
}
