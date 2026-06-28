//! Reusable site-level UI primitives for qirava.
//!
//! This is the website design system layer: route files should describe content
//! and flow, while this module owns the repeated markup contract (page frames,
//! section headers, cards, metrics, path steps, product/status/doc tiles, and
//! CTA bands). The companion [`css`] function is inlined once for the whole site
//! from `app::document`, so pages do not need bespoke CSS for common layout.
//!
//! The contract is deliberately calm and readable: global controls are color
//! mode, density, and radius. Glass/neu/gradient are scoped to Quill component
//! demos only, never applied across reading content.

use qquill_view::{el, raw, text, Node};

/// Shared arrow icon for text links and CTAs.
pub const ARROW_SVG: &str = "<svg class=\"q-arr\" viewBox=\"0 0 24 24\" width=\"15\" height=\"15\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M5 12h14M13 6l6 6-6 6\"/></svg>";

fn status_class(state: &str) -> &'static str {
    match state {
        "built" | "Built" | "BUILT" => "built",
        "partial" | "Partial" | "PARTIAL" => "partial",
        "planned" | "Planned" | "PLANNED" => "planned",
        "ssot" | "SSOT" => "ssot",
        _ => "neutral",
    }
}

/// Main frame used by marketing hubs. `class` is an optional route modifier.
pub fn page_frame(class: &str) -> Node {
    let cls = if class.is_empty() {
        "q-page".to_string()
    } else {
        format!("q-page {class}")
    };
    el("main").class(cls).id("main")
}

/// A token-driven section header. `centered` only changes alignment, not scale.
pub fn section_head(eyebrow: &str, title: &str, lead: &str, centered: bool) -> Node {
    let class = if centered {
        "q-ui-head q-ui-head--center"
    } else {
        "q-ui-head"
    };
    el("div")
        .class(class)
        .child(el("p").class("q-eyebrow").child(text(eyebrow.to_string())))
        .child(el("h2").class("q-h2").child(text(title.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())))
}

/// A page-level header with an `h1` for route landing pages.
pub fn page_head(eyebrow: &str, title: &str, lead: &str) -> Node {
    el("div")
        .class("q-ui-head q-ui-page-head")
        .child(el("p").class("q-eyebrow").child(text(eyebrow.to_string())))
        .child(el("h1").class("q-h1").child(text(title.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())))
}

/// A reusable arrow link. `tone` is a class suffix (`primary`, `ghost`, `plain`,
/// `invert`, or `on-grad`) so routes do not hand-roll link/button markup.
pub fn action_link(label: &str, href: &str, tone: &str) -> Node {
    el("a")
        .class(format!("q-ui-link q-ui-link--{tone}"))
        .attr("href", href.to_string())
        .child(text(label.to_string()))
        .child(raw(ARROW_SVG))
}

/// A status chip: `built`, `partial`, `planned`, `ssot`, or `neutral`.
pub fn chip(label: &str, state: &str) -> Node {
    el("span")
        .class(format!("q-ui-chip q-ui-chip--{}", status_class(state)))
        .child(text(label.to_string()))
}

/// A token-driven surface card root. Callers add children.
pub fn card(class: &str) -> Node {
    el("article")
        .class(format!("q-ui-card {class}"))
        .attr("data-q-tilt", "")
}

/// Metric tile used in hero and summary strips.
pub fn metric(value: &str, label: &str) -> Node {
    el("div")
        .class("q-ui-metric")
        .child(el("strong").child(text(value.to_string())))
        .child(el("span").child(text(label.to_string())))
}

/// Product/status overview card for home and `/products`.
pub fn product_card(
    name: &str,
    crate_name: &str,
    status: &str,
    summary: &str,
    bullets: &[&str],
    href: &str,
    cta: &str,
) -> Node {
    let mut list = el("ul").class("q-ui-list");
    for b in bullets {
        list = list.child(el("li").child(text((*b).to_string())));
    }

    card("q-product-card")
        .child(
            el("div")
                .class("q-product-card__top")
                .child(
                    el("div")
                        .child(
                            el("h3")
                                .class("q-product-card__name")
                                .child(text(name.to_string())),
                        )
                        .child(
                            el("code")
                                .class("q-product-card__crate")
                                .child(text(crate_name.to_string())),
                        ),
                )
                .child(chip(status, status)),
        )
        .child(
            el("p")
                .class("q-product-card__summary")
                .child(text(summary.to_string())),
        )
        .child(list)
        .child(action_link(cta, href, "plain"))
}

