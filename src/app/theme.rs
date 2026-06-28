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
            "max-width": "var(--q-page-max,80rem)";
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
            "max-width": "var(--q-page-max,80rem)";
            "margin": "0 auto";
            "padding": "3.5rem var(--q-page-pad,1.5rem) 5rem";
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
            "background": "var(--q-surface-bg,var(--q-color-surface))";
            "border-color": "var(--q-surface-border,var(--q-color-border))";
            "box-shadow": "var(--q-surface-shadow,none)";
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

        // -- footer: the single owner of footer styling is `shell::chrome_css`.
        //    (A duplicate block lived here and shipped a conflicting
        //    `display:flex` that collapsed the footer columns to the left; it
        //    was removed so the grid layout in the chrome is authoritative.)

        // -- roadmap table --------------------------------------------------
        ".q-table-wrap" {
            "overflow-x": "auto";
        }
        ".q-table-wrap .qq-table" {
            "width": "100%";
        }
    };
    // Order matters (CSS cascade, equal specificity): the layout block first,
    // then the token/type-scale layer so its base element styles win over the
    // older inline rules, then surfaces, then the density/radius re-points.
    let mut out = block.to_css();
    out.push_str(tokens_css());
    out.push_str(surface_css());
    // Density + radius axes are generated from the single design config.
    out.push_str(&crate::app::design::scale_css());
    out.push_str(motion_css());
    // Themed scrollbars: replace the default OS/browser scrollbar with one that
    // reads the design tokens, so overflow areas (code blocks, tables, the docs
    // sidebar, any pane) match the theme in both light and dark.
    out.push_str(scrollbar_css());
    out
}

/// Site-wide themed scrollbars. Authored as raw CSS because the `style!{}` macro
/// is not shaped for the `::-webkit-scrollbar*` pseudo-elements or the Firefox
/// `scrollbar-*` shorthands. Every value is a `--q-*` token (with literal
/// fallbacks) so the scrollbar restyles with theme/accent/surface like the rest
/// of the system. The thumb uses a brand-tinted, semi-transparent fill over the
/// surface track; hover/active deepen it. Thin width keeps it unobtrusive.
fn scrollbar_css() -> &'static str {
    "\
/* ---- themed scrollbars (design-system, not the default browser chrome) ---- */\
:root{--q-scrollbar-size:10px;--q-scrollbar-track:color-mix(in srgb,var(--q-color-bg) 70%,var(--q-color-surface));--q-scrollbar-thumb:color-mix(in srgb,var(--q-color-brand) 38%,var(--q-color-border));--q-scrollbar-thumb-hover:color-mix(in srgb,var(--q-color-brand) 62%,var(--q-color-border))}\
html{scrollbar-color:var(--q-scrollbar-thumb) transparent;scrollbar-width:thin}\
*{scrollbar-color:var(--q-scrollbar-thumb) transparent;scrollbar-width:thin}\
::-webkit-scrollbar{width:var(--q-scrollbar-size);height:var(--q-scrollbar-size)}\
::-webkit-scrollbar-track{background:transparent;border-radius:var(--q-radius-full)}\
::-webkit-scrollbar-thumb{background:var(--q-scrollbar-thumb);border-radius:var(--q-radius-full);border:2px solid transparent;background-clip:padding-box}\
::-webkit-scrollbar-thumb:hover{background:var(--q-scrollbar-thumb-hover);border:2px solid transparent;background-clip:padding-box}\
::-webkit-scrollbar-thumb:active{background:var(--q-color-brand);border:2px solid transparent;background-clip:padding-box}\
::-webkit-scrollbar-corner{background:transparent}\
/* Scoped scroll panes (code blocks, tables, playground stage) get a track tint\
   so the bar reads against the panel surface, and a slightly chunkier thumb. */\
.q-code,.q-table-wrap,.qq-code,.qq-table,.pg__stage,.pg__codewrap,.q-codewrap,.q-prose pre,.cl-themeit__surface{scrollbar-color:var(--q-scrollbar-thumb) var(--q-scrollbar-track)}\
.q-code::-webkit-scrollbar-track,.q-table-wrap::-webkit-scrollbar-track,.qq-code::-webkit-scrollbar-track,.qq-table::-webkit-scrollbar-track,.pg__stage::-webkit-scrollbar-track,.pg__codewrap::-webkit-scrollbar-track,.q-codewrap::-webkit-scrollbar-track,.q-prose pre::-webkit-scrollbar-track{background:var(--q-scrollbar-track);border-radius:var(--q-radius-full)}\
@media (prefers-reduced-transparency:reduce){::-webkit-scrollbar-thumb{border:0;background-clip:border-box}}"
}

