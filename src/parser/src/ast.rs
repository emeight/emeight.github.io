//! abstract syntax tree (node types)

#[derive(Debug, PartialEq)]
pub enum Block {
    Heading { level: u8, content: Vec<Inline> },
    Paragraph(Vec<Inline>),
    CodeBlock { lang: Option<String>, text: String },
    List { ordered: bool, items: Vec<Vec<Block>> },
    BlockQuote(Vec<Block>),
    ThematicBreak,
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