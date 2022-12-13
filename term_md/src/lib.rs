mod event;
use event::*;

use nu_ansi_term::{Color, Style};
use unicode_width::UnicodeWidthStr;

pub struct Renderer<T: Iterator<Item = Event>> {
    // The output string, which will be returned by `render`
    output: String,

    // Terminal width to render into
    width: usize,

    // Column in which the next character will be written
    //
    // Must always be smaller than `width`
    current_column: usize,

    // Iterator of Markdown events to render
    events: T,
}

impl<T: Iterator<Item = Event>> Renderer<T> {
    pub fn new(width: usize, events: T) -> Self {
        Self {
            output: String::new(),
            current_column: 0,
            width,
            events,
        }
    }

    pub fn render(mut self) -> String {
        while let Some(ev) = self.events.next() {
            match ev {
                Event::Start(x) => match x {
                    Tag::Paragraph => self.render_paragraph(),
                    Tag::Heading(level) => self.render_heading(level),
                    Tag::Emphasis | Tag::Strong | Tag::Strikethrough => {
                        unreachable!("Can't be the opening tag")
                    }
                },
                _ => {
                    panic!(
                        "Internal error: we assume that the markdown always \
                        starts with a start tag. If you hit this, that assumption \
                        is wrong."
                    )
                }
            }
        }

        self.output
    }

    fn render_paragraph(&mut self) {
        self.render_inline(&Tag::Paragraph, Style::new());
        self.newline();
    }

    fn render_heading(&mut self, level: HeadingLevel) {
        let style = match level {
            HeadingLevel::H1 => Style::new().bold().underline(),
            HeadingLevel::H2 => Style::new().bold(),
            _ => panic!(),
        };
        self.output.push_str(&style.prefix().to_string());
        self.render_inline(&Tag::Heading(level), style);
        self.output.push_str(&style.suffix().to_string());
        self.newline();
    }

    fn render_inline(&mut self, until: &Tag, base_style: Style) {
        let mut style = base_style.clone();
        while let Some(ev) = self.events.next() {
            match ev {
                Event::Text(x) => self.wrap_words(&x),
                Event::Code(x) => {
                    let mut code_style = style;
                    // A grayish color. The range is 232 (black) to 255 (white).
                    // This might have to depend on the terminal colors.
                    code_style.foreground = Some(Color::Fixed(250));

                    // Change to the code style, push the string and change back.
                    self.output
                        .push_str(&style.clone().infix(code_style.clone()).to_string());
                    self.wrap_words(&x);
                    self.output
                        .push_str(&code_style.infix(style.clone()).to_string());
                }
                Event::SoftBreak => {
                    if self.current_column >= self.width {
                        self.newline();
                    } else {
                        self.current_column += 1;
                        self.output.push(' ');
                    }
                }
                Event::HardBreak => {
                    self.newline();
                }
                Event::Start(tag @ (Tag::Emphasis | Tag::Strong | Tag::Strikethrough)) => {
                    self.change_style(&mut style, tag, true);
                }
                Event::End(tag @ (Tag::Emphasis | Tag::Strong | Tag::Strikethrough)) => {
                    self.change_style(&mut style, tag, false);
                }
                Event::End(tag) if &tag == until => return,
                Event::Start(Tag::Paragraph | Tag::Heading(_)) => {
                    panic!("We're already in a paragraph or heading.")
                }
                Event::End(Tag::Paragraph | Tag::Heading(_)) => {
                    unreachable!("Should have been caught above.")
                }
            }
        }
    }

    fn wrap_words(&mut self, s: &str) {
        let mut words = s.split_whitespace();
        let first = words.next();

        // The first word needs special treatment, because we only want to
        // print a space in front of it if the string actually starts with a
        // space.
        let Some(word) = first else { return };

        let width = word.width();

        if self.current_column + width >= self.width {
            self.newline();
        } else if s.starts_with(' ') {
            self.output.push(' ');
            self.current_column += 1;
        }

        self.current_column += width;
        self.output.push_str(word);

        for word in words {
            let width = word.width();

            // The +1 comes from the additional space we need in front of this
            // word.
            if self.current_column + width + 1 >= self.width {
                self.newline();
            } else {
                self.output.push(' ');
                self.current_column += 1;
            }

            self.current_column += width;
            self.output.push_str(word);
        }

        if s.ends_with(' ') {
            self.output.push(' ')
        }
    }

