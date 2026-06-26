//! `GET /components` + a page per component — the interactive showcase.
//!
//! The centerpiece is the PLAYGROUND: for each component the server
//! pre-renders the FULL variant matrix (variant × size × tone) as sibling
//! preview cells, all hidden via CSS except the default. The `playground`
//! island never builds HTML — it flips which cell is visible and rewrites the
//! `view!` code snippet to match the current selection. So the no-JS fallback
//! is fully correct (the default variant shows) and hydration stays tiny.

use qexec::FunctionResponse;
use qquill_design::{
    Badge, Button, Card, Effect, Radius, Size, Styled, Tabs, Tone, Variant, Variants,
};
use qquill_view::{el, island, text, Node, Trigger};

use crate::app::routes::section;
use crate::app::shell::page;
use crate::app::{Css, Meta};

// ---------------------------------------------------------------------------
// The generic playground
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
    let mut seg = el("div").class("pg__seg").attr("role", "group").attr("aria-label", axis.legend);
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
        .child(el("span").class("pg__legend").child(text(axis.legend.to_string())))
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
                let active = v == pg.variant.default && s == pg.size.default && t == pg.tone.default;
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

const VARIANT_VALUES: &[&str] = &["solid", "soft", "outline", "ghost"];
const SIZE_VALUES: &[&str] = &["sm", "md", "lg"];
const TONE_VALUES: &[&str] = &["brand", "neutral", "danger"];

// ---------------------------------------------------------------------------
// The index
// ---------------------------------------------------------------------------

struct CompRef {
    path: &'static str,
    name: &'static str,
    blurb: &'static str,
}

const COMPONENTS: &[CompRef] = &[
    CompRef { path: "/components/button", name: "Button", blurb: "Actions and toggles across four fill variants, three sizes, three tones." },
    CompRef { path: "/components/badge", name: "Badge", blurb: "Status pills, tags, and chips — same variant/size/tone axes." },
    CompRef { path: "/components/card", name: "Card", blurb: "Surface container with tone, surface effect, and radius treatments." },
    CompRef { path: "/components/tabs", name: "Tabs", blurb: "An interactive, ARIA-correct tab strip — a live island." },
];

const INDEX_LEAD: &str = "Every component is two crates: a headless state machine (qquill-ui) and a \
styled builder (qquill-design). Pick one to open its interactive playground — change the variant, \
size, and tone and watch both the preview and the code update.";

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let css = Css::new();

    let mut grid = el("div").class("q-comp-grid");
    for c in COMPONENTS {
        grid = grid.child(
            el("a")
                .class("q-comp-card")
                .attr("href", c.path)
                .child(el("h3").child(text(c.name.to_string())))
                .child(el("p").child(text(c.blurb.to_string()))),
        );
    }

    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text("Showcase")))
        .child(el("h1").class("q-h1").child(text("Components")))
        .child(el("p").class("q-lead").child(text(INDEX_LEAD)));

    let content = el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(grid);

    let meta = Meta {
        title: "Components — Qirava",
        description: "The Quill component showcase: an interactive playground per component, \
                      live variant/size/tone controls, and copyable code.",
        path: "/components",
    };
    page(&meta, css, content)
}

// ---------------------------------------------------------------------------
// One component page
// ---------------------------------------------------------------------------

/// Render a component page: intro + the playground + a short usage note.
fn component_page(
    meta_title: &'static str,
    meta_desc: &'static str,
    path: &'static str,
    eyebrow: &'static str,
    title_text: &'static str,
    lead: &'static str,
    pg: Playground,
    note: &'static str,
) -> FunctionResponse {
    let mut css = Css::new();

    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text(eyebrow.to_string())))
        .child(el("h1").class("q-h1").child(text(title_text.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())));

    let demo = section(
        Some("Playground"),
        "Try it",
        "Change the controls — the preview and the snippet update live. With JavaScript off, the \
         default variant renders correctly and the code shows it.",
        playground(&mut css, &pg),
    );

    let usage = section(Some("Notes"), "Usage", note, el("div"));

    let back = el("p").child(el("a").attr("href", "/components").child(text("← All components")));

    let content = el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(demo)
        .child(usage)
        .child(back);

    let meta = Meta { title: meta_title, description: meta_desc, path };
    page(&meta, css, content)
}

pub fn respond_button(_input: &[u8]) -> FunctionResponse {
    let render = |v: &str, s: &str, t: &str| -> Styled {
        Button::action("Button")
            .variants(Variants::new().variant(to_variant(v)).size(to_size(s)).tone(to_tone(t)))
            .render()
    };
    let pg = Playground {
        id: "button",
        variant: Axis { legend: "Variant", values: VARIANT_VALUES, default: "solid" },
        size: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        tone: Axis { legend: "Tone", values: TONE_VALUES, default: "brand" },
        template: "Button::action(\"Button\")\n    .variants(\n        Variants::new()\n            .variant(Variant::{variant})\n            .size(Size::{size})\n            .tone(Tone::{tone}),\n    )\n    .render()",
        render_cell: &render,
    };
    component_page(
        "Button — Qirava components",
        "The Quill Button: four fill variants, three sizes, three tones — interactive playground.",
        "/components/button",
        "Component",
        "Button",
        "An action or toggle. The fill is one of four variants (solid, soft, outline, ghost), \
         orthogonal to size and tone. Buttons are islands — they hydrate on first interaction.",
        pg,
        "Use a solid brand button for the primary action on a view; prefer ghost or outline for \
         secondary actions. Danger tone is reserved for destructive actions.",
    )
}

