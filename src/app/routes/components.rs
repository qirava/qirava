//! The Quill component library — one page per component, mounted INSIDE the
//! Quill docs (`/docs/quill/components/<name>`).
//!
//! Each page is wrapped in [`docs_kit::layout`] so it gets the Quill docs
//! sidebar + an on-page TOC (it is NOT a bare shell page). Every page has the
//! same four-part body:
//!
//!   1. **Preview** — a static, no-JS-correct showcase of ALL the component's
//!      variants/sizes/tones (the exact axes from the rebuild spec).
//!   2. **Playground / live demo** — the pre-rendered variant matrix flipped by
//!      the `playground` island, or (for interactive components) the real
//!      `.island()` builder.
//!   3. **Theme it** — the preview wrapped in a `[data-q-demo-surface]` box driven
//!      by a `demo-theme-control` island, so the reader can flip
//!      size/density/radius/surface live and SCOPED to that one box.
//!   4. **Code** — the builder call that constructs the component.
//!
//! The standalone `/components` index is gone: the docs page
//! `/docs/quill/components` (a `Render::Doc` content page) is the index.

use qexec::FunctionResponse;
use qquill_design::{
    Accordion, Alert, Badge, Breadcrumb, Button, Card, Checkbox, Crumb, Dialog, Divider, Effect,
    List, ListItem, Menu, MenuItem, Motion, Radius, Section as AccSection, Severity, Size, Stat,
    Styled, SwitchGroup, Tabs, Tone, Tooltip, Trend, Variant, Variants,
};
use qquill_docs::CodeBlock;
use qquill_view::{el, island, text, Node, Trigger};

use crate::app::docs_kit::{self, Product, Toc};
use crate::app::shell::page;
use crate::app::{Css, Meta};

// ---------------------------------------------------------------------------
// The generic playground (server-rendered variant matrix; the island flips
// which cell is visible and rewrites the snippet — no client HTML build).
// ---------------------------------------------------------------------------

/// One control axis: a legend, the option values (lowercase, = the cell's data
/// attribute + the snippet token value), and the default value.
struct Axis {
    legend: &'static str,
    values: &'static [&'static str],
    default: &'static str,
}

/// A playground spec for one component. `render_cell(variant, size, tone)` must
/// return the styled component for that exact combination; `template` is the
/// `view!`-style snippet with `{variant}`/`{size}`/`{tone}` tokens the island
/// substitutes (Title-cased) live.
struct Playground<'a> {
    id: &'static str,
    variant: Axis,
    size: Axis,
    tone: Axis,
    template: &'static str,
    render_cell: &'a dyn Fn(&str, &str, &str) -> Styled,
}

/// A segmented control group for one axis: pressed = the default at SSR time.
fn control_group(axis_name: &str, axis: &Axis) -> Node {
    let mut seg = el("div")
        .class("pg__seg")
        .attr("role", "group")
        .attr("aria-label", axis.legend);
    for &v in axis.values {
        let pressed = v == axis.default;
        seg = seg.child(
            el("button")
                .class("pg__opt")
                .attr("type", "button")
                .attr("data-axis", axis_name.to_string())
                .attr("data-value", v.to_string())
                .attr("aria-pressed", if pressed { "true" } else { "false" })
                .child(text(v.to_string())),
        );
    }
    el("div")
        .class("pg__group")
        .child(
            el("span")
                .class("pg__legend")
                .child(text(axis.legend.to_string())),
        )
        .child(seg)
}

/// Title-case a lowercase axis value for the SSR snippet (`solid` → `Solid`).
fn title(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Substitute the snippet tokens for the default combination (the SSR snippet).
fn default_snippet(template: &str, pg: &Playground) -> String {
    template
        .replace("{variant}", &title(pg.variant.default))
        .replace("{size}", &title(pg.size.default))
        .replace("{tone}", &title(pg.tone.default))
}

/// Build the playground island for `pg`, collecting every cell's CSS into `css`.
fn playground(css: &mut Css, pg: &Playground) -> Node {
    // The pre-rendered matrix: one cell per (variant, size, tone); default
    // active. The component's companion CSS for every variant is collected so
    // switching never needs new styles.
    let mut stage = el("div").class("pg__stage").attr("data-q-part", "stage");
    for &v in pg.variant.values {
        for &s in pg.size.values {
            for &t in pg.tone.values {
                let styled = (pg.render_cell)(v, s, t);
                let node = css.node(styled);
                let active =
                    v == pg.variant.default && s == pg.size.default && t == pg.tone.default;
                stage = stage.child(
                    el("div")
                        .class("pg__cell")
                        .attr("data-variant", v.to_string())
                        .attr("data-size", s.to_string())
                        .attr("data-tone", t.to_string())
                        .attr("data-active", if active { "true" } else { "false" })
                        .child(node),
                );
            }
        }
    }

    let controls = el("div")
        .class("pg__controls")
        .child(control_group("variant", &pg.variant))
        .child(control_group("size", &pg.size))
        .child(control_group("tone", &pg.tone));

    // The live code panel: SSR shows the default snippet; the island rewrites
    // it on every change.
    let code = el("pre")
        .class("q-code")
        .attr("data-q-part", "code")
        .child(el("code").child(text(default_snippet(pg.template, pg))));

    let fallback = el("div")
        .class("pg")
        .child(stage)
        .child(controls)
        .child(el("div").class("pg__codewrap").child(code));

    // Props seed the island with the default selection + the snippet template.
    let props = format!(
        "{{\"component\":\"{}\",\"variant\":\"{}\",\"size\":\"{}\",\"tone\":\"{}\",\"template\":{}}}",
        pg.id,
        pg.variant.default,
        pg.size.default,
        pg.tone.default,
        json_string(pg.template),
    );

    island(
        leak_id(pg.id, "-pg"),
        "playground",
        Trigger::Load,
        props,
        fallback,
    )
}

/// A minimal JSON string encoder for the template (the only field needing
/// escaping). Handles the characters that can appear in a Rust code snippet.
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Build a `'static` instance id `"{id}{suffix}"`. Island ids accept
/// `Cow<'static, str>`; the page set is fixed and small, so leaking these few
/// short strings once per render is acceptable and keeps the signature simple.
fn leak_id(id: &'static str, suffix: &'static str) -> &'static str {
    Box::leak(format!("{id}{suffix}").into_boxed_str())
}

// ---------------------------------------------------------------------------
// Axis enum mappers (lowercase value → design enum)
// ---------------------------------------------------------------------------

fn to_variant(s: &str) -> Variant {
    match s {
        "soft" => Variant::Soft,
        "outline" => Variant::Outline,
        "ghost" => Variant::Ghost,
        _ => Variant::Solid,
    }
}
fn to_size(s: &str) -> Size {
    match s {
        "sm" => Size::Sm,
        "lg" => Size::Lg,
        _ => Size::Md,
    }
}
fn to_tone(s: &str) -> Tone {
    match s {
        "neutral" => Tone::Neutral,
        "danger" => Tone::Danger,
        _ => Tone::Brand,
    }
}
fn to_effect(s: &str) -> Effect {
    match s {
        "glass" => Effect::Glass,
        "gradient" => Effect::Gradient,
        "elevated" => Effect::Elevated,
        _ => Effect::Flat,
    }
}
fn to_radius(s: &str) -> Radius {
    match s {
        "sm" => Radius::Sm,
        "lg" => Radius::Lg,
        "xl" => Radius::Xl,
        "full" => Radius::Full,
        _ => Radius::Md,
    }
}

fn to_severity(s: &str) -> Severity {
    match s {
        "success" => Severity::Success,
        "warn" => Severity::Warn,
        "danger" => Severity::Danger,
        _ => Severity::Info,
    }
}
fn to_trend(s: &str) -> Trend {
    match s {
        "up" => Trend::Up,
        "down" => Trend::Down,
        _ => Trend::Flat,
    }
}

