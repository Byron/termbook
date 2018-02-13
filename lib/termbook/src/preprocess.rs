use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use super::MdBookResult;
use mdbook::book::Book;
use mdbook::errors::Error;
use pulldown_cmark::{Event, Parser};
use pulldown_cmark_to_cmark::fmt::cmark;

pub struct RunShellScript;

enum Action {
    Exec {
        program: String,
        desired_exit_status: usize,
    },
}

impl Action {
    fn from_str(program: &str, key: &str, val: Option<&str>) -> Result<Option<Action>, Error> {
        Ok(match key {
            "exec" => Some(Action::Exec {
                program: program.to_owned(),
                desired_exit_status: match val {
                    Some(val) => val.parse().map_err(|e| {
                        format!("Failed to parse integer for exec key with error: {}", e)
                    })?,
                    None => 0,
                },
            }),
            _ => None,
        })
    }
}

#[derive(Default)]
struct State {
    actions: Vec<Action>,
    code: String,
    error: Option<Error>,
}

fn parse_actions(info: &str) -> Result<Vec<Action>, Error> {
    let mut res = Vec::new();
    let mut shell = "bash";
    for (tid, token) in info.trim().split(',').enumerate() {
        if tid == 0 {
            shell = token;
        } else {
            let mut kvi = token.splitn(2, '=');
            let optional_action = match (kvi.next().map(str::trim), kvi.next().map(str::trim)) {
                (Some(key), possible_value) => Action::from_str(shell, key, possible_value)?,
                _ => None,
            };
            if let Some(action) = optional_action {
                res.push(action);
            }
        }
    }
    Ok(res)
}

fn event_filter<'a>(state: &mut &mut State, event: Event<'a>) -> Option<Vec<Event<'a>>> {
    use pulldown_cmark::Event::*;
    use pulldown_cmark::Tag::*;

    match event {
        Start(CodeBlock(ref info)) => {
            state.actions = match parse_actions(info) {
                Ok(a) => a,
                Err(e) => {
                    state.error = Some(e);
                    Vec::new()
                }
            };
        }
        Text(ref text) => {
            if !state.actions.is_empty() {
                state.code.push_str(text);
            }
        }
        End(CodeBlock(_)) => {
            for action in &state.actions {
                match *action {
                    Action::Exec {
                        ref program,
                        desired_exit_status,
                    } => {}
                }
            }
            state.actions.clear();
            state.code.clear();
        }
        _ => {}
    };
    Some(vec![event])
}

fn process_chapter(item: &mut BookItem) -> Result<(), Error> {
    let mut state = State::default();
    if let &mut BookItem::Chapter(ref mut chapter) = item {
        let md = {
            let mut md = String::with_capacity(chapter.content.len() + 128);
            {
                let parser = Parser::new(&chapter.content)
                    .scan(&mut state, event_filter)
                    .flat_map(|events| events);
                cmark(parser, &mut md, None).map_err(|e| format!("{}", e))?;
            }
            md
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
