//! `GET /roadmap/quill` — the Quill UI/app framework roadmap.
//!
//! BUILT: the view! macro + SSR + islands + SSG, the qquill-* crates, the quill
//! CLI (new + static export), and the islands runtime. PARTIAL/PLANNED: folder-
//! route auto-discovery, a `quill dev` watch server, and more components. No dates
//! are promised — only state.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{hero, main_wrap, product_css, Cta, HeroStat};
use crate::app::routes::roadmap_page::{board, legend, note, roadmap_css, Item, Lane};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "Quill roadmap — built, in progress, planned";
const DESCRIPTION: &str = "An honest status board for Quill, the Rust-native UI/app framework: \
the view! macro, SSR, islands, SSG, the qquill-* crates, the quill CLI, and the islands runtime \
are shipping; folder-route auto-discovery, a quill dev server, and more components are planned.";

const BUILT: &[Item] = &[
    Item { title: "view! / el() authoring", detail: "A zero-dependency view layer (qquill-view) builds the node tree on the server and renders it to HTML." },
    Item { title: "Native SSR on the DMS worker", detail: "A page render is a registered function the worker calls; this very site is served — and exported — through exactly that path." },
    Item { title: "Islands + the client runtime", detail: "A ~4 KB hand-written, zero-import runtime (qquill-runtime) scans [data-q-island] and hydrates each in place on its declared trigger." },
    Item { title: "Per-page island bundling", detail: "Only pages that actually use an island ship a runtime; content pages ship zero JavaScript." },
    Item { title: "Static export (SSG/ISR)", detail: "qquill-build renders every route to a file and copies public/ assets — a CDN-ready dist/ that serves with no DMS running." },
    Item { title: "The qquill-* crates", detail: "view, design, theme, style, runtime, signal, docs, icons, ui, build, and cli — eleven first-party crates, each zero-dependency." },
    Item { title: "Styled component library", detail: "Navbar, Card, Button, Badge, Stat, Table, Tabs, Alert, List, Divider, Breadcrumb and more over headless state machines, theme-token driven." },
    Item { title: "Interactive island components", detail: "Dialog, menu, tooltip, checkbox, switch, accordion, copy-code, reveal, theme-control, and playground hydrate on demand." },
    Item { title: "Theme + design tokens", detail: "qquill-theme's typed --q-* tokens with a no-flicker boot and light/dark/contrast plus density/radius/surface axes — every switch is one attribute flip." },
    Item { title: "The quill CLI: new + build", detail: "quill new <name> scaffolds a real Quill app; quill build exports a static dist/ — the same render path the server uses." },
];

const PARTIAL: &[Item] = &[
    Item { title: "Folder-route auto-discovery", detail: "Today routes are listed once in the PAGES table; auto-discovering page modules from a folder convention (Next.js-style) is designed, not yet built." },
    Item { title: "Component breadth", detail: "The core component set ships; more form controls, data-display, and overlay components are being added incrementally." },
];

const PLANNED: &[Item] = &[
    Item { title: "quill dev — watch + reload", detail: "A development server with file-watch and live reload, alongside the existing serve and static-export paths." },
    Item { title: "Richer component catalog", detail: "Date pickers, combobox, command palette, data table, and the remaining shadcn-class components." },
    Item { title: "Client-side signals", detail: "Broadening qquill-signal so islands compose reactive state without leaving the zero-dependency runtime." },
];

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(roadmap_css().to_string());

    let hero = hero(
        css,
        "Quill roadmap",
        "qquill",
        "The UI framework, ",
        "honestly tracked",
        ".",
        "The view layer, native SSR, islands, per-page bundling, static export, the styled + \
         interactive component library, theming, and the quill CLI are shipping. Folder-route \
         auto-discovery, a quill dev watch server, and more components are planned. This very \
         site is a Quill app — it dogfoods the framework end to end.",
        &[
            Cta { label: "Explore Quill", href: "/products/quill", solid: true },
            Cta { label: "Read the docs", href: "/docs/quill", solid: false },
        ],
        &[
            HeroStat { value: "10", label: "capabilities shipping" },
            HeroStat { value: "11", label: "qquill-* crates" },
            HeroStat { value: "3", label: "planned" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    let mut board_section = board(
        "Status board",
        "Built, in progress, and planned",
        "Three lanes, no dates. The view layer, SSR, islands, SSG, the component library, and the \
         CLI are present and usable now; folder-route auto-discovery and component breadth have a \
         working seam; a quill dev server and the richer catalog are planned.",
        ["rm-quill-built", "rm-quill-partial", "rm-quill-planned"],
        [
            Lane { status: Status::Built, items: BUILT },
            Lane { status: Status::Partial, items: PARTIAL },
            Lane { status: Status::Planned, items: PLANNED },
        ],
    );
    board_section = board_section.child(legend()).child(note(vec![
        text("Read the ".to_string()),
        el("a").attr("href", "/docs/quill").child(text("Quill docs")),
        text(" or browse the ".to_string()),
        el("a").attr("href", "/components").child(text("component showcase")),
        text(" — every component on this site is a live Quill component.".to_string()),
    ]));

    main_wrap(vec![hero, board_section])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/roadmap/quill" };
    page(&meta, css, content)
}
