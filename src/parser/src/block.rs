//! markdown to Vec<Block>

use crate::ast::{Alignment, Block};
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
                id: slugify(content),
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

        // tables: a header row immediately followed by a `---|---` delimiter row
        if trimmed.contains('|') {
            if let Some(delim) = lines.get(i + 1) {
                if is_table_delimiter_row(delim.trim()) {
                    flush_para(&mut para, &mut blocks);
                    let headers = split_table_row(trimmed)
                        .into_iter()
                        .map(parse_inline)
                        .collect();
                    let alignments = parse_alignments(delim.trim());
                    i += 2;
                    let mut rows = Vec::new();
                    while i < lines.len() && lines[i].trim().contains('|') {
                        rows.push(
                            split_table_row(lines[i].trim())
                                .into_iter()
                                .map(parse_inline)
                                .collect(),
                        );
                        i += 1;
                    }
                    blocks.push(Block::Table { alignments, headers, rows });
                    continue;
                }
            }
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

/// Lowercase, alphanumeric-and-hyphen id so `[text](#slug)` has something to scroll to.
fn slugify(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
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

/// Split a `| a | b |` row into trimmed cell strings, dropping the outer pipes.
fn split_table_row(line: &str) -> Vec<&str> {
    let line = line.trim();
    let line = line.strip_prefix('|').unwrap_or(line);
    let line = line.strip_suffix('|').unwrap_or(line);
    line.split('|').map(str::trim).collect()
}

/// A GFM table delimiter row: cells made only of `-`, optionally `:`-wrapped for alignment.
fn is_table_delimiter_row(line: &str) -> bool {
    let cells = split_table_row(line);
    !cells.is_empty()
        && cells.iter().all(|c| {
            let c = c.trim_start_matches(':').trim_end_matches(':');
            !c.is_empty() && c.bytes().all(|b| b == b'-')
        })
}

fn parse_alignments(delim: &str) -> Vec<Alignment> {
    split_table_row(delim)
        .iter()
        .map(|c| match (c.starts_with(':'), c.ends_with(':')) {
            (true, true) => Alignment::Center,
            (true, false) => Alignment::Left,
            (false, true) => Alignment::Right,
            (false, false) => Alignment::None,
        })
        .collect()
}