/// Feature card with an optional kicker.
pub fn feature_card(kicker: &str, title: &str, body: &str) -> Node {
    let mut c = card("q-feature-card");
    if !kicker.is_empty() {
        c = c.child(
            el("span")
                .class("q-feature-card__kicker")
                .child(text(kicker.to_string())),
        );
    }
    c.child(
        el("h3")
            .class("q-feature-card__title")
            .child(text(title.to_string())),
    )
    .child(
        el("p")
            .class("q-feature-card__body")
            .child(text(body.to_string())),
    )
}

/// Ordered human path step.
pub fn path_step(n: &str, title: &str, body: &str, href: &str, cta: &str) -> Node {
    el("li")
        .class("q-path-step")
        .attr("data-q-reveal", "")
        .child(
            el("span")
                .class("q-path-step__n")
                .child(text(n.to_string())),
        )
        .child(
            el("div")
                .class("q-path-step__body")
                .child(
                    el("h3")
                        .class("q-path-step__title")
                        .child(text(title.to_string())),
                )
                .child(
                    el("p")
                        .class("q-path-step__copy")
                        .child(text(body.to_string())),
                )
                .child(action_link(cta, href, "plain")),
        )
}

/// Docs/product link card.
pub fn link_card(title: &str, eyebrow: &str, body: &str, href: &str, cta: &str) -> Node {
    el("a")
        .class("q-link-card")
        .attr("href", href.to_string())
        .attr("data-q-tilt", "")
        .child(
            el("span")
                .class("q-link-card__eyebrow")
                .child(text(eyebrow.to_string())),
        )
        .child(
            el("h3")
                .class("q-link-card__title")
                .child(text(title.to_string())),
        )
        .child(
            el("p")
                .class("q-link-card__body")
                .child(text(body.to_string())),
        )
        .child(
            el("span")
                .class("q-link-card__cta")
                .child(text(cta.to_string()))
                .child(raw(ARROW_SVG)),
        )
}

/// Roadmap summary card with built/partial/planned chips.
pub fn roadmap_card(
    product: &str,
    crate_name: &str,
    summary: &str,
    chips: &[(&str, &str)],
    href: &str,
) -> Node {
    let mut chip_row = el("div").class("q-road-card__chips");
    for (label, state) in chips {
        chip_row = chip_row.child(chip(label, state));
    }

    card("q-road-card")
        .child(
            el("div")
                .class("q-road-card__top")
                .child(
                    el("h3")
                        .class("q-road-card__title")
                        .child(text(product.to_string())),
                )
                .child(
                    el("code")
                        .class("q-road-card__crate")
                        .child(text(crate_name.to_string())),
                ),
        )
        .child(
            el("p")
                .class("q-road-card__body")
                .child(text(summary.to_string())),
        )
        .child(chip_row)
        .child(action_link("Open roadmap", href, "plain"))
}

/// Status row/card used for SSOT boards.
pub fn status_card(
    product: &str,
    built: &str,
    planned: &str,
    docs_href: &str,
    roadmap_href: &str,
) -> Node {
    card("q-status-card")
        .child(
            el("div")
                .class("q-status-card__top")
                .child(
                    el("h3")
                        .class("q-status-card__title")
                        .child(text(product.to_string())),
                )
                .child(chip("SSOT", "ssot")),
        )
        .child(
            el("div")
                .class("q-status-card__row")
                .child(chip("Built", "built"))
                .child(
                    el("p")
                        .class("q-status-card__body")
                        .child(text(built.to_string())),
                ),
        )
        .child(
            el("div")
                .class("q-status-card__row")
                .child(chip("Planned", "planned"))
                .child(
                    el("p")
                        .class("q-status-card__body")
                        .child(text(planned.to_string())),
                ),
        )
        .child(
            el("div")
                .class("q-status-card__links")
                .child(action_link("Docs", docs_href, "plain"))
                .child(action_link("Roadmap", roadmap_href, "plain")),
        )
}

/// Brand CTA band. The gradient is reserved for this short, high-contrast band,
/// not normal reading cards.
pub fn cta_band(heading: &str, body: &str, primary: (&str, &str), secondary: (&str, &str)) -> Node {
    el("section")
        .class("q-cta-band")
        .child(el("h2").class("q-h2").child(text(heading.to_string())))
        .child(el("p").child(text(body.to_string())))
        .child(
            el("div")
                .class("q-cta-row q-cta-band__row")
                .child(action_link(primary.0, primary.1, "invert"))
                .child(action_link(secondary.0, secondary.1, "on-grad")),
        )
}

