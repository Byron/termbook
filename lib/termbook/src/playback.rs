use mdbook::renderer::{RenderContext, Renderer};
use mdbook::book::BookItem;
use mdbook::errors::Error;
use syntect::parsing::SyntaxSet;
use pulldown_cmark::Parser;

use mdcat::{push_tty, ResourceAccess, Terminal, TerminalSize};
use atty::{self, Stream};

use std::io::{self, stdout, Write};
use std::env::current_dir;
use std::thread::sleep;
use std::time::Duration;

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
        }
    }
}

impl<W> Write for DelayPrinter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        if self.is_a_tty {
            for b in buf {
                sleep(self.delay_per_character);
                self.inner.write(&[*b])?;
                self.inner.flush().ok();
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
        for item in ctx.book.iter() {
            if let &BookItem::Chapter(ref chapter) = item {
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
