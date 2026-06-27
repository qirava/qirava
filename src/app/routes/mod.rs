//! Page routes. Each module exports a `respond(input: &[u8]) -> FunctionResponse`
//! that the worker calls for that page's URL. Wire a new page by adding its
//! module here and listing it in `PAGES` in `main.rs`.

pub mod api;
pub mod architecture;
pub mod components;
pub mod docs;
pub mod index;
pub mod product_cloud;
pub mod product_dms;
pub mod product_page;
pub mod product_quill;
pub mod product_stdlib;
pub mod products;
pub mod roadmap;

use qquill_design::{Badge, Size, Tone, Variant};
use qquill_view::{el, island, raw, text, Node, Trigger};

use crate::app::Css;

/// Wrap arbitrary content in a `reveal` island so its `[data-q-reveal]` children
/// fade/slide in on scroll. `instance_id` must be unique per page. The SSR
/// fallback is the content itself (revealed by the island; visible without JS
/// because the reduced-motion reset and the island both end at the shown state).
pub fn reveal(instance_id: &'static str, content: Node) -> Node {
    island(instance_id, "reveal", Trigger::Load, "{}", content)
}

/// A copy-enabled code block: a `copy` island wrapping a `<pre data-q-part=code>`
/// and a "Copy" button. `lines` build the `<pre>` (escaped); the button copies
/// the rendered text on click. Static and correct with JS off (button hidden).
pub fn copy_code(instance_id: &'static str, lines: &[CodeLine]) -> Node {
    let pre = code_block(lines).attr("data-q-part", "code");
    let btn = el("button")
        .class("q-copy")
        .attr("type", "button")
        .attr("data-q-part", "copy")
        .attr("aria-label", "Copy code")
        .child(el("span").attr("data-q-part", "label").child(text("Copy")));
    let wrap = el("div").class("q-codewrap").child(btn).child(pre);
    island(instance_id, "copy", Trigger::Load, "{}", wrap)
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

/// A monospace code block. `lines` are `(is_comment, text)`; a leading `$`
/// prompt is highlighted. Content is escaped (built from `text`), the structural
/// spans are trusted markup.
pub fn code_block(lines: &[CodeLine]) -> Node {
    let mut pre = el("pre").class("q-code");
    let mut code = el("code");
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            code = code.child(raw("\n"));
        }
        match line {
            CodeLine::Comment(t) => {
                code = code.child(el("span").class("q-comment").child(text(t.to_string())));
            }
            CodeLine::Cmd(t) => {
                code = code
                    .child(el("span").class("q-prompt").child(text("$ ")))
                    .child(text(t.to_string()));
            }
            CodeLine::Plain(t) => {
                code = code.child(text(t.to_string()));
            }
        }
    }
    pre = pre.child(code);
    pre
}

/// A line in a [`code_block`].
pub enum CodeLine {
    /// A `# ...` comment line (muted).
    Comment(&'static str),
    /// A shell command line (prefixed with a highlighted `$`).
    Cmd(&'static str),
    /// A plain output / continuation line.
    Plain(&'static str),
}

/// An inline `<code>` snippet.
pub fn inline_code(s: &str) -> Node {
    el("code").class("q-inline").child(text(s.to_string()))
}
