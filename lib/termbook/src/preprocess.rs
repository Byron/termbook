use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use super::MdBookResult;
use mdbook::book::Book;
use mdbook::errors::Error;
use pulldown_cmark::{Event, Parser};
use pulldown_cmark_to_cmark::fmt::cmark;

use std::process::{Child, Command, Stdio};
use std::io::Write;
use std::collections::HashMap;

pub struct RunCodeBlocks;

const PREPROCESSOR_NAME: &'static str = "run-code-blocks";

enum Action {
    Exec {
        program: String,
        desired_exit_status: i32,
    },
    Hide,
    Prepare(String),
    Use(String),
}

impl Action {
    fn from_str(program: &str, key: &str, val: Option<&str>) -> Result<Option<Action>, Error> {
        Ok(match key {
            "hide" => {
                if let Some(v) = val {
                    return Err(format!(
                        "Encountered value '{}' on 'hide' tag, which is not allowed.",
                        v
                    ).into());
                };
                Some(Action::Hide)
            }
            "use" => Some(Action::Use(val.map(ToOwned::to_owned).ok_or_else(|| {
                Error::from("'Use' tags need a name, like 'use=name'.")
            })?)),
            "prepare" => Some(Action::Prepare(val.map(ToOwned::to_owned).ok_or_else(
                || Error::from("'Prepare' tags need a name, like 'prepare=name'."),
            )?)),
            "exec" => Some(Action::Exec {
                program: program.to_owned(),
                desired_exit_status: match val {
                    Some(val) => val.parse().map_err(|e| {
                        format!(
                            "Failed to parse integer from '{}' for 'exec' key with error: {}",
                            val, e
                        )
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
    prepare: HashMap<String, String>,
}

impl State {
    fn should_hide(&self) -> bool {
        self.actions
            .iter()
            .any(|a| if let &Action::Hide = a { true } else { false })
    }
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

    let mut res = vec![event.clone()];
    let hide = match event {
        Start(CodeBlock(ref info)) => {
            state.actions = match parse_actions(info) {
                Ok(a) => a,
                Err(e) => {
                    state.error = Some(e);
                    Vec::new()
                }
            };
            state.should_hide()
        }
        Text(ref text) => {
            if !state.actions.is_empty() {
                state.code.push_str(text);
            }
            state.should_hide()
        }
        End(CodeBlock(_)) => {
            for action in &state.actions {
                match *action {
                    Action::Hide => {}
                    Action::Use(ref id) => match state.prepare.get(id) {
                        Some(code) => state.code.insert_str(0, code),
                        None => {
                            state.error = Some(
                                format!(
                                "Reference named '{}' was not yet added with a 'prepare' block.",
                                id
                            ).into(),
                            )
                        }
                    },
                    Action::Prepare(ref id) => {
                        state.prepare.insert(id.to_owned(), state.code.clone());
                    }
                    Action::Exec {
                        ref program,
                        desired_exit_status,
                    } => {
                        let spawn_result = Command::new(program)
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn()
                            .map_err(Into::into)
                            .and_then(|mut c: Child| {
                                c.stdin
                                    .as_mut()
                                    .expect("stdin to be configured")
                                    .write_all(state.code.as_bytes())
                                    .and_then(|_| c.wait_with_output())
                                    .map_err(Into::into)
                            });
                        match spawn_result {
                            Ok(output) => {
                                let actual_exit_status = output.status.code().unwrap_or(1);
                                if actual_exit_status != desired_exit_status {
                                    state.error = Some(
                                        format!(
                                            "After running '{}': Expected exit status '{}' to be '{}'",
                                            program,
                                            actual_exit_status, desired_exit_status
                                        ).into(),
                                    );
                                } else {
                                    res.push(Start(CodeBlock("output".into())));
                                    res.push(Text(
                                        String::from_utf8_lossy(&output.stdout).into_owned().into(),
                                    ));
                                    res.push(Text(
                                        String::from_utf8_lossy(&output.stderr).into_owned().into(),
                                    ));
                                    res.push(End(CodeBlock("output".into())));
                                }
                            }
                            Err(e) => state.error = Some(e),
                        }
                    }
                }
            }
            let hide = state.should_hide();
            state.actions.clear();
            state.code.clear();
            hide
        }
        _ => state.should_hide(),
    };
    if hide {
        res.clear();
    }
    Some(res)
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
            return Err(err.chain_err(|| {
                format!(
                    "{}: Preprocessing failed for chapter '{}' in file '{}'.",
                    PREPROCESSOR_NAME,
                    chapter.name,
                    chapter.path.display()
                )
            }));
        }
        chapter.content = md;
    }
    Ok(())
}

impl Preprocessor for RunCodeBlocks {
    fn name(&self) -> &str {
        PREPROCESSOR_NAME
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