/// Site-level design tokens layered on top of `qquill-theme`'s base set.
/// (The brand palette + density/radius axes are generated from the single config
/// in [`crate::app::design`].)
///
/// This does NOT fork the theme: it overrides the font *stacks* (so there is no
/// Inter/CDN dependency — refined system stack only) and ADDS the type-scale,
/// leading, and tracking handles the spec calls for, plus the density / control
/// tokens that `[data-q-size]` re-points. Everything still resolves to `--q-*`
/// custom properties so a theme/size/radius switch is a single attribute flip.
fn tokens_css() -> &'static str {
    "\
:root{\
--q-font-sans:ui-sans-serif,system-ui,-apple-system,\"Segoe UI\",Roboto,Helvetica,Arial,\"Apple Color Emoji\",\"Segoe UI Emoji\";\
--q-font-mono:ui-monospace,SFMono-Regular,\"SF Mono\",Menlo,Consolas,\"Liberation Mono\",monospace;\
--q-font-size-display:clamp(2.75rem,6vw,4.5rem);\
--q-font-size-xs:0.75rem;\
--q-page-max:80rem;\
--q-page-pad:1.5rem;\
--q-leading-tight:1.1;\
--q-leading-snug:1.3;\
--q-leading-relaxed:1.65;\
--q-tracking-tight:-0.025em;\
--q-tracking-wide:0.12em\
}\
/* ---- font stacks + antialiasing on the system stack (no CDN) ---- */\
body{font-family:var(--q-font-sans);-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;text-rendering:optimizeLegibility;font-feature-settings:\"cv05\" 1,\"ss01\" 0}\
code,pre,kbd,samp,.q-code,.q-inline{font-family:var(--q-font-mono)}\
/* ---- type scale (base element styles) ---- */\
.q-display{font-size:var(--q-font-size-display);font-weight:var(--q-font-weight-bold);line-height:1.04;letter-spacing:-0.03em;margin:0 0 var(--q-space-4)}\
h1,.q-h1{font-size:clamp(2.25rem,5vw,3.25rem);font-weight:var(--q-font-weight-bold);line-height:var(--q-leading-tight);letter-spacing:var(--q-tracking-tight)}\
h2,.q-h2{font-size:clamp(1.5rem,3vw,2rem);font-weight:var(--q-font-weight-bold);line-height:1.15;letter-spacing:-0.02em}\
h3,.q-h3{font-size:1.25rem;font-weight:var(--q-font-weight-medium);line-height:var(--q-leading-snug);letter-spacing:-0.01em}\
.q-lead,.q-body-lg{font-size:clamp(1.05rem,2vw,1.25rem);font-weight:var(--q-font-weight-normal);line-height:1.6}\
body,.q-body{font-size:var(--q-font-size-md);line-height:var(--q-leading-relaxed)}\
small,.q-small{font-size:var(--q-font-size-sm);line-height:1.5}\
.q-eyebrow{font-size:var(--q-font-size-xs);font-weight:var(--q-font-weight-bold);line-height:1;letter-spacing:var(--q-tracking-wide);text-transform:uppercase}\
.q-code,code.q-inline{font-size:0.9rem;line-height:1.7}\
/* ---- vertical rhythm in space-token multiples ---- */\
h1,.q-h1,h2,.q-h2{margin-top:var(--q-space-8);margin-bottom:var(--q-space-3)}\
h3,.q-h3{margin-top:var(--q-space-6);margin-bottom:var(--q-space-3)}\
p{max-width:62ch}\
.q-prose p,.q-prose li{max-width:68ch}\
.q-section{margin-top:var(--q-section-gap,4.5rem)}"
}

