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

/// How a page renders. `Fn` is a bespoke handler (marketing/components/api);
/// `Doc` is the data-driven docs renderer, dispatched by the page's `path` against
/// the authored content in `routes::docs_content`.
#[derive(Clone, Copy)]
enum Render {
    Fn(fn(&[u8]) -> FunctionResponse),
    Doc,
}

/// One page: its route id, its URL path, and how it renders.
struct Page {
    id: &'static str,
    path: &'static str,
    render: Render,
}

impl Page {
    /// Render this page to a framed HTML response (the SSG/serve render path).
    fn render(&self, input: &[u8]) -> FunctionResponse {
        match self.render {
            Render::Fn(f) => f(input),
            Render::Doc => app::routes::docs::render_doc(self.path),
        }
    }
}

/// Every page this site serves AND exports. Both `serve` and `build` walk this
/// one list, so they can never drift. All routes are static (no `:id`/`*rest`),
/// so the exporter writes a file for every one.
const PAGES: &[Page] = &[
    Page { id: "index", path: "/", render: Render::Fn(app::routes::index::respond) },
    Page { id: "products", path: "/products", render: Render::Fn(app::routes::products::respond) },
    Page { id: "products-dms", path: "/products/dms", render: Render::Fn(app::routes::product_dms::respond) },
    Page { id: "products-quill", path: "/products/quill", render: Render::Fn(app::routes::product_quill::respond) },
    Page { id: "products-stdlib", path: "/products/stdlib", render: Render::Fn(app::routes::product_stdlib::respond) },
    Page { id: "products-cloud", path: "/products/cloud", render: Render::Fn(app::routes::product_cloud::respond) },
    Page { id: "roadmap", path: "/roadmap", render: Render::Fn(app::routes::roadmap::respond) },
    Page { id: "roadmap-dms", path: "/roadmap/dms", render: Render::Fn(app::routes::roadmap_dms::respond) },
    Page { id: "roadmap-quill", path: "/roadmap/quill", render: Render::Fn(app::routes::roadmap_quill::respond) },
    Page { id: "roadmap-stdlib", path: "/roadmap/stdlib", render: Render::Fn(app::routes::roadmap_stdlib::respond) },
    Page { id: "roadmap-cloud", path: "/roadmap/cloud", render: Render::Fn(app::routes::roadmap_cloud::respond) },
    // Docs hub + the data-driven per-product manual (content in routes::docs_content).
    Page { id: "docs", path: "/docs", render: Render::Fn(app::routes::docs::respond) },
    Page { id: "docs-dms", path: "/docs/dms", render: Render::Doc },
    Page { id: "docs-dms-install", path: "/docs/dms/install", render: Render::Doc },
    Page { id: "docs-dms-quick-start", path: "/docs/dms/quick-start", render: Render::Doc },
    Page { id: "docs-dms-configuration", path: "/docs/dms/configuration", render: Render::Doc },
    Page { id: "docs-dms-tuning", path: "/docs/dms/tuning", render: Render::Doc },
    Page { id: "docs-dms-concepts", path: "/docs/dms/concepts", render: Render::Doc },
    Page { id: "docs-dms-access-model-overview", path: "/docs/dms/access-model-overview", render: Render::Doc },
    Page { id: "docs-dms-worker-pipeline", path: "/docs/dms/worker-pipeline", render: Render::Doc },
    Page { id: "docs-dms-execute-model", path: "/docs/dms/execute-model", render: Render::Doc },
    Page { id: "docs-dms-architecture-overview", path: "/docs/dms/architecture-overview", render: Render::Doc },
    Page { id: "docs-dms-architecture-security", path: "/docs/dms/architecture-security", render: Render::Doc },
    Page { id: "docs-dms-embedded-and-sync", path: "/docs/dms/embedded-and-sync", render: Render::Doc },
    Page { id: "docs-dms-qql-basics", path: "/docs/dms/qql-basics", render: Render::Doc },
    Page { id: "docs-dms-qql-reading-filters", path: "/docs/dms/qql-reading-filters", render: Render::Doc },
    Page { id: "docs-dms-qql-reading-streaming", path: "/docs/dms/qql-reading-streaming", render: Render::Doc },
    Page { id: "docs-dms-qql-reading-sort-index", path: "/docs/dms/qql-reading-sort-index", render: Render::Doc },
    Page { id: "docs-dms-qql-reading-joins", path: "/docs/dms/qql-reading-joins", render: Render::Doc },
    Page { id: "docs-dms-qql-search-inverted", path: "/docs/dms/qql-search-inverted", render: Render::Doc },
    Page { id: "docs-dms-qql-graph-traverse", path: "/docs/dms/qql-graph-traverse", render: Render::Doc },
    Page { id: "docs-dms-qql-vector-ann", path: "/docs/dms/qql-vector-ann", render: Render::Doc },
    Page { id: "docs-dms-qql-writing-acid", path: "/docs/dms/qql-writing-acid", render: Render::Doc },
    Page { id: "docs-dms-qql-ddl-tables", path: "/docs/dms/qql-ddl-tables", render: Render::Doc },
    Page { id: "docs-dms-qql-ddl-indexes", path: "/docs/dms/qql-ddl-indexes", render: Render::Doc },
    Page { id: "docs-dms-qql-return-shaping", path: "/docs/dms/qql-return-shaping", render: Render::Doc },
    Page { id: "docs-dms-qql-batch", path: "/docs/dms/qql-batch", render: Render::Doc },
    Page { id: "docs-dms-qql-plan-cache", path: "/docs/dms/qql-plan-cache", render: Render::Doc },
    Page { id: "docs-dms-qql-ttl-sweep", path: "/docs/dms/qql-ttl-sweep", render: Render::Doc },
    Page { id: "docs-dms-qql-wal-recovery", path: "/docs/dms/qql-wal-recovery", render: Render::Doc },
    Page { id: "docs-dms-session-tokens-lifecycle", path: "/docs/dms/session-tokens-lifecycle", render: Render::Doc },
    Page { id: "docs-dms-hmac-signed-api-keys", path: "/docs/dms/hmac-signed-api-keys", render: Render::Doc },
    Page { id: "docs-dms-api-keys-minting-rotation", path: "/docs/dms/api-keys-minting-rotation", render: Render::Doc },
    Page { id: "docs-dms-rbac-roles-onboarding", path: "/docs/dms/rbac-roles-onboarding", render: Render::Doc },
    Page { id: "docs-dms-table-level-rbac-grants", path: "/docs/dms/table-level-rbac-grants", render: Render::Doc },
    Page { id: "docs-dms-response-envelope", path: "/docs/dms/response-envelope", render: Render::Doc },
    Page { id: "docs-dms-api-spec-catalog", path: "/docs/dms/api-spec-catalog", render: Render::Doc },
    Page { id: "docs-dms-openapi-projection", path: "/docs/dms/openapi-projection", render: Render::Doc },
    Page { id: "docs-dms-system-catalogs", path: "/docs/dms/system-catalogs", render: Render::Doc },
    Page { id: "docs-dms-scheduler-jobs", path: "/docs/dms/scheduler-jobs", render: Render::Doc },
    Page { id: "docs-dms-studio-overview", path: "/docs/dms/studio-overview", render: Render::Doc },
    Page { id: "docs-dms-studio-authentication", path: "/docs/dms/studio-authentication", render: Render::Doc },
    Page { id: "docs-dms-studio-rbac", path: "/docs/dms/studio-rbac", render: Render::Doc },
    Page { id: "docs-dms-studio-ui-architecture", path: "/docs/dms/studio-ui-architecture", render: Render::Doc },
    Page { id: "docs-quill", path: "/docs/quill", render: Render::Doc },
    Page { id: "docs-quill-installation", path: "/docs/quill/installation", render: Render::Doc },
    Page { id: "docs-quill-project-structure", path: "/docs/quill/project-structure", render: Render::Doc },
    Page { id: "docs-quill-cli-scaffolding", path: "/docs/quill/cli-scaffolding", render: Render::Doc },
    Page { id: "docs-quill-view-authoring", path: "/docs/quill/view-authoring", render: Render::Doc },
    Page { id: "docs-quill-styling-css", path: "/docs/quill/styling-css", render: Render::Doc },
    Page { id: "docs-quill-design-tokens-theming", path: "/docs/quill/design-tokens-theming", render: Render::Doc },
    Page { id: "docs-quill-components-ui", path: "/docs/quill/components-ui", render: Render::Doc },
    Page { id: "docs-quill-styled-components", path: "/docs/quill/styled-components", render: Render::Doc },
    Page { id: "docs-quill-components", path: "/docs/quill/components", render: Render::Doc },
    Page { id: "docs-quill-islands-hydration", path: "/docs/quill/islands-hydration", render: Render::Doc },
    Page { id: "docs-quill-signals", path: "/docs/quill/signals", render: Render::Doc },
    Page { id: "docs-quill-client-runtime", path: "/docs/quill/client-runtime", render: Render::Doc },
    Page { id: "docs-quill-static-export-ssg", path: "/docs/quill/static-export-ssg", render: Render::Doc },
    Page { id: "docs-quill-examples-patterns", path: "/docs/quill/examples-patterns", render: Render::Doc },
    Page { id: "docs-stdlib", path: "/docs/stdlib", render: Render::Doc },
    Page { id: "docs-stdlib-substrate", path: "/docs/stdlib/substrate", render: Render::Doc },
    Page { id: "docs-stdlib-dependencies", path: "/docs/stdlib/dependencies", render: Render::Doc },
    Page { id: "docs-stdlib-qarray", path: "/docs/stdlib/qarray", render: Render::Doc },
    Page { id: "docs-stdlib-qobject", path: "/docs/stdlib/qobject", render: Render::Doc },
    Page { id: "docs-stdlib-qstring", path: "/docs/stdlib/qstring", render: Render::Doc },
    Page { id: "docs-stdlib-qmath", path: "/docs/stdlib/qmath", render: Render::Doc },
    Page { id: "docs-stdlib-qnumber", path: "/docs/stdlib/qnumber", render: Render::Doc },
    Page { id: "docs-stdlib-qconvert", path: "/docs/stdlib/qconvert", render: Render::Doc },
    Page { id: "docs-stdlib-qencoding", path: "/docs/stdlib/qencoding", render: Render::Doc },
    Page { id: "docs-stdlib-qcrypto", path: "/docs/stdlib/qcrypto", render: Render::Doc },
    Page { id: "docs-stdlib-qregex", path: "/docs/stdlib/qregex", render: Render::Doc },
    Page { id: "docs-stdlib-qtime", path: "/docs/stdlib/qtime", render: Render::Doc },
    Page { id: "docs-stdlib-quuid", path: "/docs/stdlib/quuid", render: Render::Doc },
    Page { id: "docs-cloud", path: "/docs/cloud", render: Render::Doc },
    Page { id: "docs-cloud-control-plane-model", path: "/docs/cloud/control-plane-model", render: Render::Doc },
    Page { id: "docs-cloud-orchestration-functions", path: "/docs/cloud/orchestration-functions", render: Render::Doc },
    Page { id: "docs-cloud-placement-binpack", path: "/docs/cloud/placement-binpack", render: Render::Doc },
    Page { id: "docs-cloud-architecture", path: "/docs/cloud/architecture", render: Render::Doc },
    Page { id: "docs-cloud-scaling-vertical", path: "/docs/cloud/scaling-vertical", render: Render::Doc },
    Page { id: "docs-cloud-scaling-horizontal", path: "/docs/cloud/scaling-horizontal", render: Render::Doc },
    Page { id: "docs-cloud-mode-switching", path: "/docs/cloud/mode-switching", render: Render::Doc },
    Page { id: "docs-cloud-scaling-architecture", path: "/docs/cloud/scaling-architecture", render: Render::Doc },
    Page { id: "docs-cloud-suspension-termination", path: "/docs/cloud/suspension-termination", render: Render::Doc },
    Page { id: "docs-cloud-billing-metering", path: "/docs/cloud/billing-metering", render: Render::Doc },
    Page { id: "docs-cloud-console-ui", path: "/docs/cloud/console-ui", render: Render::Doc },
    Page { id: "docs-cloud-rbac-enforcement", path: "/docs/cloud/rbac-enforcement", render: Render::Doc },
    Page { id: "docs-cloud-audit-trail", path: "/docs/cloud/audit-trail", render: Render::Doc },
    Page { id: "docs-cloud-built-vs-planned", path: "/docs/cloud/built-vs-planned", render: Render::Doc },
    // Component showcase (Phase 3b moves these under /docs/quill/components).
    Page { id: "components", path: "/components", render: Render::Fn(app::routes::components::respond) },
    Page { id: "components-button", path: "/components/button", render: Render::Fn(app::routes::components::respond_button) },
    Page { id: "components-badge", path: "/components/badge", render: Render::Fn(app::routes::components::respond_badge) },
    Page { id: "components-card", path: "/components/card", render: Render::Fn(app::routes::components::respond_card) },
    Page { id: "components-tabs", path: "/components/tabs", render: Render::Fn(app::routes::components::respond_tabs) },
    Page { id: "components-alert", path: "/components/alert", render: Render::Fn(app::routes::components::respond_alert) },
    Page { id: "components-stat", path: "/components/stat", render: Render::Fn(app::routes::components::respond_stat) },
    Page { id: "components-list", path: "/components/list", render: Render::Fn(app::routes::components::respond_list) },
    Page { id: "components-divider", path: "/components/divider", render: Render::Fn(app::routes::components::respond_divider) },
    Page { id: "components-breadcrumb", path: "/components/breadcrumb", render: Render::Fn(app::routes::components::respond_breadcrumb) },
    Page { id: "components-dialog", path: "/components/dialog", render: Render::Fn(app::routes::components::respond_dialog) },
    Page { id: "components-menu", path: "/components/menu", render: Render::Fn(app::routes::components::respond_menu) },
    Page { id: "components-tooltip", path: "/components/tooltip", render: Render::Fn(app::routes::components::respond_tooltip) },
    Page { id: "components-checkbox", path: "/components/checkbox", render: Render::Fn(app::routes::components::respond_checkbox) },
    Page { id: "components-switch", path: "/components/switch", render: Render::Fn(app::routes::components::respond_switch) },
    Page { id: "components-accordion", path: "/components/accordion", render: Render::Fn(app::routes::components::respond_accordion) },
    Page { id: "api", path: "/api", render: Render::Fn(app::routes::api::respond) },
];

