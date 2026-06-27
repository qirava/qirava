//! Shared building blocks for the per-product detail pages
//! (`/products/{dms,quill,stdlib,cloud}`).
//!
//! Each product page has the same shape — a strong display hero (eyebrow +
//! `.q-display` title with a brand-gradient accent + lead + CTA row + a stat
//! strip), one or more feature grids with a `data-q-surface` treatment that
//! scroll-reveal in, a "what's built" status block, and a closing CTA band — so
//! the layout, motion, and CSS live here once and every page just supplies the
//! accurate content. Everything is server-rendered and correct with JS off; the
//! only JavaScript is the shared `reveal` island. Every value references a
//! `--q-*` token so the page restyles on any theme/size/radius switch with no
//! reflow, and all motion is wrapped so the `prefers-reduced-motion` reset can
//! neutralize it.

use qquill_design::{Size, Stat};
use qquill_view::{el, raw, text, Node};

use crate::app::routes::{reveal, Status};
use crate::app::Css;

/// The project repository, linked from every product page's CTAs.
pub const GITHUB_URL: &str = "https://github.com/qirava/qirava";

/// Wrap a product page's sections in the standard `<main>` shell.
pub fn main_wrap(sections: Vec<Node>) -> Node {
    let mut main = el("main").class("q-main").id("main");
    for s in sections {
        main = main.child(s);
    }
    main
}

/// A small inline arrow used on link CTAs (decorative, inherits currentColor).
pub const ARROW_SVG: &str = "<svg class=\"q-arr\" viewBox=\"0 0 24 24\" width=\"15\" height=\"15\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M5 12h14M13 6l6 6-6 6\"/></svg>";
/// A small check glyph for "what's built" lists (decorative).
const CHECK_SVG: &str = "<svg viewBox=\"0 0 24 24\" width=\"16\" height=\"16\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M20 6 9 17l-5-5\"/></svg>";

/// One hero CTA: `(label, href, solid?)`. The first is rendered solid, the rest
/// ghost — but the explicit flag wins so a page can override.
pub struct Cta {
    pub label: &'static str,
    pub href: &'static str,
    pub solid: bool,
}

/// A single hero stat: a big value plus its label.
pub struct HeroStat {
    pub value: &'static str,
    pub label: &'static str,
}

/// The product hero: eyebrow, a `.q-display` title whose `accent` phrase carries
/// the brand gradient, a lead, a CTA row, and an optional stat strip. Each part
/// rises in, staggered; a decorative glow drifts behind.
pub fn hero(
    css: &mut Css,
    eyebrow: &str,
    crate_name: &str,
    title_lead: &str,
    title_accent: &str,
    title_tail: &str,
    lead: &str,
    ctas: &[Cta],
    stats: &[HeroStat],
) -> Node {
    let mut cta_row = el("div").class("q-cta-row q-rise d4");
    for c in ctas {
        let class = if c.solid { "q-btn q-btn--solid" } else { "q-btn q-btn--ghost" };
        let mut a = el("a").class(class).attr("href", c.href).child(text(c.label.to_string()));
        if c.solid {
            a = a.child(raw(ARROW_SVG));
        }
        cta_row = cta_row.child(a);
    }

    let mut title = el("h1").class("q-display q-pp-title");
    title = title.child(el("span").class("q-rise d2").child(text(title_lead.to_string())));
    title = title.child(el("span").class("q-rise d2 q-grad").child(text(title_accent.to_string())));
    if !title_tail.is_empty() {
        title = title.child(el("span").class("q-rise d3").child(text(title_tail.to_string())));
    }

    let mut sec = el("section")
        .class("q-pp-hero")
        .child(el("div").class("q-pp-glow").attr("aria-hidden", "true"))
        .child(
            el("p")
                .class("q-eyebrow q-rise d1")
                .child(text(eyebrow.to_string()))
                .child(el("code").class("q-pp-crate").child(text(crate_name.to_string()))),
        )
        .child(title)
        .child(el("p").class("q-body-lg q-muted q-pp-lead q-rise d3").child(text(lead.to_string())))
        .child(cta_row);

    if !stats.is_empty() {
        let mut strip = el("div").class("q-pp-stats q-rise d4");
        for (i, s) in stats.iter().enumerate() {
            strip = strip.child(css.node(
                Stat::new(format!("pp-stat-{i}"), s.label, s.value).size(Size::Lg).render(),
            ));
        }
        sec = sec.child(strip);
    }

    sec
}

