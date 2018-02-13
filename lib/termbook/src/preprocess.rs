use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use super::MdBookResult;
use mdbook::book::Book;
use mdbook::errors::Error;
use pulldown_cmark::{Event, Parser};
use pulldown_cmark_to_cmark::fmt::cmark;

pub struct RunShellScript;

enum Action {
    Exec(usize),
}

impl Action {
    fn from_str(key: &str, val: Option<&str>) -> Option<Action> {
        match key {
            "exec" => Some(Action::Exec(0)),
            _ => None,
        }
    }
}

#[derive(Default)]
struct State {
    actions: Option<Vec<Action>>,
    code: String,
    error: Option<Error>,
}

fn parse_actions(info: &str) -> Option<Vec<Action>> {
    let mut res = None::<Vec<_>>;
    for token in info.trim().split(',') {
        let mut kvi = token.splitn(2, '=');
        if let Some(action) = match (kvi.next().map(str::trim), kvi.next().map(str::trim)) {
            (Some(key), possible_value) => Action::from_str(key, possible_value),
            _ => None,
        } {
            res = match res {
                Some(mut v) => {
                    v.push(action);
                    Some(v)
                }
                None => Some(vec![action]),
            }
        }
    }
    res
}

fn event_filter<'a>(state: &mut &mut State, event: Event<'a>) -> Option<Vec<Event<'a>>> {
    use pulldown_cmark::Event::*;
    use pulldown_cmark::Tag::*;

    match event {
        Start(CodeBlock(ref info)) => {
            state.actions = parse_actions(info);
        }
        _ => {}
    };
    Some(vec![event])
}

fn process_chapter(item: &mut BookItem) -> Result<(), Error> {
    if let &mut BookItem::Chapter(ref mut chapter) = item {
        let (md, state) = {
            let mut state = State::default();
            let mut md = String::with_capacity(chapter.content.len() + 128);
            {
                let parser = Parser::new(&chapter.content)
                    .scan(&mut state, event_filter)
                    .flat_map(|events| events);
                cmark(parser, &mut md, None).map_err(|e| format!("{}", e))?;
            }
            (md, state)
        };
        if let Some(err) = state.error {
            return Err(err);
        }
        chapter.content = md;
    }
    Ok(())
}

impl Preprocessor for RunShellScript {
    fn name(&self) -> &str {
        "run_shell_scripts"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: &mut Book) -> MdBookResult<()> {
        let mut result = Ok(());
        book.for_each_mut(|item: &mut BookItem| {
            if result.is_err() {
                return;
            }
            if let Err(err) = process_chapter(item) {
                result = Err(err);
            }
        });
        result
    }
}
