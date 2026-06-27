//! Shared building blocks for the per-product roadmap pages
//! (`/roadmap/{dms,quill,stdlib,cloud}`).
//!
//! Each roadmap page has the same shape — the standard product display hero
//! (reused from [`product_page`]) followed by three scroll-revealing status
//! "lanes": **Shipping** (BUILT), **In progress** (PARTIAL), and **Planned**
//! (PLANNED). Each lane is a timeline-ish column of items with a status chip, so
//! the honest BUILT / PARTIAL / PLANNED state reads at a glance. No dates are
//! promised — only state. Everything is server-rendered and correct with JS off;
//! the only JavaScript is the shared `reveal` island. Every value references a
//! `--q-*` token so the page restyles on any theme/size/radius switch with no
//! reflow, and all motion is wrapped so the `prefers-reduced-motion` reset can
//! neutralize it.

use qquill_view::{el, text, Node};

use crate::app::routes::{reveal, Status};

/// A single roadmap item: a short title and a one-line detail.
pub struct Item {
    pub title: &'static str,
    pub detail: &'static str,
}

/// A status lane: the [`Status`] it represents plus its items. The lane heading
/// and chip are derived from the status, so call sites only supply the items.
pub struct Lane {
    pub status: Status,
    pub items: &'static [Item],
}

impl Status {
    /// The lane heading shown above each group of items.
    fn lane_heading(self) -> &'static str {
        match self {
            Status::Built => "Shipping today",
            Status::Partial => "In progress",
            Status::Planned => "Planned",
        }
    }

    /// The lead line under each lane heading.
    fn lane_lead(self) -> &'static str {
        match self {
            Status::Built => "Present in the codebase and usable now.",
            Status::Partial => "A working seam exists; the rest is designed, not yet built.",
            Status::Planned => "Designed but not yet built — no dates promised.",
        }
    }

    /// The chip label + CSS modifier for the per-item pill.
    fn chip(self) -> (&'static str, &'static str) {
        match self {
            Status::Built => ("Built", "is-built"),
            Status::Partial => ("Partial", "is-partial"),
            Status::Planned => ("Planned", "is-planned"),
        }
    }
}

/// One status lane as a scroll-revealing timeline column. `id` must be unique
/// per page (one lane id is the `reveal` instance id; its `-head` companion is
/// interned to stay `'static`).
fn lane(id: &'static str, lane: &Lane) -> Node {
    let (chip_label, chip_mod) = lane.status.chip();

    let head = el("div")
        .class(format!("q-rm-lane__head {chip_mod}"))
        .child(
            el("span")
                .class(format!("q-rm-lane__chip {chip_mod}"))
                .child(text(chip_label.to_string())),
        )
        .child(el("h3").class("q-rm-lane__title").child(text(lane.status.lane_heading().to_string())))
        .child(el("p").class("q-rm-lane__lead").child(text(lane.status.lane_lead().to_string())));

    let mut track = el("ol").class("q-rm-track").attr("role", "list");
    for (i, item) in lane.items.iter().enumerate() {
        track = track.child(
            el("li")
                .class("q-rm-item")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(el("span").class(format!("q-rm-item__dot {chip_mod}")).attr("aria-hidden", "true"))
                .child(
                    el("div")
                        .class("q-rm-item__body")
                        .child(el("span").class("q-rm-item__title").child(text(item.title.to_string())))
                        .child(el("span").class("q-rm-item__detail").child(text(item.detail.to_string()))),
                ),
        );
    }

    let column = el("section")
        .class(format!("q-rm-lane {chip_mod}"))
        .child(head)
        .child(track);

    reveal(id, column)
}

/// The three-lane roadmap board: an eyebrow + heading + lead, then the BUILT /
/// PARTIAL / PLANNED lanes side by side (stacking on narrow viewports). Pass the
/// three lanes plus three unique reveal ids (one per lane). The head's own
/// reveal id is interned from the first lane id so every id stays `'static`.
pub fn board(
    eyebrow: &str,
    heading: &str,
    lead: &str,
    ids: [&'static str; 3],
    lanes: [Lane; 3],
) -> Node {
    let head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text(eyebrow.to_string())))
        .child(el("h2").class("q-h2").child(text(heading.to_string())))
        .child(el("p").class("q-lead").child(text(lead.to_string())));

    let board = el("div")
        .class("q-rm-board")
        .child(lane(ids[0], &lanes[0]))
        .child(lane(ids[1], &lanes[1]))
        .child(lane(ids[2], &lanes[2]));

    el("section")
        .class("q-section")
        .child(reveal(leak_id(ids[0], "-head"), head))
        .child(board)
}