const VARIANT_VALUES: &[&str] = &["solid", "soft", "outline", "ghost"];
const SIZE_VALUES: &[&str] = &["sm", "md", "lg"];
const TONE_VALUES: &[&str] = &["brand", "neutral", "danger"];
const RADIUS_VALUES: &[&str] = &["sm", "md", "lg", "xl", "full"];
const EFFECT_VALUES: &[&str] = &["flat", "glass", "gradient", "elevated"];
const SEVERITY_VALUES: &[&str] = &["info", "success", "warn", "danger"];

// ---------------------------------------------------------------------------
// Shared page chrome: every component page is a Quill docs page.
// ---------------------------------------------------------------------------

/// The component-page CSS: the static preview grid, the demo-surface "Theme it"
/// box, and its scoped segmented controls (the same visual language as the
/// playground's `pg__seg`/`pg__opt`). Token-only, so it flips with the live
/// theme/size/radius/surface axes. Pushed once per page; the accumulator dedupes.
fn comp_css() -> &'static str {
    "\
/* ---- static preview showcase ---- */\
.cl-preview{margin:1.25rem 0;padding:1.75rem;border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-xl);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none)}\
.cl-row{display:flex;flex-wrap:wrap;align-items:center;gap:.85rem;margin:0 0 1rem}\
.cl-row:last-child{margin-bottom:0}\
.cl-row__label{flex:0 0 auto;min-width:6.5rem;font-size:.72rem;text-transform:uppercase;letter-spacing:.07em;font-weight:var(--q-font-weight-bold);color:var(--q-color-muted)}\
.cl-stack{display:flex;flex-direction:column;gap:.75rem;align-items:flex-start}\
.cl-grid{display:flex;flex-wrap:wrap;gap:1rem}\
/* ---- the live (island) demo surface ---- */\
.cl-demo{margin:1.25rem 0}\
.cl-demo__stage{padding:1.75rem;border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-xl);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);display:flex;flex-wrap:wrap;gap:1rem;align-items:center}\
/* ---- 'Theme it': a SCOPED demo surface + its segmented controls ---- */\
.cl-themeit{margin:1.25rem 0;border:1px solid var(--q-color-border);border-radius:var(--q-radius-xl);background:var(--q-color-surface);box-shadow:var(--q-shadow-sm);overflow:hidden}\
.cl-themeit__surface{display:flex;flex-wrap:wrap;gap:1rem;align-items:center;justify-content:center;min-height:11rem;padding:var(--q-space-6);background:color-mix(in srgb,var(--q-color-bg) 96%,var(--q-color-brand) 4%);border-bottom:1px solid var(--q-color-border);transition:padding var(--q-duration-base) var(--q-ease-out),background var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.cl-themeit__surface :where(.qq-badge,.qq-btn,.qq-card){box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);transition:background var(--q-duration-base) var(--q-ease-out),background-color var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out),box-shadow var(--q-duration-base) var(--q-ease-out),color var(--q-duration-base) var(--q-ease-out),transform var(--q-duration-base) var(--q-ease-out),filter var(--q-duration-base) var(--q-ease-out)}\
.cl-themeit__surface .qq-badge.qq-badge--soft,.cl-themeit__surface .qq-btn.qq-btn--soft,.cl-themeit__surface .qq-btn.qq-btn--neutral.qq-btn--solid{background:var(--q-surface-ring,var(--q-color-surface));border-color:var(--q-surface-border,var(--q-color-border))}\
.cl-themeit__surface .qq-card{background:var(--q-surface-bg,var(--q-color-surface));border-color:var(--q-surface-border,var(--q-color-border));box-shadow:var(--q-surface-shadow,none)}\
.cl-themeit__surface[data-q-surface=\"gradient\"] .qq-badge.qq-badge--soft,.cl-themeit__surface[data-q-surface=\"gradient\"] .qq-btn.qq-btn--soft,.cl-themeit__surface[data-q-surface=\"gradient\"] .qq-btn.qq-btn--neutral.qq-btn--solid{background:var(--q-surface-bg,var(--q-color-surface))}\
.cl-themeit__surface[data-q-surface=\"gradient\"] :where(.qq-btn--solid,.qq-badge--solid){background:linear-gradient(135deg,var(--q-color-brand),var(--q-color-accent));border-color:color-mix(in srgb,var(--q-color-brand) 70%,var(--q-color-accent));box-shadow:0 18px 46px -26px color-mix(in srgb,var(--q-color-brand) 88%,transparent),inset 0 1px 0 color-mix(in srgb,var(--q-color-on-brand) 24%,transparent)}\
.cl-themeit__surface[data-q-surface=\"gradient\"] :where(.qq-btn--outline,.qq-badge--outline){background:linear-gradient(var(--q-color-surface),var(--q-color-surface)) padding-box,linear-gradient(135deg,var(--q-color-brand),var(--q-color-accent)) border-box;border-color:transparent}\
.cl-themeit__surface[data-q-surface=\"glass\"]{background:color-mix(in srgb,var(--q-color-surface) 86%,transparent);-webkit-backdrop-filter:saturate(1.25) blur(16px);backdrop-filter:saturate(1.25) blur(16px)}\
.cl-themeit__surface[data-q-motion=\"snappy\"] :where(.qq-btn,.qq-badge,.qq-card):hover{transform:translateY(-1px)}\
.cl-themeit__surface[data-q-motion=\"playful\"] :where(.qq-btn,.qq-badge,.qq-card):hover{transform:translateY(-2px) scale(1.035) rotate(-.35deg)}\
.cl-themeit__surface[data-q-motion=\"none\"] :where(.qq-btn,.qq-badge,.qq-card),.cl-themeit__surface[data-q-motion=\"none\"] :where(.qq-btn,.qq-badge,.qq-card):hover{transform:none;transition:none}\
.cl-tc{display:flex;flex-wrap:wrap;gap:var(--q-space-5);padding:var(--q-space-4) var(--q-space-5);background:var(--q-color-surface)}\
.cl-tc__group{display:flex;flex-direction:column;gap:var(--q-space-2)}\
.cl-tc__legend{font-size:var(--q-font-size-xs);text-transform:uppercase;letter-spacing:.08em;color:var(--q-color-muted);font-weight:var(--q-font-weight-bold)}\
.cl-tc__seg{display:inline-flex;border:1px solid var(--q-color-border);border-radius:min(var(--q-radius-md),calc(var(--q-control-h,2.5rem) * .28));overflow:hidden}\
.cl-tc__opt{appearance:none;display:flex;align-items:center;justify-content:center;min-height:calc(var(--q-control-h,2.5rem) - var(--q-space-2));border:0;background:transparent;color:var(--q-color-muted);font:inherit;font-size:var(--q-font-size-sm);padding:0 var(--q-space-3);cursor:pointer;transition:background-color var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast) var(--q-ease-out)}\
	.cl-tc__opt+.cl-tc__opt{border-left:1px solid var(--q-color-border)}\
	.cl-tc__opt:hover{color:var(--q-color-fg)}\
	.cl-tc__opt[aria-pressed=\"true\"]{background:var(--q-color-brand);color:var(--q-color-on-brand)}\
	/* ---- live code panes for Playground + Theme it ---- */\
	.q-doc-body .pg__codewrap,.cl-themeit__code{position:relative;border-top:1px solid var(--q-surface-border,var(--q-color-border));background:var(--q-surface-bg,var(--q-color-surface));box-shadow:inset 0 1px 0 color-mix(in srgb,var(--q-color-fg) 4%,transparent)}\
	.q-doc-body .pg .q-code,.cl-themeit__code .q-code{display:block;width:100%;min-height:calc(var(--q-control-h,2.5rem) * 3.2);max-height:24rem;margin:0;border:0;border-radius:0;background:var(--q-surface-bg,var(--q-color-surface));color:var(--q-color-fg);padding:var(--q-space-4) var(--q-space-5);overflow:auto;white-space:pre;box-shadow:none}\
	.q-doc-body .pg .q-code code,.cl-themeit__code .q-code code{display:block;min-width:max-content;font-family:inherit;color:inherit}\
	.cl-themeit__code{border-top-color:var(--q-surface-border,var(--q-color-border))}"
}

