//! Hand-authored, VERIFIED docs content that overrides the generated
//! `docs_content` for the few pages a newcomer hits first (install → run → first
//! query → configure). Every command + output here was run against a live DMS and
//! transcribed, so the "zero to working" path actually works.
//!
//! `render_doc` checks this module first, then falls back to the generated
//! content. To hand-own a page, add a match arm here.

use crate::app::docs_kit::{Block as B, Page, Section as S};

/// Verified content for `path`, if this module owns it.
pub fn content(path: &str) -> Option<Page> {
    match path {
        "/docs/dms/install" => Some(install()),
        "/docs/dms/quick-start" => Some(quick_start()),
        "/docs/dms/configuration" => Some(configuration()),
        _ => None,
    }
}

fn install() -> Page {
    Page {
        lead: "Qirava DMS is one Rust binary with zero third-party dependencies. Clone the \
               superproject with its submodules, build, and run — there is nothing else to install."
            .into(),
        sections: vec![
            S {
                heading: "Clone and build".into(),
                blocks: vec![
                    B::Prose("The site, the engine, and the q* stdlib are submodules of the `qroot` superproject, so clone recursively. A release build is a single self-contained binary.".into()),
                    B::Code { lang: "bash".into(), code: "git clone --recursive https://github.com/qirava/qroot\ncd qroot/qdms\ncargo build --release".into() },
                    B::Callout { warn: false, label: "Requirements".into(), body: "A recent stable Rust toolchain (`rustup` + `cargo`). No database server, no Node, no system libraries — `std` + the first-party crates only.".into() },
                ],
            },
            S {
                heading: "Run it".into(),
                blocks: vec![
                    B::Prose("Start the DMS. It binds one port for both the UI (`/`, `/docs`, …) and the API (`/api/qql`), prints where it is listening, and — on a first secure start — a one-time bootstrap custodian for Studio.".into()),
                    B::Code { lang: "bash".into(), code: "cargo run --bin qdms        # or: ./target/release/qdms".into() },
                    B::Code { lang: "text".into(), code: "Qirava DMS\n  engine:  one executor + function registry\n  bound:   http://127.0.0.1:7179\n  data:    durable @ ~/.qdms/qdms-data\n  routes:  POST /api/qql   (QQL in body)\n           GET  /api/qql?q=...\n  BOOTSTRAP CUSTODIAN (shown once): user=custodian password=…".into() },
                    B::Prose("Next: the **Quick start** runs your first query in about two minutes.".into()),
                ],
            },
        ],
    }
}