/// One feature tile (title + body), optionally tagged with a small mono kicker.
pub struct Feature {
    pub kicker: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

/// A feature section: an eyebrow + heading + lead, then a grid of feature tiles
/// on the given `data-q-surface`, all wrapped in one scroll-reveal island.
pub fn feature_section(
    id: &'static str,
    surface: &str,
    eyebrow: &str,
    heading: &str,
    lead: &str,
    features: &[Feature],
) -> Node {
    let head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text(eyebrow.to_string())))
        .child(el("h2").class("q-h2").child(text(heading.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())));

    let mut grid = el("div").class("q-pp-feats");
    for (i, f) in features.iter().enumerate() {
        let mut tile = el("article").class("q-pp-feat").attr("data-q-surface", surface.to_string());
        if !f.kicker.is_empty() {
            tile = tile.child(el("span").class("q-pp-feat__kicker").child(text(f.kicker.to_string())));
        }
        tile = tile
            .child(el("h3").class("q-pp-feat__title").child(text(f.title.to_string())))
            .child(el("p").class("q-pp-feat__body").child(text(f.body.to_string())));
        grid = grid.child(
            el("div")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(tile),
        );
    }

    el("section")
        .class("q-section")
        .child(reveal(leak_id(id, "-head"), head))
        .child(reveal(id, grid))
}

/// A "what's built" status block: an eyebrow + heading + lead, then a list of
/// `(Status, label, detail)` rows, each with a status chip. Scroll-reveals.
pub fn status_section(
    id: &'static str,
    eyebrow: &str,
    heading: &str,
    lead: &str,
    rows: &[(Status, &'static str, &'static str)],
) -> Node {
    let head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text(eyebrow.to_string())))
        .child(el("h2").class("q-h2").child(text(heading.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())));

    let mut list = el("ul").class("q-pp-status").attr("role", "list");
    for (i, (status, label, detail)) in rows.iter().enumerate() {
        let (chip_label, chip_mod) = match status {
            Status::Built => ("Built", "is-built"),
            Status::Partial => ("Partial", "is-partial"),
            Status::Planned => ("Planned", "is-planned"),
        };
        let mark: Node = match status {
            Status::Built => el("span").class("q-pp-status__mark").child(raw(CHECK_SVG)),
            _ => el("span").class("q-pp-status__mark q-pp-status__mark--soon").attr("aria-hidden", "true"),
        };
        list = list.child(
            el("li")
                .class("q-pp-status__row")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(mark)
                .child(
                    el("div")
                        .class("q-pp-status__text")
                        .child(el("span").class("q-pp-status__label").child(text(label.to_string())))
                        .child(el("span").class("q-pp-status__detail").child(text(detail.to_string()))),
                )
                .child(
                    el("span")
                        .class(format!("q-pp-status__chip {chip_mod}"))
                        .child(text(chip_label.to_string())),
                ),
        );
    }

    el("section")
        .class("q-section")
        .child(reveal(leak_id(id, "-head"), head))
        .child(reveal(id, list))
}

