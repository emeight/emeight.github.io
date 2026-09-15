//! Vec<Block> to HTML string (escaping)

use crate::ast::{Alignment, Block, Inline};

pub fn render(blocks: &[Block]) -> String {
    let mut out = String::new();
    for block in blocks {
        render_block(block, &mut out);
    }
    out
}

fn render_block(block: &Block, out: &mut String) {
    match block {
        Block::Heading { level, id, content } => {
            out.push_str(&format!("<h{level} id=\"{}\">", escape_html(id)));
            render_inlines(content, out);
            out.push_str(&format!("</h{level}>\n"));
        }
        Block::Paragraph(content) => {
            if let [Inline::Image { alt, .. }] = content.as_slice() {
                out.push_str("<figure>\n");
                render_inlines(content, out);
                out.push('\n');
                if !alt.is_empty() {
                    out.push_str(&format!("<figcaption>{}</figcaption>\n", escape_html(alt)));
                }
                out.push_str("</figure>\n");
            } else {
                out.push_str("<p>");
                render_inlines(content, out);
                out.push_str("</p>\n");
            }
        }
        Block::CodeBlock { lang, text } => {
            match lang {
                Some(l) => out.push_str(&format!(
                    "<pre><code class=\"language-{}\">",
                    escape_html(l)
                )),
                None => out.push_str("<pre><code>"),
            }
            out.push_str(&escape(text));
            out.push_str("</code></pre>\n");
        }
        Block::List { ordered, items } => {
            let tag = if *ordered { "ol" } else { "ul" };
            out.push_str(&format!("<{tag}>\n"));
            for item in items {
                out.push_str("<li>");
                render_item(item, out);
                out.push_str("</li>\n");
            }
            out.push_str(&format!("</{tag}>\n"));
        }
        Block::BlockQuote(inner) => {
            out.push_str("<blockquote>\n");
            out.push_str(&render(inner));
            out.push_str("</blockquote>\n");
        }
        Block::Table { alignments, headers, rows } => {
            out.push_str("<table>\n<thead>\n<tr>\n");
            for (idx, cell) in headers.iter().enumerate() {
                render_table_cell("th", align_of(alignments, idx), cell, out);
            }
            out.push_str("</tr>\n</thead>\n<tbody>\n");
            for row in rows {
                out.push_str("<tr>\n");
                for (idx, cell) in row.iter().enumerate() {
                    render_table_cell("td", align_of(alignments, idx), cell, out);
                }
                out.push_str("</tr>\n");
            }
            out.push_str("</tbody>\n</table>\n");
        }
        Block::ThematicBreak => out.push_str("<hr>\n"),
    }
}

fn align_of(alignments: &[Alignment], idx: usize) -> Alignment {
    alignments.get(idx).copied().unwrap_or(Alignment::None)
}

fn render_table_cell(tag: &str, align: Alignment, content: &[Inline], out: &mut String) {
    let style = match align {
        Alignment::Left => " style=\"text-align:left\"",
        Alignment::Center => " style=\"text-align:center\"",
        Alignment::Right => " style=\"text-align:right\"",
        Alignment::None => "",
    };
    out.push_str(&format!("<{tag}{style}>"));
    render_inlines(content, out);
    out.push_str(&format!("</{tag}>\n"));
}

/// A list item that is exactly one paragraph renders its inline content
/// directly; anything richer falls back to full block rendering.
fn render_item(item: &[Block], out: &mut String) {
    match item {
        [Block::Paragraph(content)] => render_inlines(content, out),
        _ => out.push_str(&render(item)),
    }
}

fn render_inlines(items: &[Inline], out: &mut String) {
    for item in items {
        render_inline(item, out);
    }
}

fn render_inline(item: &Inline, out: &mut String) {
    match item {
        Inline::Text(t) => out.push_str(&escape(t)),
        Inline::Strong(inner) => {
            out.push_str("<strong>");
            render_inlines(inner, out);
            out.push_str("</strong>")
        }
        Inline::Emph(inner) => {
            out.push_str("<em>");
            render_inlines(inner, out);
            out.push_str("</em>");
        }
        Inline::Code(t) => {
            out.push_str("<code>");
            out.push_str(&escape(t));
            out.push_str("</code>");
        }
        Inline::Link { href, text } => {
            out.push_str(&format!("<a href=\"{}\">", escape_html(href)));
            render_inlines(text, out);
            out.push_str("</a>");
        }
        Inline::Image { src, alt } => {
            out.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\">",
                escape_html(src),
                escape_html(alt)
            ));
        }
    }
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

pub(crate) fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}