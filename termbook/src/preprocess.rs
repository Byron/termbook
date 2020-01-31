use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::errors::Result;
use mdbook::errors::Error;
use pulldown_cmark::{Event, Parser};
use pulldown_cmark_to_cmark::fmt::cmark;
use crate::{exclude_chapter, globset_from_strings};

use std::process::{Child, Command, Stdio};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;

/// A preprocessor which runs specifically tagged codeblocks.
pub struct RunCodeBlocks {
    globs: Vec<String>,
}

impl RunCodeBlocks {
    pub fn new(globs: Vec<String>) -> RunCodeBlocks {
        RunCodeBlocks { globs }
    }
}

const PREPROCESSOR_NAME: &str = "run-code-blocks";

enum Action {
    Exec {
        program: String,
        desired_exit_status: i32,
    },
    Hide,
    Prepare(String),
    IncludeFile(PathBuf),
    Use(String),
}

impl Action {
    fn from_str(program: &str, key: &str, val: Option<&str>) -> Result<Option<Action>> {
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
                Error::from("'use' tags need a name, like 'use=name'.")
            })?)),
            "prepare" => Some(Action::Prepare(val.map(ToOwned::to_owned).ok_or_else(
                || Error::from("'prepare' tags need a name, like 'prepare=name'."),
            )?)),
            "include-file" => Some(Action::IncludeFile(val.map(PathBuf::from).ok_or_else(
                || {
                    Error::from(
                        "'include-file' tags need a file name, like 'include-file=../file.md'.",
                    )
                },
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
    book_root: PathBuf,
}

impl State {
    fn is_in_marked_codeblock(&self) -> bool {
        !self.actions.is_empty()
    }

    fn should_hide(&self) -> bool {
        self.actions
            .iter()
            .any(|a| if let Action::Hide = *a { true } else { false })
    }

    fn apply_end_of_codeblock_actions(&mut self, events: &mut Vec<Event>, dry_run: bool) {
        for action in &self.actions {
            match *action {
                Action::IncludeFile(ref path) => {
                    let mut buf = String::new();
                    let file_path = self.book_root.join(path);
                    match File::open(&file_path) {
                        Ok(mut f) => match f.read_to_string(&mut buf) {
                            Ok(_) => {
                                if !buf.ends_with('\n') {
                                    buf.push('\n')
                                }
                                self.code.push_str(&buf);
                                let pos = events.len() - 1;
                                events.insert(pos, Event::Text(buf.into()));
                            }
                            Err(e) => {
                                self.error = Some(Error::from(e).chain_err(|| {
                                    format!("Could not read file at '{}'", file_path.display())
                                }))
                            }
                        },
                        Err(e) => {
                            self.error = Some(Error::from(e).chain_err(|| {
                                format!(
                                "include-file={} failed as the file at '{}' could not be opened",
                                path.display(), file_path.display())
                            }))
                        }
                    }
                }
                Action::Hide => {}
                Action::Use(ref id) => match self.prepare.get(id) {
                    Some(code) => self.code.insert_str(0, code),
                    None => {
                        self.error = Some(
                            format!(
                                "Reference named '{}' was not yet added with a 'prepare' block.",
                                id
                            ).into(),
                        )
                    }
                },
                Action::Prepare(ref id) => {
                    self.prepare.insert(id.to_owned(), self.code.clone());
                }
                Action::Exec {
                    ref program,
                    desired_exit_status,
                } => {
                    if dry_run {
                        return;
                    }
                    let spawn_result = Command::new(program)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .map_err(|e| {
                            format!("Failed to execute '{}' with error: {}", program, e).into()
                        })
                        .and_then(|mut c: Child| {
                            c.stdin
                                .as_mut()
                                .expect("stdin to be configured")
                                .write_all(self.code.as_bytes())
                                .and_then(|_| c.wait_with_output())
                                .map_err(Into::into)
                        });
                    match spawn_result {
                        Ok(mut output) => {
                            eprintln!(
                                "{}: Executed program '{}' with '{:?}'.",
                                PREPROCESSOR_NAME, program, self.code
                            );
                            let actual_exit_status = output.status.code().unwrap_or(1);
                            if actual_exit_status != desired_exit_status {
                                self.error = Some(
                                    format!(
                                        "After running '{}': Expected exit status '{}' to be '{}'\nstdout: {}\nstderr: {}",
                                        program, actual_exit_status, desired_exit_status,
                                        String::from_utf8_lossy(&output.stdout),
                                        String::from_utf8_lossy(&output.stderr),
                                    ).into(),
                                );
                            } else {
                                use pulldown_cmark::Event::*;
                                use pulldown_cmark::Tag::*;
                                events.push(Start(CodeBlock("output".into())));
                                events.push(Text({
                                    if let Some(c) = output.stdout.last().cloned() {
                                        if c != b'\n' {
                                            output.stdout.push(b'\n');
                                        }
                                    }
                                    String::from_utf8_lossy(&output.stdout).into_owned().into()
                                }));
                                events.push(Text({
                                    if let Some(c) = output.stderr.last().cloned() {
                                        if c != b'\n' {
                                            output.stderr.push(b'\n');
                                        }
                                    }
                                    String::from_utf8_lossy(&output.stderr).into_owned().into()
                                }));
                                events.push(End(CodeBlock("output".into())));
                            }
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
            }
        }
    }
}

fn parse_actions(info: &str) -> Result<Vec<Action>> {
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

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn event_filter<'a>(
    state: &mut &mut State,
    event: Event<'a>,
    dry_run: bool,
) -> Option<Vec<Event<'a>>> {
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
            if state.is_in_marked_codeblock() {
                state.code.push_str(text);
            }
            state.should_hide()
        }
        End(CodeBlock(_)) => {
            state.apply_end_of_codeblock_actions(&mut res, dry_run);
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

fn process_chapter(
    ctx: &PreprocessorContext,
    chapter: &mut Chapter,
    state: &mut State,
    dry_run: bool,
) -> Result<()> {
    state.book_root = ctx.root.clone();

    let md = {
        let mut md = String::with_capacity(chapter.content.len() + 128);
        {
            let parser = Parser::new(&chapter.content)
                .scan(state, |s, e| event_filter(s, e, dry_run))
                .flat_map(|events| events);
            cmark(parser, &mut md, None).map_err(|e| format!("{}", e))?;
        }
        md
    };
    chapter.content = md;

    Ok(())
}

impl Preprocessor for RunCodeBlocks {
    fn name(&self) -> &str {
        PREPROCESSOR_NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut state = State::default();
        let globs = globset_from_strings(&self.globs)?;
        let mut amount_of_included_chapters = 0;
        for item in book.sections.iter_mut() {
            if let BookItem::Chapter(ref mut chapter) = *item {
                let dry_run = exclude_chapter(&globs, chapter);
                if !dry_run {
                    amount_of_included_chapters += 1;
                }

                process_chapter(ctx, chapter, &mut state, dry_run)?;
                state.error = state.error.map(|err| {
                    err.chain_err(|| {
                        format!(
                            "{}: Preprocessing failed for chapter '{}' in file '{}'.",
                            PREPROCESSOR_NAME,
                            chapter.name,
                            chapter.path.display()
                        )
                    })
                });
            }
        };

        if let Some(error) = state.error {
            return Err(error);
        }
        if !globs.is_empty() && amount_of_included_chapters == 0 {
            return Err("globs did not match any chapter.".into());
        }
        Ok(book)
    }
}
