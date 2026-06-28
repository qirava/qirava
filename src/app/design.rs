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
        p.brand,
        p.accent,
        p.on_brand,
        p.bg,
        p.surface,
        p.surface_selected,
        p.fg,
        p.muted,
        p.border,
        p.border_strong,
        p.grad_from,
        p.grad_to,
    )
}

/// The brand palette: re-points the qquill-theme color tokens to the Qirava
/// brand, per mode. Light is the `:root` default; dark applies on
/// `[data-q-theme="dark"]`. Generated from [`LIGHT`]/[`DARK`] — change those, not
/// the CSS.
pub fn palette_css() -> String {
    let mut s = String::from("/* ---- design system: brand palette (single config) ---- */");
    s.push_str(&palette_block(
        ":root,:root[data-q-theme=\"light\"]",
        &LIGHT,
    ));
    s.push_str(&palette_block(":root[data-q-theme=\"dark\"]", &DARK));
    s
}

/// One selectable accent (the color picker). A chosen accent regenerates the
/// brand-family tokens (`--q-color-brand` / `--q-color-accent` /
/// `--q-color-on-brand` + the brand gradient) for BOTH light and dark, so the
/// whole site re-skins from one choice while bg/surface/text stay on the neutral
/// ink base (so contrast stays correct in either mode). First row is the default.
struct Accent {
    name: &'static str,
    light: (&'static str, &'static str, &'static str),
    dark: (&'static str, &'static str, &'static str),
}

/// Accent options surfaced in the appearance control. Each is a hand-tuned
/// light/dark pair so the color reads well in both modes.
const ACCENTS: &[Accent] = &[
    Accent {
        name: "azure",
        light: ("#2563eb", "#0ea5e9", "#ffffff"),
        dark: ("#5b9cff", "#38bdf8", "#07101f"),
    },
    Accent {
        name: "violet",
        light: ("#7c3aed", "#a855f7", "#ffffff"),
        dark: ("#a78bfa", "#c4b5fd", "#140a2e"),
    },
    Accent {
        name: "emerald",
        light: ("#059669", "#10b981", "#ffffff"),
        dark: ("#34d399", "#6ee7b7", "#04140d"),
    },
    Accent {
        name: "amber",
        light: ("#d97706", "#f59e0b", "#1a1205"),
        dark: ("#fbbf24", "#fcd34d", "#1a1205"),
    },
    Accent {
        name: "rose",
        light: ("#e11d48", "#f43f5e", "#ffffff"),
        dark: ("#fb7185", "#fda4af", "#2a0810"),
    },
];

/// Emit the brand-family override for one accent in one mode.
fn accent_block(selector: &str, c: (&str, &str, &str)) -> String {
    let (brand, accent, on_brand) = c;
    format!(
        "{selector}{{--q-color-brand:{brand};--q-color-accent:{accent};--q-color-on-brand:{on_brand};\
--q-effect-gradient-brand:linear-gradient(120deg,{brand} 0%,{accent} 100%)}}"
    )
}

/// The accent (color-picker) axis: `[data-q-accent="..."]` re-points the brand
/// family for both light and dark. Emitted AFTER `palette_css` so it overrides
/// the base brand.
pub fn accent_css() -> String {
    let mut s = String::from("/* ---- design system: accent (color picker) axis ---- */");
    for a in ACCENTS.iter() {
        let light_sel = format!(
            ":root[data-q-accent=\"{n}\"],:root[data-q-theme=\"light\"][data-q-accent=\"{n}\"]",
            n = a.name
        );
        s.push_str(&accent_block(&light_sel, a.light));
        s.push_str(&accent_block(
            &format!(
                ":root[data-q-theme=\"dark\"][data-q-accent=\"{n}\"]",
                n = a.name
            ),
            a.dark,
        ));
    }
    s
}

/// The accent slugs, in control order (first is the default).
pub fn accent_names() -> &'static [&'static str] {
    &["azure", "violet", "emerald", "amber", "rose"]
}

/// One motion preset (the animation option). Re-points the shared duration +
/// easing tokens every component transition/animation already reads, plus a
/// `--q-press` feedback scale. First row is the default; `none` collapses motion
/// when chosen explicitly (the OS `prefers-reduced-motion` reset always wins).
struct Motion {
    name: &'static str,
    fast: &'static str,
    base: &'static str,
    slow: &'static str,
    ease_out: &'static str,
    ease_in_out: &'static str,
    press: &'static str,
}

const MOTIONS: &[Motion] = &[
    Motion {
        name: "smooth",
        fast: "120ms",
        base: "200ms",
        slow: "320ms",
        ease_out: "cubic-bezier(.16,1,.3,1)",
        ease_in_out: "cubic-bezier(.65,0,.35,1)",
        press: ".97",
    },
    Motion {
        name: "snappy",
        fast: "80ms",
        base: "130ms",
        slow: "200ms",
        ease_out: "cubic-bezier(.2,.8,.2,1)",
        ease_in_out: "cubic-bezier(.4,0,.2,1)",
        press: ".96",
    },
    Motion {
        name: "playful",
        fast: "160ms",
        base: "300ms",
        slow: "480ms",
        ease_out: "cubic-bezier(.34,1.56,.64,1)",
        ease_in_out: "cubic-bezier(.68,-.55,.27,1.55)",
        press: ".93",
    },
    Motion {
        name: "none",
        fast: "1ms",
        base: "1ms",
        slow: "1ms",
        ease_out: "linear",
        ease_in_out: "linear",
        press: "1",
    },
];

/// The motion axis: `[data-q-motion="..."]` re-points the duration + easing
/// tokens (so every transition/animation retimes at once) and the `--q-press`
/// feedback scale, plus a press-squish `:active` rule on interactive elements.
/// The OS `prefers-reduced-motion` reset still wins.
pub fn motion_axis_css() -> String {
    let mut s = String::from("/* ---- design system: motion (animation) axis ---- */");
    for (i, m) in MOTIONS.iter().enumerate() {
        let sel = if i == 0 {
            format!(":root,[data-q-motion=\"{}\"]", m.name)
        } else {
            format!("[data-q-motion=\"{}\"]", m.name)
        };
        s.push_str(&format!(
            "{sel}{{--q-duration-fast:{f};--q-duration-base:{b};--q-duration-normal:{b};\
--q-duration-slow:{sl};--q-ease-out:{eo};--q-ease-in-out:{eio};--q-press:{p}}}",
            f = m.fast,
            b = m.base,
            sl = m.slow,
            eo = m.ease_out,
            eio = m.ease_in_out,
            p = m.press,
        ));
    }
    s.push_str(
        "html .q-btn:active,html .qq-btn:active,html .q-comp-card:active,html .q-nav__link:active,\
html .q-tc__opt:active,html .pg__opt:active,html .cl-tc__opt:active,html .q-theme-toggle:active,\
html .q-tc__trigger:active,html .q-ui-link:active,html .q-ui-card:active,html .qq-card:active{transform:scale(var(--q-press,1))}\
@media (prefers-reduced-motion:reduce){\
html .q-btn:active,html .qq-btn:active,html .q-comp-card:active,html .q-nav__link:active,\
html .q-tc__opt:active,html .pg__opt:active,html .cl-tc__opt:active,html .q-theme-toggle:active,\
html .q-tc__trigger:active,html .q-ui-link:active,html .q-ui-card:active,html .qq-card:active{transform:none}}",
    );
    s
}

/// The motion slugs, in control order (first is the default).
#[allow(dead_code)]
pub fn motion_names() -> &'static [&'static str] {
    &["smooth", "snappy", "playful", "none"]
}