/// A compact legend explaining the three states. Pure SSR.
pub fn legend() -> Node {
    let item = |status: Status, label: &'static str, desc: &'static str| -> Node {
        let (_, chip_mod) = status.chip();
        el("div")
            .class("q-rm-legend__item")
            .child(el("span").class(format!("q-rm-legend__chip {chip_mod}")).child(text(label.to_string())))
            .child(el("span").class("q-rm-legend__desc").child(text(desc.to_string())))
    };

    el("div")
        .class("q-rm-legend")
        .child(item(Status::Built, "Built", "shipping and usable today"))
        .child(item(Status::Partial, "Partial", "a working seam; the rest is designed"))
        .child(item(Status::Planned, "Planned", "designed but not yet built"))
}

/// A small note row, used to point at the source of truth (the architecture
/// docs). `children` are inline nodes (text + an anchor or two).
pub fn note(children: Vec<Node>) -> Node {
    el("p").class("q-rm-note").children(children)
}

/// `reveal()`/`board()` need `'static` ids; pages pass distinct constant
/// eyebrows, so a tiny intern keeps the derived companion id `'static`.
fn leak_id(base: &'static str, suffix: &'static str) -> &'static str {
    Box::leak(format!("{base}{suffix}").into_boxed_str())
}

/// The roadmap-page CSS — the lanes/timeline board + legend + note. Layered on
/// top of `product_css()` (which the pages also push, for the hero + reduced
/// motion). Every value references a `--q-*` token so it restyles on any
/// theme/size/radius switch; motion is neutralized under reduced-motion. Pushed
/// once per page and deduped by the accumulator.
pub fn roadmap_css() -> &'static str {
    "\
/* ---- the three-lane board ---- */\
.q-rm-board{display:grid;grid-template-columns:repeat(3,1fr);gap:var(--q-space-4);align-items:start}\
@media (max-width:900px){.q-rm-board{grid-template-columns:1fr}}\
.q-rm-lane{display:flex;flex-direction:column;gap:var(--q-space-4);height:100%;padding:var(--q-space-5);border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface)}\
.q-rm-lane.is-built{border-color:color-mix(in srgb,var(--q-color-brand) 40%,var(--q-color-border))}\
.q-rm-lane__head{display:flex;flex-direction:column;gap:var(--q-space-2)}\
.q-rm-lane__chip{align-self:flex-start;font-size:.66rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.2rem .5rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted)}\
.q-rm-lane__chip.is-built{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 45%,transparent);background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
.q-rm-lane__chip.is-partial{color:var(--q-color-fg);border-color:color-mix(in srgb,var(--q-color-fg) 30%,transparent);background:color-mix(in srgb,var(--q-color-fg) 7%,transparent)}\
.q-rm-lane__title{margin:0;font-size:1.15rem;font-weight:var(--q-font-weight-bold);letter-spacing:-.01em}\
.q-rm-lane__lead{margin:0;font-size:.9rem;line-height:1.55;color:var(--q-color-muted)}\
/* ---- the timeline track within a lane ---- */\
.q-rm-track{list-style:none;margin:0;padding:0;position:relative;display:flex;flex-direction:column;gap:var(--q-space-4)}\
.q-rm-track::before{content:\"\";position:absolute;left:.34rem;top:.4rem;bottom:.4rem;width:2px;background:linear-gradient(var(--q-color-border),transparent);border-radius:2px}\
.q-rm-item{position:relative;display:flex;gap:var(--q-space-3);padding-left:.1rem}\
.q-rm-item__dot{flex:0 0 auto;width:.75rem;height:.75rem;margin-top:.35rem;border-radius:var(--q-radius-full);background:var(--q-color-bg);border:2px solid var(--q-color-muted);box-shadow:0 0 0 3px var(--q-color-surface);z-index:1}\
.q-rm-item__dot.is-built{border-color:var(--q-color-brand);background:var(--q-color-brand)}\
.q-rm-item__dot.is-partial{border-color:var(--q-color-fg)}\
.q-rm-item__body{display:flex;flex-direction:column;gap:.1rem;min-width:0}\
.q-rm-item__title{font-weight:var(--q-font-weight-bold);font-size:.96rem;letter-spacing:-.01em}\
.q-rm-item__detail{font-size:.88rem;line-height:1.55;color:var(--q-color-muted)}\
/* ---- legend ---- */\
.q-rm-legend{display:flex;flex-wrap:wrap;gap:var(--q-space-3) var(--q-space-5);margin:var(--q-space-5) 0 0;padding:var(--q-space-4) var(--q-space-5);border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface)}\
.q-rm-legend__item{display:flex;align-items:center;gap:.5rem}\
.q-rm-legend__chip{font-size:.66rem;font-weight:var(--q-font-weight-bold);letter-spacing:.08em;text-transform:uppercase;padding:.2rem .5rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted)}\
.q-rm-legend__chip.is-built{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 45%,transparent);background:color-mix(in srgb,var(--q-color-brand) 12%,transparent)}\
.q-rm-legend__desc{font-size:.88rem;color:var(--q-color-muted)}\
.q-rm-note{margin:var(--q-space-5) 0 0;font-size:.9rem;color:var(--q-color-muted)}\
.q-rm-note a{text-decoration:underline;text-underline-offset:2px}"
}
