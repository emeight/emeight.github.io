//! Compose pages: an inner template fills `{{ ... }}` tokens, then the result is
//! wrapped in the site-wide `layout.html` (nav + footer + <head>)

use crate::frontmatter::Frontmatter;
use crate::render::escape_html;

/// Article page: pass `{{content}}` to `inner` along with frontmatter,
/// wrapped in `layout`.
pub fn render_page(layout: &str, inner: &str, fm: &Frontmatter, content: &str) -> String {
    let body = fill(inner, |key| match key {
        "content" => content.to_string(),
        _ => escape_html(fm.get(key).unwrap_or(""))
    });
    wrap(
        layout,
        fm.get("title").unwrap_or(""),
        fm.get("description").unwrap_or(""),
        &body,
    )
}

/// Listing page: `{{cards}}` in `inner` becomes `card` rendered once per entry,
/// wrapped in `layout` with a fixed title and description.
pub fn render_list(
    layout: &str,
    inner: &str,
    card: &str,
    title: &str,
    description: &str,
    entries: &[(&str, &Frontmatter)],
) -> String {
    let body = inner.replace("{{cards}}", &render_cards(card, entries));
    wrap(layout, title, description, &body)
}

/// Home page: `{{notes}}` / `{{builds}}` in `inner` become the featured cards;
/// other tokens come from `inner`'s own frontmatter.
pub fn render_home(
    layout: &str,
    inner: &str,
    fm: &Frontmatter,
    note_card: &str,
    notes: &[(&str, &Frontmatter)],
    build_card: &str,
    builds: &[(&str, &Frontmatter)],
) -> String {
    let body = fill(inner, |key| match key {
        "notes" => render_cards(note_card, notes),
        "builds" => render_cards(build_card, builds),
        _ => escape_html(fm.get(key).unwrap_or("")),
    });
    wrap(
        layout,
        fm.get("title").unwrap_or(""),
        fm.get("description").unwrap_or(""),
        &body,
    )
}

fn render_cards(card: &str, entries: &[(&str, &Frontmatter)]) -> String {
    entries
        .iter()
        .map(|&(slug, fm)| {
            fill(card, |key| match key {
                "slug" => escape_html(slug),
                "checkerboard" => checkerboard(fm.get("pattern")),
                _ => escape_html(fm.get(key).unwrap_or("")),
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn wrap(layout: &str, title: &str, description: &str, body: &str) -> String {
    fill(layout, |key| match key {
        "title" => escape_html(title),
        "description" => escape_html(description),
        "body" => body.to_string(),
        _ => String::new(),
    })
}

fn fill(template: &str, value: impl Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(template.len() + 256);
    let mut rest = template;
    while let Some(open) = rest.find("{{") {
        out.push_str(&rest[..open]);
        let after = &rest[open + 2..];
        match after.find("}}") {
            Some(close) => {
                out.push_str(&value(after[..close].trim()));
                rest = &after[close + 2..];
            }
            None => {
                out.push_str(&rest[open..]);
                return out;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Nine cells for a card's checkerboard icon. `pattern` is nine `0`/`1` characters,
/// `1` = filled; anything else falls back to the classic checkered pattern.
fn checkerboard(pattern: Option<&str>) -> String {
    const DEFAULT: &str = "101010101";
    let bits = match pattern {
        Some(p) if p.len() == 9 && p.bytes().all(|b| b == b'0' || b == b'1') => p,
        _ => DEFAULT,
    };
    bits.bytes()
        .map(|b| match b {
            b'1' => "<div class=\"filled\"></div>",
            _ => "<div></div>",
        })
        .collect::<Vec<_>>()
        .join("\n            ")
}