/// Surface styles — `[data-q-surface=glass|neu|gradient|flat]`.
///
/// A surface attribute drops onto any element (card, header, panel) and maps
/// straight onto the existing per-mode effect tokens, so light/dark/contrast
/// values flow automatically with the theme. No colors are hardcoded; contrast
/// mode collapses the effect tokens at the theme layer.
fn surface_css() -> &'static str {
    "\
/* ---- SAFE GLOBAL SURFACE AXIS. Surface is a real site-wide control again,\
   but it changes semantic surface tokens instead of rewriting text colors.\
   Gradient/neu/glass are visible treatments while foreground and muted text\
   remain readable in both light and dark. Component docs keep their own scoped\
   demo control on [data-q-demo-surface]. ---- */\
:root,[data-q-surface=\"flat\"]{--q-surface-bg:var(--q-color-surface);--q-surface-border:var(--q-color-border);--q-surface-shadow:0 1px 0 color-mix(in srgb,var(--q-color-fg) 4%,transparent);--q-surface-hover-shadow:0 22px 54px -30px color-mix(in srgb,var(--q-color-brand) 55%,transparent);--q-surface-blur:0px;--q-surface-filter:none;--q-surface-accent:transparent;--q-surface-accent-opacity:0;--q-surface-ring:var(--q-color-surface)}\
[data-q-surface=\"glass\"]{--q-surface-bg:color-mix(in srgb,var(--q-color-surface) 74%,transparent);--q-surface-border:color-mix(in srgb,var(--q-color-border) 78%,transparent);--q-surface-shadow:0 18px 50px -34px color-mix(in srgb,var(--q-color-brand) 50%,transparent);--q-surface-hover-shadow:0 28px 68px -38px color-mix(in srgb,var(--q-color-brand) 58%,transparent);--q-surface-blur:14px;--q-surface-filter:blur(14px);--q-surface-accent:linear-gradient(90deg,var(--q-color-brand),var(--q-color-accent));--q-surface-accent-opacity:.22;--q-surface-ring:color-mix(in srgb,var(--q-color-surface) 74%,transparent)}\
[data-q-surface=\"neu\"]{--q-surface-bg:var(--q-color-surface);--q-surface-border:color-mix(in srgb,var(--q-color-border) 55%,transparent);--q-surface-shadow:var(--q-effect-neu-raised);--q-surface-hover-shadow:var(--q-effect-neu-raised);--q-surface-blur:0px;--q-surface-filter:none;--q-surface-accent:linear-gradient(90deg,var(--q-color-brand),var(--q-color-accent));--q-surface-accent-opacity:.16;--q-surface-ring:var(--q-color-surface)}\
[data-q-surface=\"gradient\"]{--q-surface-bg:linear-gradient(180deg,color-mix(in srgb,var(--q-color-surface) 94%,var(--q-color-brand) 6%),var(--q-color-surface));--q-surface-border:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-color-border));--q-surface-shadow:0 22px 62px -38px color-mix(in srgb,var(--q-color-brand) 68%,transparent);--q-surface-hover-shadow:0 32px 76px -42px color-mix(in srgb,var(--q-color-brand) 76%,transparent);--q-surface-blur:0px;--q-surface-filter:none;--q-surface-accent:linear-gradient(90deg,var(--q-color-brand),var(--q-color-accent));--q-surface-accent-opacity:1;--q-surface-ring:color-mix(in srgb,var(--q-color-surface) 94%,var(--q-color-brand) 6%)}\
/* Scoped component-demo surface: the [data-q-surface] attribute on the demo\
   stage matches the SAME global [data-q-surface=...] token rules above, so the\
   --q-surface-* tokens are re-pointed for the stage subtree and INHERITED by\
   the component inside it. This is the single design system: the component\
   reacts to surface (cards/alerts/panels pick up glass/neu/gradient via the\
   shared content rule), while atoms keep their own fills. We deliberately do\
   NOT overwrite --q-color-surface here (a gradient is not a valid\
   background-color and would erase atom fills like a Badge). */\
[data-q-demo-surface]{transition:box-shadow var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
/* Every normal reading/content surface consumes the semantic safe surface\
   tokens. Text colors are never switched to on-brand, so gradient mode stays\
   readable. Neu mode never gets an :active inset shadow, avoiding click-hold\
   shadow bleed across cards. */\
