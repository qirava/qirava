//! The Qirava website — a Qirava Quill app.
//!
//! This site dogfoods Quill: it is a plain Rust binary, with two modes driven
//! from the ONE `PAGES` list and the ONE render path (`app::respond_html`), so
//! what is served and what is exported are byte-identical.
//!
//!   * `cargo run`              — open a db, register each page, and SERVE
//!                                server-rendered HTML over a worker (the default).
//!   * `cargo run -- build [out]` — render every page in-process and EXPORT a
//!                                static `dist/` (HTML per route + copied
//!                                `public/` assets + a Cloudflare Pages
//!                                `_headers`/`_redirects`/`404.html`). NO database
//!                                or socket — the output serves with no DMS.
//!
//! Every page here is pure SSR/SSG content (no islands), so the exported site
//! ships ZERO JavaScript.
//!
//! Run with `cargo run`, then open http://127.0.0.1:7179/.

use std::sync::Arc;

use qdms::engine::Qdb;
use qdms::workers::qquill::take_response_headers;
use qdms::workers::{
    ensure_system_tables, load_routes_for, system_worker, WorkerHost, SYSTEM_WORKER_ID,
};
use qexec::{FunctionResponse, FunctionScope, ResourceRequest};
use qquill_build::{
    export_static, handler_key, sys_routes_insert, ExportPage, ExportPlan, RouteDef,
};

mod app;

/// Where the site listens by default. Override with `QUILL_ADDR`,
/// e.g. `QUILL_ADDR=127.0.0.1:8080 cargo run`.
const ADDR: &str = "127.0.0.1:7179";

/// The default output directory for `cargo run -- build`.
const DEFAULT_OUT_DIR: &str = "dist";

/// The site's static-asset directory, copied verbatim into the export.
const PUBLIC_DIR: &str = "public";

/// One page: its route id, its URL path, and the handler that renders it.
///
/// A page handler takes the encoded request bytes and returns a framed HTML
/// response (see `app::respond_html`). Add a page by writing a `respond`
/// function in `app/routes/` and listing it here — that is the whole contract.
struct Page {
    id: &'static str,
    path: &'static str,
    handler: fn(&[u8]) -> FunctionResponse,
}

/// Every page this site serves AND exports. Both `serve` and `build` walk this
/// one list, so they can never drift. All routes are static (no `:id`/`*rest`),
/// so the exporter writes a file for every one.
const PAGES: &[Page] = &[
    Page { id: "index", path: "/", handler: app::routes::index::respond },
    Page { id: "products", path: "/products", handler: app::routes::products::respond },
    Page { id: "docs", path: "/docs", handler: app::routes::docs::respond },
    Page { id: "roadmap", path: "/roadmap", handler: app::routes::roadmap::respond },
];

fn main() -> std::io::Result<()> {
    // args[0] is the binary; `build [outdir]` exports a static site, anything
    // else (incl. no args) serves.
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") => cmd_build(args.get(1).map(String::as_str)),
        Some("serve") | None => cmd_serve(),
        Some(other) => {
            eprintln!("error: unknown command `{other}`");
            eprintln!("usage: qirava-site [serve | build [outdir]]");
            std::process::exit(2);
        }
    }
}

/// Render every static page in-process and write a CDN-ready `dist/`. This needs
/// NO database and NO socket: it calls each page's `respond(&[])` directly (the
/// SAME render path the server uses), strips the `__headers` response frame to
/// get the bare HTML bytes (and the page's `Cache-Control`), and hands them to
/// the `qquill-build` export helper, which owns the URL→file mapping, the
/// `public/` copy, and the Cloudflare Pages `_headers`/`_redirects`/`404.html`.
fn cmd_build(out_dir: Option<&str>) -> std::io::Result<()> {
    let out = std::path::PathBuf::from(out_dir.unwrap_or(DEFAULT_OUT_DIR));

    // Render each page exactly as the server would, then unframe to raw HTML.
    let mut pages = Vec::with_capacity(PAGES.len());
    for page in PAGES {
        // SSG pages are request-independent: an empty request record is what the
        // prerender path passes (`who` defaults to `world`).
        let resp = (page.handler)(&[]);
        let mut data = resp.data;
        // Strip the `__headers` frame: `data` becomes the pure HTML document and
        // we recover the page's `Cache-Control` for the `_headers` file.
        let headers = take_response_headers(&mut data);
        let cache_control = headers.and_then(|h| {
            h.into_iter()
                .find(|(k, _)| k.eq_ignore_ascii_case("cache-control"))
                .map(|(_, v)| v)
        });
        let route = RouteDef::page(page.id, page.path);
        pages.push(ExportPage::new(route, data).with_cache_control(cache_control));
    }

    let public = std::path::PathBuf::from(PUBLIC_DIR);
    let plan = ExportPlan {
        out_dir: &out,
        public_dir: Some(&public),
        not_found_html: None, // minimal default 404.html
    };
    let report = export_static(&plan, &pages)?;

    println!("Exported {} page(s) to {}/", report.pages_written.len(), out.display());
    for rel in &report.pages_written {
        println!("  {}/{}", out.display(), rel);
    }
    if report.assets_copied > 0 {
        println!("Copied {} asset(s) from {}/", report.assets_copied, PUBLIC_DIR);
    }
    for skipped in &report.skipped_dynamic {
        println!("  skipped dynamic route {skipped} (no static file)");
    }
    println!("Deploy {}/ to any static host — it serves with no DMS running.", out.display());
    Ok(())
}

/// Open a db, register each page's handler + route row, and serve over a worker.
fn cmd_serve() -> std::io::Result<()> {
    // 1. Open an in-memory database and ensure the framework's system tables
    //    (`_sys_routes`, `_sys_pages`, `_sys_assets`, ...).
    let db = Arc::new(Qdb::new());
    ensure_system_tables(&db);
    db.restore_catalog();

    // 2 + 3. Register each page's render handler and declare its route row.
    for page in PAGES {
        // Register the renderer under `qq.render.<id>` (a generous resource
        // budget covers the render walk).
        let _ = db.runtime().register_dynamic(
            handler_key(page.id),
            FunctionScope::Public,
            ResourceRequest::new(256 * 1024, 2048),
            false,
            {
                let handler = page.handler;
                move |_ctx, input| handler(input)
            },
        );

        // Declare the route as data: `GET <path>` -> `qq.render.<id>`, Html.
        // Idempotent so repeated boots don't duplicate the row.
        let route = RouteDef::page(page.id, page.path);
        let exists = !db
            .read_records(&format!(
                "SELECT id FROM _sys_routes WHERE id = \"{}\"",
                route.id
            ))
            .is_empty();
        if !exists {
            let _ = db.execute(&sys_routes_insert(&route, SYSTEM_WORKER_ID));
        }
    }

    // 4. Build the worker, merge in the route rows we just declared, and serve.
    let addr = std::env::var("QUILL_ADDR").unwrap_or_else(|_| ADDR.to_string());
    let host = WorkerHost::new(db.runtime().clone());
    let mut worker = system_worker(&addr);
    for route in load_routes_for(&db, SYSTEM_WORKER_ID) {
        worker = worker.route(route);
    }
    host.add_worker(worker).expect("register worker");
    let host = Arc::new(host);

    // Keep the database alive for the lifetime of the server.
    let _db = Arc::clone(&db);

    println!("qirava-site serving:");
    for page in PAGES {
        println!("  http://{addr}{}", page.path);
    }
    host.serve_blocking_reactor(SYSTEM_WORKER_ID)
}
