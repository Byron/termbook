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

#[derive(Default)]
struct State {
    action: Option<Action>,
    code: String,
    error: Option<Error>,
}

fn event_filter<'a>(state: &mut &mut State, event: Event<'a>) -> Option<Vec<Event<'a>>> {
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