html .q-surface,html .q-pcard,html .q-feat,html .q-teaser,html .q-pillar,html .q-comp-card,html .q-rm-hub-card,html .q-dochub__card,html .q-docpath,html .q-rm-lane,html .q-rm-legend,html .q-qw-step,html .q-api-card,html .q-arch,html .q-pp-status__row,html .q-cloud-banner,html .q-pp-feat,html .q-journey__step,html .q-status-card,html .q-path-step,html .q-ui-metric,html .qq-card:not(.qq-card--glass):not(.qq-card--neu):not(.qq-card--gradient):not(.qq-card--elevated),html .qq-alert,html .qq-accordion,html .qq-popover__surface,html .qq-menu__surface,html .qq-dialog__surface,html .qq-drawer__panel,html .qq-cmdk__surface,html .qq-table{background:var(--q-surface-bg);color:var(--q-color-fg);border-color:var(--q-surface-border);box-shadow:var(--q-surface-shadow);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none)}\
html .q-surface .q-muted,html .q-pcard__blurb,html .q-feat__body,html .q-comp-card p,html .q-rm-hub-card p,html .q-dochub__card p,html .q-rm-lane p,html .q-qw-step p,html .q-api-card p,html .q-pp-status__row p,html .q-pp-feat__body,html .q-status-card__body,html .q-path-step__copy,html .qq-alert__message,html .qq-dialog__body,html .qq-cmdk__empty,html .qq-stat__label{color:var(--q-color-muted)}"
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
.q-main{flex:1 1 auto;width:100%;max-width:var(--q-page-max,80rem);margin:0 auto;padding:3.5rem var(--q-page-pad,1.5rem) 5rem}\
/* ---- Quill router motion: the runtime stamps data-q-route-enter on the new
   #main after a same-origin navigation. The top bar appears while fetch/swap is
   in flight. Reduced-motion collapses both. ---- */\
