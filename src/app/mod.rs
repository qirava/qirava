//! The app: shared chrome (`shell`), the theme (`theme`), and the page routes.
//!
//! Each page in `routes/` builds its `<body>` content as a `(Node, css)` pair —
//! the rendered tree plus the CSS its styled Quill components carry — and hands
//! both to [`shell::page`], which wraps them in the site chrome (header nav +
//! footer) and a full `<html>` document, collects *all* the component CSS into a
//! single `<head>` `<style>`, and frames the HTTP response via [`respond_html`].
//!
//! ## Why pages return CSS, not just a Node
//!
//! Unlike the bare scaffold, this site uses the styled `qquill-design`
//! components (`Navbar`, `Card`, `Button`, `Badge`, `Stat`, ...). Each one
//! returns a [`Styled`](qquill_design::Styled) = `(Node, StyleBlock)`: the
//! component *carries its own CSS*. So `document()` cannot rely on the theme CSS
//! alone — it must gather every component's `StyleBlock` and concatenate it into
//! the head. The [`Css`] accumulator below is that collector; pages push each
//! component's `.style().to_css()` into it as they build their tree.

pub mod design;
pub mod docs_kit;
pub mod routes;
pub mod shell;
pub mod site_ui;
pub mod theme;

use qquill_design::Styled;
use qquill_view::{el, raw, render_into, text, Node};

/// A small CSS accumulator. Pages render styled components into their tree and
/// push each component's companion CSS here; the deduped, concatenated string is
/// inlined once into the document `<head>` by [`document`].
///
/// Dedup is by exact CSS-string identity — every instance of a given component
/// variant emits the same rules, so collecting them in a set keeps the sheet
/// small without an atomic-CSS pass.
#[derive(Default)]
pub struct Css {
    seen: Vec<String>,
}

impl Css {
    pub fn new() -> Self {
        Css { seen: Vec::new() }
    }

    /// Render a styled component into a `Node`, recording its CSS. Returns the
    /// node so call sites read naturally: `.child(css.node(Button::action("Go").render()))`.
    pub fn node(&mut self, styled: Styled) -> Node {
        let (node, block) = styled.into_parts();
        self.push(block.to_css());
        node
    }

    /// Record a raw CSS string (e.g. a page's own `style!{}` block).
    pub fn push(&mut self, css: String) {
        if !css.is_empty() && !self.seen.iter().any(|c| c == &css) {
            self.seen.push(css);
        }
    }

    /// The concatenated, deduped CSS for the `<head>`.
    pub fn into_css(self) -> String {
        self.seen.concat()
    }
}

/// One SEO-relevant page identity passed to [`document`].
pub struct Meta<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub path: &'a str,
}

/// A longer browser-cache window suits a marketing site whose content changes
/// rarely (seconds).
const CACHE_CONTROL: &str = "public, max-age=300";

/// The canonical origin used to build absolute canonical + Open Graph URLs.
const SITE_ORIGIN: &str = "https://qirava.in";

/// A tiny pre-paint script that restores the non-color appearance axes from
/// `localStorage` BEFORE first paint, so a returning visitor's stored density /
/// radius / surface / accent / motion choice is applied with no flash and no
/// jump when the `theme-control` island later hydrates (the flicker fix). It
/// only WRITES validated attributes onto `<html>` and reads (never writes)
/// storage; an unknown/missing value leaves the CSS default in place. Kept terse
/// and dependency-free, mirroring `qquill_theme::boot_script`.
fn axis_boot_script() -> &'static str {
    // For each [attr,key,allowed-regex] tuple: read the stored value, validate,
    // and set the attribute only when valid. `e` is <html>.
    "(function(){try{var e=document.documentElement,L=localStorage,m=[\
['data-q-size','q-size',/^(?:compact|cozy|comfortable)$/],\
['data-q-radius','q-radius',/^(?:sharp|rounded|pill)$/],\
['data-q-surface','q-surface',/^(?:flat|glass|neu|gradient)$/],\
['data-q-accent','q-accent',/^(?:azure|violet|emerald|amber|rose)$/],\
['data-q-motion','q-motion',/^(?:smooth|snappy|playful|none)$/]];\
for(var i=0;i<m.length;i++){var v=L.getItem(m[i][1]);if(v&&m[i][2].test(v))e.setAttribute(m[i][0],v);}}catch(_){}})();"
}

/// The axis boot wrapped in an inline `<script>` for `<head>` (runs pre-paint).
fn axis_boot_script_tag() -> String {
    let mut s = String::from("<script>");
    s.push_str(axis_boot_script());
    s.push_str("</script>");
    s
}

