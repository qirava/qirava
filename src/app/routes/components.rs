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
    Accordion, Alert, Badge, Breadcrumb, Button, Card, Checkbox, Crumb, Dialog, Divider, Effect,
    List, ListItem, Menu, MenuItem, Radius, Section as AccSection, Severity, Size, Stat, Styled,
    SwitchGroup, Tabs, Tone, Tooltip, Trend, Variant, Variants,
};
use qquill_docs::CodeBlock;
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
    CompRef { path: "/components/alert", name: "Alert", blurb: "Severity banners across four severities, surface effects, and radius." },
    CompRef { path: "/components/stat", name: "Stat", blurb: "A labeled key figure with a trend chip — size and trend axes." },
    CompRef { path: "/components/list", name: "List", blurb: "Unordered/ordered rows with leading slots — kind and size axes." },
    CompRef { path: "/components/divider", name: "Divider", blurb: "A separator rule — orientation and spacing-scale axes." },
    CompRef { path: "/components/breadcrumb", name: "Breadcrumb", blurb: "An ARIA trail — size and link-radius axes." },
    CompRef { path: "/components/dialog", name: "Dialog", blurb: "A modal dialog with a focus-trapped surface — a live island." },
    CompRef { path: "/components/menu", name: "Menu", blurb: "A popup action menu with roving keyboard nav — a live island." },
    CompRef { path: "/components/tooltip", name: "Tooltip", blurb: "A describing bubble revealed on hover/focus — a live island." },
    CompRef { path: "/components/checkbox", name: "Checkbox", blurb: "A tri-state checkbox (checked / unchecked / mixed) — a live island." },
    CompRef { path: "/components/switch", name: "Switch", blurb: "A group of on/off toggle switches — a live island." },
    CompRef { path: "/components/accordion", name: "Accordion", blurb: "A stack of collapsible disclosure sections — a live island." },
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

pub fn respond_alert(_input: &[u8]) -> FunctionResponse {
    // Alert's axes are severity × effect × radius. We map them onto the three
    // control slots; the snippet tokens follow suit.
    let render = |severity: &str, effect: &str, radius: &str| -> Styled {
        Alert::new(to_severity(severity), "Your changes have been saved.")
            .title(title(severity))
            .effect(to_effect(effect))
            .radius(to_radius(radius))
            .render()
    };
    let pg = Playground {
        id: "alert",
        variant: Axis { legend: "Severity", values: SEVERITY_VALUES, default: "info" },
        size: Axis { legend: "Effect", values: EFFECT_VALUES, default: "flat" },
        tone: Axis { legend: "Radius", values: RADIUS_VALUES, default: "md" },
        template: "Alert::new(Severity::{variant}, \"Your changes have been saved.\")\n    .title(\"{variant}\")\n    .effect(Effect::{size})\n    .radius(Radius::{tone})\n    .render()",
        render_cell: &render,
    };
    component_page(
        "Alert — Qirava components",
        "The Quill Alert: severity banners with surface-effect and radius axes — interactive playground.",
        "/components/alert",
        "Component",
        "Alert",
        "An inline status banner that announces itself (role=alert). Its primary axis is severity \
         (info, success, warn, danger), each with a tone-colored accent border and title; the \
         surface effect and corner radius are orthogonal treatments.",
        pg,
        "Use the severity that matches the message's meaning — never use danger for decoration. \
         Keep the body to one or two sentences; reach for a Toast for transient, dismissible notices.",
    )
}

pub fn respond_stat(_input: &[u8]) -> FunctionResponse {
    // Stat's real axes are size × trend; it has no tone, so the third slot is a
    // single fixed value (the matrix still pre-renders correctly).
    let render = |size: &str, trend: &str, _t: &str| -> Styled {
        Stat::new("pg-stat", "Monthly revenue", "$48,250")
            .size(to_size(size))
            .trend("8.2%", to_trend(trend))
            .render()
    };
    let pg = Playground {
        id: "stat",
        variant: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        size: Axis { legend: "Trend", values: &["up", "down", "flat"], default: "up" },
        tone: Axis { legend: "Value", values: &["figure"], default: "figure" },
        template: "Stat::new(\"revenue\", \"Monthly revenue\", \"$48,250\")\n    .size(Size::{variant})\n    .trend(\"8.2%\", Trend::{size})\n    .render()",
        render_cell: &render,
    };
    component_page(
        "Stat — Qirava components",
        "The Quill Stat: a labeled key figure with a trend chip — size and trend axes.",
        "/components/stat",
        "Component",
        "Stat",
        "A labeled key figure (label + value + optional trend), grouped so assistive tech reads it \
         as one unit. The trend direction is carried in text and a glyph, never by color alone. Its \
         axes are size and trend direction.",
        pg,
        "Pair stats in a row for a dashboard summary. Use the trend chip only when a delta is \
         meaningful; an up trend is not always good (cost up is bad) — the words carry the meaning.",
    )
}