[data-q-route-enter]{animation:q-route-enter .42s var(--q-ease-out) both}\
@keyframes q-route-enter{from{opacity:0;transform:translateY(10px);filter:blur(4px)}to{opacity:1;transform:none;filter:none}}\
html[data-q-navigating] body::before{content:\"\";position:fixed;top:0;left:0;right:0;height:2px;z-index:100;background:linear-gradient(90deg,transparent,var(--q-color-brand),transparent);transform-origin:left;animation:q-route-bar .9s var(--q-ease-in-out) infinite}\
@keyframes q-route-bar{from{transform:translateX(-100%)}to{transform:translateX(100%)}}\
@media (prefers-reduced-motion:reduce){[data-q-route-enter]{animation:none}html[data-q-navigating] body::before{display:none}}\
/* ---- reveal: a load-time fade-up that ALWAYS ends visible (no JS needed, no \
   blank sections). The `reveal` island still sets data-revealed but is no longer \
   required for visibility. ---- */\
[data-q-reveal]{animation:q-reveal .42s var(--q-ease-out) both}\
@keyframes q-reveal{from{transform:translateY(12px)}to{transform:none}}\
[data-q-reveal][data-reveal-delay=\"1\"]{animation-delay:.08s}\
[data-q-reveal][data-reveal-delay=\"2\"]{animation-delay:.16s}\
[data-q-reveal][data-reveal-delay=\"3\"]{animation-delay:.24s}\
@media (prefers-reduced-motion:reduce){[data-q-reveal]{animation:none}}\
/* ---- hero ---- */\
.q-hero{position:relative;padding:2.5rem 0 1rem;overflow:hidden}\
.q-hero__glow{position:absolute;inset:-40% -10% auto -10%;height:520px;z-index:0;pointer-events:none;background:radial-gradient(60% 60% at 30% 20%,color-mix(in srgb,var(--q-color-brand) 26%,transparent),transparent 70%),radial-gradient(50% 50% at 80% 10%,color-mix(in srgb,var(--q-color-brand) 16%,transparent),transparent 70%);filter:blur(6px);animation:q-drift 18s var(--q-ease-in-out) infinite alternate}\
.q-hero>*{position:relative;z-index:1}\
@keyframes q-drift{from{transform:translate3d(0,0,0) scale(1)}to{transform:translate3d(4%,2%,0) scale(1.08)}}\
.q-h1 .q-accent{background:linear-gradient(100deg,var(--q-color-brand),color-mix(in srgb,var(--q-color-brand) 55%,var(--q-color-fg)));-webkit-background-clip:text;background-clip:text;color:transparent}\
.q-hero__in{display:inline-block;transform:translateY(10px);animation:q-rise .45s var(--q-ease-out) both}\
.q-hero__in.d1{animation-delay:.06s}.q-hero__in.d2{animation-delay:.14s}.q-hero__in.d3{animation-delay:.22s}.q-hero__in.d4{animation-delay:.30s}\
@keyframes q-rise{from{transform:translateY(10px)}to{transform:none}}\
.q-pillars{display:grid;grid-template-columns:1fr 1fr;gap:1rem;margin:1.75rem 0 0}\
@media (max-width:640px){.q-pillars{grid-template-columns:1fr}}\
.q-pillar{border-radius:var(--q-radius-lg);padding:1.1rem 1.25rem;background:var(--q-surface-bg,var(--q-color-surface));color:var(--q-color-fg);border:1px solid var(--q-surface-border,var(--q-color-border));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);transition:box-shadow var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out),color var(--q-duration-base) var(--q-ease-out)}\
.q-pillar h3{margin:.1rem 0 .35rem;font-size:1.05rem}\
.q-pillar p{margin:0;color:var(--q-color-muted);font-size:.95rem}\
.q-pillar__k{font-family:var(--q-font-mono);font-size:.78rem;color:var(--q-color-brand)}\
/* ---- product cards hover depth ---- */\
.q-grid .qq-card{transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-grid .qq-card:hover{transform:translateY(-4px);box-shadow:0 14px 40px -18px color-mix(in srgb,var(--q-color-brand) 55%,transparent);border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-color-border))}\
/* ---- 3D tilt (the `tilt` island; flat with JS off / reduced-motion / touch) ---- */\
[data-q-tilt]{position:relative;transform:perspective(900px) rotateX(var(--q-tilt-rx,0deg)) rotateY(var(--q-tilt-ry,0deg));transform-style:preserve-3d;transition:transform .45s var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
[data-q-tilt][data-tilting=\"true\"]{transition:box-shadow var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
[data-q-tilt]>*{position:relative;z-index:1}\
[data-q-tilt]::after{content:\"\";position:absolute;inset:0;z-index:0;border-radius:inherit;background:radial-gradient(45% 45% at var(--q-tilt-mx,50%) var(--q-tilt-my,50%),color-mix(in srgb,var(--q-color-brand) 18%,transparent),transparent 70%);opacity:0;transition:opacity .35s var(--q-ease-out);pointer-events:none}\
[data-q-tilt][data-tilting=\"true\"]::after{opacity:1}\
@media (prefers-reduced-motion:reduce){[data-q-tilt]{transform:none;transition:none}[data-q-tilt]::after{display:none}}\
/* ---- live teaser ---- */\
.q-teaser{border-radius:var(--q-radius-xl);background:var(--q-surface-bg,var(--q-color-surface));color:var(--q-color-fg);border:1px solid var(--q-surface-border,var(--q-color-border));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);padding:var(--q-surface-pad);margin-top:var(--q-space-5);transition:box-shadow var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out),color var(--q-duration-base) var(--q-ease-out)}\
.q-teaser__chrome{display:flex;gap:.4rem;margin:0 0 1rem}\
.q-teaser__chrome span{width:11px;height:11px;border-radius:var(--q-radius-full);background:var(--q-color-border)}\
/* ---- docs layout ---- */\
.q-docs{display:grid;grid-template-columns:16rem minmax(0,1fr) 14rem;gap:var(--q-space-6);align-items:start;max-width:var(--q-page-max,80rem);margin:0 auto;padding:var(--q-space-8) var(--q-page-pad,1.5rem) var(--q-space-10);width:100%}\
@media (max-width:980px){.q-docs{grid-template-columns:1fr}.q-docs__toc{display:none}.q-docs__side{display:none}}\
.q-docs__side{position:sticky;top:5rem;align-self:start}\
.q-docs__sec{margin:0 0 1.25rem}\
.q-docs__sec-label{font-size:.75rem;text-transform:uppercase;letter-spacing:.09em;color:var(--q-color-muted);font-weight:var(--q-font-weight-bold);margin:0 0 .5rem}\
.q-docs__nav{display:flex;flex-direction:column;gap:.1rem}\
.q-docs__nav a{color:var(--q-color-muted);font-size:.93rem;padding:.32rem .6rem;border-radius:min(var(--q-radius-md),.7rem);border-left:2px solid transparent}\
.q-docs__nav a:hover{color:var(--q-color-fg);background:var(--q-surface-bg,var(--q-color-surface));border-color:var(--q-surface-border,var(--q-color-border));text-decoration:none}\
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
.q-copy{position:absolute;top:.55rem;right:.55rem;display:inline-flex;align-items:center;gap:.3rem;font-size:.78rem;font-family:var(--q-font-sans);padding:.3rem .55rem;border-radius:var(--q-radius-md);border:1px solid var(--q-surface-border,var(--q-color-border));background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);color:var(--q-color-muted);cursor:pointer;opacity:0;transition:opacity var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast),background var(--q-duration-fast) var(--q-ease-out),box-shadow var(--q-duration-fast) var(--q-ease-out)}\
.q-codewrap:hover .q-copy,.q-copy:focus-visible{opacity:1}\
.q-copy:hover{color:var(--q-color-fg);border-color:var(--q-color-brand)}\
.q-copy[data-copied=\"true\"]{color:var(--q-color-brand);opacity:1}\
/* ---- component showcase index ---- */\
.q-comp-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(15rem,1fr));gap:1rem;margin-top:1.5rem}\
.q-comp-card{display:block;border-radius:var(--q-radius-lg);padding:1.1rem 1.25rem;background:var(--q-surface-bg,var(--q-color-surface));color:var(--q-color-fg);border:1px solid var(--q-surface-border,var(--q-color-border));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);transition:transform var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out),color var(--q-duration-base) var(--q-ease-out)}\
.q-comp-card:hover{transform:translateY(-3px);border-color:var(--q-color-brand);text-decoration:none}\
.q-comp-card h3{margin:0 0 .25rem;color:var(--q-color-fg);font-size:1.05rem}\
.q-comp-card p{margin:0;color:var(--q-color-muted);font-size:.9rem}\
/* ---- the PLAYGROUND (the `playground` island) ---- */\
.pg{border:1px solid var(--q-color-border);border-radius:var(--q-radius-xl);overflow:hidden;margin:1.5rem 0}\
.pg__stage{display:flex;align-items:center;justify-content:center;gap:1rem;flex-wrap:wrap;min-height:180px;padding:2.5rem;background:var(--q-surface-bg,var(--q-color-surface))}\
.pg__cell{display:none}\
.pg__cell[data-active=\"true\"]{display:inline-flex}\
.pg__controls{display:flex;flex-wrap:wrap;gap:var(--q-space-5);padding:var(--q-space-4) var(--q-space-5);border-top:1px solid var(--q-surface-border,var(--q-color-border));background:var(--q-surface-bg,var(--q-color-surface))}\
.pg__group{display:flex;flex-direction:column;gap:var(--q-space-2)}\
.pg__legend{font-size:var(--q-font-size-xs);text-transform:uppercase;letter-spacing:.08em;color:var(--q-color-muted);font-weight:var(--q-font-weight-bold)}\
.pg__seg{display:inline-flex;border:1px solid var(--q-color-border);border-radius:min(var(--q-radius-md),calc(var(--q-control-h,2.5rem) * .28));overflow:hidden}\
.pg__opt{appearance:none;display:flex;align-items:center;justify-content:center;min-height:calc(var(--q-control-h,2.5rem) - var(--q-space-2));border:0;background:transparent;color:var(--q-color-muted);font:inherit;font-size:var(--q-font-size-sm);padding:0 var(--q-space-3);cursor:pointer;transition:background-color var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast) var(--q-ease-out)}\
.pg__opt+.pg__opt{border-left:1px solid var(--q-color-border)}\
.pg__opt:hover{color:var(--q-color-fg)}\
.pg__opt[aria-pressed=\"true\"]{background:var(--q-color-brand);color:var(--q-color-on-brand)}\
.pg__codewrap{position:relative}\
.pg .q-code{border-radius:0;border-left:0;border-right:0;border-bottom:0}\
.pg__code-empty{display:none}"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_axis_is_global_but_readable() {
        let css = surface_css();
        // Full site-level surface controls exist again.
        assert!(css.contains("[data-q-surface=\"glass\"]"));
        assert!(css.contains("[data-q-surface=\"gradient\"]"));
        assert!(css.contains("[data-q-surface=\"neu\"]"));
        assert!(css.contains("--q-surface-bg"));
        // The scoped component demo reuses the SAME global surface tokens (one
        // design system) and never clobbers --q-color-surface (a gradient is not
        // a valid background-color and would erase atom fills like a Badge).
        assert!(css.contains("[data-q-demo-surface]{"));
        assert!(
            !css.contains("[data-q-demo-surface][data-q-surface=\"gradient\"]{--q-color-surface")
        );
        assert!(!css.contains("--q-surf-bg"));
        // Normal content consumes safe semantic surface tokens and keeps readable text.
        assert!(css.contains("html .q-pcard"));
        assert!(css.contains("background:var(--q-surface-bg)"));
        assert!(css.contains("color:var(--q-color-fg)"));
        assert!(css.contains("html .q-path-step"));
        // Guard against the old broken gradient smear: normal content must not switch
        // foreground text to on-brand.
        assert!(!css.contains("html .q-pcard{background:var(--q-effect-gradient-brand)"));
        assert!(!css.contains("html .q-pcard{color:var(--q-color-on-brand)"));
        // RENDER-PERF GUARD: the global content rule must NOT carry an always-on
        // backdrop blur (blur(var(--q-surface-blur))) — that forced a compositing
        // layer on dozens of elements and caused the delayed/blank repaint after
        // a theme-control change. It must use the none-by-default filter token.
        assert!(!css.contains("backdrop-filter:blur(var(--q-surface-blur))"));
        assert!(css.contains("backdrop-filter:var(--q-surface-filter,none)"));
        // Only glass turns the filter into a real blur.
        assert!(css.contains("--q-surface-filter:blur(14px)"));
        assert!(css.contains("--q-surface-filter:none"));
    }

    #[test]
    fn motion_css_has_router_and_reveal_hooks() {
        let css = motion_css();
        assert!(css.contains("data-q-route-enter"));
        assert!(css.contains("data-q-navigating"));
        assert!(css.contains("data-q-reveal"));
        assert!(css.contains(".q-docs"));
        assert!(css.contains(".q-teaser"));
        assert!(
            css.contains(".pg__controls{display:flex;flex-wrap:wrap;gap:var(--q-space-5);padding:var(--q-space-4) var(--q-space-5)"),
            "playground controls must use density spacing tokens"
        );
        assert!(
            css.contains(".pg__opt{appearance:none;display:flex;align-items:center;justify-content:center;min-height:calc(var(--q-control-h,2.5rem) - var(--q-space-2))"),
            "playground options must scale from the shared control height"
        );
    }

    /// Themed scrollbars must be part of the global layout CSS (Firefox + WebKit),
    /// driven by design tokens so they restyle with theme/accent — never the raw
    /// browser default.
    #[test]
    fn scrollbars_are_themed_and_token_driven() {
        let s = scrollbar_css();
        // WebKit/Chromium pseudo-elements present.
        assert!(s.contains("::-webkit-scrollbar{"));
        assert!(s.contains("::-webkit-scrollbar-thumb{"));
        assert!(s.contains("::-webkit-scrollbar-thumb:hover"));
        // Firefox shorthands present.
        assert!(s.contains("scrollbar-width:thin"));
        assert!(s.contains("scrollbar-color:"));
        // Token-driven (not hardcoded colors).
        assert!(s.contains("--q-scrollbar-thumb:color-mix(in srgb,var(--q-color-brand)"));
        assert!(s.contains("--q-scrollbar-track:color-mix(in srgb,var(--q-color-bg)"));
        // Scoped scroll panes (code/table) get a track tint.
        assert!(s.contains(".q-code,.q-table-wrap,.qq-code,.qq-table"));
        // And it is wired into the global layout CSS.
        let full = layout_css();
        assert!(full.contains("::-webkit-scrollbar-thumb{"));
        assert!(full.contains("scrollbar-width:thin"));
    }
}