/// Wrap a page's `<body>` content in a full HTML document.
///
/// `head_css` is the collected component + page CSS (from a [`Css`] accumulator)
/// that is inlined *after* the theme variables so component rules can reference
/// the theme tokens. The theme's no-flicker boot script runs first so the
/// correct `data-q-theme` is set before paint.
pub fn document(meta: &Meta, head_css: String, body: Node) -> Node {
    // The full stylesheet: theme variable blocks (the dark/light contract) +
    // the site layout + the collected component CSS + the a11y media resets.
    let themes = qquill_theme::default_theme_set();
    let boot = qquill_theme::BootConfig::new(qquill_theme::ThemeMode::Dark);

    let mut css = themes.to_css();
    // The Qirava brand palette (Ink + Azure) re-points the theme's default tokens
    // immediately after the theme block, so every `var(--q-color-*)` downstream
    // resolves to the brand identity. Generated from the single config in `design`.
    css.push_str(&design::palette_css());
    // The accent (color-picker) axis re-points the brand family per chosen
    // accent, for both light + dark. Emitted after the base palette so it wins.
    css.push_str(&design::accent_css());
    // The motion (animation) axis re-points the duration/easing tokens + the
    // press-feedback scale every component reads.
    css.push_str(&design::motion_axis_css());
    css.push_str(&theme::layout_css());
    css.push_str(site_ui::css());
    css.push_str(&head_css);
    css.push_str(&qquill_design::reduced_motion_css());
    css.push_str(&qquill_design::reduced_transparency_css());

    // The concrete bg the page paints at boot, for the chrome theme-color meta.
    let theme_color = themes
        .resolve(boot.mode(), qquill_theme::Token::color("bg"))
        .or_else(|| themes.base().get(qquill_theme::Token::color("bg")))
        .unwrap_or("#0b0c10")
        .to_string();

    let canonical = format!("{SITE_ORIGIN}{}", meta.path);

    let head = el("head")
        .child(el("meta").attr("charset", "utf-8"))
        .child(
            el("meta")
                .attr("name", "viewport")
                .attr("content", "width=device-width, initial-scale=1"),
        )
        .child(el("title").child(text(meta.title.to_string())))
        .child(
            el("meta")
                .attr("name", "description")
                .attr("content", meta.description.to_string()),
        )
        .child(
            el("link")
                .attr("rel", "icon")
                .attr("type", "image/svg+xml")
                .attr("href", "/favicon.svg"),
        )
        .child(
            el("link")
                .attr("rel", "manifest")
                .attr("href", "/manifest.webmanifest"),
        )
        .child(
            el("meta")
                .attr("name", "theme-color")
                .attr("content", theme_color),
        )
        .child(
            el("link")
                .attr("rel", "canonical")
                .attr("href", canonical.clone()),
        )
        // Open Graph: rich link previews.
        .child(
            el("meta")
                .attr("property", "og:type")
                .attr("content", "website"),
        )
        .child(
            el("meta")
                .attr("property", "og:title")
                .attr("content", meta.title.to_string()),
        )
        .child(
            el("meta")
                .attr("property", "og:description")
                .attr("content", meta.description.to_string()),
        )
        .child(
            el("meta")
                .attr("property", "og:url")
                .attr("content", canonical),
        )
        .child(
            el("meta")
                .attr("property", "og:site_name")
                .attr("content", "Qirava"),
        )
        // The no-flicker theme boot MUST run before the stylesheet so the correct
        // `data-q-theme` is set pre-paint.
        .child(raw(qquill_theme::boot_script_tag(&boot)))
        // The site axis boot: restore the OTHER appearance axes (density, radius,
        // surface, accent, motion) pre-paint too, so a stored choice is applied
        // before first paint with NO flicker/jump when the island later hydrates.
        .child(raw(axis_boot_script_tag()))
        // Critical CSS inlined (theme vars + layout + components). The compiled
        // CSS is trusted (we generated it) -> Raw.
        .child(el("style").child(Node::Raw(css.into())));

    el("html")
        .attr("lang", "en")
        // Opt into the Quill client router: same-origin links navigate by swapping
        // #main (+ title + inlined CSS) instead of a full page reload.
        .attr("data-q-router", "")
        .child(head)
        .child(el("body").child(body))
}

/// Render a full document `tree` to a framed `text/html` response.
///
/// These pages contain no islands, so [`page_has_islands`] is false and zero
/// JavaScript is injected — the site ships no runtime. The bytes are prefixed
/// with `<!doctype html>` and framed with a `Cache-Control` header so the worker
/// serves them as native HTML.
pub fn respond_html(tree: &Node) -> qexec::FunctionResponse {
    let mut html = String::from("<!doctype html>");
    render_into(tree, &mut html);

    // Ship the runtime on EVERY page: the client router must be present
    // everywhere, and after a navigation any incoming page's islands must be able
    // to hydrate — so we ship the full bundle rather than per-page subsets. (This
    // site opts into smooth navigation; the per-page zero-JS path still exists in
    // the framework for apps that don't.)
    let mut payload = String::new();
    qquill_view::collect_island_sidecars_into(tree, &mut payload);
    payload.push_str("<script>");
    payload.push_str(&qquill_runtime::runtime_bundle());
    payload.push_str("</script>");
    if let Some(pos) = html.rfind("</body>") {
        html.insert_str(pos, &payload);
    } else {
        html.push_str(&payload);
    }

    let headers = vec![("Cache-Control".to_string(), CACHE_CONTROL.to_string())];
    qexec::FunctionResponse::ok(qdms::workers::qquill::frame_response(
        html.as_bytes(),
        &headers,
    ))
}
