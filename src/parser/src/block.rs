//! markdown to Vec<Block>

use crate::ast::Block;
use crate::inline::parse_inline;

pub fn parse_blocks(md: &str) -> Vec<Block> {
    let lines: Vec<&str> = md.lines().collect();
    parse_lines(&lines)
}

fn parse_lines<'a>(lines: &[&'a str]) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut para: Vec<&'a str> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let raw = lines[i];
        let trimmed = raw.trim();

        // blank line: end current paragraph
        if trimmed.is_empty() {
            flush_para(&mut para, &mut blocks);
            i += 1;
            continue;
        }

        // fenced code block: ```lang ...```
        if let Some(info) = trimmed.strip_prefix("```") {
            flush_para(&mut para, &mut blocks);
            let lang = match info.trim() {
                "" => None,
                l => Some(l.to_string()),
            };
            let mut text = String::new();
            i += 1;
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                text.push_str(lines[i]);
                text.push('\n');
                i += 1;
            }
            i += 1; // step past the closing fence
            blocks.push(Block::CodeBlock { lang, text });
            continue;
        }

        // ATX heading: # to ######
        if let Some((level, content)) = parse_heading(trimmed) {
            flush_para(&mut para, &mut blocks);
            blocks.push(Block::Heading {
                level,
                content: parse_inline(content),
            });
            i += 1;
            continue;
        }

        // thematic break: ---, ***, ___
        if is_thematic_break(trimmed) {
            flush_para(&mut para, &mut blocks);
            blocks.push(Block::ThematicBreak);
            i += 1;
            continue;
        }

        // blockquote: consecutive lines starting with >
        if trimmed.starts_with('>') {
            flush_para(&mut para, &mut blocks);
            let mut inner: Vec<&str> = Vec::new();
            while i < lines.len() && lines[i].trim_start().starts_with('>') {
                let l = &lines[i].trim_start()[1..];
                inner.push(l.strip_prefix(' ').unwrap_or(l));
                i += 1;
            }
            blocks.push(Block::BlockQuote(parse_lines(&inner)));
            continue;
        }

        // lists
        if let Some((ordered, _)) = list_marker(trimmed) {
            flush_para(&mut para, &mut blocks);
            let mut items: Vec<Vec<Block>> = Vec::new();
            while i < lines.len() {
                match list_marker(lines[i].trim()) {
                    Some((o, content)) if o == ordered => {
                        items.push(vec![Block::Paragraph(parse_inline(content))]);
                        i += 1;
                    }
                    _ => break,
                }
            }
            blocks.push(Block::List { ordered, items });
            continue;
        }

        // all else is paragraph text
        para.push(trimmed);
        i += 1;
    }

    flush_para(&mut para, &mut blocks);
    blocks
}

fn flush_para<'a>(para: &mut Vec<&'a str>, blocks: &mut Vec<Block>) {
    if !para.is_empty() {
        blocks.push(Block::Paragraph(parse_inline(&para.join("\n"))));
        para.clear();
    }
}

fn parse_heading(line: &str) -> Option<(u8, &str)> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = line[hashes..].strip_prefix(' ')?;
    Some((hashes as u8, rest.trim_end()))
}

fn is_thematic_break(line: &str) -> bool {
    let s: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    s.len() >= 3
        && (s.bytes().all(|b| b == b'-')
            || s.bytes().all(|b| b == b'*')
            || s.bytes().all(|b| b == b'_'))
}

/// Recognise a list item marker. Returns `(ordered, content_after_marker)`.
fn list_marker(line: &str) -> Option<(bool, &str)> {
    for m in ['-', '*', '+'] {
        if let Some(rest) = line.strip_prefix(m) {
            if let Some(content) = rest.strip_prefix(' ') {
                return Some((false, content.trim_end()));
            }
        }
    }
    let digits = line.len() - line.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if digits > 0 {
        let rest = &line[digits..];
        let rest = rest.strip_prefix('.').or_else(|| rest.strip_prefix(')'))?;
        if let Some(content) = rest.strip_prefix(' ') {
            return Some((true, content.trim_end()));
        }
    }
    None
}