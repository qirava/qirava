//! `GET /` — the landing page.
//!
//! A stunning, modern, animated home: a display-type hero (brand gradient +
//! ambient drift, rises in on load), an animated products overview using the
//! `data-q-surface` treatments (glass / neu / gradient / flat) with hover depth
//! and scroll-reveal, a concise why/features grid, a live islands teaser, and
//! clear CTAs to Products + Developer Docs.
//!
//! Everything is server-rendered and correct with JS off. The only JavaScript is
//! Quill islands (`reveal`, `tabs`); all motion is token-based and neutralized by
//! `prefers-reduced-motion`. Type/spacing/color come from the `--q-*` scale; the
//! page adds only home-specific layout CSS, all referencing those tokens.

use qexec::FunctionResponse;
use qquill_design::{Size, Stat, Tabs};
use qquill_view::{el, raw, text, Node};

use crate::app::routes::{reveal, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Qirava — an AI-native, zero-dependency data system";
const DESCRIPTION: &str = "Qirava is an AI-native, zero-dependency data system with a Rust-native \
UI framework to build on it. Apache-2.0, security- and performance-first.";

/// A small inline arrow used on link CTAs (decorative, inherits currentColor).
const ARROW_SVG: &str = "<svg class=\"q-arr\" viewBox=\"0 0 24 24\" width=\"15\" height=\"15\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M5 12h14M13 6l6 6-6 6\"/></svg>";

// ---------------------------------------------------------------------------
// HERO
// ---------------------------------------------------------------------------

/// The hero: an eyebrow, a display-scale headline (the brand gradient on the key
/// phrase), a lead paragraph, two clear CTAs, and a metrics bar — each part rises
/// in, staggered. A decorative `q-home-glow` drifts behind everything.
fn hero(css: &mut Css) -> Node {
    let cta = el("div")
        .class("q-cta-row q-rise d4")
        .child(
            el("a")
                .class("q-btn q-btn--solid")
                .attr("href", "/products")
                .child(text("Explore the products"))
                .child(raw(ARROW_SVG)),
        )
        .child(
            el("a")
                .class("q-btn q-btn--ghost")
                .attr("href", "/docs")
                .child(text("Read the developer docs")),
        );

    let stats = el("div")
        .class("q-home-stats q-rise d4")
        .child(css.node(Stat::new("s-crates", "stdlib crates", "13").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-deps", "third-party deps", "0").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-checks", "auth checkpoints", "3").size(Size::Lg).render()))
        .child(css.node(Stat::new("s-port", "one port: HTTP·WS·SSR", "7179").size(Size::Lg).render()));

    el("section")
        .class("q-home-hero")
        .child(el("div").class("q-home-glow").attr("aria-hidden", "true"))
        .child(
            el("p")
                .class("q-eyebrow q-rise d1")
                .child(text("Data system + UI framework")),
        )
        .child(
            el("h1").class("q-display q-home-title").children([
                el("span").class("q-rise d2").child(text("An AI-native, zero-dependency ")),
                el("span").class("q-rise d2 q-grad").child(text("data system")),
                el("span").class("q-rise d3").child(text(" — and a Rust-native UI framework to build on it.")),
            ]),
        )
        .child(
            el("p").class("q-body-lg q-muted q-home-lead q-rise d3").child(text(
                "Qirava DMS fuses governance, KMS, database, jobs, and replication behind one \
                 bounded executor, served over HTTP, WS, and native SSR on a single port. Quill is \
                 the zero-dependency UI framework you build the front end with. Security- and \
                 performance-first. Apache-2.0.",
            )),
        )
        .child(cta)
        .child(stats)
}

// ---------------------------------------------------------------------------
// PRODUCTS OVERVIEW
// ---------------------------------------------------------------------------

/// One product card, hand-built so it can carry a `data-q-surface` treatment and
/// the home hover-depth. Renders eyebrow (name + crate), blurb, a status chip,
/// and a "Learn more" link.
fn product_card(
    surface: &str,
    name: &str,
    crate_name: &str,
    blurb: &str,
    status: Status,
    cta: &str,
    href: &str,
) -> Node {
    let (status_label, status_mod) = match status {
        Status::Built => ("Built", "is-built"),
        Status::Partial => ("Partial", "is-partial"),
        Status::Planned => ("Planned", "is-planned"),
    };

    let head = el("div")
        .class("q-pcard__head")
        .child(
            el("div")
                .class("q-pcard__id")
                .child(el("span").class("q-pcard__name").child(text(name.to_string())))
                .child(el("code").class("q-pcard__crate").child(text(crate_name.to_string()))),
        )
        .child(
            el("span")
                .class(format!("q-pcard__status {status_mod}"))
                .child(text(status_label.to_string())),
        );

    el("article")
        .class("q-pcard")
        .attr("data-q-surface", surface.to_string())
        .child(head)
        .child(el("p").class("q-pcard__blurb").child(text(blurb.to_string())))
        .child(
            el("a")
                .class("q-pcard__link")
                .attr("href", href.to_string())
                .child(text(cta.to_string()))
                .child(raw(ARROW_SVG)),
        )
}

