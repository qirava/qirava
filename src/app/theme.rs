//! Site layout styles.
//!
//! The *color* system comes from `qquill-theme` (the typed `--q-*` design
//! tokens + the light/dark/contrast contract + the no-flicker boot), inlined by
//! [`crate::app::document`]. This module adds only the SITE-SPECIFIC layout: the
//! page shell, the marketing header/footer, the hero, and the responsive grids
//! the styled components are dropped into. Every value references a `--q-*`
//! token so the whole site restyles on theme switch with no reflow.
//!
//! Authored with the `style!{}` macro (compiled to a compact CSS string at build
//! time, no runtime cost).

/// The site layout CSS, appended after the theme variables in the `<head>`.
pub fn layout_css() -> String {
    let block = qquill_style::style! {
        "*" {
            "box-sizing": "border-box";
        }
        "html" {
            "scroll-behavior": "smooth";
        }
        "body" {
            "margin": "0";
            "min-height": "100vh";
            "display": "flex";
            "flex-direction": "column";
            "background-color": "var(--q-color-bg)";
            "color": "var(--q-color-fg)";
            "font-family": "var(--q-font-sans)";
            "line-height": "1.6";
            "-webkit-font-smoothing": "antialiased";
        }
        "a" {
            "color": "var(--q-color-brand)";
            "text-decoration": "none";
        }
        "a:hover" {
            "text-decoration": "underline";
        }

        // -- page shell -----------------------------------------------------
        ".q-skiplink" {
            "position": "absolute";
            "left": "-9999px";
            "top": "0";
            "padding": "0.5rem 0.875rem";
            "background-color": "var(--q-color-brand)";
            "color": "var(--q-color-on-brand)";
            "border-radius": "0 0 var(--q-radius-md) 0";
            "z-index": "20";
        }
        ".q-skiplink:focus" {
            "left": "0";
        }
        ".q-site-header" {
            "position": "sticky";
            "top": "0";
            "z-index": "10";
            "backdrop-filter": "blur(8px)";
        }
        ".q-site-header .qq-navbar" {
            "max-width": "72rem";
            "margin": "0 auto";
            "justify-content": "space-between";
            "padding-left": "1.5rem";
            "padding-right": "1.5rem";
        }
        ".q-nav-spacer" {
            "flex": "1 1 auto";
        }
        "main.q-main" {
            "flex": "1 1 auto";
            "width": "100%";
            "max-width": "72rem";
            "margin": "0 auto";
            "padding": "3.5rem 1.5rem 5rem";
        }

        // -- typography -----------------------------------------------------
        ".q-eyebrow" {
            "text-transform": "uppercase";
            "letter-spacing": "0.12em";
            "font-size": "0.75rem";
            "font-weight": "var(--q-font-weight-bold)";
            "color": "var(--q-color-brand)";
            "margin": "0 0 0.75rem";
        }
        ".q-h1" {
            "font-size": "clamp(2.25rem, 5vw, 3.25rem)";
            "line-height": "1.08";
            "letter-spacing": "-0.025em";
            "margin": "0 0 1rem";
            "max-width": "20ch";
        }
        ".q-h2" {
            "font-size": "clamp(1.5rem, 3vw, 2rem)";
            "letter-spacing": "-0.02em";
            "margin": "0 0 0.5rem";
        }
        ".q-lead" {
            "color": "var(--q-color-muted)";
            "font-size": "clamp(1.05rem, 2vw, 1.25rem)";
            "max-width": "60ch";
            "margin": "0 0 1.75rem";
        }
        ".q-section" {
            "margin": "4.5rem 0 0";
        }
        ".q-section__head" {
            "max-width": "60ch";
            "margin": "0 0 1.75rem";
        }
        ".q-muted" {
            "color": "var(--q-color-muted)";
        }
        ".q-prose p" {
            "max-width": "62ch";
        }
        ".q-prose a" {
            "text-decoration": "underline";
            "text-underline-offset": "2px";
        }

        // -- hero -----------------------------------------------------------
        ".q-hero" {
            "padding": "1rem 0 0.5rem";
        }
        ".q-cta-row" {
            "display": "flex";
            "flex-wrap": "wrap";
            "gap": "0.75rem";
            "align-items": "center";
            "margin": "0 0 2.5rem";
        }

        // -- link buttons (CTAs navigate, so they are <a>, not <button>) -----
        ".q-btn" {
            "display": "inline-flex";
            "align-items": "center";
            "gap": "0.4rem";
            "padding": "0.6rem 1.1rem";
            "border-radius": "var(--q-radius-md)";
            "font-weight": "var(--q-font-weight-medium)";
            "font-size": "var(--q-font-size-md)";
            "line-height": "1";
            "border": "1px solid transparent";
            "cursor": "pointer";
            "transition": "background-color var(--q-duration-fast), color var(--q-duration-fast)";
        }
        ".q-btn:hover" {
            "text-decoration": "none";
        }
        ".q-btn:focus-visible" {
            "outline": "2px solid var(--q-color-brand)";
            "outline-offset": "2px";
        }
        ".q-btn--solid" {
            "background-color": "var(--q-color-brand)";
            "color": "var(--q-color-on-brand)";
        }
        ".q-btn--solid:hover" {
            "background-color": "var(--q-color-brand)";
            "filter": "brightness(1.08)";
        }
        ".q-btn--ghost" {
            "color": "var(--q-color-fg)";
            "border-color": "var(--q-color-border)";
        }
        ".q-btn--ghost:hover" {
            "background-color": "var(--q-color-surface)";
        }
        ".q-statbar" {
            "display": "flex";
            "flex-wrap": "wrap";
            "gap": "2.5rem";
            "padding": "1.75rem 0 0";
            "border-top": "1px solid var(--q-color-border)";
        }
        ".q-statbar .qq-stat__value" {
            "color": "var(--q-color-brand)";
        }

        // -- grids ----------------------------------------------------------
        ".q-grid" {
            "display": "grid";
            "grid-template-columns": "repeat(auto-fit, minmax(min(100%, 18rem), 1fr))";
            "gap": "1.25rem";
        }
        ".q-grid .qq-card" {
            "height": "100%";
        }
        ".q-card-eyebrow" {
            "display": "flex";
            "align-items": "center";
            "gap": "0.625rem";
            "font-weight": "var(--q-font-weight-bold)";
            "font-size": "1.05rem";
        }
        ".q-card-eyebrow code" {
            "font-size": "0.8em";
            "color": "var(--q-color-muted)";
            "font-weight": "var(--q-font-weight-normal)";
        }
        ".q-card-actions" {
            "display": "flex";
            "gap": "0.5rem";
            "flex-wrap": "wrap";
        }
        ".q-list" {
            "margin": "0.5rem 0 0";
            "padding-left": "1.1rem";
            "color": "var(--q-color-muted)";
        }
        ".q-list li" {
            "margin": "0.25rem 0";
        }

        // -- code block -----------------------------------------------------
        ".q-code" {
            "background-color": "var(--q-color-surface)";
            "border": "1px solid var(--q-color-border)";
            "border-radius": "var(--q-radius-lg)";
            "padding": "1.1rem 1.25rem";
            "overflow-x": "auto";
            "margin": "0";
            "font-family": "var(--q-font-mono)";
            "font-size": "0.9rem";
            "line-height": "1.7";
        }
        ".q-code .q-prompt" {
            "color": "var(--q-color-brand)";
            "user-select": "none";
        }
        ".q-code .q-comment" {
            "color": "var(--q-color-muted)";
        }
        "code.q-inline" {
            "background-color": "var(--q-color-surface)";
            "border": "1px solid var(--q-color-border)";
            "border-radius": "var(--q-radius-sm)";
            "padding": "0.1rem 0.35rem";
            "font-family": "var(--q-font-mono)";
            "font-size": "0.88em";
        }

        // -- footer ---------------------------------------------------------
        ".q-site-footer" {
            "border-top": "1px solid var(--q-color-border)";
            "background-color": "var(--q-color-surface)";
            "color": "var(--q-color-muted)";
        }
        ".q-site-footer__inner" {
            "max-width": "72rem";
            "margin": "0 auto";
            "padding": "2rem 1.5rem";
            "display": "flex";
            "flex-wrap": "wrap";
            "gap": "1rem 2rem";
            "align-items": "center";
            "justify-content": "space-between";
            "font-size": "var(--q-font-size-sm)";
        }
        ".q-site-footer__links" {
            "display": "flex";
            "flex-wrap": "wrap";
            "gap": "1.25rem";
        }
        ".q-site-footer a" {
            "color": "var(--q-color-muted)";
        }
        ".q-site-footer a:hover" {
            "color": "var(--q-color-fg)";
        }

        // -- roadmap table --------------------------------------------------
        ".q-table-wrap" {
            "overflow-x": "auto";
        }
        ".q-table-wrap .qq-table" {
            "width": "100%";
        }
    };
    let mut out = block.to_css();
    out.push_str(motion_css());
    out
}

