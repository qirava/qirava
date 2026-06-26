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
//! Unlike the bare scaffold, this site uses the styled `tqquill-design`
//! components (`Navbar`, `Card`, `Button`, `Badge`, `Stat`, ...). Each one
//! returns a [`Styled`](tqquill_design::Styled) = `(Node, StyleBlock)`: the
//! component *carries its own CSS*. So `document()` cannot rely on the theme CSS
//! alone — it must gather every component's `StyleBlock` and concatenate it into
//! the head. The [`Css`] accumulator below is that collector; pages push each
//! component's `.style().to_css()` into it as they build their tree.

pub mod routes;
pub mod shell;
pub mod theme;

use tqquill_design::Styled;
use tqquill_view::{el, raw, render_into, text, Node};

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
const SITE_ORIGIN: &str = "https://qirava.dev";

/// Wrap a page's `<body>` content in a full HTML document.
///
/// `head_css` is the collected component + page CSS (from a [`Css`] accumulator)
/// that is inlined *after* the theme variables so component rules can reference
/// the theme tokens. The theme's no-flicker boot script runs first so the
/// correct `data-q-theme` is set before paint.
pub fn document(meta: &Meta, head_css: String, body: Node) -> Node {
    // The full stylesheet: theme variable blocks (the dark/light contract) +
    // the site layout + the collected component CSS + the a11y media resets.
    let themes = tqquill_theme::default_theme_set();
    let boot = tqquill_theme::BootConfig::new(tqquill_theme::ThemeMode::Dark);

    let mut css = themes.to_css();
    css.push_str(&theme::layout_css());
    css.push_str(&head_css);
    css.push_str(&tqquill_design::reduced_motion_css());
    css.push_str(&tqquill_design::reduced_transparency_css());

    // The concrete bg the page paints at boot, for the chrome theme-color meta.
    let theme_color = themes
        .resolve(boot.mode(), tqquill_theme::Token::color("bg"))
        .or_else(|| themes.base().get(tqquill_theme::Token::color("bg")))
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
        .child(el("link").attr("rel", "manifest").attr("href", "/manifest.webmanifest"))
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
        .child(el("meta").attr("property", "og:type").attr("content", "website"))
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
        .child(el("meta").attr("property", "og:url").attr("content", canonical))
        .child(
            el("meta")
                .attr("property", "og:site_name")
                .attr("content", "Qirava"),
        )
        // The no-flicker theme boot MUST run before the stylesheet so the correct
        // `data-q-theme` is set pre-paint.
        .child(raw(tqquill_theme::boot_script_tag(&boot)))
        // Critical CSS inlined (theme vars + layout + components). The compiled
        // CSS is trusted (we generated it) -> Raw.
        .child(el("style").child(Node::Raw(css.into())));

    el("html")
        .attr("lang", "en")
        .child(head)
        .child(el("body").child(body))
}

/// Render a full document `tree` to a framed `text/html` response.
///
/// These pages contain no islands, so [`page_has_islands`] is false and zero
/// JavaScript is injected — the site ships no runtime. The bytes are prefixed
/// with `<!doctype html>` and framed with a `Cache-Control` header so the worker
/// serves them as native HTML.
pub fn respond_html(tree: &Node) -> tqexec::FunctionResponse {
    let mut html = String::from("<!doctype html>");
    render_into(tree, &mut html);

    // The island rule: only pages that actually use islands ship a runtime.
    if tqquill_view::page_has_islands(tree) {
        let kinds = tqquill_view::collect_island_kinds(tree);
        let kind_refs: Vec<&str> = kinds.iter().map(|s| s.as_str()).collect();
        let mut payload = String::new();
        tqquill_view::collect_island_sidecars_into(tree, &mut payload);
        payload.push_str("<script>");
        payload.push_str(&tqquill_runtime::runtime_bundle_for(&kind_refs));
        payload.push_str("</script>");
        if let Some(pos) = html.rfind("</body>") {
            html.insert_str(pos, &payload);
        } else {
            html.push_str(&payload);
        }
    }

    let headers = vec![("Cache-Control".to_string(), CACHE_CONTROL.to_string())];
    tqexec::FunctionResponse::ok(tqdms::workers::qquill::frame_response(
        html.as_bytes(),
        &headers,
    ))
}