/// The products overview: four cards, each a different surface treatment, each
/// a staggered `[data-q-reveal]` child, wrapped in one scroll-reveal island.
fn products() -> Node {
    let cards = [
        ("gradient", "Qirava DMS", "qdms",
         "One AI-native, zero-dep data system: a single execute primitive and one function \
          registry. Governance, KMS, database, jobs, and replication are functions; a worker \
          layer serves HTTP, WS, and native SSR/SSG/ISR on one port.",
         Status::Built, "Explore the DMS", "/products"),
        ("glass", "Quill", "qquill",
         "A Rust-native, zero-dependency UI framework: shadcn-like components, Next.js-like \
          authoring — native SSR, islands, and SSG/ISR — behind a hand-written ~4 KB runtime. \
          This very site is built with it.",
         Status::Built, "Browse components", "/components"),
        ("neu", "The q* stdlib", "qpkgs",
         "13 zero-dependency crates: the substrate qexec (bounded executor) and qvalue \
          (value/ABI), plus array, object, string, math, number, convert, crypto, encoding, \
          regex, time, and uuid — shared across every product.",
         Status::Built, "Read the concepts", "/docs/concepts"),
        ("flat", "Qirava Cloud", "—",
         "A managed control plane for the DMS — confidential compute (SEV-SNP), custodian-gated \
          key management, and single-leader replication, operated for you. Open-core; the engine \
          stays Apache-2.0.",
         Status::Planned, "See the roadmap", "/roadmap"),
    ];

    let mut grid = el("div").class("q-pcards");
    for (i, (surface, name, cr, blurb, status, cta, href)) in cards.iter().enumerate() {
        grid = grid.child(
            el("div")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(product_card(surface, name, cr, blurb, *status, cta, href)),
        );
    }

    let head = el("div")
        .class("q-home-head")
        .child(el("p").class("q-eyebrow").child(text("What's here")))
        .child(el("h2").class("q-h2").child(text("Three products, one substrate")))
        .child(el("p").class("q-lead").child(text(
            "Everything is first-party and zero-dependency. Products depend on the q* stdlib; \
             the stdlib never depends on the products.",
        )));

    el("section")
        .class("q-section")
        .child(reveal("reveal-head", head))
        .child(reveal("reveal-products", grid))
}

// ---------------------------------------------------------------------------
// WHY / FEATURES
// ---------------------------------------------------------------------------

/// One feature tile (icon + title + copy). Reveals on scroll.
fn feature(delay: usize, icon: &str, title: &str, body: &str) -> Node {
    let tile = el("div")
        .class("q-feat")
        .child(el("div").class("q-feat__icon").attr("aria-hidden", "true").child(raw(icon.to_string())))
        .child(el("h3").class("q-feat__title").child(text(title.to_string())))
        .child(el("p").class("q-feat__body q-muted").child(text(body.to_string())));

    el("div")
        .attr("data-q-reveal", "")
        .attr("data-reveal-delay", (((delay) % 3) + 1).to_string())
        .child(tile)
}

