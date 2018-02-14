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

pub struct Playback;

impl Playback {
    pub fn new() -> Playback {
        Playback
    }
}

struct DelayPrinter<W>
where
    W: Write,
{
    is_a_tty: bool,
    inner: W,
}

impl<W> DelayPrinter<W>
where
    W: Write,
{
    fn new(w: W) -> DelayPrinter<W> {
        DelayPrinter {
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
            sleep(Duration::from_millis(20));
            let res = self.inner.write(buf);
            self.inner.flush().ok();
            res
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
        for item in ctx.book.iter() {
            if let &BookItem::Chapter(ref chapter) = item {
                let syntax_set = SyntaxSet::load_defaults_newlines();
                push_tty(
                    &mut DelayPrinter::new(stdout()),
                    Terminal::BasicAnsi,
                    TerminalSize {
                        width: 80,
                        height: 30,
                    },
                    Parser::new(&chapter.content),
                    &cd,
                    ResourceAccess::LocalOnly,
                    syntax_set,
                ).map_err(|e| Error::from(format!("{}", e)))?;
            }
        }
        Ok(())
    }
}