pub fn respond_list(_input: &[u8]) -> FunctionResponse {
    // List's axes are kind (unordered/ordered) × size. The third slot is fixed.
    let render = |kind: &str, size: &str, _t: &str| -> Styled {
        let base = if kind == "ordered" { List::ordered() } else { List::new() };
        base.size(to_size(size))
            .item(ListItem::new(text("Authenticate the request".to_string())))
            .item(ListItem::new(text("Authorize against the grant".to_string())))
            .item(ListItem::new(text("Plan and execute the query".to_string())))
            .render()
    };
    let pg = Playground {
        id: "list",
        variant: Axis { legend: "Kind", values: &["unordered", "ordered"], default: "unordered" },
        size: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        tone: Axis { legend: "Rows", values: &["items"], default: "items" },
        template: "List::new() // or List::ordered()\n    .size(Size::{size})\n    .item(ListItem::new(text(\"Authenticate the request\")))\n    .item(ListItem::new(text(\"Authorize against the grant\")))\n    .item(ListItem::new(text(\"Plan and execute the query\")))\n    .render()",
        render_cell: &render,
    };
    component_page(
        "List — Qirava components",
        "The Quill List: unordered/ordered rows with leading slots — kind and size axes.",
        "/components/list",
        "Component",
        "List",
        "A native ul/ol of rows, each with an optional leading slot (an icon or avatar) and main \
         content. Using the native list element means list semantics come for free. Its axes are the \
         kind (unordered or ordered) and the row size.",
        pg,
        "Prefer an ordered list only when sequence is meaningful. Keep each row to a single line of \
         primary content; push secondary detail into a muted second line inside the main slot.",
    )
}

pub fn respond_divider(_input: &[u8]) -> FunctionResponse {
    // Divider's axes are orientation × spacing size. Third slot fixed.
    let render = |orientation: &str, size: &str, _t: &str| -> Styled {
        let mut d = Divider::new().size(to_size(size));
        if orientation == "vertical" {
            d = d.vertical();
        }
        d.render()
    };
    let pg = Playground {
        id: "divider",
        variant: Axis { legend: "Orientation", values: &["horizontal", "vertical"], default: "horizontal" },
        size: Axis { legend: "Spacing", values: SIZE_VALUES, default: "md" },
        tone: Axis { legend: "Rule", values: &["line"], default: "line" },
        template: "Divider::new()\n    .size(Size::{size})\n    // .vertical() for a vertical hairline\n    .render()",
        render_cell: &render,
    };
    component_page(
        "Divider — Qirava components",
        "The Quill Divider: a separator rule with orientation and spacing-scale axes.",
        "/components/divider",
        "Component",
        "Divider",
        "A separator rule. A semantic divider renders as role=separator (announced as a structural \
         break); a decorative one is a hidden hr. Its axes are orientation (horizontal or vertical) \
         and the spacing scale around the line.",
        pg,
        "Use a semantic divider to split unrelated content groups; use Divider::decorative() for \
         purely visual rules so screen readers don't announce a meaningless break. A vertical \
         divider needs a flex row context to show its full height.",
    )
}

