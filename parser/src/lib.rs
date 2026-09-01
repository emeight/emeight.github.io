pub fn block_parser(markdown: &str) -> String {
    let mut html = String::new();

    // state trackers
    let mut in_paragraph = false;
    let mut in_list = false;
    let mut in_code_block = false;

    for line in markdown.lines() {
        let trimmed = line.trim();

        // code blocks break markdown rules, handle first
        if trimmed.starts_with("```") {
            if in_code_block {
                if html.ends_with('\n') {
                    html.pop();
                }
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                // close paragraph/list before starting code (if inside of them)
                if in_paragraph { html.push_str("</p>\n"); in_paragraph = false; }
                if in_list { html.push_str("</ul>\n"); in_list = false; }

                // do not push a new line here to prevent ghost spacing (empty lines)
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }

        // if already in code block, push raw text
        if in_code_block {
            html.push_str(line);
            html.push_str("\n");
            continue;
        }

        // empty lines
        if trimmed.is_empty() {
            if in_paragraph { html.push_str("</p>\n"); in_paragraph = false; }
            if in_list { html.push_str("</ul>\n"); in_list = false; }
            continue;
        }
    }

    // clean up
    if in_paragraph { html.push_str("</p>\n"); }
    if in_list { html.push_str("</ul>\n"); }
    if in_code_block { html.push_str("</code></pre>\n"); }

    html
}

// fn inline_parser(markdown: &str) -> String {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_h1() {
        assert_eq!(block_parser("# Header"), "<h1>Header</h1>")
    }

    #[test]
    fn test_parse_code() {
        assert_eq!(block_parser("```\nlet x = 1;\n```"), "<pre><code>let x = 1;</code></pre>\n")
    }
}