use std::fmt;
use std::borrow::Borrow;
use pulldown_cmark::Event;
use display;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub newlines_before_start: usize,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub newlines_after_headline: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            newlines_after_headline: 2,
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
            End(ref tag) => match *tag {
                Header(_) => state.newlines_before_start += options.newlines_after_headline,
                _ => {},
            },
            _ => {},
        }
        write!(f, "{}", display::Event(event.borrow()))?;
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