pub fn respond_badge(_input: &[u8]) -> FunctionResponse {
    let render = |v: &str, s: &str, t: &str| -> Styled {
        Badge::badge("Badge")
            .variant(to_variant(v))
            .size(to_size(s))
            .tone(to_tone(t))
            .render()
    };
    let pg = Playground {
        id: "badge",
        variant: Axis { legend: "Variant", values: VARIANT_VALUES, default: "soft" },
        size: Axis { legend: "Size", values: SIZE_VALUES, default: "sm" },
        tone: Axis { legend: "Tone", values: TONE_VALUES, default: "brand" },
        template: "Badge::badge(\"Badge\")\n    .variant(Variant::{variant})\n    .size(Size::{size})\n    .tone(Tone::{tone})\n    .render()",
        render_cell: &render,
    };
    component_page(
        "Badge — Qirava components",
        "The Quill Badge: status pills, tags, and chips with variant/size/tone axes.",
        "/components/badge",
        "Component",
        "Badge",
        "A compact status pill or tag. Soft brand reads as an active status; outline neutral reads \
         as an inert label. Same variant/size/tone axes as the rest of the system.",
        pg,
        "Keep badge text to one or two words. Use tone to encode meaning (brand = active, danger = \
         error) rather than decoration.",
    )
}

pub fn respond_card(_input: &[u8]) -> FunctionResponse {
    // Card's axes are tone × effect × radius. We map them onto the three control
    // slots and label them accordingly; the snippet tokens follow suit.
    let render = |effect: &str, radius: &str, tone: &str| -> Styled {
        let body = el("p").class("q-muted").child(text("A surface container.".to_string()));
        Card::new(format!("pg-card-{effect}-{radius}-{tone}"))
            .region()
            .header(el("div").class("q-card-eyebrow").child(text("Card".to_string())))
            .body(body)
            .effect(to_effect(effect))
            .radius(to_radius(radius))
            .tone(to_tone(tone))
            .render()
    };
    let pg = Playground {
        id: "card",
        variant: Axis { legend: "Effect", values: &["flat", "glass", "gradient", "elevated"], default: "elevated" },
        size: Axis { legend: "Radius", values: &["sm", "md", "lg", "xl"], default: "lg" },
        tone: Axis { legend: "Tone", values: TONE_VALUES, default: "neutral" },
        template: "Card::new(\"card\")\n    .header(/* ... */)\n    .body(/* ... */)\n    .effect(Effect::{variant})\n    .radius(Radius::{size})\n    .tone(Tone::{tone})\n    .render()",
        render_cell: &render,
    };
    component_page(
        "Card — Qirava components",
        "The Quill Card: surface effects, radius, and tone — interactive playground.",
        "/components/card",
        "Component",
        "Card",
        "A surface container for grouped content. Its axes are the surface effect (flat, glass, \
         gradient, elevated), the corner radius, and the tone — orthogonal treatments you compose.",
        pg,
        "Cards are not islands — they are static surfaces. Reach for an elevated effect to lift a \
         card off the page; keep flat for dense grids.",
    )
}

pub fn respond_tabs(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();

    let panel = |title: &str, body: &str| -> Node {
        el("div")
            .child(el("h3").class("q-h2").child(text(title.to_string())))
            .child(el("p").class("q-muted").child(text(body.to_string())))
    };

    let tabs = Tabs::new(
        "demo-tabs",
        0,
        vec![
            ("Overview".to_string(), panel("Overview", "Tabs hydrate in place from the server-selected tab — no markup change, full ARIA tab pattern.")),
            ("Keyboard".to_string(), panel("Keyboard", "Arrow keys move selection (roving tabindex); Home/End jump to first/last. Orientation picks the axis.")),
            ("A11y".to_string(), panel("Accessibility", "role=tablist/tab/tabpanel, aria-selected, aria-controls, and aria-labelledby are all wired by the headless layer.")),
        ],
    );

    let demo = section(
        Some("Playground"),
        "Live tabs",
        "Tabs are interactive themselves — this is a real island. Click a tab or use the arrow keys; \
         the server-rendered selection is preserved on hydrate.",
        el("div").class("q-teaser").child(css.node(tabs.island("demo-tabs-island"))),
    );

    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text("Component")))
        .child(el("h1").class("q-h1").child(text("Tabs")))
        .child(el("p").class("q-lead").child(text(
            "An ARIA-correct tab strip backed by the headless qquill-ui Tabs state machine. Unlike \
             Button/Badge/Card, Tabs is interactive by nature, so its showcase is a live island \
             rather than a variant matrix.",
        )));

    let usage = section(
        Some("Notes"),
        "Usage",
        "Render with Tabs::new(id, selected, tabs) for static SSR; call .island(instance_id) to \
         ship the interactive behavior. The selected tab's panel is shown server-side and preserved \
         on hydrate.",
        el("div"),
    );

    let back = el("p").child(el("a").attr("href", "/components").child(text("← All components")));

    let content = el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(demo)
        .child(usage)
        .child(back);

    let meta = Meta {
        title: "Tabs — Qirava components",
        description: "The Quill Tabs component: an interactive, ARIA-correct tab strip island.",
        path: "/components/tabs",
    };
    page(&meta, css, content)
}
