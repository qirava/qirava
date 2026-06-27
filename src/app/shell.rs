//! The shared site chrome: a polished sticky header (the FULL qbrand logo
//! lockup + nav with working dropdown-menu islands + a design-system control
//! panel island) and a refined footer.
//!
//! [`page`] is the one entry every marketing/route page calls. It wraps the
//! page's `<body>` content between the header and footer and hands the lot to
//! [`document`] for the full `<html>` + framed response.
//!
//! Brand: the header brand is the **full lockup** — `qbrand::LOGO_LOWERCASE_SVG`
//! inlined and recolored to `currentColor` (the only sanctioned recolor) so it
//! inherits the link's text color and flips ink-on-light / light-on-dark with
//! the theme automatically — no second asset, no `<img>` swap. The favicon is
//! `qbrand::ICON_SVG`, served at `/favicon.svg`.

use qquill_view::{el, island, raw, text, Node, Trigger};

use crate::app::{document, respond_html, Css, Meta};

const GITHUB_URL: &str = "https://github.com/qirava/qirava";

/// A single nav dropdown's contents: a label plus its menu links.
struct Menu {
    label: &'static str,
    id: &'static str,
    items: &'static [(&'static str, &'static str, &'static str)], // (title, desc, href)
}

/// The dropdown nav menus, in order. Each becomes a hover/click `menu` island.
const MENUS: &[Menu] = &[
    Menu {
        label: "Products",
        id: "products",
        items: &[
            ("Qirava DMS", "AI-native, zero-dependency data system", "/products"),
            ("Quill UI", "Rust-native UI + app framework", "/components"),
            ("Architecture", "How the pieces fit together", "/architecture"),
        ],
    },
    Menu {
        label: "Developer Docs",
        id: "docs",
        items: &[
            ("Documentation", "Guides, concepts, references", "/docs"),
            ("Getting started", "From zero to first app", "/docs/getting-started"),
            ("API reference", "The function + worker surface", "/api"),
            ("Components", "The Quill component catalog", "/components"),
        ],
    },
    Menu {
        label: "Roadmap",
        id: "roadmap",
        items: &[
            ("Roadmap", "What's shipped and what's next", "/roadmap"),
            ("Architecture", "Security + performance pillars", "/architecture"),
        ],
    },
];

/// The full brand lockup: `LOGO_LOWERCASE_SVG` recolored to `currentColor`,
/// inlined inside the home link so it inherits `color` (ink on light, light on
/// dark) and keeps its intrinsic aspect ratio.
fn brand() -> Node {
    let lockup = raw(qbrand::recolor_currentcolor(qbrand::LOGO_LOWERCASE_SVG));
    el("a")
        .class("q-brand")
        .attr("href", "/")
        .attr("aria-label", "qirava — home")
        .child(el("span").class("q-brand__lockup").child(lockup))
}

/// One dropdown nav item, built as a `menu` island with hover + click open.
///
/// The SSR fallback is the CLOSED menu: the trigger button visible, the surface
/// (`role="menu"`) hidden. The `menu` behavior wires keyboard + click + (on
/// fine-pointer devices) hover-open. `data-q-hover="true"` opts into hover.
fn nav_menu(menu: &Menu, active_path: &str) -> Node {
    let active = menu.items.iter().any(|(_, _, href)| {
        active_path == *href || (*href != "/" && active_path.starts_with(href))
    });

    let trigger = el("button")
        .class("q-nav__link q-nav__trigger")
        .attr("type", "button")
        .attr("data-q-part", "trigger")
        .attr("aria-haspopup", "menu")
        .attr("aria-expanded", "false")
        .child(text(menu.label))
        .child(raw(CHEVRON_SVG));
    let trigger = if active {
        trigger.attr("aria-current", "page")
    } else {
        trigger
    };

    let mut surface = el("div")
        .class("q-menu__surface")
        .attr("data-q-part", "surface")
        .attr("data-q-surface", "glass")
        .attr("role", "menu")
        .attr("hidden", "")
        .attr("aria-hidden", "true")
        .attr("aria-label", menu.label);
    for (title, desc, href) in menu.items {
        surface = surface.child(
            el("a")
                .class("q-menu__item")
                .attr("data-q-part", "item")
                .attr("role", "menuitem")
                .attr("tabindex", "-1")
                .attr("href", *href)
                .child(el("span").class("q-menu__item-title").child(text(*title)))
                .child(el("span").class("q-menu__item-desc").child(text(*desc))),
        );
    }

    let fallback = el("div")
        .class("q-menu")
        .attr("data-q-hover", "true")
        .child(trigger)
        .child(surface);

    let id = format!("nav-menu-{}", menu.id);
    island(id, "menu", Trigger::Load, "{}", fallback)
}

