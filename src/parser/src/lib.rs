mod ast;
mod block;
mod inline;
mod render;

pub mod frontmatter;
pub mod site;

pub use ast::{Alignment, Block, Inline};

/// Convert markdown to html
pub fn to_html(markdown: &str) -> String {
    let blocks = block::parse_blocks(markdown);
    render::render(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_h1() {
        assert_eq!(to_html("# Header"), "<h1 id=\"header\">Header</h1>\n");
    }

    #[test]
    fn heading_levels_and_no_space_is_paragraph() {
        assert_eq!(to_html("### Deep"), "<h3 id=\"deep\">Deep</h3>\n");
        assert_eq!(to_html("#no space"), "<p>#no space</p>\n");
        assert_eq!(to_html("####### too deep"), "<p>####### too deep</p>\n");
    }

    #[test]
    fn code_block_is_escaped() {
        assert_eq!(
            to_html("```\nlet x = 1;\n```"),
            "<pre><code>let x = 1;\n</code></pre>\n"
        );
        assert_eq!(
            to_html("```rust\n<T>\n```"),
            "<pre><code class=\"language-rust\">&lt;T&gt;\n</code></pre>\n"
        );
    }

    #[test]
    fn paragraphs_split_on_blank_line() {
        assert_eq!(to_html("a\nb\n\nc"), "<p>a\nb</p>\n<p>c</p>\n");
    }

    #[test]
    fn inline_formatting() {
        assert_eq!(
            to_html("a **b** *c* `d`"),
            "<p>a <strong>b</strong> <em>c</em> <code>d</code></p>\n"
        );
    }

    #[test]
    fn link_and_image() {
        assert_eq!(
            to_html("[x](http://e.com)"),
            "<p><a href=\"http://e.com\">x</a></p>\n"
        );
        assert_eq!(
            to_html("![alt](a.png)"),
            "<figure>\n<img src=\"a.png\" alt=\"alt\">\n<figcaption>alt</figcaption>\n</figure>\n"
        );
    }

    #[test]
    fn anchor_link_targets_heading_id() {
        assert_eq!(
            to_html("# My Section\n\n[jump](#my-section)"),
            "<h1 id=\"my-section\">My Section</h1>\n<p><a href=\"#my-section\">jump</a></p>\n"
        );
    }

    #[test]
    fn unordered_and_ordered_lists() {
        assert_eq!(
            to_html("- a\n- b"),
            "<ul>\n<li>a</li>\n<li>b</li>\n</ul>\n"
        );
        assert_eq!(
            to_html("1. a\n2. b"),
            "<ol>\n<li>a</li>\n<li>b</li>\n</ol>\n"
        );
    }

    #[test]
    fn blockquote_nests_blocks() {
        assert_eq!(
            to_html("> quoted\n> text"),
            "<blockquote>\n<p>quoted\ntext</p>\n</blockquote>\n"
        );
    }

    #[test]
    fn table_with_alignment() {
        assert_eq!(
            to_html("| A | B |\n| :-- | --: |\n| 1 | 2 |"),
            "<table>\n<thead>\n<tr>\n<th style=\"text-align:left\">A</th>\n<th style=\"text-align:right\">B</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style=\"text-align:left\">1</td>\n<td style=\"text-align:right\">2</td>\n</tr>\n</tbody>\n</table>\n"
        );
    }

    #[test]
    fn thematic_break() {
        assert_eq!(to_html("---"), "<hr>\n");
    }

    #[test]
    fn text_is_escaped() {
        assert_eq!(to_html("a < b & c"), "<p>a &lt; b &amp; c</p>\n");
    }
}