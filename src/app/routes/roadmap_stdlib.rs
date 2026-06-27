//! `GET /roadmap/stdlib` — the q* stdlib roadmap.
//!
//! BUILT: the 13 zero-dependency q* crates. PLANNED: more cryptographic
//! primitives — SHA-512 / HMAC-SHA512 / HKDF / PBKDF2, ChaCha20-Poly1305,
//! Shamir SSS, Ed25519 / ES256-verify, and ML-KEM-768 / ML-DSA-65 — behind the
//! Crypto provider trait, per the security docs. No dates promised — only state.

use qexec::FunctionResponse;
use qquill_view::{el, text, Node};

use crate::app::routes::product_page::{hero, main_wrap, product_css, Cta, HeroStat};
use crate::app::routes::roadmap_page::{board, legend, note, roadmap_css, Item, Lane};
use crate::app::routes::Status;
use crate::app::shell::page;
use crate::app::{Css, Meta};

const TITLE: &str = "The q* stdlib roadmap — built, in progress, planned";
const DESCRIPTION: &str = "An honest status board for the q* stdlib: the 13 zero-dependency shared \
crates (qexec, qvalue, and the utilities) are shipping; the planned work is the cryptographic \
primitives — HKDF, ChaCha20-Poly1305, Shamir, Ed25519, and ML-KEM/ML-DSA — behind the Crypto trait.";

const BUILT: &[Item] = &[
    Item { title: "qexec — the bounded executor", detail: "The single chokepoint to the engine: one execute primitive and one function registry (static + dynamic), bounding memory and work per call." },
    Item { title: "qvalue — the value model + ABI", detail: "The arbitrary-precision value model that every function speaks; the ABI that crosses the execute boundary." },
    Item { title: "qarray, qobject, qstring", detail: "Collection and text utilities over the qvalue model — array, object, and string operations, all QQL-callable." },
    Item { title: "qmath, qnumber, qconvert", detail: "Numeric and conversion utilities: math operations, number handling, and value coercion." },
    Item { title: "qencoding, qregex", detail: "Encoding/decoding (base/hex/JSON-shaped) and a from-scratch regex engine." },
    Item { title: "qtime, quuid", detail: "Time/duration handling and UUID generation — no system-clock or RNG crate dependency." },
    Item { title: "qcrypto — the Crypto trait + base primitives", detail: "The provider-trait seam that keeps cryptography swappable (the sole sanctioned dependency surface), with the primitives needed by auth today." },
];

const PARTIAL: &[Item] = &[
    Item { title: "Crypto behind a provider trait", detail: "Cryptography lives behind the Crypto trait so the implementation can be swapped without touching callers; today it carries the HMAC path auth relies on." },
];

const PLANNED: &[Item] = &[
    Item { title: "SHA-512 / HMAC-SHA512 / HKDF / PBKDF2", detail: "The hash + key-derivation family, self-implemented with known-answer tests and constant-time review, first in the crypto order." },
    Item { title: "ChaCha20-Poly1305", detail: "Authenticated encryption for at-rest secrets (the per-key sign_secret and the master seed)." },
    Item { title: "Shamir Secret Sharing", detail: "M-of-N splitting of the master seed for the custodian ceremony — any one of passkey-PRF, a second passkey, or a BIP39 plate recovers a share." },
    Item { title: "Ed25519 / ES256 verify", detail: "Signature verification for request signing and tokens, dispatched through the one before-auth security funnel." },
    Item { title: "ML-KEM-768 / ML-DSA-65 (PQC)", detail: "Post-quantum key encapsulation and signatures for API keys and tokens, behind the verify_request seam." },
];

fn body(css: &mut Css) -> Node {
    css.push(product_css().to_string());
    css.push(roadmap_css().to_string());

    let hero = hero(
        css,
        "q* stdlib roadmap",
        "qpkgs",
        "The shared substrate, ",
        "honestly tracked",
        ".",
        "The 13 zero-dependency q* crates — qexec and qvalue plus the focused utilities — are \
         shipping today and shared across every product. The planned work is the cryptographic \
         primitives the security model needs, self-implemented behind the Crypto provider trait: \
         the hash/KDF family, ChaCha20-Poly1305, Shamir, Ed25519, and ML-KEM/ML-DSA. No dates \
         promised — only state.",
        &[
            Cta { label: "Explore the stdlib", href: "/products/stdlib", solid: true },
            Cta { label: "Read the docs", href: "/docs/stdlib", solid: false },
        ],
        &[
            HeroStat { value: "13", label: "crates shipping" },
            HeroStat { value: "5", label: "crypto families planned" },
            HeroStat { value: "1", label: "Crypto provider trait" },
            HeroStat { value: "0", label: "third-party deps" },
        ],
    );

    let mut board_section = board(
        "Status board",
        "Built, in progress, and planned",
        "Three lanes, no dates. All 13 crates are present and usable now; cryptography sits behind a \
         provider trait so it can grow without churn; the planned lane is the crypto order from the \
         security docs, each primitive self-implemented with known-answer tests and a constant-time \
         audit.",
        ["rm-stdlib-built", "rm-stdlib-partial", "rm-stdlib-planned"],
        [
            Lane { status: Status::Built, items: BUILT },
            Lane { status: Status::Partial, items: PARTIAL },
            Lane { status: Status::Planned, items: PLANNED },
        ],
    );
    board_section = board_section.child(legend()).child(note(vec![
        text("The crypto order is sourced from the repo's PENDING list and ".to_string()),
        el("a").attr("href", "/docs/dms/architecture-overview").child(text("the architecture")),
        text("; the dependency arrow points one way — products depend on q*, never the reverse."
            .to_string()),
    ]));

    main_wrap(vec![hero, board_section])
}

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    let content = body(&mut css);
    let meta = Meta { title: TITLE, description: DESCRIPTION, path: "/roadmap/stdlib" };
    page(&meta, css, content)
}