/// One labelled row in the static preview (`label:` + the rendered cells).
fn preview_row(label: &str, cells: Vec<Node>) -> Node {
    let mut grid = el("div").class("cl-grid");
    for c in cells {
        grid = grid.child(c);
    }
    el("div")
        .class("cl-row")
        .child(
            el("span")
                .class("cl-row__label")
                .child(text(label.to_string())),
        )
        .child(grid)
}

/// One scoped segmented control group for the "Theme it" island (size / radius /
/// surface). Mirrors `control_group`, but emits the `cl-tc__*` classes and the
/// `data-q-axis`/`data-q-value` attributes the `demo-theme-control` behavior reads.
fn tc_group(legend: &str, axis: &str, values: &[(&str, &str)], default: &str) -> Node {
    let mut seg = el("div")
        .class("cl-tc__seg")
        .attr("role", "group")
        .attr("aria-label", legend.to_string());
    for (value, shown) in values {
        let pressed = *value == default;
        seg = seg.child(
            el("button")
                .class("cl-tc__opt")
                .attr("type", "button")
                .attr("data-q-axis", axis.to_string())
                .attr("data-q-value", value.to_string())
                .attr("aria-pressed", if pressed { "true" } else { "false" })
                .child(text(shown.to_string())),
        );
    }
    el("div")
        .class("cl-tc__group")
        .child(
            el("span")
                .class("cl-tc__legend")
                .child(text(legend.to_string())),
        )
        .child(seg)
}

