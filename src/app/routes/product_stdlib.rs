//! `GET /products/stdlib` — the q* stdlib (`qpkgs`) product page.
//!
//! The substrate in one read: 13 zero-dependency crates — qexec (the bounded
//! executor/runtime) and qvalue (the value model + ABI) plus eleven focused
//! utility crates (array, object, string, math, number, convert, crypto,
//! encoding, regex, time, uuid). Shared across every product; the dependency
//! arrow points one way — products depend on q*, q* never depends on a product.
//! Content is accurate to `AGENTS.md`.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{
    closing, feature_section, hero, main_wrap, product_css, status_section, Cta, Feature, HeroStat,
    GITHUB_URL,
};
use crate::app::routes::{reveal, Status};
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "The q* stdlib — 13 zero-dependency crates";
const DESCRIPTION: &str = "qpkgs is the q* stdlib: 13 zero-dependency crates — qexec (the bounded \
executor) and qvalue (the value/ABI) plus array, object, string, math, number, convert, crypto, \
encoding, regex, time, and uuid. Shared across every product.";

/// One crate in the stdlib catalog.
struct Crate {
    name: &'static str,
    role: &'static str,
}

const SUBSTRATE: &[Crate] = &[
    Crate { name: "qexec", role: "The bounded executor + runtime: governs memory and work for every call." },
    Crate { name: "qvalue", role: "The value model + ABI: the shared representation every function speaks." },
];

const UTILITIES: &[Crate] = &[
    Crate { name: "qarray", role: "Array operations and helpers over the value model." },
    Crate { name: "qobject", role: "Object/map construction, access, and merging." },
    Crate { name: "qstring", role: "String utilities — slicing, casing, formatting." },
    Crate { name: "qmath", role: "Math operations beyond the language primitives." },
    Crate { name: "qnumber", role: "Numeric parsing, formatting, and conversions." },
    Crate { name: "qconvert", role: "Typed conversions between value forms." },
    Crate { name: "qcrypto", role: "Cryptography behind a Crypto trait — the one dependency seam." },
    Crate { name: "qencoding", role: "Encoders/decoders (base64, hex, and friends)." },
    Crate { name: "qregex", role: "A regular-expression engine, no external crate." },
    Crate { name: "qtime", role: "Time and duration handling." },
    Crate { name: "quuid", role: "UUID generation and parsing." },
];

/// A grid of crate cards, one cell per crate, wrapped in a scroll-reveal island.
fn crate_grid(id: &'static str, items: &[Crate]) -> Node {
    let mut grid = el("div").class("q-pp-crates");
    for (i, c) in items.iter().enumerate() {
        grid = grid.child(
            el("div")
                .attr("data-q-reveal", "")
                .attr("data-reveal-delay", ((i % 3) + 1).to_string())
                .child(
                    el("article")
                        .class("q-pp-crate-card")
                        .child(el("code").class("q-pp-crate-card__name").child(text(c.name.to_string())))
                        .child(el("p").class("q-pp-crate-card__role").child(text(c.role.to_string()))),
                ),
        );
    }
    reveal(id, grid)
}