/// The why/features section: a concise grid of the project's defining traits.
fn why() -> Node {
    // Small, hand-drawn stroke icons (decorative, inherit currentColor).
    const SHIELD: &str = "<svg viewBox=\"0 0 24 24\" width=\"22\" height=\"22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10Z\"/><path d=\"m9 12 2 2 4-4\"/></svg>";
    const BOLT: &str = "<svg viewBox=\"0 0 24 24\" width=\"22\" height=\"22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M13 2 4 14h7l-1 8 9-12h-7l1-8Z\"/></svg>";
    const CUBE: &str = "<svg viewBox=\"0 0 24 24\" width=\"22\" height=\"22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"m21 16-9 5-9-5V8l9-5 9 5v8Z\"/><path d=\"m3 8 9 5 9-5M12 13v8\"/></svg>";
    const LAYERS: &str = "<svg viewBox=\"0 0 24 24\" width=\"22\" height=\"22\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"m12 2 9 5-9 5-9-5 9-5Z\"/><path d=\"m3 12 9 5 9-5M3 17l9 5 9-5\"/></svg>";

    let grid = el("div")
        .class("q-feats")
        .child(feature(0, SHIELD, "Authorized by construction",
            "Every read and mutate flows through three ordered checkpoints. The planner is the \
             only door to data — no path skips it."))
        .child(feature(1, BOLT, "Performance-first",
            "One bounded executor governs all work, so a hot path is never quietly regressed. \
             Benchmarks gate the engine."))
        .child(feature(2, CUBE, "Zero third-party deps",
            "std and first-party crates only — even the fonts are a refined system stack. The \
             one exception is crypto, kept behind a trait."))
        .child(feature(0, LAYERS, "One substrate, two products",
            "A single execute primitive and function registry power the DMS; Quill renders the \
             front end. Both dogfood the same q* stdlib."));

    let head = el("div")
        .class("q-home-head q-home-head--center")
        .child(el("p").class("q-eyebrow").child(text("Why Qirava")))
        .child(el("h2").class("q-h2").child(text("Built on two pillars")))
        .child(el("p").class("q-lead").child(text(
            "Security and performance are not features bolted on — they are the shape of the \
             system. Everything else follows from staying small and first-party.",
        )));

    el("section")
        .class("q-section")
        .child(reveal("reveal-why-head", head))
        .child(reveal("reveal-why", grid))
}

// ---------------------------------------------------------------------------
// LIVE TEASER
// ---------------------------------------------------------------------------

/// A live, interactive teaser: a real `Tabs` island, proving the islands runtime
/// renders and hydrates on the landing page itself.
fn teaser(css: &mut Css) -> Node {
    let tab_panel = |title: &str, body: &str| -> Node {
        el("div")
            .class("q-teaser__panel")
            .child(el("h3").class("q-teaser__h").child(text(title.to_string())))
            .child(el("p").class("q-muted").child(text(body.to_string())))
    };

    let tabs = Tabs::new(
        "teaser-tabs",
        0,
        vec![
            ("Server-first".to_string(), tab_panel(
                "Server-rendered by default",
                "Every page renders to HTML on the server. Zero JavaScript ships unless a page \
                 actually uses an island.",
            )),
            ("Islands".to_string(), tab_panel(
                "Interactive where it matters",
                "Islands hydrate in place on their trigger — load, visible, interaction, or idle — \
                 carrying only the behaviors a page uses.",
            )),
            ("Themed".to_string(), tab_panel(
                "One token system, light & dark",
                "Every color is a --q-* token. Flip data-q-theme and the whole UI restyles with no \
                 reflow — try the appearance control in the header.",
            )),
        ],
    );

    let chrome = el("div")
        .class("q-teaser__chrome")
        .attr("aria-hidden", "true")
        .child(el("span"))
        .child(el("span"))
        .child(el("span"));

    let frame = el("div")
        .class("q-teaser")
        .attr("data-q-surface", "glass")
        .child(chrome)
        .child(css.node(tabs.island("teaser-tabs-island")));

    let head = el("div")
        .class("q-home-head")
        .child(el("p").class("q-eyebrow").child(text("Live, not a screenshot")))
        .child(el("h2").class("q-h2").child(text("Interactive, server-first, hydrated in place")))
        .child(el("p").class("q-lead").child(text(
            "This tab strip is a real Quill island on this page — click it. The showcase has a \
             full interactive playground for every component.",
        )));

    el("section")
        .class("q-section")
        .child(reveal("reveal-teaser-head", head))
        .child(reveal("reveal-teaser", frame))
}

// ---------------------------------------------------------------------------
// CLOSING CTA
// ---------------------------------------------------------------------------

/// A closing call-to-action band on a brand-gradient surface.
fn closing() -> Node {
    let band = el("div")
        .class("q-cta-band")
        .attr("data-q-surface", "gradient")
        .child(el("h2").class("q-h2 q-cta-band__h").child(text("Start building on Qirava")))
        .child(el("p").class("q-cta-band__p").child(text(
            "Read the getting-started guide, browse the component catalog, or dive into the \
             architecture behind the two pillars.",
        )))
        .child(
            el("div")
                .class("q-cta-row q-cta-band__row")
                .child(
                    el("a")
                        .class("q-btn q-btn--invert")
                        .attr("href", "/docs/getting-started")
                        .child(text("Get started"))
                        .child(raw(ARROW_SVG)),
                )
                .child(
                    el("a")
                        .class("q-btn q-btn--on-grad")
                        .attr("href", "/architecture")
                        .child(text("Read the architecture")),
                ),
        );

    el("section").class("q-section").child(reveal("reveal-closing", band))
}