/// The design-system control panel, shipped as a `theme-control` island. Four
/// segmented groups (theme / size / radius / surface) whose buttons set the
/// `data-q-*` attributes on `<html>` and persist the choice. The SSR fallback is
/// real, labelled `<button>`s; with JS off they are inert but present.
fn theme_panel() -> Node {
    let axis = "data-q-axis";
    let value = "data-q-value";

    let group = |legend: &'static str,
                 ax: &'static str,
                 opts: &[(&'static str, &'static str)],
                 default: &'static str| {
        let g = el("div")
            .class("q-tc__group")
            .attr("role", "group")
            .attr("aria-label", legend)
            .child(el("span").class("q-tc__legend").child(text(legend)));
        let mut seg = el("div").class("q-tc__seg");
        for (label, val) in opts {
            let mut b = el("button")
                .class("q-tc__opt")
                .attr("type", "button")
                .attr(axis, ax)
                .attr(value, *val)
                .child(text(*label));
            if *val == default {
                b = b.attr("aria-pressed", "true");
            } else {
                b = b.attr("aria-pressed", "false");
            }
            seg = seg.child(b);
        }
        g.child(seg)
    };

    let panel = el("div")
        .class("q-tc__panel")
        .attr("data-q-surface", "glass")
        .attr("role", "menu")
        .attr("aria-label", "Appearance settings")
        .child(group("Theme", "theme", &[("Light", "light"), ("Dark", "dark")], "dark"))
        .child(group(
            "Density",
            "size",
            &[("Compact", "compact"), ("Cozy", "cozy"), ("Comfortable", "comfortable")],
            "cozy",
        ))
        .child(group(
            "Radius",
            "radius",
            &[("Sharp", "sharp"), ("Rounded", "rounded"), ("Pill", "pill")],
            "rounded",
        ))
        .child(group(
            "Surface",
            "surface",
            &[("Flat", "flat"), ("Glass", "glass"), ("Neu", "neu"), ("Gradient", "gradient")],
            "flat",
        ));

    let trigger = el("button")
        .class("q-tc__trigger")
        .attr("type", "button")
        .attr("data-q-part", "trigger")
        .attr("aria-haspopup", "menu")
        .attr("aria-expanded", "false")
        .attr("aria-label", "Appearance settings")
        .attr("title", "Appearance")
        .child(raw(SLIDERS_SVG));

    // The panel is opened by a `menu` island (click + hover), and its body is a
    // `theme-control` island that wires the segmented buttons. Two islands, one
    // wrapper — the menu drives open/close, theme-control drives the switches.
    let surface = el("div")
        .class("q-menu__surface q-tc__surface")
        .attr("data-q-part", "surface")
        .attr("role", "menu")
        .attr("hidden", "")
        .attr("aria-hidden", "true")
        .child(island("theme-control", "theme-control", Trigger::Load, "{}", panel));

    let menu_fallback = el("div")
        .class("q-menu q-tc")
        .attr("data-q-hover", "true")
        .child(trigger)
        .child(surface);

    island("theme-menu", "menu", Trigger::Load, "{}", menu_fallback)
}