/// Motion, scroll-reveal, the animated hero, card hover depth, the docs layout,
/// and the component-playground styling. Authored as a plain string (it uses
/// `@keyframes`, custom-property easing tokens, and attribute selectors the
/// `style!{}` macro is not shaped for). Every value still references `--q-*`
/// theme tokens so it restyles on theme switch. All motion is wrapped so the
/// `prefers-reduced-motion` reset (`qquill_design::reduced_motion_css`) can
/// neutralize it.
fn motion_css() -> &'static str {
    "\
:root{--q-ease-out:cubic-bezier(.16,1,.3,1);--q-ease-in-out:cubic-bezier(.65,0,.35,1)}\
.q-main{flex:1 1 auto;width:100%;max-width:72rem;margin:0 auto;padding:3.5rem 1.5rem 5rem}\
/* ---- scroll reveal (the `reveal` island toggles data-revealed) ---- */\
[data-q-reveal]{opacity:0;transform:translateY(18px);transition:opacity .6s var(--q-ease-out),transform .6s var(--q-ease-out)}\
[data-q-reveal][data-revealed=\"true\"]{opacity:1;transform:none}\
[data-q-reveal][data-reveal-delay=\"1\"]{transition-delay:.08s}\
[data-q-reveal][data-reveal-delay=\"2\"]{transition-delay:.16s}\
[data-q-reveal][data-reveal-delay=\"3\"]{transition-delay:.24s}\
/* ---- hero ---- */\
.q-hero{position:relative;padding:2.5rem 0 1rem;overflow:hidden}\
.q-hero__glow{position:absolute;inset:-40% -10% auto -10%;height:520px;z-index:0;pointer-events:none;background:radial-gradient(60% 60% at 30% 20%,color-mix(in srgb,var(--q-color-brand) 26%,transparent),transparent 70%),radial-gradient(50% 50% at 80% 10%,color-mix(in srgb,var(--q-color-brand) 16%,transparent),transparent 70%);filter:blur(6px);animation:q-drift 18s var(--q-ease-in-out) infinite alternate}\
.q-hero>*{position:relative;z-index:1}\
@keyframes q-drift{from{transform:translate3d(0,0,0) scale(1)}to{transform:translate3d(4%,2%,0) scale(1.08)}}\
.q-h1 .q-accent{background:linear-gradient(100deg,var(--q-color-brand),color-mix(in srgb,var(--q-color-brand) 55%,var(--q-color-fg)));-webkit-background-clip:text;background-clip:text;color:transparent}\
.q-hero__in{display:inline-block;opacity:0;transform:translateY(14px);animation:q-rise .7s var(--q-ease-out) forwards}\
.q-hero__in.d1{animation-delay:.06s}.q-hero__in.d2{animation-delay:.14s}.q-hero__in.d3{animation-delay:.22s}.q-hero__in.d4{animation-delay:.30s}\
@keyframes q-rise{to{opacity:1;transform:none}}\
.q-pillars{display:grid;grid-template-columns:1fr 1fr;gap:1rem;margin:1.75rem 0 0}\
@media (max-width:640px){.q-pillars{grid-template-columns:1fr}}\
.q-pillar{border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);padding:1.1rem 1.25rem;background:var(--q-color-surface)}\
.q-pillar h3{margin:.1rem 0 .35rem;font-size:1.05rem}\
.q-pillar p{margin:0;color:var(--q-color-muted);font-size:.95rem}\
.q-pillar__k{font-family:var(--q-font-mono);font-size:.78rem;color:var(--q-color-brand)}\
/* ---- product cards hover depth ---- */\
.q-grid .qq-card{transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-grid .qq-card:hover{transform:translateY(-4px);box-shadow:0 14px 40px -18px color-mix(in srgb,var(--q-color-brand) 55%,transparent);border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-color-border))}\
/* ---- live teaser ---- */\
.q-teaser{border:1px solid var(--q-color-border);border-radius:var(--q-radius-xl);background:var(--q-color-surface);padding:1.25rem;margin-top:1.5rem}\
.q-teaser__chrome{display:flex;gap:.4rem;margin:0 0 1rem}\
.q-teaser__chrome span{width:11px;height:11px;border-radius:var(--q-radius-full);background:var(--q-color-border)}\
/* ---- docs layout ---- */\
.q-docs{display:grid;grid-template-columns:16rem minmax(0,1fr) 14rem;gap:2rem;align-items:start;max-width:80rem;margin:0 auto;padding:2.5rem 1.5rem 5rem;width:100%}\
@media (max-width:980px){.q-docs{grid-template-columns:1fr}.q-docs__toc{display:none}.q-docs__side{display:none}}\
.q-docs__side{position:sticky;top:5rem;align-self:start}\
.q-docs__sec{margin:0 0 1.25rem}\
.q-docs__sec-label{font-size:.75rem;text-transform:uppercase;letter-spacing:.09em;color:var(--q-color-muted);font-weight:var(--q-font-weight-bold);margin:0 0 .5rem}\
.q-docs__nav{display:flex;flex-direction:column;gap:.1rem}\
.q-docs__nav a{color:var(--q-color-muted);font-size:.93rem;padding:.32rem .6rem;border-radius:var(--q-radius-md);border-left:2px solid transparent}\
.q-docs__nav a:hover{color:var(--q-color-fg);background:var(--q-color-surface);text-decoration:none}\
.q-docs__nav a[aria-current=\"page\"]{color:var(--q-color-brand);border-left-color:var(--q-color-brand);background:color-mix(in srgb,var(--q-color-brand) 8%,transparent)}\
.q-docs__main{min-width:0}\
.q-docs__main h1{font-size:clamp(1.9rem,4vw,2.5rem);letter-spacing:-.02em;margin:0 0 .75rem}\
.q-docs__toc{position:sticky;top:5rem;align-self:start;font-size:.88rem}\
.q-docs__toc-label{text-transform:uppercase;letter-spacing:.09em;font-size:.72rem;color:var(--q-color-muted);font-weight:var(--q-font-weight-bold);margin:0 0 .6rem}\
.q-docs__toc a{display:block;color:var(--q-color-muted);padding:.2rem 0;line-height:1.4}\
.q-docs__toc a:hover{color:var(--q-color-fg);text-decoration:none}\
.q-docs__main .qq-heading{scroll-margin-top:5rem;margin-top:2rem}\
.q-docs__main p{max-width:68ch;color:var(--q-color-fg)}\
.q-docs__main ul{max-width:68ch;color:var(--q-color-fg)}\
/* ---- copy-code button (the `copy` island) ---- */\
.q-codewrap{position:relative;margin:1.25rem 0}\
.q-copy{position:absolute;top:.55rem;right:.55rem;display:inline-flex;align-items:center;gap:.3rem;font-size:.78rem;font-family:var(--q-font-sans);padding:.3rem .55rem;border-radius:var(--q-radius-md);border:1px solid var(--q-color-border);background:var(--q-color-bg);color:var(--q-color-muted);cursor:pointer;opacity:0;transition:opacity var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast)}\
.q-codewrap:hover .q-copy,.q-copy:focus-visible{opacity:1}\
.q-copy:hover{color:var(--q-color-fg);border-color:var(--q-color-brand)}\
.q-copy[data-copied=\"true\"]{color:var(--q-color-brand);opacity:1}\
/* ---- component showcase index ---- */\
.q-comp-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(15rem,1fr));gap:1rem;margin-top:1.5rem}\
.q-comp-card{display:block;border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);padding:1.1rem 1.25rem;background:var(--q-color-surface);transition:transform var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-comp-card:hover{transform:translateY(-3px);border-color:var(--q-color-brand);text-decoration:none}\
.q-comp-card h3{margin:0 0 .25rem;color:var(--q-color-fg);font-size:1.05rem}\
.q-comp-card p{margin:0;color:var(--q-color-muted);font-size:.9rem}\
/* ---- the PLAYGROUND (the `playground` island) ---- */\
.pg{border:1px solid var(--q-color-border);border-radius:var(--q-radius-xl);overflow:hidden;margin:1.5rem 0}\
.pg__stage{display:flex;align-items:center;justify-content:center;gap:1rem;flex-wrap:wrap;min-height:180px;padding:2.5rem;background:radial-gradient(120% 120% at 50% 0%,var(--q-color-surface),var(--q-color-bg))}\
.pg__cell{display:none}\
.pg__cell[data-active=\"true\"]{display:inline-flex}\
.pg__controls{display:flex;flex-wrap:wrap;gap:1.5rem;padding:1rem 1.25rem;border-top:1px solid var(--q-color-border);background:var(--q-color-surface)}\
.pg__group{display:flex;flex-direction:column;gap:.4rem}\
.pg__legend{font-size:.72rem;text-transform:uppercase;letter-spacing:.08em;color:var(--q-color-muted);font-weight:var(--q-font-weight-bold)}\
.pg__seg{display:inline-flex;border:1px solid var(--q-color-border);border-radius:var(--q-radius-md);overflow:hidden}\
.pg__opt{appearance:none;border:0;background:transparent;color:var(--q-color-muted);font:inherit;font-size:.85rem;padding:.35rem .7rem;cursor:pointer;transition:background-color var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast) var(--q-ease-out)}\
.pg__opt+.pg__opt{border-left:1px solid var(--q-color-border)}\
.pg__opt:hover{color:var(--q-color-fg)}\
.pg__opt[aria-pressed=\"true\"]{background:var(--q-color-brand);color:var(--q-color-on-brand)}\
.pg__codewrap{position:relative}\
.pg .q-code{border-radius:0;border-left:0;border-right:0;border-bottom:0}\
.pg__code-empty{display:none}"
}
