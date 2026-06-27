//! **The single design-system config.** Edit the values here and the whole site
//! follows: every component reads the `--q-*` design tokens these emit, and the
//! live appearance control (theme / density / radius / surface) re-points them —
//! so nothing is hardcoded in a component and there is one source of truth for
//! the brand palette and the density/radius scales.
//!
//! Layering (all token-driven, applied in this order by `app::document`):
//!   1. `qquill_theme` base tokens (spacing, type sizes, shadows, effects).
//!   2. THIS module: the Qirava brand palette (per mode) + the density/radius axes.
//!   3. `theme::layout_css` (type scale, surface axis, motion) + component CSS.
//!
//! Both the qquill-design components and the site's own components consume the
//! same tokens, so they are one design system that restyles together.

/// A full color set for one mode. Every field is a CSS color (or gradient stop).
struct Palette {
    brand: &'static str,
    accent: &'static str,
    on_brand: &'static str,
    bg: &'static str,
    surface: &'static str,
    surface_selected: &'static str,
    fg: &'static str,
    muted: &'static str,
    border: &'static str,
    border_strong: &'static str,
    grad_from: &'static str,
    grad_to: &'static str,
}

/// Light mode — brand ink (`#0c1e3c`) text on near-white, deep-azure brand.
const LIGHT: Palette = Palette {
    brand: "#2563eb",
    accent: "#0ea5e9",
    on_brand: "#ffffff",
    bg: "#f6f8fc",
    surface: "#ffffff",
    surface_selected: "#e7eefb",
    fg: "#0c1e3c",
    muted: "#51637e",
    border: "#e1e7f0",
    border_strong: "#64748b",
    grad_from: "#2563eb",
    grad_to: "#0ea5e9",
};

/// Dark mode (the site default) — ink-navy base with a bright azure brand.
const DARK: Palette = Palette {
    brand: "#5b9cff",
    accent: "#38bdf8",
    on_brand: "#07101f",
    bg: "#0a1322",
    surface: "#111c30",
    surface_selected: "#1b2a45",
    fg: "#e8eef7",
    muted: "#93a4be",
    border: "#1f2d45",
    border_strong: "#3a4e70",
    grad_from: "#5b9cff",
    grad_to: "#38bdf8",
};

/// Emit one `selector { --q-color-*: … }` block from a [`Palette`].
fn palette_block(selector: &str, p: &Palette) -> String {
    format!(
        "{selector}{{\
         --q-color-brand:{};--q-color-accent:{};--q-color-on-brand:{};\
         --q-color-bg:{};--q-color-surface:{};--q-color-surface-selected:{};\
         --q-color-fg:{};--q-color-muted:{};--q-color-border:{};--q-color-border-strong:{};\
         --q-effect-gradient-brand:linear-gradient(120deg,{} 0%,{} 100%)}}",
        p.brand, p.accent, p.on_brand, p.bg, p.surface, p.surface_selected, p.fg, p.muted,
        p.border, p.border_strong, p.grad_from, p.grad_to,
    )
}

/// The brand palette: re-points the qquill-theme color tokens to the Qirava
/// brand, per mode. Light is the `:root` default; dark applies on
/// `[data-q-theme="dark"]`. Generated from [`LIGHT`]/[`DARK`] — change those, not
/// the CSS.
pub fn palette_css() -> String {
    let mut s = String::from("/* ---- design system: brand palette (single config) ---- */");
    s.push_str(&palette_block(":root,:root[data-q-theme=\"light\"]", &LIGHT));
    s.push_str(&palette_block(":root[data-q-theme=\"dark\"]", &DARK));
    s
}

/// Density scale rows: `(name, --q-density, --q-control-h, --q-control-pad-x,
/// --q-field-gap)`. The first row is the default (also applied at bare `:root`).
const DENSITY: &[(&str, &str, &str, &str, &str)] = &[
    ("cozy", "1", "2.5rem", "1.1rem", "var(--q-space-4)"),
    ("compact", ".85", "2.1rem", ".85rem", "var(--q-space-3)"),
    ("comfortable", "1.15", "2.9rem", "1.35rem", "var(--q-space-5)"),
];

/// Radius scale rows: `(name, sm, md, lg, xl)`. The first is the default. `pill`
/// rounds small controls fully (sm/md) but BOUNDS large surfaces (lg/xl) so menus
/// and cards never collapse into circles.
const RADIUS: &[(&str, &str, &str, &str, &str)] = &[
    ("rounded", "4px", "8px", "12px", "20px"),
    ("sharp", "0", "0", "2px", "4px"),
    ("pill", "9999px", "9999px", "20px", "26px"),
];

/// The density + radius axes the appearance control switches via `[data-q-size]`
/// and `[data-q-radius]`. Generated from [`DENSITY`]/[`RADIUS`].
pub fn scale_css() -> String {
    let mut s = String::from("/* ---- design system: density + radius axes ---- */");
    for (i, (name, density, h, pad, gap)) in DENSITY.iter().enumerate() {
        let sel = if i == 0 {
            format!(":root,[data-q-size=\"{name}\"]")
        } else {
            format!("[data-q-size=\"{name}\"]")
        };
        s.push_str(&format!(
            "{sel}{{--q-density:{density};--q-control-h:{h};--q-control-pad-x:{pad};--q-field-gap:{gap}}}"
        ));
    }
    s.push_str(".q-btn{min-height:var(--q-control-h);padding-inline:var(--q-control-pad-x)}");
    for (i, (name, sm, md, lg, xl)) in RADIUS.iter().enumerate() {
        let sel = if i == 0 {
            format!(":root,[data-q-radius=\"{name}\"]")
        } else {
            format!("[data-q-radius=\"{name}\"]")
        };
        s.push_str(&format!(
            "{sel}{{--q-radius-sm:{sm};--q-radius-md:{md};--q-radius-lg:{lg};--q-radius-xl:{xl}}}"
        ));
    }
    s
}