fn theme_component_from_instance(instance: &'static str) -> &'static str {
    instance.strip_suffix("-themeit").unwrap_or(instance)
}

fn theme_it_snippet(component: &str, default_variant: Option<&str>) -> String {
    let mut out = String::from("// This demo is scoped: these axes apply only to this preview.\n");
    out.push_str("el(\"div\")\n");
    out.push_str("    .attr(\"data-q-size\", \"cozy\")\n");
    out.push_str("    .attr(\"data-q-radius\", \"rounded\")\n");
    out.push_str("    .attr(\"data-q-surface\", \"flat\")\n");
    out.push_str("    .attr(\"data-q-motion\", \"smooth\")\n");
    if let Some(v) = default_variant {
        out.push_str(&format!("    .attr(\"data-q-variant\", \"{v}\")\n"));
    }
    out.push_str("    .child(\n");
    out.push_str(&theme_it_component_body(component));
    out.push_str("    )");
    out
}

/// The real builder body for a component's Theme-it snippet. Every component
/// gets a genuine `render()`/`.island()` call (no placeholder comments) so the
/// reader can copy working code. The `demo-theme-control` runtime preserves this
/// SSR body verbatim and only swaps the `Variant::*` token for button/badge as
/// the variant axis changes — so this is the SINGLE source of truth for bodies.
fn theme_it_component_body(component: &str) -> String {
    let body = match component {
        "button" => "Button::action(\"Button\")\n    .variants(Variants::new().variant(Variant::Solid))\n    .render()",
        "badge" => "Badge::badge(\"Badge\")\n    .variant(Variant::Soft)\n    .render()",
        "card" => "Card::new(\"summary\")\n    .header(el(\"div\").child(text(\"Card\")))\n    .body(el(\"p\").child(text(\"A surface container.\")))\n    .effect(Effect::Elevated)\n    .radius(Radius::Lg)\n    .tone(Tone::Neutral)\n    .motion(Motion::Tilt3d)\n    .render()",
        "alert" => "Alert::new(Severity::Info, \"Your changes have been saved.\")\n    .title(\"Info\")\n    .effect(Effect::Flat)\n    .radius(Radius::Md)\n    .render()",
        "stat" => "Stat::new(\"revenue\", \"Monthly revenue\", \"$48,250\")\n    .size(Size::Md)\n    .trend(\"8.2%\", Trend::Up)\n    .render()",
        "list" => "List::new()\n    .size(Size::Md)\n    .item(ListItem::new(text(\"Authenticate the request\")))\n    .item(ListItem::new(text(\"Authorize against the grant\")))\n    .render()",
        "divider" => "Divider::new().size(Size::Md).render()",
        "breadcrumb" => "Breadcrumb::new(vec![\n    Crumb::new(\"Home\", \"/\"),\n    Crumb::new(\"Components\", \"/docs/quill/components\"),\n])\n.size(Size::Md)\n.radius(Radius::Md)\n.render()",
        "tabs" => "Tabs::new(\"demo\", /* sections */).island(\"demo-island\")",
        "dialog" => "Dialog::new(\"demo\", /* ... */).island(\"demo-island\", \"Open dialog\")",
        "menu" => "Menu::new(\"demo\", /* items */).island(\"demo-island\")",
        "tooltip" => "Tooltip::new(\"Helpful hint\").island(\"demo-island\")",
        "checkbox" => "Checkbox::new(\"cb\", \"Email me product news\", false).island(\"cb-island\")",
        "switch" => "SwitchGroup::new(\"radios\", /* tracks */).tone(Tone::Brand).island(\"radios-island\")",
        "accordion" => "Accordion::new(\"faq\", /* sections */).open(0).island(\"faq-island\")",
        other => return format!("        /* {other}::new(...).render() */\n"),
    };
    // Indent the (possibly multi-line) builder body to sit under `.child(`.
    let mut out = String::new();
    for line in body.lines() {
        out.push_str("        ");
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// The "Theme it" section: the `preview` is placed DIRECTLY on a
/// `[data-q-demo-surface]` stage whose `data-q-size`/`data-q-radius`/
/// `data-q-surface` are flipped live and SCOPED by a `demo-theme-control`
/// island. The axes re-point the design tokens (`--q-space-*`, `--q-radius-*`,
/// and the surface tokens) on the stage, and the COMPONENT itself consumes
/// them — there is no decorative wrapper box being skinned in its place.
fn theme_it(toc: &mut Toc, instance: &'static str, preview: Node) -> Node {
    theme_it_with_variant(toc, instance, preview, None)
}

/// Variant-aware flavor of [`theme_it`]. Buttons and badges have a real
/// `solid | soft | outline | ghost` component axis, so those pages get an
/// explicit Variant control. Other components still get density/radius/surface/
/// motion without a fake no-op Variant row.
fn theme_it_with_variant(
    toc: &mut Toc,
    instance: &'static str,
    preview: Node,
    default_variant: Option<&'static str>,
) -> Node {
    let component = theme_component_from_instance(instance);
    let surface = el("div")
        .class("cl-themeit__surface")
        .attr("data-q-component", component)
        .attr("data-q-demo-surface", "")
        .attr("data-q-size", "cozy")
        .attr("data-q-radius", "rounded")
        .attr("data-q-surface", "flat")
        .attr("data-q-motion", "smooth");
    let surface = if let Some(v) = default_variant {
        surface.attr("data-q-variant", v)
    } else {
        surface
    }
    .child(preview);

    let controls = el("div")
        .class("cl-tc")
        .children(default_variant.map(|v| {
            tc_group(
                "Variant",
                "variant",
                &[
                    ("solid", "solid"),
                    ("soft", "soft"),
                    ("outline", "outline"),
                    ("ghost", "ghost"),
                ],
                v,
            )
        }))
        .child(tc_group(
            "Density",
            "size",
            &[
                ("compact", "compact"),
                ("cozy", "cozy"),
                ("comfortable", "comfortable"),
            ],
            "cozy",
        ))
        .child(tc_group(
            "Radius",
            "radius",
            &[("sharp", "sharp"), ("rounded", "rounded"), ("pill", "pill")],
            "rounded",
        ))
        .child(tc_group(
            "Surface",
            "surface",
            &[
                ("flat", "flat"),
                ("glass", "glass"),
                ("neu", "neu"),
                ("gradient", "gradient"),
            ],
            "flat",
        ))
        .child(tc_group(
            "Motion",
            "motion",
            &[
                ("smooth", "smooth"),
                ("snappy", "snappy"),
                ("playful", "playful"),
                ("none", "off"),
            ],
            "smooth",
        ));

    // The control island is OnLoad so the segmented state seeds + reflects
    // immediately; it writes only onto the scoped [data-q-demo-surface] box.
    let demo = island(
        instance,
        "demo-theme-control",
        Trigger::Load,
        format!(
            "{{\"component\":\"{}\",\"hasVariant\":{}}}",
            component,
            if default_variant.is_some() {
                "true"
            } else {
                "false"
            }
        ),
        el("div")
            .class("cl-themeit")
            .child(surface)
            .child(controls)
            .child(
                el("div").class("cl-themeit__code").child(
                    el("pre")
                        .class("q-code")
                        .attr("data-q-part", "theme-code")
                        .child(
                            el("code").child(text(theme_it_snippet(component, default_variant))),
                        ),
                ),
            ),
    );

    el("div")
        .child(toc.h2("Theme it"))
        .child(docs_kit::p(
            "Flip the component axes for this preview alone — the controls re-point scoped design \
             tokens on the demo stage and the **component itself** restyles. Variant controls swap \
             the component class, surface controls change the visual treatment, and motion controls \
             change the interaction feel without leaking to the page.",
        ))
        .child(demo)
}

/// A `## Code` section: a heading + lead + a copyable `CodeBlock`.
fn code_example(toc: &mut Toc, code: &str) -> Node {
    el("div")
        .child(toc.h2("Code"))
        .child(docs_kit::p("Construct the component with its builder:"))
        .child(CodeBlock::new("rust", code.to_string()).render())
}

/// Assemble + ship a component page inside the Quill docs (sidebar + TOC).
/// `path` MUST be the page's `DocRef` path under `Product::Quill`.
fn ship(
    path: &'static str,
    title_text: &str,
    lead: &str,
    body: Node,
    toc: Toc,
    mut css: Css,
) -> FunctionResponse {
    css.push(docs_kit::docs_extras_css().to_string());
    css.push(docs_kit::pager_css().to_string());
    css.push(comp_css().to_string());
    // The docs content primitives' CSS (the `.qq-code` block used by CodeBlock).
    css.push(qquill_docs::layout_css().to_css());

    let main = docs_kit::layout(Product::Quill, path, title_text, lead, body, toc);

    let full_title = format!("{title_text} — Qirava docs");
    let desc: String = lead.chars().take(155).collect();
    let meta = Meta {
        title: &full_title,
        description: &desc,
        path,
    };
    page(&meta, css, main)
}

// ===========================================================================
// Variant-matrix components (Button / Badge / Card / Alert / Stat / List /
// Divider / Breadcrumb): a static preview of every axis + the playground island
// + a scoped "Theme it" box + the construction code.
// ===========================================================================

pub fn respond_button(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |v: &str, s: &str, t: &str| -> Styled {
        Button::action("Button")
            .variants(
                Variants::new()
                    .variant(to_variant(v))
                    .size(to_size(s))
                    .tone(to_tone(t)),
            )
            .render()
    };

    // Preview: variants (one row), sizes (one row), tones (one row).
    let variants: Vec<Node> = VARIANT_VALUES
        .iter()
        .map(|v| css.node(cell(v, "md", "brand")))
        .collect();
    let sizes: Vec<Node> = SIZE_VALUES
        .iter()
        .map(|s| css.node(cell("solid", s, "brand")))
        .collect();
    let tones: Vec<Node> = TONE_VALUES
        .iter()
        .map(|t| css.node(cell("solid", "md", t)))
        .collect();
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("variant", variants))
        .child(preview_row("size", sizes))
        .child(preview_row("tone", tones));

    let pg = Playground {
        id: "button",
        variant: Axis { legend: "Variant", values: VARIANT_VALUES, default: "solid" },
        size: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        tone: Axis { legend: "Tone", values: TONE_VALUES, default: "brand" },
        template: "Button::action(\"Button\")\n    .variants(\n        Variants::new()\n            .variant(Variant::{variant})\n            .size(Size::{size})\n            .tone(Tone::{tone}),\n    )\n    .render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("solid", "md", "brand")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p("Every fill variant, size, and tone — server-rendered, correct with JavaScript off."))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it_with_variant(&mut toc, "button-themeit", theme_preview, Some("solid")))
        .child(code_example(
            &mut toc,
            "Button::action(\"Save changes\")\n    .variants(\n        Variants::new()\n            .variant(Variant::Solid)\n            .size(Size::Md)\n            .tone(Tone::Brand),\n    )\n    .render()",
        ));

    ship(
        "/docs/quill/components/button",
        "Button",
        "An action or toggle. The fill is one of four variants (solid, soft, outline, ghost), \
         orthogonal to size and tone. Use a solid brand button for the primary action; prefer ghost \
         or outline for secondary actions; reserve danger tone for destructive actions.",
        body,
        toc,
        css,
    )
}

pub fn respond_badge(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |v: &str, s: &str, t: &str| -> Styled {
        Badge::badge("Badge")
            .variant(to_variant(v))
            .size(to_size(s))
            .tone(to_tone(t))
            .render()
    };

    let variants: Vec<Node> = VARIANT_VALUES
        .iter()
        .map(|v| css.node(cell(v, "sm", "brand")))
        .collect();
    let sizes: Vec<Node> = SIZE_VALUES
        .iter()
        .map(|s| css.node(cell("soft", s, "brand")))
        .collect();
    let tones: Vec<Node> = TONE_VALUES
        .iter()
        .map(|t| css.node(cell("soft", "sm", t)))
        .collect();
    // The kind axis: a plain badge, a tag, and a removable chip.
    let kinds: Vec<Node> = vec![
        css.node(
            Badge::badge("Badge")
                .tone(Tone::Brand)
                .variant(Variant::Soft)
                .render(),
        ),
        css.node(
            Badge::tag("Tag")
                .tone(Tone::Neutral)
                .variant(Variant::Outline)
                .render(),
        ),
        css.node(
            Badge::chip("Chip")
                .removable(true)
                .tone(Tone::Brand)
                .variant(Variant::Soft)
                .render(),
        ),
    ];
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("variant", variants))
        .child(preview_row("size", sizes))
        .child(preview_row("tone", tones))
        .child(preview_row("kind", kinds));

    let pg = Playground {
        id: "badge",
        variant: Axis { legend: "Variant", values: VARIANT_VALUES, default: "soft" },
        size: Axis { legend: "Size", values: SIZE_VALUES, default: "sm" },
        tone: Axis { legend: "Tone", values: TONE_VALUES, default: "brand" },
        template: "Badge::badge(\"Badge\")\n    .variant(Variant::{variant})\n    .size(Size::{size})\n    .tone(Tone::{tone})\n    .render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("soft", "sm", "brand")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "Status pills, tags, and removable chips across every variant, size, and tone.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it_with_variant(&mut toc, "badge-themeit", theme_preview, Some("soft")))
        .child(code_example(
            &mut toc,
            "Badge::badge(\"Active\").variant(Variant::Soft).tone(Tone::Brand).render()\nBadge::tag(\"Draft\").variant(Variant::Outline).tone(Tone::Neutral).render()\nBadge::chip(\"Filter\").removable(true).render()",
        ));

    ship(
        "/docs/quill/components/badge",
        "Badge",
        "A compact status pill, tag, or chip. Soft brand reads as an active status; outline neutral \
         reads as an inert label; a chip can be removable. Same variant/size/tone axes as the rest \
         of the system — use tone to encode meaning, not decoration.",
        body,
        toc,
        css,
    )
}

pub fn respond_card(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |effect: &str, radius: &str, tone: &str| -> Styled {
        let body = el("p")
            .class("q-muted")
            .child(text("A surface container.".to_string()));
        Card::new(format!("cl-card-{effect}-{radius}-{tone}"))
            .region()
            .header(
                el("div")
                    .class("q-card-eyebrow")
                    .child(text("Card".to_string())),
            )
            .body(body)
            .effect(to_effect(effect))
            .radius(to_radius(radius))
            .tone(to_tone(tone))
            .render()
    };

    let effects: Vec<Node> = EFFECT_VALUES
        .iter()
        .map(|e| css.node(cell(e, "lg", "neutral")))
        .collect();
    let radii: Vec<Node> = ["sm", "md", "lg", "xl"]
        .iter()
        .map(|r| css.node(cell("elevated", r, "neutral")))
        .collect();
    let tones: Vec<Node> = TONE_VALUES
        .iter()
        .map(|t| css.node(cell("elevated", "lg", t)))
        .collect();
    let motion_cell = |id: &str, label: &str, motion: Motion| -> Styled {
        Card::new(format!("cl-card-motion-{id}"))
            .region()
            .header(
                el("div")
                    .class("q-card-eyebrow")
                    .child(text(label.to_string())),
            )
            .body(
                el("p")
                    .class("q-muted")
                    .child(text("Design-system motion, not page CSS.".to_string())),
            )
            .effect(Effect::Elevated)
            .radius(Radius::Lg)
            .tone(Tone::Neutral)
            .motion(motion)
            .render()
    };
    let motions: Vec<Node> = vec![
        css.node(motion_cell("lift", "Lift", Motion::Lift)),
        css.node(motion_cell("tilt3d", "3D tilt", Motion::Tilt3d)),
    ];
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("effect", effects))
        .child(preview_row("radius", radii))
        .child(preview_row("tone", tones))
        .child(preview_row("motion", motions));
    // The 3D card in the preview carries data-q-tilt; hydrate it with Quill's
    // generic `motion` behavior so docs prove the runtime path, not just CSS.
    let preview = island(
        "card-motion-preview",
        "motion",
        Trigger::Load,
        "{}",
        preview,
    );

    let pg = Playground {
        id: "card",
        variant: Axis { legend: "Effect", values: EFFECT_VALUES, default: "elevated" },
        size: Axis { legend: "Radius", values: &["sm", "md", "lg", "xl"], default: "lg" },
        tone: Axis { legend: "Tone", values: TONE_VALUES, default: "neutral" },
        template: "Card::new(\"card\")\n    .header(/* ... */)\n    .body(/* ... */)\n    .effect(Effect::{variant})\n    .radius(Radius::{size})\n    .tone(Tone::{tone})\n    .render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("elevated", "lg", "neutral")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "The surface effect, corner radius, tone, and motion — orthogonal treatments you compose.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it(&mut toc, "card-themeit", theme_preview))
        .child(code_example(
            &mut toc,
            "Card::new(\"summary\")\n    .header(el(\"div\").child(text(\"Card\")))\n    .body(el(\"p\").child(text(\"A surface container.\")))\n    .effect(Effect::Elevated)\n    .radius(Radius::Lg)\n    .tone(Tone::Neutral)\n    .motion(Motion::Tilt3d)\n    .render()",
        ));

    ship(
        "/docs/quill/components/card",
        "Card",
        "A surface container for grouped content. Its axes are the surface effect (flat, glass, \
         gradient, elevated), corner radius, tone, and motion. Use lift or 3D tilt for hero/product \
         cards; keep dense grids static.",
        body,
        toc,
        css,
    )
}

pub fn respond_alert(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |severity: &str, effect: &str, radius: &str| -> Styled {
        Alert::new(to_severity(severity), "Your changes have been saved.")
            .title(title(severity))
            .effect(to_effect(effect))
            .radius(to_radius(radius))
            .render()
    };

    let severities: Vec<Node> = SEVERITY_VALUES
        .iter()
        .map(|sv| {
            el("div")
                .class("cl-stack")
                .child(css.node(cell(sv, "flat", "md")))
        })
        .collect();
    let effects: Vec<Node> = EFFECT_VALUES
        .iter()
        .map(|e| {
            el("div")
                .class("cl-stack")
                .child(css.node(cell("info", e, "md")))
        })
        .collect();
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("severity", severities))
        .child(preview_row("effect", effects));

    let pg = Playground {
        id: "alert",
        variant: Axis { legend: "Severity", values: SEVERITY_VALUES, default: "info" },
        size: Axis { legend: "Effect", values: EFFECT_VALUES, default: "flat" },
        tone: Axis { legend: "Radius", values: RADIUS_VALUES, default: "md" },
        template: "Alert::new(Severity::{variant}, \"Your changes have been saved.\")\n    .title(\"{variant}\")\n    .effect(Effect::{size})\n    .radius(Radius::{tone})\n    .render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("info", "flat", "md")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "Each severity (info, success, warn, danger) with its accent, plus the surface-effect axis.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it(&mut toc, "alert-themeit", theme_preview))
        .child(code_example(
            &mut toc,
            "Alert::new(Severity::Success, \"Your changes have been saved.\")\n    .title(\"Saved\")\n    .effect(Effect::Flat)\n    .radius(Radius::Md)\n    .render()",
        ));

    ship(
        "/docs/quill/components/alert",
        "Alert",
        "An inline status banner that announces itself (role=alert). Its primary axis is severity \
         (info, success, warn, danger), each with a tone-colored accent; the surface effect and \
         corner radius are orthogonal. Use the severity that matches the message — never danger for \
         decoration.",
        body,
        toc,
        css,
    )
}