fn quick_start() -> Page {
    Page {
        lead: "From a clean checkout to a working query in about two minutes. Boot the DMS, open \
               it for local development, then run real QQL over plain HTTP — no driver, no SDK, no \
               schema migration step."
            .into(),
        sections: vec![
            S {
                heading: "1. Build and run".into(),
                blocks: vec![
                    B::Prose("Build the workspace and start the DMS. It tells you the address it bound and prints a one-time bootstrap custodian (for Studio).".into()),
                    B::Code { lang: "bash".into(), code: "git clone --recursive https://github.com/qirava/qroot\ncd qroot/qdms\ncargo run --bin qdms".into() },
                    B::Code { lang: "text".into(), code: "  bound:   http://127.0.0.1:7179\n  routes:  POST /api/qql   (QQL in body)\n           GET  /api/qql?q=...".into() },
                ],
            },
            S {
                heading: "2. Open it for local development".into(),
                blocks: vec![
                    B::Prose("By default the API requires a signed key (`require_api_key = true`) — right for anything real, but it blocks a quick `curl`. For local development, set it to `false` in the config and restart so you can talk to `/api/qql` directly.".into()),
                    B::Code { lang: "text".into(), code: "# ~/.qdms/dms.config  (auto-generated on first run)\naddr = 127.0.0.1:7179\nrequire_api_key = false\ndata_dir = memory          # ephemeral; use a path for a durable WAL".into() },
                    B::Callout { warn: true, label: "Local only".into(), body: "`require_api_key = false` removes auth on the API — use it only on your own machine. In production keep it `true` and sign requests with an API key (see Authentication & RBAC).".into() },
                ],
            },
            S {
                heading: "3. Create a table, insert, and query".into(),
                blocks: vec![
                    B::Prose("Everything is QQL over `POST /api/qql`, with the statement as the request body. Every response is one uniform envelope: `{ error, data, root }`.".into()),
                    B::Example {
                        request: "curl -X POST http://127.0.0.1:7179/api/qql \\\n  --data 'CREATE TABLE users SCHEMALESS'".into(),
                        response: "{\"error\":null,\"data\":null,\"root\":{\"count\":0}}".into(),
                        output: String::new(),
                    },
                    B::Example {
                        request: "curl -X POST http://127.0.0.1:7179/api/qql \\\n  --data 'INSERT INTO users { email: \"ada@example.com\", age: 36 }'".into(),
                        response: "{\"error\":null,\n \"data\":[{\"_id\":1,\"email\":\"ada@example.com\",\"age\":36}],\n \"root\":{\"count\":1}}".into(),
                        output: "The DB assigns the immutable _id and echoes back the stored row.".into(),
                    },
                    B::Example {
                        request: "curl -X POST http://127.0.0.1:7179/api/qql \\\n  --data 'SELECT email, age FROM users WHERE age >= 18 SORT age DESC LIMIT 10'".into(),
                        response: "{\"error\":null,\n \"data\":[\n   {\"email\":\"bob@example.com\",\"age\":42},\n   {\"email\":\"ada@example.com\",\"age\":36}\n ],\n \"root\":{\"count\":2}}".into(),
                        output: String::new(),
                    },
                ],
            },
            S {
                heading: "4. The two HTTP forms".into(),
                blocks: vec![
                    B::Prose("Writes use `POST` with the statement in the body. Read-only queries can also use `GET /api/qql?q=...`, which is handy straight from a browser.".into()),
                    B::Example {
                        request: "curl 'http://127.0.0.1:7179/api/qql?q=SELECT%20email%20FROM%20users'".into(),
                        response: "{\"error\":null,\"data\":[{\"email\":\"ada@example.com\"},{\"email\":\"bob@example.com\"}],\"root\":{\"count\":2}}".into(),
                        output: String::new(),
                    },
                    B::Callout { warn: false, label: "What next".into(), body: "You just used the relational core. The same engine does full-text **SEARCH**, **graph** traversal, and **vector** k-NN over the same rows — see the QQL section. Always bind user values with `:name` placeholders to stay injection-safe.".into() },
                ],
            },
        ],
    }
}

fn configuration() -> Page {
    Page {
        lead: "The DMS reads a tiny `key = value` config, resolved in order: `$QDMS_CONFIG` → \
               `./dms.config` → `~/.qdms/dms.config` (auto-generated on first run). A few keys also \
               have environment overrides."
            .into(),
        sections: vec![
            S {
                heading: "The config file".into(),
                blocks: vec![
                    B::Prose("Edit the file and restart to apply. Blank lines and `#` comments are ignored.".into()),
                    B::Code { lang: "text".into(), code: "# ~/.qdms/dms.config\naddr = 127.0.0.1:7179      # UI + API share this port\nrequire_api_key = true     # API needs a signed key (UI routes stay public)\ndata_dir = /home/you/.qdms/qdms-data   # or `memory` for ephemeral".into() },
                ],
            },
            S {
                heading: "Keys".into(),
                blocks: vec![
                    B::Defs(vec![
                        ("addr".into(), "Address the DMS binds. The UI and `/api/qql` share it. Env override: `QDMS_ADDR`.".into()),
                        ("require_api_key".into(), "When `true`, `/api/qql` needs an HMAC-signed API key (a bootstrap admin key is minted + printed once on first start). Set `false` for local dev. UI routes are always public.".into()),
                        ("data_dir".into(), "Durable data directory (WAL + crash recovery). Use `memory` for an ephemeral instance. Env override: `QDMS_DATA`.".into()),
                    ]),
                ],
            },
            S {
                heading: "Other environment knobs".into(),
                blocks: vec![
                    B::Defs(vec![
                        ("QDMS_CONFIG".into(), "Path to a config file to use instead of the default lookup.".into()),
                        ("QDMS_MAX_CONN".into(), "Cap on concurrent connections.".into()),
                        ("QDMS_REPL_ROLE".into(), "`master` | `follower` to enable single-leader replication (default: standalone).".into()),
                    ]),
                    B::Prose("See **Performance tuning** for the executor budget and WAL knobs.".into()),
                ],
            },
        ],
    }
}