/// One density level. The KEY insight: density must re-point the SHARED
/// `--q-space-*` scale itself, because every component (the 33 Quill design
/// components AND the site's own classes) sizes its padding/gaps from
/// `var(--q-space-N)`. Re-pointing the scale makes the density control flow into
/// *everything* with no per-class overrides — the single source of truth for
/// spacing.
///
/// Fields: `(name, density-multiplier, control-height, control-pad-x,
/// logo-height, hero-pad, section-gap, [space-1, space-2, space-3, space-4,
/// space-5, space-6, space-8, space-10])`. The first row is the default (also
/// emitted at bare `:root`). Base (cozy) matches the qquill-theme base scale; the
/// other levels scale it ~0.8x / ~1.2x.
struct Density {
    name: &'static str,
    mult: &'static str,
    control_h: &'static str,
    control_pad_x: &'static str,
    logo_h: &'static str,
    hero_pad: &'static str,
    section_gap: &'static str,
    /// space-1,2,3,4,5,6,8,10 (px), re-pointing the shared spacing scale.
    space: [&'static str; 8],
}

const DENSITY: &[Density] = &[
    Density {
        name: "cozy",
        mult: "1",
        control_h: "2.5rem",
        control_pad_x: "1.1rem",
        logo_h: "2.05rem",
        hero_pad: "4.5rem",
        section_gap: "4.5rem",
        space: ["4px", "8px", "12px", "16px", "24px", "32px", "48px", "64px"],
    },
    Density {
        name: "compact",
        mult: ".85",
        control_h: "2.1rem",
        control_pad_x: ".85rem",
        logo_h: "1.82rem",
        hero_pad: "3.25rem",
        section_gap: "3.25rem",
        space: ["3px", "6px", "9px", "12px", "18px", "24px", "36px", "50px"],
    },
    Density {
        name: "comfortable",
        mult: "1.15",
        control_h: "2.9rem",
        control_pad_x: "1.35rem",
        logo_h: "2.25rem",
        hero_pad: "5.75rem",
        section_gap: "5.75rem",
        space: [
            "5px", "10px", "15px", "20px", "30px", "40px", "60px", "82px",
        ],
    },
];

/// Radius scale rows: `(name, sm, md, lg, xl, full)`. The first is the default. `pill`
/// rounds small controls fully (sm/md) but BOUNDS large surfaces (lg/xl) so menus
/// and cards never collapse into circles.
const RADIUS: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("rounded", "4px", "8px", "12px", "20px", "9999px"),
    ("sharp", "0", "0", "2px", "4px", "3px"),
    ("pill", "9999px", "9999px", "20px", "26px", "9999px"),
];

