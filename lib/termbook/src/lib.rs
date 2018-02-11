extern crate mdbook;

use mdbook::{BookItem, MDBook};
use mdbook::book::Book;
use mdbook::renderer::{RenderContext, Renderer};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::errors::Result as MdBookResult;
use std::path::Path;

pub struct RunShellScript;

impl Preprocessor for RunShellScript {
    fn name(&self) -> &str {
        "run_shell_scripts"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: &mut Book) -> MdBookResult<()> {
        book.for_each_mut(|item: &mut BookItem| println!("{:?}", item));
        Ok(())
    }
}

pub struct Rewrite;

impl Rewrite {
    pub fn new() -> Self {
        Rewrite
    }
}

impl Renderer for Rewrite {
    fn name(&self) -> &str {
        "markdown-rewrite"
    }

    fn render(&self, _ctx: &RenderContext) -> MdBookResult<()> {
        unimplemented!()
    }
}

pub fn new(dir: &Path) -> MdBookResult<MDBook> {
    let mut md = MDBook::load(dir)?;
    md.with_preprecessor(RunShellScript);
    Ok(md)
}
