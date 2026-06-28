//! `GET /products/stdlib` — the q* stdlib (`qpkgs`) product page.
//!
//! A focused marketing page for the substrate: 13 zero-dependency crates —
//! qexec (the bounded executor/runtime) and qvalue (the value model + ABI) plus
//! eleven focused utility crates (array, object, string, math, number, convert,
//! crypto, encoding, regex, time, uuid). Everything — crypto, encoding, regex,
//! big numbers — is written from scratch. Products depend on q*; q* never
//! depends on a product. Content is accurate to `AGENTS.md`.

use qexec::FunctionResponse;
use qquill_view::Node;

use crate::app::routes::product_page::{
    arch_anim, arch_anim_css, closing, feature_section, hero, main_wrap, product_css,
    status_section, ArchNode, Cta, Feature, HeroStat, GITHUB_URL,
};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "The q* stdlib — 13 zero-dependency crates";
const DESCRIPTION: &str = "qpkgs is the q* stdlib: 13 zero-dependency crates — qexec (the bounded \
executor) and qvalue (the value/ABI) plus array, object, string, math, number, convert, crypto, \
encoding, regex, time, and uuid. Crypto, encoding, regex, and big numbers are all from scratch.";

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(arch_anim_css().to_string());

    let hero = hero(
        css,
        "The q* stdlib",
        "qpkgs",
        "13 zero-dependency crates, ",
        "one substrate",
        ".",
        "The shared foundation under every Qirava product. Two crates form the substrate — qexec, \
         the bounded executor, and qvalue, the value model and ABI — and eleven focused utilities \
         sit on top. Crypto, encoding, regex, and arbitrary-precision numbers are all written from \
         scratch. Products depend on q*; q* never depends on a product.",
        &[
            Cta {
                label: "Read the concepts",
                href: "/docs/stdlib",
                solid: true,
            },
            Cta {
                label: "View on GitHub",
                href: GITHUB_URL,
                solid: false,
            },
        ],
        &[
            HeroStat {
                value: "13",
                label: "zero-dep crates",
            },
            HeroStat {
                value: "2",
                label: "substrate crates",
            },
            HeroStat {
                value: "1",
                label: "way the arrow points",
            },
            HeroStat {
                value: "0",
                label: "third-party deps",
            },
        ],
    );

    // What it is.
    let what = feature_section(
        "stdlib-what",
        "gradient",
        "What it is",
        "One substrate, written from scratch",
        "The q* stdlib is everything the products need that would otherwise come from crates.io — \
         built first-party instead. Thirteen small crates: a bounded executor, a shared value \
         model, and eleven utilities that own their own crypto, encoding, regex, and big-number \
         math.",
        &[
            Feature {
                kicker: "qexec",
                title: "The bounded executor",
                body:
                    "Every call runs inside qexec, the runtime that governs memory and work with \
                       explicit resource budgets. Nothing executes unbounded.",
            },
            Feature {
                kicker: "qvalue",
                title: "The value model + ABI",
                body: "One unified Value and Record codec is the language every function speaks. \
                       It is the contract shared across the whole ecosystem.",
            },
            Feature {
                kicker: "eleven utilities",
                title: "Batteries, no dependencies",
                body:
                    "array, object, string, math, number, convert, crypto, encoding, regex, \
                       time, and uuid — each replacing a third-party package with first-party code.",
            },
        ],
    );

    // Features grid — the concrete capabilities.
    let feats = feature_section(
        "stdlib-feats",
        "flat",
        "What's inside",
        "Real capabilities, all first-party",
        "These are the building blocks the products actually call. Each one is std plus first-party \
         code only — the supply chain is exactly the team that ships it.",
        &[
            Feature {
                kicker: "qexec",
                title: "Bounded execution + budgets",
                body: "Resource budgets bound memory and work per call, so a runaway function is \
                       stopped by the runtime rather than the host.",
            },
            Feature {
                kicker: "qvalue",
                title: "Unified Value / Record codec",
                body: "One representation encodes and decodes every value and record — the single \
                       shape that crosses every function boundary.",
            },
            Feature {
                kicker: "qcrypto",
                title: "From-scratch cryptography",
                body: "SHA-256, HMAC, and SHA-1 implemented in-house behind a Crypto trait — the \
                       one dependency seam, kept first-party to avoid lock-in.",
            },
            Feature {
                kicker: "qencoding",
                title: "Base64 + hex",
                body: "Encoders and decoders for base64 and hex, written from scratch — no pulled-in \
                       encoding crate.",
            },
            Feature {
                kicker: "qregex",
                title: "A regex VM",
                body: "A real regular-expression engine — a compiled pattern run on a small virtual \
                       machine, not a wrapper around an external crate.",
            },
            Feature {
                kicker: "qnumber + quuid",
                title: "Big numbers + UUIDs",
                body: "Arbitrary-precision number math, plus UUID v4 and v7 generation and parsing \
                       — both implemented directly on the value model.",
            },
        ],
    );

    // How it works — concrete and plain.
    let how = feature_section(
        "stdlib-how",
        "glass",
        "How it works",
        "The Record convention, one register() per package",
        "The whole stdlib follows one calling convention and one wiring pattern. Once you have seen \
         a single function, you have seen them all.",
        &[
            Feature {
                kicker: "the Record convention",
                title: "Positional args in, {result} out",
                body: "Every function takes its arguments positionally as a Record and returns its \
                       answer as a Record with a {result} field. Same shape in, same shape out — \
                       across all thirteen crates.",
            },
            Feature {
                kicker: "register()",
                title: "One register() per package",
                body: "Each utility crate exposes a single register() that adds its functions to the \
                       executor's registry. Wiring the stdlib in is a handful of register() calls — \
                       nothing hidden.",
            },
            Feature {
                kicker: "no cycles",
                title: "qexec does not depend on qvalue",
                body: "The executor stays independent of the value model, so the substrate has no \
                       circular dependency. The arrow only ever points up: products → utilities → \
                       substrate.",
            },
        ],
    );

    // Architecture animation — the layering, no circular deps.
    let arch = arch_anim(
        "stdlib-arch",
        "Architecture",
        "Layered, never circular",
        "The substrate sits at the bottom — qexec and qvalue, each independent. Eleven utilities \
         layer on top of the value model, and the products sit above everything. The dependency \
         arrow only points one way, so the graph is acyclic by rule.",
        &[
            ArchNode {
                label: "qexec",
                sub: "bounded executor",
                badge: "SUBSTRATE",
            },
            ArchNode {
                label: "qvalue",
                sub: "value model + ABI",
                badge: "SUBSTRATE",
            },
            ArchNode {
                label: "11 utilities",
                sub: "crypto · regex · uuid …",
                badge: "STDLIB",
            },
            ArchNode {
                label: "products",
                sub: "DMS · Quill · Cloud",
                badge: "ON TOP",
            },
        ],
    );

    // Honest status.
    let status = status_section(
        "stdlib-status",
        "Status",
        "What's built today",
        "All 13 crates are shipping and in use across the products. The substrate and every utility \
         are first-party and zero-dependency; a few areas are still maturing.",
        &[
            (Status::Built, "qexec — the bounded executor",
             "Resource budgets governing memory and work for every call; the runtime the products serve on."),
            (Status::Built, "qvalue — the value model + ABI",
             "One unified Value/Record codec; the shared contract every function speaks."),
            (Status::Built, "From-scratch crypto + encoding",
             "SHA-256, HMAC, SHA-1 behind a Crypto trait, plus base64 and hex — all first-party."),
            (Status::Built, "regex VM, big numbers, UUID v4/v7",
             "A compiled regex engine, arbitrary-precision number math, and UUID generation/parsing."),
            (Status::Built, "One-way dependency direction",
             "Products depend on q*; q* never depends on a product — enforced as a structural rule."),
            (Status::Partial, "Broader utility coverage",
             "The eleven utilities cover what the products need today; surface area grows as the products do."),
        ],
    );

    let closing = closing(
        "stdlib-closing",
        "Stand on the substrate",
        "Read the concepts behind the value model and the bounded executor, or see how the products \
         are composed from these crates.",
        Cta { label: "Read the concepts", href: "/docs/stdlib", solid: true },
        Cta { label: "View on GitHub", href: GITHUB_URL, solid: false },
    );

    main_wrap(vec![hero, what, feats, how, arch, status, closing])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta {
        title: TITLE,
        description: DESCRIPTION,
        path: "/products/stdlib",
    };
    page(&meta, css, content)
}