/// Central website CSS for the qirava site UI primitives. Inlined once in
/// `app::document` after the theme/layout tokens and before route/component CSS.
pub fn css() -> &'static str {
    "\
/* ---- qirava website design system: page, cards, flow ---- */\
.q-page{flex:1 1 auto;width:100%;max-width:var(--q-page-max,80rem);margin:0 auto;padding:var(--q-space-8) var(--q-page-pad,1.5rem) var(--q-space-10)}\
.q-ui-page-head{max-width:66ch;margin-bottom:var(--q-space-6)}\
.q-ui-head{max-width:64ch;margin:0 0 var(--q-space-6)}\
.q-ui-head--center{margin-left:auto;margin-right:auto;text-align:center}.q-ui-head--center .q-lead{margin-left:auto;margin-right:auto}\
.q-ui-head .q-h2{margin-top:var(--q-space-2)}.q-ui-head .q-lead{margin-bottom:0}\
.q-section{margin-top:var(--q-section-gap)}\
.q-section--tight{margin-top:var(--q-space-8)}\
.q-ui-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,18rem),1fr));gap:var(--q-space-4)}\
.q-ui-grid--2{grid-template-columns:repeat(2,minmax(0,1fr))}.q-ui-grid--3{grid-template-columns:repeat(3,minmax(0,1fr))}.q-ui-grid--4{grid-template-columns:repeat(4,minmax(0,1fr))}\
@media (max-width:900px){.q-ui-grid--2,.q-ui-grid--3,.q-ui-grid--4{grid-template-columns:1fr}}\
.q-arr{transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-ui-link{display:inline-flex;align-items:center;gap:.38rem;font-weight:var(--q-font-weight-medium);font-size:.93rem;color:var(--q-color-brand);line-height:1.2;transition:color var(--q-duration-fast) var(--q-ease-out),background var(--q-duration-fast) var(--q-ease-out),border-color var(--q-duration-fast) var(--q-ease-out),transform var(--q-duration-fast) var(--q-ease-out)}\
.q-ui-link:hover{text-decoration:none}.q-ui-link:hover .q-arr{transform:translateX(3px)}\
.q-ui-link--primary,.q-ui-link--ghost,.q-ui-link--invert,.q-ui-link--on-grad{min-height:var(--q-control-h);padding-inline:var(--q-control-pad-x);border-radius:var(--q-radius-md);border:1px solid transparent;box-shadow:none}\
.q-ui-link--primary{background:linear-gradient(120deg,var(--q-color-brand),var(--q-color-accent));color:var(--q-color-on-brand);box-shadow:0 16px 42px -28px var(--q-color-brand)}\
.q-ui-link--primary:hover{filter:saturate(1.08) brightness(1.04);box-shadow:0 22px 54px -30px var(--q-color-brand)}\
.q-ui-link--ghost{color:var(--q-color-fg);border-color:var(--q-surface-border,var(--q-color-border));background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none)}\
.q-ui-link--ghost:hover{border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-surface-border,var(--q-color-border)));background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-hover-shadow,var(--q-surface-shadow,none))}\
.q-ui-link--invert{background:var(--q-color-on-brand);color:var(--q-color-brand)}\
.q-ui-link--on-grad{color:var(--q-color-on-brand);border-color:color-mix(in srgb,var(--q-color-on-brand) 45%,transparent)}\
.q-ui-link--on-grad:hover{background:color-mix(in srgb,var(--q-color-on-brand) 14%,transparent)}\
.q-ui-chip{display:inline-flex;align-items:center;justify-content:center;font-size:.66rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.23rem .55rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted);line-height:1.15;white-space:nowrap}\
.q-ui-chip--built,.q-ui-chip--ssot{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 45%,transparent);background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
.q-ui-chip--partial{color:var(--q-color-fg);border-color:color-mix(in srgb,var(--q-color-fg) 30%,transparent);background:color-mix(in srgb,var(--q-color-fg) 7%,transparent)}\
.q-ui-chip--planned,.q-ui-chip--neutral{color:var(--q-color-muted);background:color-mix(in srgb,var(--q-color-fg) 5%,transparent)}\
.q-ui-card,.q-link-card{position:relative;overflow:hidden;display:flex;flex-direction:column;gap:var(--q-space-3);padding:var(--q-surface-pad);border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-lg);background:var(--q-surface-bg,var(--q-color-surface));color:var(--q-color-fg);box-shadow:var(--q-surface-shadow,0 1px 0 color-mix(in srgb,var(--q-color-fg) 4%,transparent));-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out)}\
.q-ui-card::before,.q-link-card::before{content:\"\";position:absolute;inset:0 0 auto 0;height:3px;background:var(--q-surface-accent,transparent);opacity:var(--q-surface-accent-opacity,0);pointer-events:none}\
.q-ui-card:hover,.q-link-card:hover{border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-surface-border,var(--q-color-border)));box-shadow:var(--q-surface-hover-shadow,0 22px 54px -30px color-mix(in srgb,var(--q-color-brand) 55%,transparent));text-decoration:none}\
.q-ui-card p,.q-link-card p{color:var(--q-color-muted)}\
.q-ui-list{margin:0;padding-left:1.1rem;color:var(--q-color-muted);font-size:.93rem;line-height:1.6}.q-ui-list li{margin:.22rem 0}\
.q-ui-metric{position:relative;overflow:hidden;display:flex;flex-direction:column;gap:var(--q-space-1);min-width:0;padding:var(--q-space-4);border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-lg);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out)}\
.q-ui-metric:hover{transform:translateY(-4px);border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-surface-border,var(--q-color-border)));box-shadow:var(--q-surface-hover-shadow,0 22px 54px -30px color-mix(in srgb,var(--q-color-brand) 55%,transparent))}\
.q-ui-metric::before{content:\"\";position:absolute;inset:0 0 auto 0;height:3px;background:var(--q-surface-accent,transparent);opacity:var(--q-surface-accent-opacity,0);pointer-events:none}\
.q-ui-metric strong{font-size:clamp(1.45rem,3vw,2.1rem);line-height:1;color:var(--q-color-brand);letter-spacing:-.04em}.q-ui-metric span{font-size:.82rem;color:var(--q-color-muted);line-height:1.35}\
.q-product-card__top,.q-road-card__top,.q-status-card__top{display:flex;align-items:flex-start;justify-content:space-between;gap:var(--q-space-3)}\
.q-product-card__name,.q-road-card__title,.q-status-card__title{margin:0;font-size:1.16rem;line-height:1.2;letter-spacing:-.02em}\
.q-product-card__crate,.q-road-card__crate{font-family:var(--q-font-mono);font-size:.78rem;color:var(--q-color-muted)}\
.q-product-card__summary,.q-road-card__body,.q-status-card__body{margin:0;line-height:1.62}\
.q-product-card .q-ui-link,.q-road-card .q-ui-link{margin-top:auto}\
.q-feature-card__kicker,.q-link-card__eyebrow{font-family:var(--q-font-mono);font-size:.74rem;letter-spacing:.04em;color:var(--q-color-brand)}\
.q-feature-card__title,.q-link-card__title{margin:0;font-size:1.08rem;line-height:1.25;letter-spacing:-.015em}.q-feature-card__body,.q-link-card__body{margin:0;line-height:1.62}\
.q-link-card__cta{margin-top:auto;display:inline-flex;align-items:center;gap:.35rem;color:var(--q-color-brand);font-weight:var(--q-font-weight-medium);font-size:.92rem}.q-link-card:hover .q-arr{transform:translateX(3px)}\
.q-path{list-style:none;margin:0;padding:0;display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,20rem),1fr));gap:var(--q-space-4)}\
.q-path-step{position:relative;overflow:hidden;display:flex;gap:var(--q-space-4);padding:var(--q-surface-pad);border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-lg);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none)}\
.q-path-step::before{content:\"\";position:absolute;inset:0 0 auto 0;height:3px;background:var(--q-surface-accent,transparent);opacity:var(--q-surface-accent-opacity,0);pointer-events:none}\
.q-path-step__n{flex:0 0 auto;display:inline-flex;align-items:center;justify-content:center;width:2.2rem;height:2.2rem;border-radius:var(--q-radius-full);font-family:var(--q-font-mono);font-weight:var(--q-font-weight-bold);font-size:.85rem;color:var(--q-color-on-brand);background:var(--q-color-brand)}\
.q-path-step__body{display:flex;flex-direction:column;gap:.35rem;min-width:0}.q-path-step__title{margin:0;font-size:1.05rem;letter-spacing:-.01em}.q-path-step__copy{margin:0;color:var(--q-color-muted);font-size:.93rem;line-height:1.6}\
.q-status-card__row{display:grid;grid-template-columns:auto minmax(0,1fr);gap:var(--q-space-3);align-items:start}.q-status-card__links{display:flex;gap:var(--q-space-4);flex-wrap:wrap;margin-top:auto}\
.q-road-card__chips{display:flex;flex-wrap:wrap;gap:.45rem}.q-road-card .q-ui-link{margin-top:auto}\
.q-hero2{position:relative;isolation:isolate;display:grid;grid-template-columns:minmax(0,1.1fr) minmax(18rem,.9fr);gap:var(--q-space-8);align-items:center;padding:var(--q-space-8) 0 var(--q-space-6);overflow:visible}\
@media (max-width:900px){.q-hero2{grid-template-columns:1fr;gap:var(--q-space-6)}}\
.q-hero2::before{content:\"\";position:absolute;left:-7rem;top:-4rem;width:min(46rem,62vw);height:min(46rem,62vw);z-index:-1;pointer-events:none;border-radius:var(--q-radius-full);background:radial-gradient(circle at 34% 28%,color-mix(in srgb,var(--q-color-brand) 18%,transparent),transparent 58%),radial-gradient(circle at 68% 42%,color-mix(in srgb,var(--q-color-accent) 12%,transparent),transparent 64%);filter:blur(34px);opacity:.44;animation:q-drift 22s var(--q-ease-in-out) infinite alternate}\
.q-hero2__title{font-size:clamp(2.6rem,7vw,5.8rem);line-height:.96;letter-spacing:-.065em;margin:0 0 var(--q-space-5);max-width:11ch}.q-hero2__accent,.q-grad{background:linear-gradient(105deg,var(--q-color-brand),var(--q-color-accent));-webkit-background-clip:text;background-clip:text;color:transparent}\
.q-hero2__lead{max-width:65ch;margin:0 0 var(--q-space-6);font-size:clamp(1.05rem,2.1vw,1.28rem);line-height:1.65;color:var(--q-color-muted)}\
.q-hero2__metrics{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:var(--q-space-3)}\
.q-hero-panel{position:relative;overflow:hidden;padding:var(--q-space-5);border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-xl);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-hover-shadow,0 30px 80px -48px color-mix(in srgb,var(--q-color-brand) 70%,transparent));-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none)}\
.q-hero-panel::before{content:\"\";position:absolute;inset:0 0 auto 0;height:3px;background:var(--q-surface-accent,transparent);opacity:var(--q-surface-accent-opacity,0);pointer-events:none}\
.q-hero-panel__label{margin:0 0 var(--q-space-3);font-size:.78rem;text-transform:uppercase;letter-spacing:.1em;color:var(--q-color-brand);font-weight:var(--q-font-weight-bold)}\
.q-hero-flow{list-style:none;margin:0;padding:0;display:flex;flex-direction:column;gap:var(--q-space-3)}\
.q-hero-flow li{display:grid;grid-template-columns:2.6rem minmax(0,1fr);gap:var(--q-space-3);align-items:start;padding:var(--q-space-3);border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-lg);background:color-mix(in srgb,var(--q-color-bg) 82%,var(--q-color-surface))}\
.q-hero-flow strong{font-size:.93rem}.q-hero-flow span{display:block;color:var(--q-color-muted);font-size:.82rem;line-height:1.45}.q-hero-flow code{font-family:var(--q-font-mono);font-size:.78rem;color:var(--q-color-brand)}\
.q-cta-row{display:flex;flex-wrap:wrap;gap:var(--q-space-3);align-items:center}\
.q-cta-band{position:relative;overflow:hidden;text-align:center;margin-top:var(--q-section-gap);padding:var(--q-hero-pad,var(--q-space-8)) var(--q-space-5);border-radius:var(--q-radius-xl);background:var(--q-effect-gradient-brand);color:var(--q-color-on-brand);box-shadow:var(--q-shadow-lg)}\
.q-cta-band::after{content:\"\";position:absolute;inset:0;z-index:0;pointer-events:none;background:radial-gradient(60% 120% at 15% 0%,color-mix(in srgb,#fff 20%,transparent),transparent 60%),radial-gradient(50% 120% at 90% 100%,color-mix(in srgb,#000 16%,transparent),transparent 60%)}\
.q-cta-band>*{position:relative;z-index:1}.q-cta-band h2{max-width:22ch;margin:0 auto var(--q-space-3)}.q-cta-band p{max-width:58ch;margin:0 auto var(--q-space-6);color:color-mix(in srgb,var(--q-color-on-brand) 88%,transparent);font-size:1.04rem;line-height:1.6}.q-cta-band__row{justify-content:center}\
@media (prefers-reduced-motion:reduce){.q-hero2::before{animation:none}}\
"
}
