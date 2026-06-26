//! Site layout styles.
//!
//! The *color* system comes from `tqquill-theme` (the typed `--q-*` design
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
    let block = tqquill_style::style! {
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
    block.to_css()
}
