//! text span to Vec<Inline>

use crate::ast::Inline;

pub fn parse_inline(input: &str) -> Vec<Inline> {
    let chars: Vec<char> = input.chars().collect();
    let mut out: Vec<Inline> = Vec::new();
    let mut buf = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // `code`
        if c == '`' {
            if let Some(end) = find(&chars, i + 1, '`') {
                push_text(&mut out, &mut buf);
                out.push(Inline::Code(chars[i + 1..end].iter().collect()));
                i = end + 1;
                continue;
            }
        }

        // ![alt](src)
        if c == '!' && chars.get(i + 1) == Some(&'[') {
            if let Some((alt, src, next)) = parse_bracket_paren(&chars, i + 1) {
                push_text(&mut out, &mut buf);
                out.push(Inline::Image { src, alt });
                i = next;
                continue;
            }
        }

        // [text](href)
        if c == '[' {
            if let Some((text, href, next)) = parse_bracket_paren(&chars, i) {
                push_text(&mut out, &mut buf);
                out.push(Inline::Link {
                    href,
                    text: parse_inline(&text),
                });
                i = next;
                continue;
            }
        }

        // **strong**
        if c == '*' && chars.get(i + 1) == Some(&'*') {
            if let Some(end) = find_seq(&chars, i + 2, &['*', '*']) {
                push_text(&mut out, &mut buf);
                let inner: String = chars[i + 2..end].iter().collect();
                out.push(Inline::Strong(parse_inline(&inner)));
                i = end + 2;
                continue;
            }
        }

        // *emph*
        if c == '*' {
            if let Some(end) = find(&chars, i + 1, '*') {
                push_text(&mut out, &mut buf);
                let inner: String = chars[i + 1..end].iter().collect();
                out.push(Inline::Emph(parse_inline(&inner)));
                i = end + 1;
                continue;
            }
        }

        buf.push(c);
        i += 1;
    }

    push_text(&mut out, &mut buf);
    out
}

fn push_text(out: &mut Vec<Inline>, buf: &mut String) {
    if !buf.is_empty() {
        out.push(Inline::Text(std::mem::take(buf)));
    }
}

fn find(chars: &[char], start: usize, target: char) -> Option<usize> {
    (start..chars.len()).find(|&j| chars[j] == target)
}

fn find_seq(chars: &[char], start: usize, seq: &[char]) -> Option<usize> {
    if chars.len() < seq.len() {
        return None;
    }
    (start..=chars.len() - seq.len()).find(|&j| &chars[j..j + seq.len()] == seq)
}

/// Parse `[a](b)` beginning at `open` (the `[`).
/// Returns `(inside_brackets, inside_parens, index_past_close_paren)`.
fn parse_bracket_paren(chars: &[char], open: usize) -> Option<(String, String, usize)> {
    let close_b = find(chars, open + 1, ']')?;
    if chars.get(close_b + 1) != Some(&'(') {
        return None;
    }
    let close_p = find(chars, close_b + 2, ')')?;
    Some((
        chars[open + 1..close_b].iter().collect(),
        chars[close_b + 2..close_p].iter().collect(),
        close_p + 1,
    ))
}