/// Inline chevron for dropdown triggers (decorative).
const CHEVRON_SVG: &str = "<svg class=\"q-chev\" viewBox=\"0 0 24 24\" width=\"14\" height=\"14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"m6 9 6 6 6-6\"/></svg>";
const EXTERNAL_SVG: &str = "<svg class=\"q-ext\" viewBox=\"0 0 24 24\" width=\"13\" height=\"13\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M7 17 17 7M9 7h8v8\"/></svg>";
const SLIDERS_SVG: &str = "<svg viewBox=\"0 0 24 24\" width=\"18\" height=\"18\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M4 21v-7M4 10V3M12 21v-9M12 8V3M20 21v-5M20 12V3M1 14h6M9 8h6M17 16h6\"/></svg>";

/// Build the header: brand lockup left, nav (Home + dropdowns + GitHub) and the
/// appearance control right. `active_path` marks the current top-level section.
fn header(active_path: &str) -> Node {
    let home_active = active_path == "/";
    let mut home = el("a")
        .class("q-nav__link")
        .attr("href", "/")
        .child(text("Home"));
    if home_active {
        home = home.attr("aria-current", "page");
    }

    let mut nav = el("nav").class("q-nav").attr("aria-label", "Primary").child(home);
    for menu in MENUS {
        nav = nav.child(nav_menu(menu, active_path));
    }
    nav = nav.child(
        el("a")
            .class("q-nav__link q-nav__link--external")
            .attr("href", GITHUB_URL)
            .attr("rel", "noopener")
            .attr("target", "_blank")
            .child(text("GitHub"))
            .child(raw(EXTERNAL_SVG)),
    );

    el("header").class("q-site-header").child(
        el("div")
            .class("q-site-header__inner")
            .child(brand())
            .child(
                el("div")
                    .class("q-nav-wrap")
                    .child(nav)
                    .child(theme_panel()),
            ),
    )
}

/// The site footer: brand lockup, tagline, grouped link columns, and a legal row.
fn footer() -> Node {
    let col = |title: &str, links: &[(&'static str, &'static str)]| {
        let mut c = el("div")
            .class("q-foot__col")
            .child(el("p").class("q-foot__title").child(text(title.to_string())));
        for (label, href) in links {
            c = c.child(el("a").attr("href", *href).child(text(*label)));
        }
        c
    };

    let lockup = raw(qbrand::recolor_currentcolor(qbrand::LOGO_LOWERCASE_SVG));
    let brand_block = el("div")
        .class("q-foot__brand")
        .child(el("span").class("q-brand__lockup q-foot__lockup").child(lockup))
        .child(el("p").class("q-foot__tag").child(text(
            "An AI-native, zero-dependency data system — and the Rust-native UI framework to build on it.",
        )));

    el("footer").class("q-site-footer").child(
        el("div")
            .class("q-site-footer__inner")
            .child(
                el("div")
                    .class("q-foot__top")
                    .child(brand_block)
                    .child(col(
                        "Product",
                        &[("Docs", "/docs"), ("Components", "/components"), ("Products", "/products")],
                    ))
                    .child(col(
                        "Developers",
                        &[("Getting started", "/docs/getting-started"), ("API reference", "/api"), ("Architecture", "/architecture")],
                    ))
                    .child(col(
                        "Project",
                        &[("Roadmap", "/roadmap"), ("GitHub", GITHUB_URL)],
                    )),
            )
            .child(
                el("p").class("q-foot__legal").child(text(
                    "Qirava — Apache-2.0 licensed. \"Qirava\" is a trademark. Built with Quill, dogfooding itself.",
                )),
            ),
    )
}

/// Assemble a full page response: skip-link, header, the page `content`, footer.
pub fn page(meta: &Meta, mut css: Css, content: Node) -> qexec::FunctionResponse {
    css.push(chrome_css().to_string());
    let body = el("div")
        .class("q-app")
        .child(
            el("a")
                .class("q-skiplink")
                .attr("href", "#main")
                .child(text("Skip to content")),
        )
        .child(header(meta.path))
        .child(content)
        .child(footer());

    let head_css = css.into_css();
    let tree = document(meta, head_css, body);
    respond_html(&tree)
}

