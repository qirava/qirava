//! Page routes. Each module exports a `respond(input: &[u8]) -> FunctionResponse`
//! that the worker calls for that page's URL. Wire a new page by adding its
//! module here and listing it in `PAGES` in `main.rs`.

pub mod api;
pub mod components;
pub mod docs;
pub mod docs_authored;
pub mod docs_content;
pub mod index;
pub mod product_cloud;
pub mod product_dms;
pub mod product_page;
pub mod product_quill;
pub mod product_stdlib;
pub mod products;
pub mod roadmap;
pub mod roadmap_cloud;
pub mod roadmap_dms;
pub mod roadmap_page;
pub mod roadmap_quill;
pub mod roadmap_stdlib;

use qquill_design::{Badge, Size, Tone, Variant};
use qquill_view::{el, island, text, Node, Trigger};

use crate::app::Css;

/// Wrap arbitrary content in a `reveal` island so its `[data-q-reveal]` children
/// fade/slide in on scroll. `instance_id` must be unique per page. The SSR
/// fallback is the content itself (revealed by the island; visible without JS
/// because the reduced-motion reset and the island both end at the shown state).
pub fn reveal(instance_id: &'static str, content: Node) -> Node {
    island(instance_id, "reveal", Trigger::Load, "{}", content)
}

/// Wrap content in a `tilt` island so its `[data-q-tilt]` descendants tilt in 3D
/// toward the pointer on hover (with a brand glare that follows the cursor).
/// `instance_id` must be unique per page. With JS off — or under reduced-motion
/// or on a touch device — the elements sit flat, fully functional.
pub fn tilt(instance_id: &'static str, content: Node) -> Node {
    island(instance_id, "tilt", Trigger::Load, "{}", content)
}

/// A status pill, used across pages for the BUILT / PARTIAL / PLANNED legend.
/// Tone maps: built -> brand, partial -> neutral, planned -> neutral (outline).
#[derive(Clone, Copy)]
pub enum Status {
    Built,
    Partial,
    Planned,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Status::Built => "BUILT",
            Status::Partial => "PARTIAL",
            Status::Planned => "PLANNED",
        }
    }

    fn tone(self) -> Tone {
        match self {
            Status::Built => Tone::Brand,
            Status::Partial => Tone::Neutral,
            Status::Planned => Tone::Neutral,
        }
    }

    fn variant(self) -> Variant {
        match self {
            // Solid-ish "done", soft "in progress", outline "not yet".
            Status::Built => Variant::Soft,
            Status::Partial => Variant::Soft,
            Status::Planned => Variant::Outline,
        }
    }
}

/// Render a status badge into `css`, returning its node.
pub fn status_badge(css: &mut Css, status: Status) -> Node {
    let badge = Badge::badge(status.label())
        .tone(status.tone())
        .variant(status.variant())
        .size(Size::Sm);
    css.node(badge.render())
}

/// A page section with an `<h2>` heading + a lead paragraph + arbitrary body.
pub fn section(eyebrow: Option<&str>, heading: &str, lead: &str, body: Node) -> Node {
    let mut head = el("div").class("q-section__head");
    if let Some(e) = eyebrow {
        head = head.child(el("p").class("q-eyebrow").child(text(e.to_string())));
    }
    head = head
        .child(el("h2").class("q-h2").child(text(heading.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())));

    el("section")
        .class("q-section")
        .child(head)
        .child(body)
}

/// An inline `<code>` snippet.
pub fn inline_code(s: &str) -> Node {
    el("code").class("q-inline").child(text(s.to_string()))
}
