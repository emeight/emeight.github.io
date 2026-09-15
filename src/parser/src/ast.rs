//! abstract syntax tree (node types)

#[derive(Debug, PartialEq)]
pub enum Block {
    Heading { level: u8, id: String, content: Vec<Inline> },
    Paragraph(Vec<Inline>),
    CodeBlock { lang: Option<String>, text: String },
    List { ordered: bool, items: Vec<Vec<Block>> },
    BlockQuote(Vec<Block>),
    Table {
        alignments: Vec<Alignment>,
        headers: Vec<Vec<Inline>>,
        rows: Vec<Vec<Vec<Inline>>>,
    },
    ThematicBreak,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(String),
    Strong(Vec<Inline>),
    Emph(Vec<Inline>),
    Code(String),
    Link { href: String, text: Vec<Inline> },
    Image { src: String, alt: String },
}