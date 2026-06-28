//! `GET /api` — the products API reference, sourced from the LIVE catalog.
//!
//! At BUILD/render time this page constructs a real `qdms` function runtime
//! (`qdms::functions::runtime(ExecConfig::auto())` — the same call the DMS uses
//! in production) and asks it for the served `/api/spec` document via
//! `qdms::functions::api::build_catalog(&rt)`. That JSON is the single source of
//! truth the running DMS serves at `GET /api/spec`; this page parses it back into
//! Rust and renders it as a browsable reference. There is no second, hand-kept
//! list of functions to drift — change a function's metadata in `qdms` and this
//! page changes with it.
//!
//! Rendering uses the `qquill-docs` primitives (`ApiEntry`, `Callout`,
//! `CodeBlock`, `heading`) where they fit, plus the site's own `section`/`Css`
//! shell so the page matches the rest of the docs. No islands — pure SSR/SSG,
//! zero JavaScript.

use qexec::FunctionResponse;
use qquill_docs::{ApiEntry, Callout, CodeBlock};
use qquill_view::{el, text, Node};

use crate::app::routes::section;
use crate::app::shell::page;
use crate::app::{Css, Meta};

// ---------------------------------------------------------------------------
// A tiny, purpose-built JSON reader for the catalog document.
//
// The catalog is emitted by `qdms`'s `json_escape` (escapes only `" \ \n \r \t`
// and `\u00XX` controls), so the reader needs exactly those un-escapes plus the
// container/number forms the catalog uses. It is NOT a general JSON parser — it
// reads the one document shape `build_catalog` produces, and is only ever fed
// our own first-party output.
// ---------------------------------------------------------------------------

/// A parsed JSON value from the catalog (only the forms the catalog emits).
enum Json {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn as_str(&self) -> &str {
        match self {
            Json::Str(s) => s,
            _ => "",
        }
    }
    fn as_arr(&self) -> &[Json] {
        match self {
            Json::Arr(v) => v,
            _ => &[],
        }
    }
    fn as_bool(&self) -> bool {
        matches!(self, Json::Bool(true))
    }
    fn as_u64(&self) -> u64 {
        match self {
            Json::Num(n) => *n as u64,
            _ => 0,
        }
    }
    /// Field lookup for an object value.
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
}

/// A cursor over the catalog bytes (chars).
struct Reader<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Reader<'a> {
    fn new(s: &'a str) -> Self {
        Reader {
            chars: s.chars().peekable(),
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.chars.peek(), Some(c) if c.is_whitespace()) {
            self.chars.next();
        }
    }

    fn value(&mut self) -> Json {
        self.skip_ws();
        match self.chars.peek() {
            Some('"') => Json::Str(self.string()),
            Some('{') => self.object(),
            Some('[') => self.array(),
            Some('t') | Some('f') => self.boolean(),
            Some('n') => self.null(),
            _ => self.number(),
        }
    }

    fn string(&mut self) -> String {
        self.chars.next(); // opening quote
        let mut out = String::new();
        while let Some(c) = self.chars.next() {
            match c {
                '"' => break,
                '\\' => match self.chars.next() {
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('/') => out.push('/'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some('b') => out.push('\u{0008}'),
                    Some('f') => out.push('\u{000c}'),
                    Some('u') => {
                        let mut code = 0u32;
                        for _ in 0..4 {
                            if let Some(h) = self.chars.next().and_then(|h| h.to_digit(16)) {
                                code = code * 16 + h;
                            }
                        }
                        if let Some(ch) = char::from_u32(code) {
                            out.push(ch);
                        }
                    }
                    Some(other) => out.push(other),
                    None => break,
                },
                c => out.push(c),
            }
        }
        out
    }

    fn object(&mut self) -> Json {
        self.chars.next(); // '{'
        let mut fields = Vec::new();
        loop {
            self.skip_ws();
            match self.chars.peek() {
                Some('}') => {
                    self.chars.next();
                    break;
                }
                Some(',') => {
                    self.chars.next();
                }
                Some('"') => {
                    let key = self.string();
                    self.skip_ws();
                    if self.chars.peek() == Some(&':') {
                        self.chars.next();
                    }
                    let val = self.value();
                    fields.push((key, val));
                }
                _ => break,
            }
        }
        Json::Obj(fields)
    }

    fn array(&mut self) -> Json {
        self.chars.next(); // '['
        let mut items = Vec::new();
        loop {
            self.skip_ws();
            match self.chars.peek() {
                Some(']') => {
                    self.chars.next();
                    break;
                }
                Some(',') => {
                    self.chars.next();
                }
                None => break,
                _ => items.push(self.value()),
            }
        }
        Json::Arr(items)
    }

    fn boolean(&mut self) -> Json {
        let mut word = String::new();
        while matches!(self.chars.peek(), Some(c) if c.is_ascii_alphabetic()) {
            word.push(self.chars.next().unwrap());
        }
        Json::Bool(word == "true")
    }

    fn null(&mut self) -> Json {
        while matches!(self.chars.peek(), Some(c) if c.is_ascii_alphabetic()) {
            self.chars.next();
        }
        Json::Null
    }

    fn number(&mut self) -> Json {
        let mut num = String::new();
        while matches!(self.chars.peek(), Some(c) if c.is_ascii_digit() || *c == '-' || *c == '+' || *c == '.' || *c == 'e' || *c == 'E')
        {
            num.push(self.chars.next().unwrap());
        }
        Json::Num(num.parse().unwrap_or(0.0))
    }
}

