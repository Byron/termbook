use std::fmt;
use pulldown_cmark::Event;
use display;

#[derive(Default, Clone, Debug)]
pub struct CmarkState;

pub fn cmark<'a, I, E, F>(
    events: I,
    mut f: F,
    state: Option<CmarkState>,
) -> Result<CmarkState, fmt::Error>
where
    I: Iterator<Item = E>,
    E: ::std::borrow::Borrow<Event<'a>>,
    F: fmt::Write,
{
    let state = state.unwrap_or_default();
    for event in events {
        write!(f, "{}", display::Event(event.borrow()))?;
    }
    Ok(state)
}