/// Chrome-specific CSS (header / brand lockup / nav dropdowns / control panel /
/// footer). Every value references a `--q-*` token so it restyles on any
/// theme/size/radius switch with no reflow. Motion uses the duration/easing
/// tokens so `reduced_motion_css` can neutralize it.
fn chrome_css() -> &'static str {
    "\
/* ---- sticky header ---- */\
.q-site-header{position:sticky;top:0;z-index:40;-webkit-backdrop-filter:saturate(1.4) blur(12px);backdrop-filter:saturate(1.4) blur(12px);background:color-mix(in srgb,var(--q-color-bg) 80%,transparent);border-bottom:1px solid var(--q-color-border)}\
.q-site-header__inner{max-width:80rem;margin:0 auto;display:flex;align-items:center;justify-content:space-between;gap:1.5rem;padding:.7rem 1.5rem;min-height:calc(var(--q-control-h,2.5rem) + 1.2rem)}\
/* ---- brand lockup (full LOGO_LOWERCASE, inherits currentColor) ---- */\
.q-brand{display:inline-flex;align-items:center;line-height:0;color:var(--q-color-fg);min-height:var(--q-control-h,2.5rem)}\
.q-brand:hover{text-decoration:none;opacity:.85}\
.q-brand__lockup{display:inline-flex;height:calc(1.5rem * var(--q-density,1))}\
.q-brand__lockup svg{height:100%;width:auto;display:block}\
/* ---- nav ---- */\
.q-nav-wrap{display:flex;align-items:center;gap:.5rem}\
.q-nav{display:flex;align-items:center;gap:.15rem}\
.q-nav__link{display:inline-flex;align-items:center;gap:.3rem;color:var(--q-color-muted);font-weight:var(--q-font-weight-medium);font-size:.95rem;line-height:1;padding:.5rem .7rem;min-height:var(--q-control-h,2.5rem);border-radius:var(--q-radius-md);border:0;background:transparent;cursor:pointer;font-family:inherit;transition:color var(--q-duration-fast) var(--q-ease-out),background-color var(--q-duration-fast) var(--q-ease-out)}\
.q-nav__link:hover{color:var(--q-color-fg);background-color:var(--q-color-surface);text-decoration:none}\
.q-nav__link:focus-visible{outline:2px solid var(--q-color-brand);outline-offset:2px}\
.q-nav__link[aria-current=\"page\"]{color:var(--q-color-fg)}\
.q-nav__trigger .q-chev{opacity:.7;transition:transform var(--q-duration-fast) var(--q-ease-out)}\
.q-nav__trigger[aria-expanded=\"true\"]{color:var(--q-color-fg);background-color:var(--q-color-surface)}\
.q-nav__trigger[aria-expanded=\"true\"] .q-chev{transform:rotate(180deg)}\
.q-ext{opacity:.55}\
/* ---- dropdown menus (the `menu` island) ---- */\
.q-menu{position:relative;display:inline-flex}\
.q-menu__surface{position:absolute;top:calc(100% + .5rem);left:0;z-index:50;min-width:17rem;display:flex;flex-direction:column;gap:.1rem;padding:.4rem;border-radius:var(--q-radius-lg);transform-origin:top left;animation:q-menu-in var(--q-duration-fast) var(--q-ease-out)}\
.q-menu__surface[hidden]{display:none}\
@keyframes q-menu-in{from{opacity:0;transform:translateY(-6px) scale(.98)}to{opacity:1;transform:none}}\
.q-menu__item{display:flex;flex-direction:column;gap:.1rem;padding:.55rem .7rem;border-radius:var(--q-radius-md);color:var(--q-color-fg);transition:background-color var(--q-duration-fast) var(--q-ease-out)}\
.q-menu__item:hover,.q-menu__item[data-state=\"active\"]{background-color:var(--q-color-surface-selected,var(--q-color-surface));text-decoration:none}\
.q-menu__item:focus-visible{outline:2px solid var(--q-color-brand);outline-offset:-2px}\
.q-menu__item-title{font-weight:var(--q-font-weight-medium);font-size:.92rem;line-height:1.3}\
.q-menu__item-desc{color:var(--q-color-muted);font-size:.8rem;line-height:1.35}\
/* ---- appearance control panel (menu + theme-control islands) ---- */\
.q-tc__trigger{display:inline-flex;align-items:center;justify-content:center;width:var(--q-control-h,2.5rem);height:var(--q-control-h,2.5rem);border-radius:var(--q-radius-md);border:1px solid var(--q-color-border);background:var(--q-color-surface);color:var(--q-color-fg);cursor:pointer;transition:border-color var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast) var(--q-ease-out)}\
.q-tc__trigger:hover{border-color:var(--q-color-brand);color:var(--q-color-brand)}\
.q-tc__trigger:focus-visible{outline:2px solid var(--q-color-brand);outline-offset:2px}\
.q-tc__trigger[aria-expanded=\"true\"]{border-color:var(--q-color-brand);color:var(--q-color-brand)}\
.q-tc__surface{left:auto;right:0;min-width:16rem;transform-origin:top right}\
.q-tc__panel{display:flex;flex-direction:column;gap:1rem;padding:1rem}\
.q-tc__group{display:flex;flex-direction:column;gap:.4rem}\
.q-tc__legend{font-size:var(--q-font-size-xs);text-transform:uppercase;letter-spacing:var(--q-tracking-wide);color:var(--q-color-muted);font-weight:var(--q-font-weight-bold)}\
.q-tc__seg{display:flex;flex-wrap:wrap;border:1px solid var(--q-color-border);border-radius:var(--q-radius-md);overflow:hidden}\
.q-tc__opt{flex:1 1 auto;appearance:none;border:0;background:transparent;color:var(--q-color-muted);font:inherit;font-size:.82rem;padding:.4rem .6rem;cursor:pointer;white-space:nowrap;transition:background-color var(--q-duration-fast) var(--q-ease-out),color var(--q-duration-fast) var(--q-ease-out)}\
.q-tc__opt+.q-tc__opt{border-left:1px solid var(--q-color-border)}\
.q-tc__opt:hover{color:var(--q-color-fg)}\
.q-tc__opt[aria-pressed=\"true\"]{background:var(--q-color-brand);color:var(--q-color-on-brand)}\
.q-tc__opt:focus-visible{outline:2px solid var(--q-color-brand);outline-offset:-2px}\
/* ---- mobile: collapse the text nav, keep brand + appearance ---- */\
@media (max-width:820px){.q-nav{display:none}}\
/* ---- footer ---- */\
.q-site-footer{border-top:1px solid var(--q-color-border);background:var(--q-color-surface);color:var(--q-color-muted);margin-top:calc(var(--q-space-10) * var(--q-density,1))}\
.q-site-footer__inner{max-width:80rem;margin:0 auto;padding:3rem 1.5rem 2.5rem}\
.q-foot__top{display:grid;grid-template-columns:2.2fr 1fr 1fr 1fr;gap:2rem;padding:0 0 2rem;border-bottom:1px solid var(--q-color-border)}\
@media (max-width:820px){.q-foot__top{grid-template-columns:1fr 1fr}}\
@media (max-width:520px){.q-foot__top{grid-template-columns:1fr}}\
.q-foot__brand{color:var(--q-color-fg)}\
.q-foot__lockup{height:1.6rem}\
.q-foot__tag{color:var(--q-color-muted);max-width:36ch;margin:1rem 0 0;font-size:.92rem;line-height:1.6}\
.q-foot__col{display:flex;flex-direction:column;gap:.6rem}\
.q-foot__title{color:var(--q-color-fg);font-weight:var(--q-font-weight-bold);margin:0 0 .35rem;font-size:.78rem;text-transform:uppercase;letter-spacing:.1em}\
.q-foot__col a{color:var(--q-color-muted);font-size:.92rem}\
.q-foot__col a:hover{color:var(--q-color-fg);text-decoration:none}\
.q-foot__legal{color:var(--q-color-muted);font-size:.85rem;margin:1.75rem 0 0}"
}