pub fn respond_breadcrumb(_input: &[u8]) -> FunctionResponse {
    // Breadcrumb's axes are size × radius (on the link). Third slot fixed.
    let render = |size: &str, radius: &str, _t: &str| -> Styled {
        Breadcrumb::new(vec![
            Crumb::new("Home", "/"),
            Crumb::new("Components", "/components"),
            Crumb::new("Breadcrumb", "/components/breadcrumb"),
        ])
        .size(to_size(size))
        .radius(to_radius(radius))
        .render()
    };
    let pg = Playground {
        id: "breadcrumb",
        variant: Axis { legend: "Size", values: SIZE_VALUES, default: "md" },
        size: Axis { legend: "Radius", values: RADIUS_VALUES, default: "md" },
        tone: Axis { legend: "Trail", values: &["crumbs"], default: "crumbs" },
        template: "Breadcrumb::new(vec![\n    Crumb::new(\"Home\", \"/\"),\n    Crumb::new(\"Components\", \"/components\"),\n    Crumb::new(\"Breadcrumb\", \"/components/breadcrumb\"),\n])\n.size(Size::{variant})\n.radius(Radius::{size})\n.render()",
        render_cell: &render,
    };
    component_page(
        "Breadcrumb — Qirava components",
        "The Quill Breadcrumb: an ARIA trail with size and link-radius axes — interactive playground.",
        "/components/breadcrumb",
        "Component",
        "Breadcrumb",
        "An ARIA-correct trail (nav[aria-label=Breadcrumb] > ol). Intermediate crumbs are links; the \
         current page is plain text carrying aria-current. The separator is decorative CSS. Its axes \
         are the text size and the focus-radius on the links.",
        pg,
        "Put the breadcrumb above the page title, not in the global nav. The last crumb is the \
         current page and is never a link. Hrefs are scheme-checked, so a hostile href collapses to #.",
    )
}

// ---------------------------------------------------------------------------
// Interactive island pages
//
// Unlike Button/Badge/Card (static variant matrices), these components are
// interactive by nature: their showcase is a LIVE island (the real `.island()`
// builder), the `view!`-style construction code shown via `qquill_docs::CodeBlock`,
// and a short prose intro. The island runtime for each kind ships automatically
// (the page-render path injects only the behaviors the page actually uses).
// ---------------------------------------------------------------------------

/// Render an interactive-component page: intro + a LIVE island demo + the
/// construction code (via `CodeBlock`) + a short usage note. `demo` is the
/// already-css-collected island node; `code` is the verbatim Rust source shown.
fn island_page(
    meta_title: &'static str,
    meta_desc: &'static str,
    path: &'static str,
    title_text: &'static str,
    lead: &'static str,
    demo_lead: &'static str,
    demo: Node,
    code: &'static str,
    note: &'static str,
    mut css: Css,
) -> FunctionResponse {
    // The docs content-primitive CSS supplies the `.qq-code` block styling used
    // by the `CodeBlock` below.
    css.push(qquill_docs::layout_css().to_css());

    let intro = el("div")
        .child(el("p").class("q-eyebrow").child(text("Component")))
        .child(el("h1").class("q-h1").child(text(title_text.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())));

    let demo_section = section(
        Some("Playground"),
        "Live demo",
        demo_lead,
        el("div").class("q-teaser").child(demo),
    );

    // The construction code, escaped + copyable, via the docs CodeBlock.
    let code_section = section(
        Some("Code"),
        "view!",
        "The exact builder call that produces the island above.",
        CodeBlock::new("rust", code.to_string()).render(),
    );

    let usage = section(Some("Notes"), "Usage", note, el("div"));

    let back = el("p").child(el("a").attr("href", "/components").child(text("← All components")));

    let content = el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(demo_section)
        .child(code_section)
        .child(usage)
        .child(back);

    let meta = Meta { title: meta_title, description: meta_desc, path };
    page(&meta, css, content)
}

pub fn respond_dialog(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let body = el("p")
        .class("q-muted")
        .child(text("This surface is focus-trapped while open; Escape or the backdrop closes it.".to_string()));
    let dialog = Dialog::modal("demo-dialog", "Delete project?", body).effect(Effect::Elevated);
    let demo = css.node(dialog.island("demo-dialog-island", "Open dialog"));

    let code = "let body = el(\"p\").child(text(\"This surface is focus-trapped while open …\"));\nDialog::modal(\"demo-dialog\", \"Delete project?\", body)\n    .effect(Effect::Elevated)\n    .island(\"demo-dialog-island\", \"Open dialog\")";

    island_page(
        "Dialog — Qirava components",
        "The Quill Dialog: a modal surface with a focus trap and an accessible backdrop — a live island.",
        "/components/dialog",
        "Dialog",
        "A modal dialog: a trigger button opens a backdrop + a role=dialog surface (aria-modal) that \
         traps focus until dismissed. Its title is auto-wired via aria-labelledby; Escape, the close \
         button, and a backdrop click all close it. The surface effect and radius are orthogonal.",
        "Click \"Open dialog\". The surface traps focus; Escape, the × button, or a click on the dimmed \
         backdrop closes it. With JavaScript off, the trigger + a closed dialog render as a valid fallback.",
        demo,
        code,
        "Reserve a modal for a decision that must block the rest of the page (a destructive confirm, \
         a required form). For transient notices reach for a Toast; for side content reach for a Drawer.",
        css,
    )
}