pub fn respond_stat(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |size: &str, trend: &str, _t: &str| -> Styled {
        Stat::new("cl-stat", "Monthly revenue", "$48,250")
            .size(to_size(size))
            .trend("8.2%", to_trend(trend))
            .render()
    };

    let sizes: Vec<Node> = SIZE_VALUES
        .iter()
        .map(|s| css.node(cell(s, "up", "")))
        .collect();
    let trends: Vec<Node> = ["up", "down", "flat"]
        .iter()
        .map(|tr| css.node(cell("md", tr, "")))
        .collect();
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("size", sizes))
        .child(preview_row("trend", trends));

    let pg = Playground {
        id: "stat",
        variant: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        size: Axis { legend: "Trend", values: &["up", "down", "flat"], default: "up" },
        tone: Axis { legend: "Value", values: &["figure"], default: "figure" },
        template: "Stat::new(\"revenue\", \"Monthly revenue\", \"$48,250\")\n    .size(Size::{variant})\n    .trend(\"8.2%\", Trend::{size})\n    .render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("md", "up", "")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "A labeled key figure with an optional trend chip — size and trend-direction axes.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it(&mut toc, "stat-themeit", theme_preview))
        .child(code_example(
            &mut toc,
            "Stat::new(\"revenue\", \"Monthly revenue\", \"$48,250\")\n    .size(Size::Md)\n    .trend(\"8.2%\", Trend::Up)\n    .render()",
        ));

    ship(
        "/docs/quill/components/stat",
        "Stat",
        "A labeled key figure (label + value + optional trend), grouped so assistive tech reads it \
         as one unit. The trend direction is carried in text and a glyph, never by color alone. Its \
         axes are size and trend direction.",
        body,
        toc,
        css,
    )
}

