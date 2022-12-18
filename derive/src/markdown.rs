use proc_macro2::TokenStream;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use quote::quote;

fn prefix(t: TokenStream) -> TokenStream {
    quote!(uutils_args::term_md::#t)
}

fn md_to_quote(event: Event) -> TokenStream {
    let tokens = match event {
        Event::Start(tag) => {
            let tag = quote_tag(tag);
            quote!(Event::Start(#tag))
        }
        Event::End(tag) => {
            let tag = quote_tag(tag);
            quote!(Event::End(#tag))
        }
        Event::Text(t) => {
            let t = t.to_string();
            let text = quote!(String::from(#t));
            quote!(Event::Text(#text))
        }
        Event::Code(t) => {
            let t = t.to_string();
            let text = quote!(String::from(#t));
            quote!(Event::Code(#text))
        }
        Event::SoftBreak => quote!(Event::SoftBreak),
        Event::HardBreak => quote!(Event::HardBreak),
        Event::Rule => quote!(Event::Rule),

        // Below are unsupported in term_md
        Event::Html(_) => todo!(),
        Event::FootnoteReference(_) => todo!(),
        Event::TaskListMarker(_) => todo!(),
    };
    prefix(tokens)
}

pub(crate) fn str_to_renderer(s: &str) -> TokenStream {
    let events = Parser::new(s);
    let parsed_events = events.map(md_to_quote);

    prefix(quote!(Renderer::new(
        60,
        vec![#(#parsed_events),*].into_iter()
    )))
}

pub(crate) fn get_h2(heading_name: &str, s: &str) -> TokenStream {
    let mut events = Parser::new(s);
    let mut selected_events = Vec::new();
    while let Some(event) = events.next() {
        if let Event::Start(Tag::Heading(HeadingLevel::H2, _, _)) = event {
            if let Some(Event::Text(s)) = events.next() {
                if s.to_lowercase() == heading_name.to_lowercase() {
                    selected_events.extend(
                        (&mut events)
                            .skip_while(|e| {
                                !matches!(e, Event::End(Tag::Heading(HeadingLevel::H2, _, _)))
                            })
                            .skip(1)
                            .take_while(|e| {
                                !matches!(e, Event::Start(Tag::Heading(HeadingLevel::H2, _, _)))
                            }),
                    )
                }
            }
        }
    }

    let parsed_events = selected_events.into_iter().map(md_to_quote);
    prefix(quote!(Renderer::new(
        80,
        vec![#(#parsed_events),*].into_iter()
    )))
}

fn quote_tag(tag: Tag) -> TokenStream {
    let tokens = match tag {
        Tag::Paragraph => quote!(Paragraph),
        Tag::Heading(level, _, _) => {
            let level = match level {
                HeadingLevel::H1 => quote!(H1),
                HeadingLevel::H2 => quote!(H2),
                HeadingLevel::H3 => quote!(H3),
                HeadingLevel::H4 => quote!(H4),
                HeadingLevel::H5 => quote!(H5),
                HeadingLevel::H6 => quote!(H6),
            };
            let level = prefix(quote!(HeadingLevel::#level));
            quote!(Heading(#level))
        }
        Tag::Emphasis => quote!(Emphasis),
        Tag::Strong => quote!(Strong),
        Tag::Strikethrough => quote!(Strikethrough),

        // Below are unsupported in term_md
        Tag::BlockQuote => todo!(),
        Tag::CodeBlock(_) => todo!(),
        Tag::List(_) => todo!(),
        Tag::Item => todo!(),
        Tag::FootnoteDefinition(_) => todo!(),
        Tag::Table(_) => todo!(),
        Tag::TableHead => todo!(),
        Tag::TableRow => todo!(),
        Tag::TableCell => todo!(),
        Tag::Link(_, _, _) => todo!(),
        Tag::Image(_, _, _) => todo!(),
    };

    prefix(quote!(Tag::#tokens))
}