/// The closing CTA band on a brand-gradient surface: a heading, a line, and two
/// CTAs (the first inverted, the second outlined-on-gradient). Scroll-reveals.
pub fn closing(
    id: &'static str,
    heading: &str,
    body: &str,
    primary: Cta,
    secondary: Cta,
) -> Node {
    let band = el("div")
        .class("q-pp-cta")
        .attr("data-q-surface", "gradient")
        .child(el("h2").class("q-h2 q-pp-cta__h").child(text(heading.to_string())))
        .child(el("p").class("q-pp-cta__p").child(text(body.to_string())))
        .child(
            el("div")
                .class("q-cta-row q-pp-cta__row")
                .child(
                    el("a")
                        .class("q-btn q-btn--invert")
                        .attr("href", primary.href)
                        .child(text(primary.label.to_string()))
                        .child(raw(ARROW_SVG)),
                )
                .child(
                    el("a")
                        .class("q-btn q-btn--on-grad")
                        .attr("href", secondary.href)
                        .attr("rel", "noopener")
                        .child(text(secondary.label.to_string())),
                ),
        );

    el("section").class("q-section").child(reveal(id, band))
}

/// `reveal()` needs a `'static` id; the four pages pass distinct constant ids,
/// so a tiny intern keeps the `-head` companion id `'static` too.
fn leak_id(base: &'static str, suffix: &'static str) -> &'static str {
    Box::leak(format!("{base}{suffix}").into_boxed_str())
}