/// Write a **theme-aware favicon** to `dir/favicon.svg`, overwriting the flat one
/// `qbrand::write_assets` emits. It is the brand mark with an embedded
/// `prefers-color-scheme` media query: the fill follows the OS/browser theme —
/// ink (`#0c1e3c`) on light, near-white (`#f8fafc`) on dark — so the mark stays
/// visible in dark browser chrome instead of disappearing. A CSS `[fill]` rule
/// overrides the inline presentation attribute on the mark's `<g>`.
fn write_smart_favicon(dir: &std::path::Path) -> std::io::Result<()> {
    const STYLE: &str = "<style>:root{--qf:#0c1e3c}@media (prefers-color-scheme:dark){:root{--qf:#f8fafc}}[fill]{fill:var(--qf)}</style>";
    let icon = qbrand::ICON_SVG;
    let smart = match icon.find('>') {
        // Insert the style block right after the opening `<svg …>` tag.
        Some(i) => format!("{}{}{}", &icon[..=i], STYLE, &icon[i + 1..]),
        None => icon.to_string(),
    };
    std::fs::write(dir.join("favicon.svg"), smart)
}

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

    // Brand assets come from the ONE source (the qbrand crate), never a
    // hand-copied SVG. Write the canonical icon/logo lockups + favicon.svg into
    // `public/` so the export (which copies `public/`) ships them verbatim.
    qbrand::write_assets(std::path::Path::new(PUBLIC_DIR))?;
    write_smart_favicon(std::path::Path::new(PUBLIC_DIR))?;

    // Render each page exactly as the server would, then unframe to raw HTML.
    let mut pages = Vec::with_capacity(PAGES.len());
    for page in PAGES {
        // SSG pages are request-independent: an empty request record is what the
        // prerender path passes (`who` defaults to `world`).
        let resp = page.render(&[]);
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
    // Brand assets from qbrand (the single source) into `public/` so the served
    // favicon + lockups match the exported site exactly.
    qbrand::write_assets(std::path::Path::new(PUBLIC_DIR))?;
    write_smart_favicon(std::path::Path::new(PUBLIC_DIR))?;

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
            move |_ctx, input| page.render(input),
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