pub fn respond_list(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |kind: &str, size: &str, _t: &str| -> Styled {
        let base = if kind == "ordered" {
            List::ordered()
        } else {
            List::new()
        };
        base.size(to_size(size))
            .item(ListItem::new(text("Authenticate the request".to_string())))
            .item(ListItem::new(text(
                "Authorize against the grant".to_string(),
            )))
            .item(ListItem::new(text(
                "Plan and execute the query".to_string(),
            )))
            .render()
    };

    let kinds: Vec<Node> = ["unordered", "ordered"]
        .iter()
        .map(|k| css.node(cell(k, "md", "")))
        .collect();
    let sizes: Vec<Node> = SIZE_VALUES
        .iter()
        .map(|s| css.node(cell("unordered", s, "")))
        .collect();
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("kind", kinds))
        .child(preview_row("size", sizes));

    let pg = Playground {
        id: "list",
        variant: Axis { legend: "Kind", values: &["unordered", "ordered"], default: "unordered" },
        size: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        tone: Axis { legend: "Rows", values: &["items"], default: "items" },
        template: "List::new() // or List::ordered()\n    .size(Size::{size})\n    .item(ListItem::new(text(\"Authenticate the request\")))\n    .item(ListItem::new(text(\"Authorize against the grant\")))\n    .item(ListItem::new(text(\"Plan and execute the query\")))\n    .render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("unordered", "md", "")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "Native ul/ol rows, each with an optional leading slot — the kind and size axes.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it(&mut toc, "list-themeit", theme_preview))
        .child(code_example(
            &mut toc,
            "List::new() // or List::ordered() for a numbered list\n    .size(Size::Md)\n    .item(ListItem::new(text(\"Authenticate the request\")))\n    .item(ListItem::new(text(\"Authorize against the grant\")))\n    .item(ListItem::new(text(\"Plan and execute the query\")))\n    .render()",
        ));

    ship(
        "/docs/quill/components/list",
        "List",
        "A native ul/ol of rows, each with an optional leading slot (an icon or avatar) and main \
         content. Using the native list element means list semantics come for free. Its axes are the \
         kind (unordered or ordered) and the row size.",
        body,
        toc,
        css,
    )
}