/// Parse the catalog document.
fn parse(s: &str) -> Json {
    Reader::new(s).value()
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// A prose paragraph (escaped).
fn p(s: &str) -> Node {
    el("p").child(text(s.to_string()))
}

/// A heading captured for the on-page anchor (h2 with a permalink).
fn h2(title: &str) -> Node {
    qquill_docs::heading(2, title)
}

/// A friendly title + one-line blurb for each catalog category id.
fn category_meta(id: &str) -> (&'static str, &'static str) {
    match id {
        "planner" => (
            "Planner",
            "Parse and plan a QQL statement — the only door to a read or mutate.",
        ),
        "read" => (
            "Read",
            "Key, filter, sort, limit, join, and aggregate read paths.",
        ),
        "write" => (
            "Write",
            "Insert, update, upsert, delete, schema upgrade, and shard move. All FIFO-ordered.",
        ),
        "search" => ("Search", "Full-text scoring, with an optional post-filter."),
        "graph" => (
            "Graph",
            "Resolve a node, traverse edges, and resolve a path between nodes.",
        ),
        "vector" => (
            "Vector",
            "Approximate nearest-neighbor search, optionally filtered.",
        ),
        "inline" => (
            "Inline",
            "Composed, concurrent fan-outs that span several access paths in one call.",
        ),
        "auth" => (
            "Auth & governance",
            "Sessions, signed keys, API keys, and the deny-by-default RBAC before-chain.",
        ),
        other => (Box::leak(other.to_string().into_boxed_str()), ""),
    }
}

/// Render one function's input or output field table. Returns an empty `<div>`
/// when there are no fields (e.g. a `before`-gate with no documented record).
fn field_table(label: &str, fields: &[Json]) -> Node {
    if fields.is_empty() {
        return el("div");
    }
    let mut rows = el("tbody");
    for f in fields {
        let req = if f.get("required").map(|v| v.as_bool()).unwrap_or(false) {
            "required"
        } else {
            "optional"
        };
        rows = rows.child(
            el("tr")
                .child(el("td").child(el("code").class("q-inline").child(text(
                    f.get("name").map(|v| v.as_str()).unwrap_or("").to_string(),
                ))))
                .child(el("td").child(el("code").class("q-inline").child(text(
                    f.get("type").map(|v| v.as_str()).unwrap_or("").to_string(),
                ))))
                .child(el("td").class("q-api-req").child(text(req.to_string()))),
        );
    }
    el("div")
        .class("q-api-fields")
        .child(
            el("p")
                .class("q-api-fields__label")
                .child(text(label.to_string())),
        )
        .child(
            el("table")
                .class("q-api-table")
                .child(
                    el("thead").child(
                        el("tr")
                            .child(el("th").child(text("Field".to_string())))
                            .child(el("th").child(text("Type".to_string())))
                            .child(el("th").child(text("".to_string()))),
                    ),
                )
                .child(rows),
        )
}

/// A small inline metadata chip (scope / origin / fifo).
fn chip(kind: &str, value: &str) -> Node {
    el("span")
        .class(format!("q-api-chip q-api-chip--{kind}"))
        .child(text(value.to_string()))
}

/// Render one function as a `<details>` card: a summary row (key + scope chips),
/// the description, the I/O field tables, and its error codes.
fn function_card(f: &Json) -> Node {
    let key = f.get("key").map(|v| v.as_str()).unwrap_or("");
    let scope = f.get("scope").map(|v| v.as_str()).unwrap_or("");
    let origin = f.get("origin").map(|v| v.as_str()).unwrap_or("");
    let fifo = f.get("fifo").map(|v| v.as_bool()).unwrap_or(false);
    let summary = f.get("summary").map(|v| v.as_str()).unwrap_or("");
    let description = f.get("description").map(|v| v.as_str()).unwrap_or("");

    let mut chips = el("span")
        .class("q-api-chips")
        .child(chip("scope", scope))
        .child(chip("origin", origin));
    if fifo {
        chips = chips.child(chip("fifo", "fifo"));
    }

    let head = el("summary")
        .class("q-api-card__sum")
        .child(
            el("code")
                .class("q-api-card__key")
                .child(text(key.to_string())),
        )
        .child(
            el("span")
                .class("q-api-card__summary")
                .child(text(summary.to_string())),
        )
        .child(chips);

    // Error-code badges from the function's documented subset.
    let mut codes = el("div").class("q-api-codes");
    codes = codes.child(
        el("span")
            .class("q-api-codes__label")
            .child(text("Errors:".to_string())),
    );
    for c in f.get("error_codes").map(|v| v.as_arr()).unwrap_or(&[]) {
        codes = codes.child(
            el("code")
                .class("q-inline q-api-code-pill")
                .child(text(c.as_str().to_string())),
        );
    }

    el("details").class("q-api-card").child(head).child(
        el("div")
            .class("q-api-card__body")
            .child(p(description))
            .child(field_table(
                "Input",
                f.get("input").map(|v| v.as_arr()).unwrap_or(&[]),
            ))
            .child(field_table(
                "Output",
                f.get("output").map(|v| v.as_arr()).unwrap_or(&[]),
            ))
            .child(codes),
    )
}

/// Render the error-code table from the catalog's `error_codes` array.
fn error_code_table(codes: &[Json]) -> Node {
    let mut rows = el("tbody");
    for c in codes {
        let code = c.get("code").map(|v| v.as_str()).unwrap_or("");
        let http = c.get("http").map(|v| v.as_u64()).unwrap_or(0);
        rows = rows.child(
            el("tr")
                .child(el("td").child(el("code").class("q-inline").child(text(code.to_string()))))
                .child(el("td").child(text(http.to_string()))),
        );
    }
    el("table")
        .class("q-api-table")
        .child(
            el("thead").child(
                el("tr")
                    .child(el("th").child(text("Stable code".to_string())))
                    .child(el("th").child(text("HTTP status".to_string()))),
            ),
        )
        .child(rows)
}

/// Pretty-print the envelope object shape as a JSON snippet for a CodeBlock.
fn envelope_snippet(env: &Json) -> String {
    // Render the two documented shapes from the live catalog so the snippet can
    // never drift from what the server emits.
    let render = |v: &Json| -> String {
        match v {
            Json::Obj(fields) => {
                let inner: Vec<String> = fields
                    .iter()
                    .map(|(k, val)| format!("    \"{k}\": {}", scalar(val)))
                    .collect();
                format!("{{\n{}\n  }}", inner.join(",\n").replace('\n', "\n  "))
            }
            other => scalar(other),
        }
    };
    let ok = env.get("ok").map(render).unwrap_or_default();
    let err = env.get("error").map(render).unwrap_or_default();
    format!("// success\n{ok}\n\n// error\n{err}")
}

/// Render a scalar/leaf catalog value as a JSON literal for the snippet.
fn scalar(v: &Json) -> String {
    match v {
        Json::Str(s) => format!("\"{s}\""),
        Json::Num(n) => n.to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Null => "null".to_string(),
        Json::Obj(fields) => {
            let inner: Vec<String> = fields
                .iter()
                .map(|(k, val)| format!("\"{k}\": {}", scalar(val)))
                .collect();
            format!("{{ {} }}", inner.join(", "))
        }
        Json::Arr(items) => {
            let inner: Vec<String> = items.iter().map(scalar).collect();
            format!("[{}]", inner.join(", "))
        }
    }
}

// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

const LEAD: &str = "Every Qirava data operation is a small, named function reachable over one HTTP \
surface. This reference is generated from the running DMS itself: at build time the site constructs \
a real qdms runtime and reads its served /api/spec catalog, so what you see here is exactly what the \
server answers — there is no second list to drift.";

pub fn respond(_input: &[u8]) -> FunctionResponse {
    let mut css = Css::new();
    // Pull in the qquill-docs content-primitive CSS (`.qq-api`, `.qq-callout`,
    // `.qq-heading`, `.qq-code`) plus this page's own table/card CSS.
    css.push(qquill_docs::layout_css().to_css());
    css.push(api_css().to_string());

    // SOURCE OF TRUTH: build a live runtime and read the served catalog. This is
    // the same `runtime(ExecConfig::auto())` the DMS boots with; `build_catalog`
    // is the body of `GET /api/spec`.
    let rt = qdms::functions::runtime(qdms::ExecConfig::auto())
        .expect("qdms runtime for the API catalog");
    let spec_json = qdms::functions::api::build_catalog(&rt);
    let catalog = parse(&spec_json);

    let version = catalog
        .get("qirava_api_version")
        .map(|v| v.as_str())
        .unwrap_or("1");

    let intro = el("div")
        .child(
            el("p")
                .class("q-eyebrow")
                .child(text("Reference".to_string())),
        )
        .child(
            el("h1")
                .class("q-h1")
                .child(text("Products API".to_string())),
        )
        .child(el("p").class("q-lead").child(text(LEAD.to_string())));

    // --- The HTTP surface ---
    let surface = section(
        Some("Surface"),
        "The HTTP surface",
        "Two discovery endpoints describe the whole API; every operation below is invoked through the \
         single execute path behind a worker. The catalog is generated once at boot and served \
         verbatim — it reads no row data and reflects no request body.",
        el("div")
            .child(ApiEntry::new("GET", "/api/spec", "The live JSON catalog this page is built from.").render())
            .child(ApiEntry::new("GET", "/api/spec/openapi", "The same catalog as an OpenAPI 3.1 document.").render())
            .child(
                Callout::note(p(
                    "Wire version is captured in the catalog as qirava_api_version. This reference was \
                     generated from version 1 of the spec.",
                ))
                .render(),
            ),
    );

    // --- The envelope ---
    let envelope = catalog.get("envelope");
    let envelope_section = section(
        Some("Contract"),
        "The response envelope",
        "Every call returns the same uniform envelope: a success carries data, an error carries a \
         stable code and message, and both carry timing and a request id in root.",
        envelope
            .map(|env| {
                el("div").child(
                    CodeBlock::new("json", envelope_snippet(env))
                        .id("api-envelope")
                        .render(),
                )
            })
            .unwrap_or_else(|| el("div")),
    );

    // --- The error codes ---
    let codes = catalog
        .get("error_codes")
        .map(|v| v.as_arr())
        .unwrap_or(&[]);
    let errors_section = section(
        Some("Contract"),
        "Stable error codes",
        "There are eight stable error codes, each mapped to one HTTP status. They are the single \
         source of truth for failure handling — every function documents the subset it can return.",
        el("div")
            .class("q-api-table-wrap")
            .child(error_code_table(codes)),
    );

    // --- The catalog of functions, by category ---
    let mut categories_body = el("div").class("q-api-cats");
    for cat in catalog.get("categories").map(|v| v.as_arr()).unwrap_or(&[]) {
        let id = cat.get("name").map(|v| v.as_str()).unwrap_or("");
        let (title, blurb) = category_meta(id);
        let mut group = el("div").class("q-api-cat").child(h2(title));
        if !blurb.is_empty() {
            group = group.child(p(blurb));
        }
        let mut list = el("div").class("q-api-cardlist");
        for f in cat.get("functions").map(|v| v.as_arr()).unwrap_or(&[]) {
            list = list.child(function_card(f));
        }
        group = group.child(list);
        categories_body = categories_body.child(group);
    }

    let catalog_section = section(
        Some("Catalog"),
        "Functions",
        "The complete function catalog, grouped by category. Each entry shows its key, access scope, \
         origin, FIFO ordering, input and output fields, and the error codes it may return. Expand a \
         row for the full detail.",
        categories_body,
    );

    // --- Studio + Cloud ---
    let studio = section(
        Some("Apps"),
        "Studio — the system app",
        "Qirava Studio is the built-in admin app: schema and data browsing, user and grant \
         management, and API-key minting. It is an ordinary DMS client with no backdoor — every \
         action it takes goes through the same execute → worker → planner path and the same three \
         authorization checkpoints as any caller.",
        Callout::tip(p(
            "Because Studio uses the public API exclusively, anything Studio can do, your own app can \
             do too — the catalog above is the whole surface.",
        ))
        .render(),
    );

    let cloud = section(
        Some("Planned"),
        "Cloud — managed Qirava",
        "Managed Qirava Cloud is planned: the same open-core engine and API, run for you with \
         single-leader replication, custodian-gated onboarding, and hardware-attested key custody. \
         The wire API is identical to self-hosted — the catalog on this page is the contract either \
         way.",
        Callout::note(p(
            "Cloud is on the roadmap, not yet shipped. The self-hosted DMS is the supported way to run \
             Qirava today.",
        ))
        .render(),
    );

    let _ = version; // documented inline above; kept for clarity of provenance.

    let content = el("main")
        .class("q-main")
        .id("main")
        .child(intro)
        .child(surface)
        .child(envelope_section)
        .child(errors_section)
        .child(catalog_section)
        .child(studio)
        .child(cloud);

    let meta = Meta {
        title: "Products API — Qirava",
        description:
            "The Qirava products API reference, generated from the live /api/spec catalog: \
                      every function with its scope, inputs, outputs, and error codes, plus the \
                      response envelope and the eight stable error codes.",
        path: "/api",
    };
    page(&meta, css, content)
}

/// Page-local CSS for the API reference tables and function cards. Uses theme
/// tokens only, so it restyles on a theme switch with no reflow.
fn api_css() -> &'static str {
    "\
.q-api-table-wrap{overflow-x:auto}\
.q-api-table{width:100%;border-collapse:collapse;font-size:.92rem;margin:.5rem 0}\
.q-api-table th{text-align:left;color:var(--q-color-muted);font-weight:var(--q-font-weight-medium);font-size:.8rem;text-transform:uppercase;letter-spacing:.06em;padding:.4rem .75rem;border-bottom:1px solid var(--q-color-border)}\
.q-api-table td{padding:.4rem .75rem;border-bottom:1px solid var(--q-color-border);vertical-align:top}\
.q-api-req{color:var(--q-color-muted);font-size:.85rem}\
.q-api-cats{display:flex;flex-direction:column;gap:2.5rem}\
.q-api-cat>p{color:var(--q-color-muted);margin:.25rem 0 1rem}\
.q-api-cardlist{display:flex;flex-direction:column;gap:.6rem}\
.q-api-card{border:1px solid var(--q-surface-border,var(--q-color-border));border-radius:var(--q-radius-lg);background:var(--q-surface-bg,var(--q-color-surface));box-shadow:var(--q-surface-shadow,none);-webkit-backdrop-filter:var(--q-surface-filter,none);backdrop-filter:var(--q-surface-filter,none);overflow:hidden}\
.q-api-card[open]{border-color:color-mix(in srgb,var(--q-color-brand) 40%,var(--q-color-border))}\
.q-api-card__sum{display:flex;align-items:center;flex-wrap:wrap;gap:.6rem;padding:.8rem 1rem;cursor:pointer;list-style:none}\
.q-api-card__sum::-webkit-details-marker{display:none}\
.q-api-card__sum::before{content:\"\\203A\";color:var(--q-color-muted);transition:transform var(--q-duration-fast) var(--q-ease-out);font-size:1.1rem;line-height:1}\
.q-api-card[open] .q-api-card__sum::before{transform:rotate(90deg)}\
.q-api-card__key{font-weight:var(--q-font-weight-bold);color:var(--q-color-fg)}\
.q-api-card__summary{color:var(--q-color-muted);flex:1 1 16ch;min-width:12ch}\
.q-api-chips{display:inline-flex;gap:.35rem;flex-wrap:wrap}\
.q-api-chip{font-size:.72rem;font-weight:var(--q-font-weight-medium);padding:.1rem .45rem;border-radius:var(--q-radius-full);border:1px solid var(--q-color-border);color:var(--q-color-muted);text-transform:uppercase;letter-spacing:.04em}\
.q-api-chip--scope{color:var(--q-color-brand);border-color:color-mix(in srgb,var(--q-color-brand) 35%,transparent)}\
.q-api-chip--fifo{color:var(--q-color-fg)}\
.q-api-card__body{padding:0 1rem 1rem;border-top:1px solid var(--q-color-border)}\
.q-api-card__body>p{margin:.9rem 0}\
.q-api-fields{margin:.75rem 0}\
.q-api-fields__label{color:var(--q-color-fg);font-weight:var(--q-font-weight-bold);font-size:.82rem;text-transform:uppercase;letter-spacing:.06em;margin:0 0 .25rem}\
.q-api-codes{display:flex;align-items:center;flex-wrap:wrap;gap:.4rem;margin-top:.9rem}\
.q-api-codes__label{color:var(--q-color-muted);font-size:.85rem}\
.q-api-code-pill{font-size:.82rem}"
}
