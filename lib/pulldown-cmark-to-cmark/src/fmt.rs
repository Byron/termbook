use std::fmt;
use std::borrow::Borrow;
use pulldown_cmark::Event;
use display;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub newlines_before_start: usize,
    pub list_stack: Vec<Option<usize>>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub newlines_after_headline: usize,
    pub newlines_after_paragraph: usize,
    pub newlines_after_rest: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            newlines_after_headline: 2,
            newlines_after_paragraph: 2,
            newlines_after_rest: 1,
        }
    }
}

pub fn cmark_with_options<'a, I, E, F>(
    events: I,
    mut f: F,
    state: Option<State>,
    options: Options,
) -> Result<State, fmt::Error>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    F: fmt::Write,
{
    let mut state = state.unwrap_or_default();
    for event in events {
        use pulldown_cmark::Event::*;
        use pulldown_cmark::Tag::*;
        match *event.borrow() {
            ref e @ Html(_) | ref e @ Start(_) => {
                match *e {
                    Start(List(ref list_type)) => state.list_stack.push(list_type.clone()),
                    _ => {}
                }
                while state.newlines_before_start != 0 {
                    f.write_char('\n')?;
                    state.newlines_before_start -= 1;
                }
            }
            End(ref tag) => match *tag {
                Header(_) => state.newlines_before_start += options.newlines_after_headline,
                Paragraph => state.newlines_before_start += options.newlines_after_paragraph,
                Table(_) | TableRow | TableHead | Rule | CodeBlock(_) | Item => {
                    state.newlines_before_start += options.newlines_after_rest
                }
                List(_) => {
                    state.list_stack.pop();
                }
                BlockQuote
                | Strong
                | Emphasis
                | Code
                | Image(_, _)
                | Link(_, _)
                | TableCell
                | FootnoteDefinition(_) => {}
            },
            _ => {}
        }
        match *event.borrow() {
            Event::Start(Item) => {
                match state.list_stack.last() {
                    Some(&Some(n)) => write!(f, "{}", display::Item(display::ItemType::Ordered(n))),
                    Some(&None) => write!(f, "{}", display::Item(display::ItemType::Unordered)),
                    None => Ok(()),
                }
            },
            _ => write!(f, "{}", display::Event(event.borrow())),
        }?;
    }
    Ok(state)
}

pub fn cmark<'a, I, E, F>(events: I, f: F, state: Option<State>) -> Result<State, fmt::Error>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    F: fmt::Write,
{
    cmark_with_options(events, f, state, Options::default())
}