/// The density + radius axes the appearance control switches via `[data-q-size]`
/// and `[data-q-radius]`. Generated from [`DENSITY`]/[`RADIUS`].
pub fn scale_css() -> String {
    let mut s = String::from("/* ---- design system: density + radius axes ---- */");
    for (i, d) in DENSITY.iter().enumerate() {
        let sel = if i == 0 {
            format!(":root,[data-q-size=\"{}\"]", d.name)
        } else {
            format!("[data-q-size=\"{}\"]", d.name)
        };
        // Re-point the SHARED space scale + the control/layout handles. Because
        // every component reads `var(--q-space-N)`, this single block makes the
        // density control flow into all 33 Quill components AND the site's own
        // surfaces with no per-class overrides.
        s.push_str(&format!(
            "{sel}{{--q-density:{m};\
--q-space-1:{s1};--q-space-2:{s2};--q-space-3:{s3};--q-space-4:{s4};\
--q-space-5:{s5};--q-space-6:{s6};--q-space-8:{s8};--q-space-10:{s10};\
--q-control-h:{h};--q-control-pad-x:{pad};--q-field-gap:var(--q-space-4);\
--q-surface-pad:var(--q-space-5);--q-section-gap:{gap};\
--q-logo-h:{logo};--q-hero-pad:{hero}}}",
            m = d.mult,
            s1 = d.space[0],
            s2 = d.space[1],
            s3 = d.space[2],
            s4 = d.space[3],
            s5 = d.space[4],
            s6 = d.space[5],
            s8 = d.space[6],
            s10 = d.space[7],
            h = d.control_h,
            pad = d.control_pad_x,
            gap = d.section_gap,
            logo = d.logo_h,
            hero = d.hero_pad,
        ));
    }
    // Controls bind their height/padding to the density handles so buttons,
    // inputs, and the site CTAs all resize together. Sizes are derived from the
    // control height so sm/lg track density automatically.
    s.push_str(
        "html .q-btn,html .qq-btn{min-height:var(--q-control-h);padding-inline:var(--q-control-pad-x);padding-block:0}\
html .qq-btn--sm,html .qq-field input,html .qq-select select{min-height:calc(var(--q-control-h) - 0.4rem)}\
html .qq-btn--lg{min-height:calc(var(--q-control-h) + 0.5rem)}\
html .q-section{margin-top:var(--q-section-gap)}"
    );
    for (i, (name, sm, md, lg, xl, full)) in RADIUS.iter().enumerate() {
        let sel = if i == 0 {
            format!(":root,[data-q-radius=\"{name}\"]")
        } else {
            format!("[data-q-radius=\"{name}\"]")
        };
        s.push_str(&format!(
            "{sel}{{--q-radius-sm:{sm};--q-radius-md:{md};--q-radius-lg:{lg};--q-radius-xl:{xl};--q-radius-full:{full}}}"
        ));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Density MUST re-point the shared `--q-space-*` scale (not just bespoke
    /// handles), because every Quill component sizes from `var(--q-space-N)`.
    /// This is the regression guard for "the density control does nothing to
    /// real components".
    #[test]
    fn density_repoints_shared_space_scale() {
        let css = scale_css();
        // The default (cozy) row pins the base scale at :root.
        assert!(
            css.contains("--q-space-4:16px"),
            "cozy base space-4 missing"
        );
        // Compact + comfortable must override the SAME token, so components shift.
        assert!(
            css.contains("[data-q-size=\"compact\"]{--q-density:.85;--q-space-1:3px"),
            "compact must re-point the space scale: {css}"
        );
        assert!(
            css.contains("--q-space-4:20px"),
            "comfortable must enlarge space-4: {css}"
        );
    }

    /// All three radius levels re-point the shared radius tokens components read.
    #[test]
    fn radius_axis_repoints_shared_tokens() {
        let css = scale_css();
        assert!(css.contains("[data-q-radius=\"sharp\"]{--q-radius-sm:0"));
        assert!(css.contains("[data-q-radius=\"pill\"]{--q-radius-sm:9999px"));
        assert!(
            css.contains("--q-radius-full:3px"),
            "sharp must also affect components that use Radius::Full / --q-radius-full: {css}"
        );
    }

    /// The brand palette must define both light and dark color blocks so the
    /// theme toggle has something to flip.
    #[test]
    fn palette_defines_light_and_dark() {
        let css = palette_css();
        assert!(css.contains(":root,:root[data-q-theme=\"light\"]"));
        assert!(css.contains(":root[data-q-theme=\"dark\"]"));
    }

    /// The accent (color-picker) axis must regenerate the brand family per
    /// chosen accent for BOTH light and dark, so a picked color reads correctly
    /// in either mode.
    #[test]
    fn accent_axis_regenerates_brand_per_mode() {
        let css = accent_css();
        // A non-default accent applies in light (root/explicit-light) ...
        assert!(css.contains("[data-q-accent=\"violet\"]"));
        // ... and has a distinct dark override.
        assert!(css.contains(":root[data-q-theme=\"dark\"][data-q-accent=\"violet\"]"));
        // It re-points the brand token (not bg/fg), keeping neutral contrast.
        assert!(css.contains("--q-color-brand:#7c3aed"));
        assert!(css.contains("--q-color-brand:#a78bfa"));
        // The control list and the CSS slugs must agree.
        for name in accent_names() {
            assert!(
                css.contains(&format!("data-q-accent=\"{name}\"")),
                "missing accent {name}"
            );
        }
    }

    /// The motion (animation) axis must re-point the shared duration + easing
    /// tokens so every component transition retimes from one choice, and expose
    /// a press-feedback scale, with `none` collapsing motion.
    #[test]
    fn motion_axis_repoints_durations_and_press() {
        let css = motion_axis_css();
        assert!(css.contains(":root,[data-q-motion=\"smooth\"]"));
        assert!(css.contains("[data-q-motion=\"snappy\"]"));
        assert!(css.contains("[data-q-motion=\"playful\"]"));
        // `none` collapses durations + press so motion truly turns off.
        assert!(css.contains("[data-q-motion=\"none\"]{--q-duration-fast:1ms"));
        assert!(css.contains("--q-press:1}"));
        // Re-points the shared tokens every component already reads.
        assert!(css.contains("--q-duration-base:"));
        assert!(css.contains("--q-ease-out:"));
        // Press feedback is applied on :active and neutralized under reduced motion.
        assert!(css.contains(":active{transform:scale(var(--q-press,1))}"));
        assert!(css.contains("prefers-reduced-motion:reduce"));
    }
}
