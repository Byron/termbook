use std::fmt;
use std::borrow::Borrow;
use std::borrow::Cow;
use pulldown_cmark::Event;
use display;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct State<'a> {
    pub newlines_before_start: usize,
    pub list_stack: Vec<Option<usize>>,
    pub padding: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub newlines_after_headline: usize,
    pub newlines_after_paragraph: usize,
    pub newlines_after_codeblock: usize,
    pub newlines_after_rest: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            newlines_after_headline: 2,
            newlines_after_paragraph: 2,
            newlines_after_codeblock: 2,
            newlines_after_rest: 1,
        }
    }
}

pub fn cmark_with_options<'a, I, E, F>(
    events: I,
    mut f: F,
    state: Option<State<'static>>,
    options: Options,
) -> Result<State<'static>, fmt::Error>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    F: fmt::Write,
{
    let mut state = state.unwrap_or_default();
    fn with_padding<F>(f: &mut F, p: &[Cow<'static, str>]) -> fmt::Result
    where
        F: fmt::Write,
    {
        for padding in p {
            write!(f, "{}", padding)?;
        }
        Ok(())
    }

    for event in events {
        use pulldown_cmark::Event::*;
        use pulldown_cmark::Tag::*;
        match *event.borrow() {
            ref e @ Html(_) | ref e @ Start(_) => {
                match *e {
                    Start(BlockQuote) => state.padding.push(" > ".into()),
                    Start(List(ref list_type)) => {
                        state.list_stack.push(list_type.clone());
                        if state.list_stack.len() > 1 {
                            state.padding.push(
                                match state.list_stack[state.list_stack.len() - 2] {
                                    None => "  ".into(),
                                    Some(n) => format!("{}. ", n)
                                        .chars()
                                        .map(|_| ' ')
                                        .collect::<String>()
                                        .into(),
                                },
                            );
                            state.newlines_before_start += options.newlines_after_rest;
                        }
                    }
                    _ => {}
                }
                while state.newlines_before_start != 0 {
                    f.write_char('\n')?;
                    with_padding(&mut f, &state.padding)?;
                    state.newlines_before_start -= 1;
                }
            }
            End(ref tag) => match *tag {
                Header(_) => state.newlines_before_start += options.newlines_after_headline,
                Paragraph => state.newlines_before_start += options.newlines_after_paragraph,
                CodeBlock(_) => state.newlines_before_start += options.newlines_after_codeblock,
                Table(_) | TableRow | TableHead | Rule | Item => {
                    if state.newlines_before_start < options.newlines_after_rest {
                        state.newlines_before_start = options.newlines_after_rest
                    }
                }
                List(_) => {
                    state.list_stack.pop();
                    if !state.list_stack.is_empty() {
                        state.padding.pop();
                    }
                }
                BlockQuote => {
                    state.padding.pop();
                }
                Strong
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
            Event::Start(Item) => match state.list_stack.last() {
                Some(inner) => write!(
                    f,
                    "{}",
                    match inner {
                        &Some(n) => display::Item(display::ItemType::Ordered(n)),
                        &None => display::Item(display::ItemType::Unordered),
                    }
                ),
                None => Ok(()),
            },
            _ => write!(f, "{}", display::Event(event.borrow())),
        }?;
    }
    Ok(state)
}

pub fn cmark<'a, I, E, F>(
    events: I,
    f: F,
    state: Option<State<'static>>,
) -> Result<State<'static>, fmt::Error>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    F: fmt::Write,
{
    cmark_with_options(events, f, state, Options::default())
}
