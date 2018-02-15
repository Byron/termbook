use mdbook::renderer::{RenderContext, Renderer};
use mdbook::book::BookItem;
use mdbook::errors::Error;
use syntect::parsing::SyntaxSet;
use pulldown_cmark::{Event, Parser, Tag};

use mdcat::{push_tty, ResourceAccess, Terminal, TerminalSize};
use atty::{self, Stream};

use std::io::{self, stdout, Write};
use std::env::current_dir;
use std::fmt::Write as FmtWrite;
use std::thread::sleep;
use std::time::Duration;
use std::str;

pub struct Playback {
    delay_per_character: Duration,
}

impl Playback {
    pub fn new(characters_per_second: usize) -> Playback {
        Playback {
            delay_per_character: Duration::from_millis(
                (1000.0 / characters_per_second as f32) as u64,
            ),
        }
    }
}

struct DelayPrinter<W>
where
    W: Write,
{
    is_a_tty: bool,
    delay_per_character: Duration,
    terminal_write_level: usize,
    inner: W,
}

impl<W> DelayPrinter<W>
where
    W: Write,
{
    fn new(w: W, delay_per_character: Duration) -> DelayPrinter<W> {
        DelayPrinter {
            delay_per_character,
            inner: w,
            is_a_tty: atty::is(Stream::Stdout),
            terminal_write_level: 0,
        }
    }

    fn is_not_writing_terminal_escape_code(&self) -> bool {
        self.terminal_write_level == 0
    }
}

impl<W> Write for DelayPrinter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        if buf == &[0x1b, 0x5d] || buf == &[0x1b, 0x5b] {
            self.terminal_write_level += 1;
        } else if buf == &[0x07] || buf == &[0x6d] {
            self.terminal_write_level -= 1;
        }

        if self.is_a_tty && self.is_not_writing_terminal_escape_code() {
            match str::from_utf8(buf) {
                Ok(s) => for c in s.chars() {
                    sleep(self.delay_per_character);
                    write!(self.inner, "{}", c)?;
                    self.inner.flush().ok();
                },
                Err(_) => for b in buf {
                    sleep(self.delay_per_character);
                    self.inner.write(&[*b])?;
                    self.inner.flush().ok();
                },
            }
            Ok(buf.len())
        } else {
            self.inner.write(buf)
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.inner.flush()
    }
}

impl Renderer for Playback {
    fn name(&self) -> &str {
        "playback"
    }

    fn render(&self, ctx: &RenderContext) -> Result<(), Error> {
        let cd = current_dir()?;
        let mut events = Vec::new();
        for (item_id, item) in ctx.book.iter().enumerate() {
            if let &BookItem::Chapter(ref chapter) = item {
                if item_id != 0 {
                    events.push(Event::SoftBreak);
                }
                events.push(Event::Start(Tag::Strong));
                let mut buf = String::new();
                if let Some(ref section_number) = chapter.number {
                    write!(buf, "{} ", section_number).ok();
                }
                buf.push_str(&chapter.name);

                let buf_len = buf.len() + 2;
                events.push(Event::SoftBreak);
                events.push(Event::Start(Tag::Header(1)));
                events.push(Event::Text(buf.into()));
                events.push(Event::End(Tag::Header(1)));
                events.push(Event::Text(
                    (0..buf_len).map(|_| '=').collect::<String>().into(),
                ));
                events.push(Event::End(Tag::Strong));
                events.push(Event::SoftBreak);

                events.extend(Parser::new(&chapter.content));
            }
        }
        push_tty(
            &mut DelayPrinter::new(stdout(), self.delay_per_character.clone()),
            Terminal::detect(),
            TerminalSize::detect().unwrap_or_default(),
            events.into_iter(),
            &cd,
            ResourceAccess::LocalOnly,
            SyntaxSet::load_defaults_newlines(),
        ).map_err(|e| Error::from(format!("{}", e)))?;
        Ok(())
    }
}
