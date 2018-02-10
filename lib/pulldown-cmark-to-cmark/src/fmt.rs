use std::fmt;
use std::borrow::Borrow;
use std::borrow::Cow;
use pulldown_cmark::Event;

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
    pub newlines_after_list: usize,
    pub newlines_after_rest: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            newlines_after_headline: 2,
            newlines_after_paragraph: 2,
            newlines_after_codeblock: 2,
            newlines_after_list: 2,
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
    fn padding<'a, F>(f: &mut F, p: &[Cow<'a, str>]) -> fmt::Result
    where
        F: fmt::Write,
    {
        for padding in p {
            write!(f, "{}", padding)?;
        }
        Ok(())
    }
    fn consume_newlines<F>(f: &mut F, s: &mut State) -> fmt::Result
    where
        F: fmt::Write,
    {
        while s.newlines_before_start != 0 {
            f.write_char('\n')?;
            padding(f, &s.padding)?;
            s.newlines_before_start -= 1;
        }
        Ok(())
    }

    fn print_text<'a, F>(t: &str, f: &mut F, p: &[Cow<'a, str>]) -> fmt::Result
    where
        F: fmt::Write,
    {
        if t.contains('\n') {
            let ntokens = t.split('\n').count();
            for (tid, token) in t.split('\n').enumerate() {
                f.write_str(token).and(if tid + 1 == ntokens {
                    Ok(())
                } else {
                    f.write_char('\n').and(padding(f, p))
                })?;
            }
            Ok(())
        } else {
            f.write_str(t)
        }
    }

    fn padding_of(l: Option<usize>) -> Cow<'static, str> {
        match l {
            None => "  ".into(),
            Some(n) => format!("{}. ", n)
                .chars()
                .map(|_| ' ')
                .collect::<String>()
                .into(),
        }
    }

    for event in events {
        use pulldown_cmark::Event::*;
        use pulldown_cmark::Tag::*;
        match *event.borrow() {
            Start(ref tag) => {
                match *tag {
                    BlockQuote => state.padding.push(" > ".into()),
                    List(ref list_type) => {
                        state.list_stack.push(list_type.clone());
                        if state.list_stack.len() > 1 {
                            state.newlines_before_start += options.newlines_after_rest;
                        }
                    }
                    _ => {}
                }
                let left_on_padded_newlines = state.newlines_before_start != 0;
                consume_newlines(&mut f, &mut state)?;
                match *tag {
                    Item => match state.list_stack.last() {
                        Some(inner) => {
                            state.padding.push(padding_of(*inner));
                            match inner {
                            &Some(n) => write!(f, "{}. ", n),
                            &None => f.write_str("* "),
                        }},
                        None => Ok(()),
                    },
                    Table(_) => Ok(()),
                    TableHead => Ok(()),
                    TableRow => Ok(()),
                    TableCell => f.write_char('|'),
                    Link(_, _) => f.write_char('['),
                    Image(_, _) => f.write_str("!["),
                    Emphasis => f.write_char('*'),
                    Strong => f.write_str("**"),
                    Code => f.write_char('`'),
                    FootnoteDefinition(ref name) => write!(f, "[^{}]: ", name),
                    Paragraph => Ok(()),
                    Rule => f.write_str("---"),
                    Header(n) => {
                        for _ in 0..n {
                            f.write_char('#')?;
                        }
                        f.write_char(' ')
                    }
                    BlockQuote => if !left_on_padded_newlines {
                        padding(&mut f, &state.padding)
                    } else {
                        Ok(())
                    },
                    CodeBlock(ref info) => f.write_str("```")
                        .and(f.write_str(info))
                        .and(f.write_char('\n'))
                        .and(padding(&mut f, &state.padding)),
                    List(_) => Ok(()),
                }
            }
            End(ref tag) => match *tag {
                Image(ref uri, ref title) | Link(ref uri, ref title) => {
                    if title.is_empty() {
                        write!(f, "]({})", uri)
                    } else {
                        write!(f, "]({uri} \"{title}\")", uri = uri, title = title)
                    }
                }
                Code => f.write_char('`'),
                TableCell => f.write_char('|'),
                Emphasis => f.write_char('*'),
                Strong => f.write_str("**"),
                Header(_) => {
                    if state.newlines_before_start < options.newlines_after_headline {
                        state.newlines_before_start += options.newlines_after_headline;
                    }
                    Ok(())
                }
                Paragraph => {
                    if state.newlines_before_start < options.newlines_after_paragraph {
                        state.newlines_before_start += options.newlines_after_paragraph;
                    }
                    Ok(())
                }
                CodeBlock(_) => {
                    state.newlines_before_start += options.newlines_after_codeblock;
                    f.write_str("```")
                }
                ref t @ Table(_) | ref t @ TableRow | ref t @ TableHead | ref t @ Rule | ref t @ Item => {
                    if let &Item = t {
                        state.padding.pop();
                    }
                    if state.newlines_before_start < options.newlines_after_rest {
                        state.newlines_before_start = options.newlines_after_rest;
                    }
                    Ok(())
                }
                List(_) => {
                    state.list_stack.pop();
                    if state.list_stack.len() == 0 && state.newlines_before_start < options.newlines_after_list {
                        state.newlines_before_start = options.newlines_after_list;
                    }
                    if !state.list_stack.is_empty() {
                        state.padding.pop();
                    }
                    Ok(())
                }
                BlockQuote => {
                    state.padding.pop();
                    Ok(())
                }
                FootnoteDefinition(_) => Ok(()),
            },
            HardBreak => f.write_str("  \n").and(padding(&mut f, &state.padding)),
            SoftBreak => f.write_char('\n').and(padding(&mut f, &state.padding)),
            Html(ref text) | Text(ref text) => {
                consume_newlines(&mut f, &mut state)?;
                print_text(text, &mut f, &state.padding)
            }
            InlineHtml(ref name) => f.write_str(name),
            FootnoteReference(ref name) => write!(f, "[^{}]", name),
        }?
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