/// The product-page CSS. Self-contained (does not depend on the home page's
/// sheet): every value references a `--q-*` token so it restyles on any
/// theme/size/radius switch with no reflow; motion is neutralized under
/// `prefers-reduced-motion`. Pushed once per page and deduped by the accumulator.
pub fn product_css() -> &'static str {
    "\
/* ---- entrance motion (runs once on load) ---- */\
.q-rise{opacity:0;transform:translateY(14px);animation:q-rise .7s var(--q-ease-out) forwards}\
.q-rise.d1{animation-delay:.05s}.q-rise.d2{animation-delay:.13s}.q-rise.d3{animation-delay:.21s}.q-rise.d4{animation-delay:.30s}\
@keyframes q-rise{to{opacity:1;transform:none}}\
.q-arr{transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-btn:hover .q-arr{transform:translateX(3px)}\
.q-grad{background:linear-gradient(100deg,var(--q-color-brand),color-mix(in srgb,var(--q-color-brand) 50%,var(--q-color-fg)));-webkit-background-clip:text;background-clip:text;color:transparent}\
/* ---- hero ---- */\
.q-pp-hero{position:relative;padding:calc(var(--q-space-8) * var(--q-density,1)) 0 var(--q-space-6);overflow:hidden}\
.q-pp-hero>*{position:relative;z-index:1}\
.q-pp-glow{position:absolute;inset:-45% -15% auto -15%;height:600px;z-index:0;pointer-events:none;background:radial-gradient(55% 60% at 28% 18%,color-mix(in srgb,var(--q-color-brand) 30%,transparent),transparent 70%),radial-gradient(45% 55% at 82% 8%,color-mix(in srgb,var(--q-color-brand) 20%,transparent),transparent 70%);filter:blur(8px);animation:q-drift 20s var(--q-ease-in-out) infinite alternate}\
@keyframes q-drift{from{transform:translate3d(0,0,0) scale(1)}to{transform:translate3d(4%,2%,0) scale(1.1)}}\
.q-eyebrow .q-pp-crate{margin-left:.6rem;font-family:var(--q-font-mono);font-size:.78rem;text-transform:none;letter-spacing:0;color:var(--q-color-muted);font-weight:var(--q-font-weight-normal)}\
.q-pp-title{max-width:20ch;margin:0 0 var(--q-space-5)}\
.q-pp-lead{max-width:62ch;margin:0 0 var(--q-space-6)}\
/* ---- hero stats ---- */\
.q-pp-stats{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,9rem),1fr));gap:var(--q-space-5);margin:var(--q-space-6) 0 0;padding:var(--q-space-5) 0 0;border-top:1px solid var(--q-color-border)}\
.q-pp-stats .qq-stat__value{color:var(--q-color-brand)}\
/* ---- section heads ---- */\
.q-pp-head{max-width:60ch;margin:0 0 var(--q-space-6)}\
.q-pp-head .q-h2{margin-top:var(--q-space-2)}\
.q-pp-head .q-lead{margin-bottom:0}\
/* ---- feature grid ---- */\
.q-pp-feats{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,17rem),1fr));gap:var(--q-space-4)}\
.q-pp-feats>*{height:100%}\
.q-pp-feat{display:flex;flex-direction:column;gap:var(--q-space-2);height:100%;padding:var(--q-space-5);border-radius:var(--q-radius-lg);transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-pp-feat:hover{transform:translateY(-4px);box-shadow:0 18px 48px -22px color-mix(in srgb,var(--q-color-brand) 55%,transparent)}\
.q-pp-feat[data-q-surface=\"flat\"]:hover,.q-pp-feat[data-q-surface=\"neu\"]:hover{border-color:color-mix(in srgb,var(--q-color-brand) 50%,var(--q-color-border))}\
.q-pp-feat__kicker{font-family:var(--q-font-mono);font-size:.74rem;letter-spacing:.04em;color:var(--q-color-brand)}\
.q-pp-feat[data-q-surface=\"gradient\"] .q-pp-feat__kicker{color:color-mix(in srgb,var(--q-color-on-brand) 80%,transparent)}\
.q-pp-feat__title{margin:0;font-size:1.1rem;font-weight:var(--q-font-weight-bold);letter-spacing:-.01em}\
.q-pp-feat__body{margin:0;font-size:.94rem;line-height:1.65;color:var(--q-color-muted)}\
.q-pp-feat[data-q-surface=\"gradient\"] .q-pp-feat__body{color:color-mix(in srgb,var(--q-color-on-brand) 85%,transparent)}\
/* ---- what's built status list ---- */\
.q-pp-status{list-style:none;margin:0;padding:0;display:flex;flex-direction:column;gap:var(--q-space-3)}\
.q-pp-status__row{display:flex;align-items:flex-start;gap:var(--q-space-3);padding:var(--q-space-4) var(--q-space-5);border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface)}\
.q-pp-status__mark{flex:0 0 auto;display:inline-flex;align-items:center;justify-content:center;width:1.7rem;height:1.7rem;border-radius:var(--q-radius-full);color:var(--q-color-on-brand);background:var(--q-color-brand)}\
.q-pp-status__mark--soon{background:transparent;border:1.5px dashed var(--q-color-muted)}\
.q-pp-status__text{display:flex;flex-direction:column;gap:.15rem;flex:1 1 auto;min-width:0}\
.q-pp-status__label{font-weight:var(--q-font-weight-bold);font-size:1rem;letter-spacing:-.01em}\
.q-pp-status__detail{font-size:.92rem;line-height:1.55;color:var(--q-color-muted)}\
.q-pp-status__chip{flex:0 0 auto;align-self:flex-start;font-size:.66rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.2rem .5rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted)}\
.q-pp-status__chip.is-built{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 45%,transparent);background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
/* ---- closing CTA band ---- */\
.q-pp-cta{text-align:center;padding:calc(var(--q-space-8) * var(--q-density,1)) var(--q-space-5);border-radius:var(--q-radius-xl)}\
.q-pp-cta__h{margin:0 auto var(--q-space-3);max-width:20ch}\
.q-pp-cta__p{margin:0 auto var(--q-space-6);max-width:54ch;color:color-mix(in srgb,var(--q-color-on-brand) 85%,transparent);font-size:1.05rem;line-height:1.6}\
.q-pp-cta__row{justify-content:center;margin:0}\
.q-btn--invert{background:var(--q-color-on-brand);color:var(--q-color-brand)}\
.q-btn--invert:hover{filter:brightness(.96);text-decoration:none}\
.q-btn--on-grad{color:var(--q-color-on-brand);border-color:color-mix(in srgb,var(--q-color-on-brand) 45%,transparent)}\
.q-btn--on-grad:hover{background:color-mix(in srgb,var(--q-color-on-brand) 14%,transparent);text-decoration:none}\
/* ---- reduced motion ---- */\
@media (prefers-reduced-motion:reduce){\
.q-rise{opacity:1;transform:none;animation:none}\
.q-pp-glow{animation:none}\
.q-pp-feat:hover{transform:none}\
}"
}