pub fn respond_menu(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let menu = Menu::new(
        "demo-menu",
        "Actions",
        vec![
            MenuItem::new("Rename"),
            MenuItem::new("Duplicate"),
            MenuItem::new("Archive").disabled(true),
            MenuItem::new("Delete"),
        ],
    );
    let demo = css.node(menu.island("demo-menu-island"));

    let code = "Menu::new(\n    \"demo-menu\",\n    \"Actions\",\n    vec![\n        MenuItem::new(\"Rename\"),\n        MenuItem::new(\"Duplicate\"),\n        MenuItem::new(\"Archive\").disabled(true),\n        MenuItem::new(\"Delete\"),\n    ],\n)\n.island(\"demo-menu-island\")";

    island_page(
        "Menu — Qirava components",
        "The Quill Menu: a popup action list with roving-tabindex keyboard navigation — a live island.",
        "/components/menu",
        "Menu",
        "A popup action menu: a trigger button (aria-haspopup / aria-expanded / aria-controls) opens a \
         role=menu surface of role=menuitem rows. The active item is tracked with a roving tabindex; \
         disabled items are skipped. The surface is hidden when closed.",
        "Click \"Actions\" to open the menu, then use the arrow keys (Home/End jump to the ends) — the \
         highlighted item gets a brand tint, and the disabled \"Archive\" row is skipped. Escape closes it.",
        demo,
        code,
        "Use a menu for a small set of actions on a single object. Keep labels to a verb or short \
         phrase. For navigation between pages use a Navbar; for command search use the Command Palette.",
        css,
    )
}

pub fn respond_tooltip(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let tooltip = Tooltip::new(
        "demo-tooltip",
        "Hover me",
        "Tooltips reveal on hover or focus, after a short delay.",
    );
    let demo = css.node(tooltip.island("demo-tooltip-island"));

    let code = "Tooltip::new(\n    \"demo-tooltip\",\n    \"Hover me\",\n    \"Tooltips reveal on hover or focus, after a short delay.\",\n)\n.island(\"demo-tooltip-island\")";

    island_page(
        "Tooltip — Qirava components",
        "The Quill Tooltip: a describing bubble revealed on hover/focus per the WAI-ARIA pattern — a live island.",
        "/components/tooltip",
        "Tooltip",
        "A tooltip: a trigger permanently aria-describedby a role=tooltip bubble, per the WAI-ARIA APG \
         pattern. The bubble is hidden until the trigger is hovered or focused (with a short delay); \
         the description is always available to assistive tech via the describedby link.",
        "Hover over (or tab to) the \"Hover me\" trigger — the bubble reveals after a short delay and \
         hides when you leave. With JavaScript off, the description is still announced on focus via \
         aria-describedby.",
        demo,
        code,
        "Tooltips carry supplementary hints, never essential content (they vanish and are not reachable \
         by touch). Never put interactive controls inside a tooltip; use a Popover for that.",
        css,
    )
}