fn catalog() -> Node {
    let sub_head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text("The catalog")))
        .child(el("h2").class("q-h2").child(text("The substrate: two crates")))
        .child(el("p").class("q-lead").child(text(
            "Everything else stands on qexec and qvalue — the bounded executor and the value/ABI. \
             Together they are the contract every function and every product shares.",
        )));

    let util_head = el("div")
        .class("q-pp-head")
        .child(el("p").class("q-eyebrow").child(text("The catalog")))
        .child(el("h2").class("q-h2").child(text("Eleven focused utilities")))
        .child(el("p").class("q-lead").child(text(
            "Small, single-purpose crates — each replacing what would otherwise be a third-party \
             dependency. Crypto is the sole exception to zero-dep, kept behind a trait.",
        )));

    el("section")
        .class("q-section")
        .child(reveal("stdlib-sub-head", sub_head))
        .child(crate_grid("stdlib-sub", SUBSTRATE))
        .child(reveal("stdlib-util-head", util_head))
        .child(crate_grid("stdlib-util", UTILITIES))
}

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(stdlib_css().to_string());

    let hero = hero(
        css,
        "The q* stdlib",
        "qpkgs",
        "13 zero-dependency crates, ",
        "one substrate",
        ".",
        "The shared foundation under every Qirava product. The substrate is qexec (the bounded \
         executor) and qvalue (the value model + ABI); the rest are eleven focused utility crates \
         that replace what would otherwise be third-party dependencies. Products depend on q*; \
         q* never depends on a product.",
        &[
            Cta { label: "Read the concepts", href: "/docs/stdlib", solid: true },
            Cta { label: "View on GitHub", href: GITHUB_URL, solid: false },
        ],
        &[
            HeroStat { value: "13", label: "zero-dep crates" },
            HeroStat { value: "2", label: "substrate crates" },
            HeroStat { value: "1", label: "way the arrow points" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    // Why a stdlib at all.
    let why = feature_section(
        "stdlib-why",
        "gradient",
        "Why a stdlib",
        "Replace dependencies, don't accumulate them",
        "Zero third-party dependencies is a hard rule. The way to keep it is to own the building \
         blocks — std plus first-party crates only — so the supply chain is exactly the team that \
         ships it.",
        &[
            Feature {
                kicker: "std + first-party only",
                title: "Zero third-party deps",
                body: "Every utility the products need lives here instead of in a crates.io \
                       dependency. The one exception is cryptography, kept behind a Crypto trait to \
                       avoid lock-in.",
            },
            Feature {
                kicker: "products → q*",
                title: "The arrow points one way",
                body: "Products (the DMS, Quill) depend on q*; q* never depends on a product. This \
                       keeps the substrate reusable and the dependency graph acyclic by rule, not by \
                       luck.",
            },
            Feature {
                kicker: "shared substrate",
                title: "Dogfooded everywhere",
                body: "The same qexec executor that bounds a DMS call also bounds the work in any \
                       product built on it. One value model, one executor, shared across the whole \
                       ecosystem.",
            },
        ],
    );

    let status = status_section(
        "stdlib-status",
        "Status",
        "What's built today",
        "All 13 crates are shipping and in use across the products. The substrate and every \
         utility are first-party and zero-dependency.",
        &[
            (Status::Built, "qexec — the bounded executor",
             "Governs memory and work for every execute() call; the runtime the products serve on."),
            (Status::Built, "qvalue — the value model + ABI",
             "The shared representation every function speaks; the contract across products."),
            (Status::Built, "Eleven utility crates",
             "array, object, string, math, number, convert, crypto, encoding, regex, time, uuid — all first-party."),
            (Status::Built, "One-way dependency direction",
             "Products depend on q*; q* never depends on a product — enforced as a structural rule."),
        ],
    );

    let closing = closing(
        "stdlib-closing",
        "Stand on the substrate",
        "Read the concepts behind the value model and the bounded executor, or see how the \
         products are composed from these crates.",
        Cta { label: "Read the concepts", href: "/docs/stdlib", solid: true },
        Cta { label: "See the architecture", href: "/architecture", solid: false },
    );

    main_wrap(vec![hero, why, catalog(), status, closing])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/products/stdlib" };
    page(&meta, css, content)
}

/// Stdlib-only CSS: the crate catalog grid. Token-driven; pushed once + deduped.
fn stdlib_css() -> &'static str {
    "\
.q-pp-crates{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,15rem),1fr));gap:var(--q-space-3);margin-bottom:var(--q-space-5)}\
.q-pp-crates>*{height:100%}\
.q-pp-crate-card{display:flex;flex-direction:column;gap:.35rem;height:100%;padding:var(--q-space-4) var(--q-space-5);border:1px solid var(--q-color-border);border-radius:var(--q-radius-lg);background:var(--q-color-surface);border-left:3px solid var(--q-color-brand);transition:transform var(--q-duration-base) var(--q-ease-out),border-color var(--q-duration-base) var(--q-ease-out)}\
.q-pp-crate-card:hover{transform:translateY(-3px);border-color:color-mix(in srgb,var(--q-color-brand) 45%,var(--q-color-border))}\
.q-pp-crate-card__name{font-family:var(--q-font-mono);font-size:1rem;font-weight:var(--q-font-weight-bold);color:var(--q-color-fg)}\
.q-pp-crate-card__role{margin:0;font-size:.9rem;line-height:1.55;color:var(--q-color-muted)}\
@media (prefers-reduced-motion:reduce){.q-pp-crate-card:hover{transform:none}}"
}
