use mdbook::renderer::{RenderContext, Renderer};
use mdbook::book::BookItem;
use super::MdBookResult;

use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;

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

    fn render(&self, ctx: &RenderContext) -> MdBookResult<()> {
        for item in ctx.book.iter() {
            if let &BookItem::Chapter(ref chapter) = item {
                let dir = ctx.destination
                    .join(&chapter.path.parent().expect("at least filename"));
                create_dir_all(&dir)?;
                let output_file = dir.join(&chapter.path);
                let mut fout = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&output_file)?;
                fout.write_all(chapter.content.as_bytes())?;
                eprintln!("Wrote markdown file at '{}'.", output_file.display());
            }
        }
        Ok(())
    }
}