// ---------------------------------------------------------------------------
// PAGE
// ---------------------------------------------------------------------------

/// Home-only layout CSS. Every value references a `--q-*` token so it restyles
/// on any theme/size/radius switch with no reflow; all motion is wrapped so the
/// `prefers-reduced-motion` reset can neutralize it.
fn home_css() -> &'static str {
    "\
/* ---- entrance motion (runs once on load) ---- */\
.q-rise{opacity:0;transform:translateY(14px);animation:q-rise .7s var(--q-ease-out) forwards}\
.q-rise.d1{animation-delay:.05s}.q-rise.d2{animation-delay:.13s}.q-rise.d3{animation-delay:.21s}.q-rise.d4{animation-delay:.30s}\
@keyframes q-rise{to{opacity:1;transform:none}}\
/* ---- hero ---- */\
.q-home-hero{position:relative;padding:calc(var(--q-space-8) * var(--q-density,1)) 0 var(--q-space-6);overflow:hidden}\
.q-home-hero>*{position:relative;z-index:1}\
.q-home-glow{position:absolute;inset:-45% -15% auto -15%;height:600px;z-index:0;pointer-events:none;\
background:radial-gradient(55% 60% at 28% 18%,color-mix(in srgb,var(--q-color-brand) 30%,transparent),transparent 70%),\
radial-gradient(45% 55% at 82% 8%,color-mix(in srgb,var(--q-color-accent,var(--q-color-brand)) 22%,transparent),transparent 70%);\
filter:blur(8px);animation:q-drift 20s var(--q-ease-in-out) infinite alternate}\
@keyframes q-drift{from{transform:translate3d(0,0,0) scale(1)}to{transform:translate3d(4%,2%,0) scale(1.1)}}\
.q-home-title{max-width:22ch;margin:0 0 var(--q-space-5)}\
.q-grad{background:linear-gradient(100deg,var(--q-color-brand),color-mix(in srgb,var(--q-color-brand) 50%,var(--q-color-fg)));\
-webkit-background-clip:text;background-clip:text;color:transparent}\
.q-home-lead{max-width:62ch;margin:0 0 var(--q-space-6)}\
.q-arr{transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-btn:hover .q-arr{transform:translateX(3px)}\
/* ---- hero stats ---- */\
.q-home-stats{display:grid;grid-template-columns:repeat(4,1fr);gap:var(--q-space-5);margin:var(--q-space-6) 0 0;\
padding:var(--q-space-5) 0 0;border-top:1px solid var(--q-color-border)}\
.q-home-stats .qq-stat__value{color:var(--q-color-brand)}\
@media (max-width:720px){.q-home-stats{grid-template-columns:1fr 1fr;gap:var(--q-space-4)}}\
/* ---- section heads ---- */\
.q-home-head{max-width:60ch;margin:0 0 var(--q-space-6)}\
.q-home-head--center{margin-left:auto;margin-right:auto;text-align:center}\
.q-home-head--center .q-lead{margin-left:auto;margin-right:auto}\
.q-home-head .q-h2{margin-top:var(--q-space-2)}\
.q-home-head .q-lead{margin-bottom:0}\
/* ---- products overview ---- */\
.q-pcards{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,19rem),1fr));gap:var(--q-space-4)}\
.q-pcards>*{height:100%}\
.q-pcard{display:flex;flex-direction:column;gap:var(--q-space-3);height:100%;padding:var(--q-space-5);\
border-radius:var(--q-radius-lg);transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-pcard:hover{transform:translateY(-4px);box-shadow:0 18px 48px -20px color-mix(in srgb,var(--q-color-brand) 55%,transparent)}\
.q-pcard[data-q-surface=\"flat\"]:hover,.q-pcard[data-q-surface=\"neu\"]:hover{border-color:color-mix(in srgb,var(--q-color-brand) 50%,var(--q-color-border))}\
.q-pcard__head{display:flex;align-items:flex-start;justify-content:space-between;gap:var(--q-space-3)}\
.q-pcard__id{display:flex;flex-direction:column;gap:.15rem}\
.q-pcard__name{font-weight:var(--q-font-weight-bold);font-size:1.15rem;line-height:1.2;letter-spacing:-.01em}\
.q-pcard__crate{font-family:var(--q-font-mono);font-size:.78rem;color:var(--q-color-muted)}\
.q-pcard[data-q-surface=\"gradient\"] .q-pcard__crate{color:color-mix(in srgb,var(--q-color-on-brand) 75%,transparent)}\
.q-pcard__status{flex:0 0 auto;font-size:.68rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;\
padding:.2rem .5rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted)}\
.q-pcard__status.is-built{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 45%,transparent);\
background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
.q-pcard[data-q-surface=\"gradient\"] .q-pcard__status.is-built{color:var(--q-color-on-brand);\
border-color:color-mix(in srgb,var(--q-color-on-brand) 45%,transparent);background:color-mix(in srgb,var(--q-color-on-brand) 16%,transparent)}\
.q-pcard__blurb{margin:0;font-size:.95rem;line-height:1.65;color:var(--q-color-muted);flex:1 1 auto}\
.q-pcard[data-q-surface=\"gradient\"] .q-pcard__blurb{color:color-mix(in srgb,var(--q-color-on-brand) 85%,transparent)}\
.q-pcard__link{display:inline-flex;align-items:center;gap:.35rem;font-weight:var(--q-font-weight-medium);font-size:.92rem;\
color:var(--q-color-brand)}\
.q-pcard__link:hover{text-decoration:none}\
.q-pcard[data-q-surface=\"gradient\"] .q-pcard__link{color:var(--q-color-on-brand)}\
.q-pcard__link:hover .q-arr{transform:translateX(3px)}\
/* ---- why / features ---- */\
.q-feats{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,15rem),1fr));gap:var(--q-space-4)}\
.q-feat{height:100%;padding:var(--q-space-5);border-radius:var(--q-radius-lg);background:var(--q-color-surface);\
border:1px solid var(--q-color-border);transition:transform var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-feat:hover{transform:translateY(-3px);border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-color-border))}\
.q-feat__icon{display:inline-flex;align-items:center;justify-content:center;width:2.75rem;height:2.75rem;margin:0 0 var(--q-space-3);\
border-radius:var(--q-radius-md);color:var(--q-color-brand);background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
.q-feat__title{margin:0 0 var(--q-space-2);font-size:1.05rem;font-weight:var(--q-font-weight-bold);letter-spacing:-.01em}\
.q-feat__body{margin:0;font-size:.93rem;line-height:1.6}\
/* ---- live teaser ---- */\
.q-teaser{padding:var(--q-space-5);border-radius:var(--q-radius-xl)}\
.q-teaser__chrome{display:flex;gap:.4rem;margin:0 0 var(--q-space-4)}\
.q-teaser__chrome span{width:11px;height:11px;border-radius:var(--q-radius-full);background:var(--q-color-border)}\
.q-teaser__panel{padding:var(--q-space-3) 0 0}\
.q-teaser__h{margin:0 0 var(--q-space-2);font-size:1.15rem;font-weight:var(--q-font-weight-bold)}\
/* ---- closing CTA band ---- */\
.q-cta-band{text-align:center;padding:calc(var(--q-space-8) * var(--q-density,1)) var(--q-space-5);border-radius:var(--q-radius-xl)}\
.q-cta-band__h{margin:0 auto var(--q-space-3);max-width:18ch}\
.q-cta-band__p{margin:0 auto var(--q-space-6);max-width:52ch;color:color-mix(in srgb,var(--q-color-on-brand) 85%,transparent);font-size:1.05rem;line-height:1.6}\
.q-cta-band__row{justify-content:center;margin:0}\
.q-btn--invert{background:var(--q-color-on-brand);color:var(--q-color-brand)}\
.q-btn--invert:hover{filter:brightness(.96);text-decoration:none}\
.q-btn--on-grad{color:var(--q-color-on-brand);border-color:color-mix(in srgb,var(--q-color-on-brand) 45%,transparent)}\
.q-btn--on-grad:hover{background:color-mix(in srgb,var(--q-color-on-brand) 14%,transparent);text-decoration:none}\
/* ---- reduced motion: straight to shown, no drift/rise ---- */\
@media (prefers-reduced-motion:reduce){\
.q-rise{opacity:1;transform:none;animation:none}\
.q-home-glow{animation:none}\
.q-pcard:hover,.q-feat:hover{transform:none}\
}"
}

/// The page body.
fn body(css: &mut Css) -> Node {
    css.push(home_css().to_string());
    el("main")
        .class("q-main")
        .id("main")
        .child(hero(css))
        .child(products())
        .child(why())
        .child(teaser(css))
        .child(closing())
}

/// The route handler.
pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/" };
    page(&meta, css, content)
}
