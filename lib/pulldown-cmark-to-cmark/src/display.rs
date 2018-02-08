use pulldown_cmark::Event;

use std::fmt::{self, Display};

/// A struct offering a strictly stateless way of displaying [Events][pdcEvent].
/// [pdcEvent]: https://docs.rs/pulldown-cmark/0.1.0/pulldown_cmark/enum.Event.html
#[derive(Debug, Clone)]
pub struct EventDisplay<'a>(pub &'a Event<'a>);

impl<'a> Display for EventDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use pulldown_cmark::Tag::*;
        match *self.0 {
            Event::FootnoteReference(ref name) => write!(f, "[^{}]", name),
            Event::End(ref tag) => match *tag {
                Header(_) => Ok(()),
                Table(_) => Ok(()),
                Rule => Ok(()),
                Paragraph => Ok(()),
                BlockQuote => Ok(()),
                FootnoteDefinition(_) => Ok(()),
                List(_) => Ok(()),
                Item => Ok(()),
                CodeBlock(_) => "```".fmt(f),
                TableHead => Ok(()),
                TableRow => Ok(()),
                TableCell => '|'.fmt(f),
                Image(ref uri, ref title)|Link(ref uri, ref title) => {
                    if title.is_empty() {
                        write!(f, "]({})", uri)
                    } else {
                        write!(f, "]({uri} \"{title}\")", uri = uri, title = title)
                    }
                },
                Emphasis => '*'.fmt(f),
                Strong => "**".fmt(f),
                Code => '`'.fmt(f),
            },
            Event::Start(ref tag) => match *tag {
                Table(_) => Ok(()),
                TableHead => Ok(()),
                TableRow => Ok(()),
                TableCell => '|'.fmt(f),
                Link(_, _) => '['.fmt(f),
                Image(_, _) => "![".fmt(f),
                Emphasis => '*'.fmt(f),
                Strong => "**".fmt(f),
                Code => '`'.fmt(f),
                FootnoteDefinition(ref name) => write!(f, "[^{}]: ", name),
                Paragraph => Ok(()),
                Rule => "---".fmt(f),
                Header(n) => {
                    for _ in 0..n {
                        '#'.fmt(f)?;
                    }
                    ' '.fmt(f)
                }
                BlockQuote => Ok(()),
                CodeBlock(ref info) => "```".fmt(f).and(info.fmt(f)),
                List(_) => Ok(()),
                Item => Ok(()),
            },
            _ => unimplemented!("TBD"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Unordered,
    Ordered(usize),
}

#[derive(Debug, Clone)]
pub struct ItemDisplay(pub ItemType);

impl Display for ItemDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ItemType::Unordered => "* ".fmt(f),
            ItemType::Ordered(n) => write!(f, "{}. ", n),
        }
    }
}