pub fn respond_divider(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |orientation: &str, size: &str, _t: &str| -> Styled {
        let mut d = Divider::new().size(to_size(size));
        if orientation == "vertical" {
            d = d.vertical();
        }
        d.render()
    };

    // Horizontal dividers want their own full-width row; verticals sit in a flex
    // row to show their height.
    let h_sizes: Vec<Node> = SIZE_VALUES
        .iter()
        .map(|s| {
            el("div")
                .attr("style", "width:14rem")
                .child(css.node(cell("horizontal", s, "")))
        })
        .collect();
    let v_demo = el("div")
        .attr(
            "style",
            "display:flex;align-items:center;gap:1rem;height:3rem",
        )
        .child(text("Left".to_string()))
        .child(css.node(cell("vertical", "md", "")))
        .child(text("Right".to_string()));
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("horizontal", h_sizes))
        .child(preview_row("vertical", vec![v_demo]));

    let pg = Playground {
        id: "divider",
        variant: Axis { legend: "Orientation", values: &["horizontal", "vertical"], default: "horizontal" },
        size: Axis { legend: "Spacing", values: SIZE_VALUES, default: "md" },
        tone: Axis { legend: "Rule", values: &["line"], default: "line" },
        template: "Divider::new()\n    .size(Size::{size})\n    // .vertical() for a vertical hairline\n    .render()",
        render_cell: &cell,
    };

    let theme_preview =
        el("div")
            .attr("style", "width:14rem")
            .child(css.node(cell("horizontal", "md", "")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "A separator rule — horizontal and vertical, across the spacing scale. A semantic \
             divider is role=separator; a decorative one is a hidden hr.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it(&mut toc, "divider-themeit", theme_preview))
        .child(code_example(
            &mut toc,
            "Divider::new().size(Size::Md).render()\n// A decorative, screen-reader-silent rule:\nDivider::decorative().render()\n// A vertical hairline (needs a flex row context):\nDivider::new().vertical().render()",
        ));

    ship(
        "/docs/quill/components/divider",
        "Divider",
        "A separator rule. A semantic divider renders as role=separator (announced as a structural \
         break); a decorative one is a hidden hr. Its axes are orientation (horizontal or vertical) \
         and the spacing scale around the line.",
        body,
        toc,
        css,
    )
}

pub fn respond_breadcrumb(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut toc = Toc::new();

    let cell = |size: &str, radius: &str, _t: &str| -> Styled {
        Breadcrumb::new(vec![
            Crumb::new("Home", "/"),
            Crumb::new("Components", "/docs/quill/components"),
            Crumb::new("Breadcrumb", "/docs/quill/components/breadcrumb"),
        ])
        .size(to_size(size))
        .radius(to_radius(radius))
        .render()
    };

    let sizes: Vec<Node> = SIZE_VALUES
        .iter()
        .map(|s| {
            el("div")
                .class("cl-stack")
                .child(css.node(cell(s, "md", "")))
        })
        .collect();
    let radii: Vec<Node> = RADIUS_VALUES
        .iter()
        .map(|r| {
            el("div")
                .class("cl-stack")
                .child(css.node(cell("md", r, "")))
        })
        .collect();
    let preview = el("div")
        .class("cl-preview")
        .child(preview_row("size", sizes))
        .child(preview_row("radius", radii));

    let pg = Playground {
        id: "breadcrumb",
        variant: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        size: Axis { legend: "Radius", values: RADIUS_VALUES, default: "md" },
        tone: Axis { legend: "Trail", values: &["crumbs"], default: "crumbs" },
        template: "Breadcrumb::new(vec![\n    Crumb::new(\"Home\", \"/\"),\n    Crumb::new(\"Components\", \"/docs/quill/components\"),\n    Crumb::new(\"Breadcrumb\", \"/docs/quill/components/breadcrumb\"),\n])\n.size(Size::{variant})\n.radius(Radius::{size})\n.render()",
        render_cell: &cell,
    };

    let theme_preview = el("div")
        .class("cl-stack")
        .child(css.node(cell("md", "md", "")));

    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Preview"))
        .child(docs_kit::p(
            "An ARIA trail (nav[aria-label=Breadcrumb] > ol) across the text-size and link-radius axes.",
        ))
        .child(preview)
        .child(toc.h2("Playground"))
        .child(docs_kit::p("Change the controls — the preview and the snippet update live."))
        .child(playground(&mut css, &pg))
        .child(theme_it(&mut toc, "breadcrumb-themeit", theme_preview))
        .child(code_example(
            &mut toc,
            "Breadcrumb::new(vec![\n    Crumb::new(\"Home\", \"/\"),\n    Crumb::new(\"Components\", \"/docs/quill/components\"),\n    Crumb::new(\"Breadcrumb\", \"/docs/quill/components/breadcrumb\"),\n])\n.size(Size::Md)\n.radius(Radius::Md)\n.render()",
        ));

    ship(
        "/docs/quill/components/breadcrumb",
        "Breadcrumb",
        "An ARIA-correct trail (nav[aria-label=Breadcrumb] > ol). Intermediate crumbs are links; the \
         current page is plain text carrying aria-current. The separator is decorative CSS. Its axes \
         are the text size and the focus-radius on the links.",
        body,
        toc,
        css,
    )
}

// ===========================================================================
// Interactive (island) components: Tabs / Dialog / Menu / Tooltip / Checkbox /
// Switch / Accordion. Their preview IS the live island (the real `.island()`
// builder); they still get a scoped "Theme it" box and the construction code.
// ===========================================================================

/// Build an interactive-component page: a live island demo + a scoped "Theme it"
/// box (re-mounting the same island under a fresh instance id) + the code. `demo`
/// and `theme_demo` are already-css-collected island nodes.
#[allow(clippy::too_many_arguments)]
fn island_page(
    path: &'static str,
    title_text: &str,
    lead: &str,
    demo_lead: &str,
    demo: Node,
    theme_demo: Node,
    theme_instance: &'static str,
    code: &str,
    mut toc: Toc,
    css: Css,
) -> FunctionResponse {
    let body = el("div")
        .class("q-doc-body")
        .child(toc.h2("Live demo"))
        .child(docs_kit::p(demo_lead))
        .child(
            el("div")
                .class("cl-demo")
                .child(el("div").class("cl-demo__stage").child(demo)),
        )
        .child(theme_it(
            &mut toc,
            theme_instance,
            el("div").class("cl-stack").child(theme_demo),
        ))
        .child(code_example(&mut toc, code));

    ship(path, title_text, lead, body, toc, css)
}

pub fn respond_tabs(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();

    let panel = |title: &str, body: &str| -> Node {
        el("div")
            .child(el("h3").class("q-h3").child(text(title.to_string())))
            .child(el("p").class("q-muted").child(text(body.to_string())))
    };
    let mut make = |id: &str, instance: &str| -> Node {
        let tabs = Tabs::new(
            id.to_string(),
            0,
            vec![
                ("Overview".to_string(), panel("Overview", "Tabs hydrate in place from the server-selected tab — no markup change, full ARIA tab pattern.")),
                ("Keyboard".to_string(), panel("Keyboard", "Arrow keys move selection (roving tabindex); Home/End jump to first/last.")),
                ("A11y".to_string(), panel("Accessibility", "role=tablist/tab/tabpanel, aria-selected, aria-controls, aria-labelledby — all wired by the headless layer.")),
            ],
        );
        css.node(tabs.island(instance.to_string()))
    };
    let demo = make("demo-tabs", "demo-tabs-island");
    let theme_demo = make("demo-tabs-2", "demo-tabs-island-2");

    island_page(
        "/docs/quill/components/tabs",
        "Tabs",
        "An ARIA-correct tab strip backed by the headless qquill-ui Tabs state machine. Tabs are \
         interactive by nature, so the showcase is a live island: the server-rendered selection is \
         preserved on hydrate, and arrow keys move selection with a roving tabindex.",
        "Click a tab or use the arrow keys; the server-rendered selection is preserved on hydrate.",
        demo,
        theme_demo,
        "tabs-themeit",
        "Tabs::new(\n    \"settings\",\n    0, // server-selected index\n    vec![\n        (\"Overview\".into(), panel(\"Overview\", \"…\")),\n        (\"Keyboard\".into(), panel(\"Keyboard\", \"…\")),\n        (\"A11y\".into(), panel(\"Accessibility\", \"…\")),\n    ],\n)\n.island(\"settings-tabs\")",
        Toc::new(),
        css,
    )
}

pub fn respond_dialog(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut make = |id: &str, instance: &str| -> Node {
        let body = el("p").class("q-muted").child(text(
            "This surface is focus-trapped while open; Escape or the backdrop closes it."
                .to_string(),
        ));
        let dialog =
            Dialog::modal(id.to_string(), "Delete project?", body).effect(Effect::Elevated);
        css.node(dialog.island(instance.to_string(), "Open dialog"))
    };
    let demo = make("demo-dialog", "demo-dialog-island");
    let theme_demo = make("demo-dialog-2", "demo-dialog-island-2");

    island_page(
        "/docs/quill/components/dialog",
        "Dialog",
        "A modal dialog: a trigger opens a backdrop + a role=dialog surface (aria-modal) that traps \
         focus until dismissed. Its title is auto-wired via aria-labelledby; Escape, the close \
         button, and a backdrop click all close it. The surface effect and radius are orthogonal.",
        "Click \"Open dialog\". The surface traps focus; Escape, the × button, or a backdrop click \
         closes it. With JavaScript off, the trigger + a closed dialog render as a valid fallback.",
        demo,
        theme_demo,
        "dialog-themeit",
        "let body = el(\"p\").child(text(\"This surface is focus-trapped while open …\"));\nDialog::modal(\"confirm-delete\", \"Delete project?\", body)\n    .effect(Effect::Elevated)\n    .island(\"confirm-delete-island\", \"Open dialog\")",
        Toc::new(),
        css,
    )
}

pub fn respond_menu(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut make = |id: &str, instance: &str| -> Node {
        let menu = Menu::new(
            id.to_string(),
            "Actions",
            vec![
                MenuItem::new("Rename"),
                MenuItem::new("Duplicate"),
                MenuItem::new("Archive").disabled(true),
                MenuItem::new("Delete"),
            ],
        );
        css.node(menu.island(instance.to_string()))
    };
    let demo = make("demo-menu", "demo-menu-island");
    let theme_demo = make("demo-menu-2", "demo-menu-island-2");

    island_page(
        "/docs/quill/components/menu",
        "Menu",
        "A popup action menu: a trigger (aria-haspopup / aria-expanded / aria-controls) opens a \
         role=menu surface of role=menuitem rows. The active item is tracked with a roving tabindex; \
         disabled items are skipped. The surface is hidden when closed.",
        "Click \"Actions\" to open the menu, then use the arrow keys (Home/End jump to the ends) — \
         the highlighted item gets a brand tint, and the disabled \"Archive\" row is skipped. Escape closes it.",
        demo,
        theme_demo,
        "menu-themeit",
        "Menu::new(\n    \"row-actions\",\n    \"Actions\",\n    vec![\n        MenuItem::new(\"Rename\"),\n        MenuItem::new(\"Duplicate\"),\n        MenuItem::new(\"Archive\").disabled(true),\n        MenuItem::new(\"Delete\"),\n    ],\n)\n.island(\"row-actions-island\")",
        Toc::new(),
        css,
    )
}

pub fn respond_tooltip(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut make = |id: &str, instance: &str| -> Node {
        let tooltip = Tooltip::new(
            id.to_string(),
            "Hover me",
            "Tooltips reveal on hover or focus, after a short delay.",
        );
        css.node(tooltip.island(instance.to_string()))
    };
    let demo = make("demo-tooltip", "demo-tooltip-island");
    let theme_demo = make("demo-tooltip-2", "demo-tooltip-island-2");

    island_page(
        "/docs/quill/components/tooltip",
        "Tooltip",
        "A tooltip: a trigger permanently aria-describedby a role=tooltip bubble, per the WAI-ARIA \
         APG pattern. The bubble is hidden until the trigger is hovered or focused (with a short \
         delay); the description is always available to assistive tech via the describedby link.",
        "Hover over (or tab to) the \"Hover me\" trigger — the bubble reveals after a short delay and \
         hides when you leave. With JavaScript off, the description is still announced on focus.",
        demo,
        theme_demo,
        "tooltip-themeit",
        "Tooltip::new(\n    \"save-hint\",\n    \"Hover me\",\n    \"Tooltips reveal on hover or focus, after a short delay.\",\n)\n.island(\"save-hint-island\")",
        Toc::new(),
        css,
    )
}

pub fn respond_checkbox(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut make = |suffix: &str| -> Node {
        let unchecked = css.node(
            Checkbox::new(format!("cb-news{suffix}"), "Email me product news", false)
                .island(format!("cb-news-island{suffix}")),
        );
        let checked = css.node(
            Checkbox::new(format!("cb-terms{suffix}"), "I accept the terms", true)
                .island(format!("cb-terms-island{suffix}")),
        );
        let mixed = css.node(
            Checkbox::mixed(format!("cb-all{suffix}"), "Select all (some selected)")
                .island(format!("cb-all-island{suffix}")),
        );
        el("div")
            .class("cl-stack")
            .child(unchecked)
            .child(checked)
            .child(mixed)
    };
    let demo = make("");
    let theme_demo = make("-2");

    island_page(
        "/docs/quill/components/checkbox",
        "Checkbox",
        "A checkbox: a role=checkbox box carrying aria-checked (true / false / mixed) next to its \
         label. The mixed state models a parent of partially-selected children. The box is focusable \
         and toggles on click or Space; the styled layer draws the check/dash glyph.",
        "Click any box (or focus it and press Space) to toggle it. The third box starts in the \
         indeterminate (mixed) state — a first click resolves it. Each renders in its initial state with JS off.",
        demo,
        theme_demo,
        "checkbox-themeit",
        "// Unchecked, checked, and indeterminate (mixed) — each a live island.\nCheckbox::new(\"cb-news\", \"Email me product news\", false).island(\"cb-news-island\")\nCheckbox::new(\"cb-terms\", \"I accept the terms\", true).island(\"cb-terms-island\")\nCheckbox::mixed(\"cb-all\", \"Select all (some selected)\").island(\"cb-all-island\")",
        Toc::new(),
        css,
    )
}

pub fn respond_switch(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let mut make = |id: &str, instance: &str| -> Node {
        let switches = SwitchGroup::new(
            id.to_string(),
            vec![
                ("Wi-Fi".to_string(), true),
                ("Bluetooth".to_string(), false),
                ("Airplane mode".to_string(), false),
            ],
        )
        .tone(Tone::Brand);
        css.node(switches.island(instance.to_string()))
    };
    let demo = make("demo-switch", "demo-switch-island");
    let theme_demo = make("demo-switch-2", "demo-switch-island-2");

    island_page(
        "/docs/quill/components/switch",
        "Switch",
        "A switch group: a labelled role=group of role=switch tracks, each a binary on/off toggle. \
         Unlike a checkbox (which submits a form value), a switch takes effect immediately. The \
         on-state fills the track with the tone accent and slides the knob; tone is an axis.",
        "Click a track (or focus it and press Space/Enter) to flip it — the knob slides and the track \
         fills with the brand accent. Each switch renders in its initial on/off state with JS off.",
        demo,
        theme_demo,
        "switch-themeit",
        "SwitchGroup::new(\n    \"radios\",\n    vec![\n        (\"Wi-Fi\".into(), true),\n        (\"Bluetooth\".into(), false),\n        (\"Airplane mode\".into(), false),\n    ],\n)\n.tone(Tone::Brand)\n.island(\"radios-island\")",
        Toc::new(),
        css,
    )
}

pub fn respond_accordion(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let panel = |body: &str| -> Node { el("p").class("q-muted").child(text(body.to_string())) };
    let mut make = |id: &str, instance: &str| -> Node {
        let accordion = Accordion::new(
            id.to_string(),
            vec![
                AccSection::new(
                    "What is an island?",
                    panel("A server-rendered region that hydrates in place on first interaction — never a client re-render."),
                ),
                AccSection::new(
                    "Does it work without JavaScript?",
                    panel("Yes. The server-open section shows and the rest are valid collapsed regions; hydration only adds toggling."),
                ),
                AccSection::new(
                    "How is it kept accessible?",
                    panel("The WAI-ARIA APG accordion pattern: heading > button with aria-expanded/-controls, and a labelled region panel."),
                ),
            ],
        )
        .open(0);
        css.node(accordion.island(instance.to_string()))
    };
    let demo = make("demo-accordion", "demo-accordion-island");
    let theme_demo = make("demo-accordion-2", "demo-accordion-island-2");

    island_page(
        "/docs/quill/components/accordion",
        "Accordion",
        "An accordion: a stack of disclosure sections, each a heading > button (aria-expanded / \
         aria-controls) over a labelled region panel. By default one section is open at a time; \
         .multiple() allows several. A rotating chevron reflects the open state.",
        "Click a section header (or focus it and press Enter/Space) to expand it; opening one \
         collapses the others. The first section starts open. With JavaScript off, the open section \
         shows and the rest are valid collapsed regions.",
        demo,
        theme_demo,
        "accordion-themeit",
        "Accordion::new(\n    \"faq\",\n    vec![\n        Section::new(\"What is an island?\", panel(\"A server-rendered region …\")),\n        Section::new(\"Does it work without JavaScript?\", panel(\"Yes. …\")),\n        Section::new(\"How is it kept accessible?\", panel(\"The WAI-ARIA APG accordion pattern …\")),\n    ],\n)\n.open(0)\n.island(\"faq-island\")",
        Toc::new(),
        css,
    )
}

#[cfg(test)]
mod tests {
    use super::{comp_css, theme_it_snippet};

    #[test]
    fn theme_it_surface_styles_the_demo_component_not_a_gradient_canvas() {
        let css = comp_css();
        assert!(css.contains(".cl-themeit__surface :where(.qq-badge,.qq-btn,.qq-card)"));
        assert!(css.contains(
            ".cl-themeit__surface[data-q-surface=\"gradient\"] .qq-badge.qq-badge--soft"
        ));
        assert!(css.contains(".cl-themeit__surface .qq-card{background:var(--q-surface-bg"));
        assert!(css.contains(".cl-themeit{margin:1.25rem 0;border:1px solid var(--q-color-border)"));
        assert!(
            !css.contains(".cl-themeit{margin:1.25rem 0;border:1px solid var(--q-surface-border")
        );
        assert!(!css.contains(".cl-themeit__surface{display:flex;flex-wrap:wrap;gap:1rem;align-items:center;justify-content:center;min-height:11rem;padding:var(--q-space-6);background:var(--q-surface-bg"));
        // Gradient surface gives solid/outline components a real gradient fill,
        // not just a wrapper recolor.
        assert!(css.contains(".cl-themeit__surface[data-q-surface=\"gradient\"] :where(.qq-btn--solid,.qq-badge--solid)"));
        // Motion axis changes interaction feel on the previewed component.
        assert!(css.contains(".cl-themeit__surface[data-q-motion=\"playful\"]"));
        // Segmented controls use the shared density/control tokens and never
        // become full stadiums at pill radius.
        assert!(css.contains(".cl-tc__seg{display:inline-flex;border:1px solid var(--q-color-border);border-radius:min(var(--q-radius-md),calc(var(--q-control-h,2.5rem) * .28))"));
        assert!(css.contains(".cl-tc__opt{appearance:none;display:flex;align-items:center;justify-content:center;min-height:calc(var(--q-control-h,2.5rem) - var(--q-space-2))"));
    }

    #[test]
    fn playground_and_theme_it_ship_visible_live_code_panes() {
        let css = comp_css();
        assert!(css.contains(".q-doc-body .pg__codewrap,.cl-themeit__code"));
        assert!(css.contains(".q-doc-body .pg .q-code,.cl-themeit__code .q-code{display:block;width:100%;min-height:calc(var(--q-control-h,2.5rem) * 3.2)"));
        assert!(css.contains("overflow:auto;white-space:pre"));

        let button = theme_it_snippet("button", Some("solid"));
        assert!(button.contains(".attr(\"data-q-size\", \"cozy\")"));
        assert!(button.contains(".attr(\"data-q-surface\", \"flat\")"));
        assert!(button.contains(".attr(\"data-q-motion\", \"smooth\")"));
        assert!(button.contains(".attr(\"data-q-variant\", \"solid\")"));
        assert!(button.contains("Button::action(\"Button\")"));
        assert!(button.contains("Variant::Solid"));

        // Every component (not just button/badge) ships REAL builder code in its
        // Theme-it snippet — no placeholder comments like `/* card render() */`.
        let card = theme_it_snippet("card", None);
        assert!(card.contains("Card::new("));
        assert!(card.contains(".render()"));
        assert!(!card.contains("/*"));
        let switch = theme_it_snippet("switch", None);
        assert!(switch.contains("SwitchGroup::new("));
        assert!(switch.contains(".island("));
    }
}
