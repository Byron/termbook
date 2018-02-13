use mdbook::renderer::{RenderContext, Renderer};
use mdbook::book::BookItem;
use mdbook::errors::Error;
use syntect::parsing::SyntaxSet;
use pulldown_cmark::Parser;

use mdcat::{push_tty, ResourceAccess, Terminal, TerminalSize};

use std::io::stdout;
use std::env::current_dir;

pub struct Playback;

impl Playback {
    pub fn new() -> Playback {
        Playback
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
                    &mut stdout(),
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
