use pulldown_cmark;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeBlockKind {
    Indented,
    Fenced(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    Paragraph,
    Heading(HeadingLevel),
    // BlockQuote,
    // CodeBlock(CodeBlockKind),
    // List(Option<u64>),
    // Item,
    // FootnoteDefinition(CowStr<'a>),
    // Table(Vec<Alignment>),
    // TableHead,
    // TableRow,
    // TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    // Link(LinkType, CowStr<'a>, CowStr<'a>),
    // Image(LinkType, CowStr<'a>, CowStr<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Start(Tag),
    End(Tag),
    Text(String),
    Code(String),
    // FootnoteReference(CowStr(<'a>)),
    SoftBreak,
    HardBreak,
    // Rule,
    // TaskListMarker(bool),
}

impl From<pulldown_cmark::HeadingLevel> for HeadingLevel {
    fn from(heading_level: pulldown_cmark::HeadingLevel) -> Self {
        match heading_level {
            pulldown_cmark::HeadingLevel::H1 => HeadingLevel::H1,
            pulldown_cmark::HeadingLevel::H2 => HeadingLevel::H2,
            pulldown_cmark::HeadingLevel::H3 => HeadingLevel::H3,
            pulldown_cmark::HeadingLevel::H4 => HeadingLevel::H4,
            pulldown_cmark::HeadingLevel::H5 => HeadingLevel::H5,
            pulldown_cmark::HeadingLevel::H6 => HeadingLevel::H6,
        }
    }
}

impl<'a> From<pulldown_cmark::CodeBlockKind<'a>> for CodeBlockKind {
    fn from(code_block_kind: pulldown_cmark::CodeBlockKind) -> Self {
        match code_block_kind {
            pulldown_cmark::CodeBlockKind::Indented => CodeBlockKind::Indented,
            pulldown_cmark::CodeBlockKind::Fenced(x) => CodeBlockKind::Fenced(x.to_string()),
        }
    }
}

impl From<pulldown_cmark::Alignment> for Alignment {
    fn from(alignment: pulldown_cmark::Alignment) -> Self {
        match alignment {
            pulldown_cmark::Alignment::None => Alignment::None,
            pulldown_cmark::Alignment::Left => Alignment::Left,
            pulldown_cmark::Alignment::Center => Alignment::Center,
            pulldown_cmark::Alignment::Right => Alignment::Right,
        }
    }
}

impl<'a> From<pulldown_cmark::Event<'a>> for Event {
    fn from(event: pulldown_cmark::Event) -> Self {
        match event {
            pulldown_cmark::Event::Start(tag) => Event::Start(tag.into()),
            pulldown_cmark::Event::End(tag) => Event::End(tag.into()),
            pulldown_cmark::Event::Text(text) => Event::Text(text.to_string()),
            pulldown_cmark::Event::Code(text) => Event::Code(text.to_string()),
            pulldown_cmark::Event::FootnoteReference(_) => todo!(),
            pulldown_cmark::Event::SoftBreak => Event::SoftBreak,
            pulldown_cmark::Event::HardBreak => Event::HardBreak,
            pulldown_cmark::Event::Rule => todo!(),
            pulldown_cmark::Event::TaskListMarker(_) => todo!(),

            // We're never going to be able to support the events below
            pulldown_cmark::Event::Html(_) => panic!("HTML is unsupported"),
        }
    }
}

impl<'a> From<pulldown_cmark::Tag<'a>> for Tag {
    fn from(tag: pulldown_cmark::Tag) -> Self {
        match tag {
            pulldown_cmark::Tag::Paragraph => Tag::Paragraph,
            pulldown_cmark::Tag::Heading(level, _, _) => Tag::Heading(level.into()),
            pulldown_cmark::Tag::BlockQuote => todo!(),
            pulldown_cmark::Tag::CodeBlock(_) => todo!(),
            pulldown_cmark::Tag::List(_) => todo!(),
            pulldown_cmark::Tag::Item => todo!(),
            pulldown_cmark::Tag::FootnoteDefinition(_) => todo!(),
            pulldown_cmark::Tag::Table(_) => todo!(),
            pulldown_cmark::Tag::TableHead => todo!(),
            pulldown_cmark::Tag::TableRow => todo!(),
            pulldown_cmark::Tag::TableCell => todo!(),
            pulldown_cmark::Tag::Emphasis => Tag::Emphasis,
            pulldown_cmark::Tag::Strong => Tag::Strong,
            pulldown_cmark::Tag::Strikethrough => Tag::Strikethrough,
            pulldown_cmark::Tag::Link(_, _, _) => todo!(),
            pulldown_cmark::Tag::Image(_, _, _) => todo!(),
        }
    }
}