pub fn respond_checkbox(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let unchecked = css.node(Checkbox::new("cb-news", "Email me product news", false).island("cb-news-island"));
    let checked = css.node(Checkbox::new("cb-terms", "I accept the terms", true).island("cb-terms-island"));
    let mixed = css.node(Checkbox::mixed("cb-all", "Select all (some selected)").island("cb-all-island"));

    let demo = el("div")
        .class("q-stack")
        .child(unchecked)
        .child(checked)
        .child(mixed);

    let code = "// Unchecked, checked, and indeterminate (mixed) — each a live island.\nCheckbox::new(\"cb-news\", \"Email me product news\", false).island(\"cb-news-island\")\nCheckbox::new(\"cb-terms\", \"I accept the terms\", true).island(\"cb-terms-island\")\nCheckbox::mixed(\"cb-all\", \"Select all (some selected)\").island(\"cb-all-island\")";

    island_page(
        "Checkbox — Qirava components",
        "The Quill Checkbox: a tri-state checkbox (checked / unchecked / mixed) — a live island.",
        "/components/checkbox",
        "Checkbox",
        "A checkbox: a role=checkbox box carrying aria-checked (true / false / mixed) next to its \
         label. The mixed state models a parent of partially-selected children. The box is focusable \
         and toggles on click or Space; the styled layer draws the check/dash glyph.",
        "Click any box (or focus it and press Space) to toggle it. The third box starts in the \
         indeterminate (mixed) state — a first click resolves it. Each box renders in its initial \
         state with JavaScript off.",
        demo,
        code,
        "Use a checkbox for an independent on/off choice; use a radio group when exactly one of \
         several options applies. Reserve the mixed state for a \"select all\" that mirrors its children.",
        css,
    )
}

pub fn respond_switch(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let switches = SwitchGroup::new(
        "demo-switch",
        vec![
            ("Wi-Fi".to_string(), true),
            ("Bluetooth".to_string(), false),
            ("Airplane mode".to_string(), false),
        ],
    )
    .tone(Tone::Brand);
    let demo = css.node(switches.island("demo-switch-island"));

    let code = "SwitchGroup::new(\n    \"demo-switch\",\n    vec![\n        (\"Wi-Fi\".into(), true),\n        (\"Bluetooth\".into(), false),\n        (\"Airplane mode\".into(), false),\n    ],\n)\n.tone(Tone::Brand)\n.island(\"demo-switch-island\")";

    island_page(
        "Switch — Qirava components",
        "The Quill Switch group: a list of on/off toggle switches with a tone accent — a live island.",
        "/components/switch",
        "Switch",
        "A switch group: a labelled role=group of role=switch tracks, each a binary on/off toggle. \
         Unlike a checkbox (which submits a form value), a switch takes effect immediately. The \
         on-state fills the track with the tone accent and slides the knob; tone is an axis.",
        "Click a track (or focus it and press Space/Enter) to flip it — the knob slides and the track \
         fills with the brand accent. Each switch renders in its initial on/off state with JavaScript off.",
        demo,
        code,
        "Use a switch for a setting that applies instantly (a preference, a feature flag). Use a \
         checkbox inside a form whose value is submitted on save. Label each switch by what \"on\" means.",
        css,
    )
}

pub fn respond_accordion(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let panel = |body: &str| -> Node { el("p").class("q-muted").child(text(body.to_string())) };
    let accordion = Accordion::new(
        "demo-accordion",
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
    let demo = css.node(accordion.island("demo-accordion-island"));

    let code = "Accordion::new(\n    \"demo-accordion\",\n    vec![\n        Section::new(\"What is an island?\", panel(\"A server-rendered region …\")),\n        Section::new(\"Does it work without JavaScript?\", panel(\"Yes. …\")),\n        Section::new(\"How is it kept accessible?\", panel(\"The WAI-ARIA APG accordion pattern …\")),\n    ],\n)\n.open(0)\n.island(\"demo-accordion-island\")";

    island_page(
        "Accordion — Qirava components",
        "The Quill Accordion: a stack of collapsible disclosure sections (WAI-ARIA APG) — a live island.",
        "/components/accordion",
        "Accordion",
        "An accordion: a stack of disclosure sections, each a heading > button (aria-expanded / \
         aria-controls) over a labelled region panel. By default one section is open at a time; \
         .multiple() allows several. A rotating chevron reflects the open state.",
        "Click a section header (or focus it and press Enter/Space) to expand it; opening one collapses \
         the others. The first section starts open. With JavaScript off, the open section shows and the \
         rest are valid collapsed regions.",
        demo,
        code,
        "Use an accordion to let readers scan headings and open only what they need (an FAQ, a settings \
         group). Call .multiple() when sections are independent. Keep each panel to a focused chunk.",
        css,
    )
}