    fn newline(&mut self) {
        self.current_column = 0;
        self.output.push('\n');
    }

    fn change_style(&mut self, style: &mut Style, tag: Tag, enable: bool) {
        let old_style = style.clone();

        let setting = match tag {
            Tag::Emphasis => &mut style.is_italic,
            Tag::Strong => &mut style.is_bold,
            Tag::Strikethrough => &mut style.is_strikethrough,
            Tag::Paragraph => panic!("Paragraph is not a style"),
            Tag::Heading(_) => panic!("Heading is not a style"),
        };

        *setting = enable;

        // Add the ansi code to mode between the styles to the output
        self.output
            .push_str(&old_style.infix(style.clone()).to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::event::Event;
    use super::Renderer;
    use pulldown_cmark::{Options, Parser};

    #[test]
    fn it_works() {
        let events: Vec<Event> = Parser::new("This is *some* markdown **paragraph**!")
            .map(|e| e.into())
            .collect();

        let output = Renderer::new(40, events.into_iter()).render();

        assert_eq!(
            output,
            "This is \u{1b}[3msome\u{1b}[0m markdown \u{1b}[1mparagraph\u{1b}[0m!\n"
        );
    }

    #[test]
    fn styles() {
        let events = Parser::new_ext(
            "We have *emphasis*, **bold**, and ~~strikethrough~~.",
            Options::ENABLE_STRIKETHROUGH,
        )
        .map(|e| e.into());

        let output = Renderer::new(40, events).render();

        println!("{}", output);
        assert_eq!(
            output,
            "We have \u{1b}[3memphasis\u{1b}[0m, \u{1b}[1mbold\u{1b}[0m, and \u{1b}[9mstrikethrough\u{1b}[0m.\n"
        );
    }

    #[test]
    fn code_style() {
        let events = Parser::new("To render, call the `render` method.").map(Into::into);

        let output = Renderer::new(40, events).render();
        println!("{}", output);
        assert_eq!(
            output,
            "To render, call the \u{1b}[38;5;250mrender\u{1b}[0m method.\n"
        );
    }

    #[test]
    fn heading() {
        let text = "\
            # Heading 1\n\
            Some text\n\
            ## Heading 2\n\
            Some more text\
        ";
        let events = Parser::new(text).map(Into::into);
        let events: Vec<Event> = events.collect();
        let output = Renderer::new(40, events.into_iter()).render();
        println!("{}", output);
        assert_eq!(
            output,
            "\u{1b}[1;4mHeading 1\u{1b}[0m\n\
            Some text\n\
            \u{1b}[1mHeading 2\u{1b}[0m\n\
            Some more text\n"
        )
    }

    #[test]
    fn wrapping() {
        let text = "This is some very long text that will definitely need to get wrapped, so we better do that **right**!";
        let events = Parser::new(text).map(Into::into);
        let output = Renderer::new(10, events.into_iter()).render();
        println!("{}", output);

        // The lone `!` at the end is technically a bug, because words across
        // styles need be preserved, but that's a can of worms I don't want to
        // open right now. You'd need to keep track of the last word and its
        // width and render it either at the end of the block or at the start
        // of the next inline Text or Code event. It could also happen that
        // there are more than 2 styles per word.
        assert_eq!(
            output,
            "This is\n\
            some very\n\
            long text\n\
            that will\n\
            definitely\n\
            need to\n\
            get\n\
            wrapped,\n\
            so we\n\
            better do\n\
            that \u{1b}[1mright\u{1b}[0m\n\
            !\n"
        )
    }

    #[test]
    fn soft_break() {
        let text = "This is text\nwith a soft break.";
        let events: Vec<Event> = Parser::new(text).map(Into::into).collect();
        dbg!(&events);
        let output = Renderer::new(40, events.into_iter()).render();
        println!("{}", output);

        assert_eq!(output, "This is text with a soft break.\n");
    }

    #[test]
    fn hard_break() {
        let text = "This is text\\\nwith a hard break.";
        let events: Vec<Event> = Parser::new(text).map(Into::into).collect();
        dbg!(&events);
        let output = Renderer::new(40, events.into_iter()).render();
        println!("{}", output);

        assert_eq!(output, "This is text\nwith a hard break.\n");
    }
}
