use mdbook::renderer::{RenderContext, Renderer};
use mdbook::book::BookItem;
use mdbook::errors::Result;

use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;

/// An implementation of a renderer which writes all preprocessed markdown files.
///
/// This can be useful for debugging.
#[derive(Default)]
pub struct Rewrite;

impl Renderer for Rewrite {
    fn name(&self) -> &str {
        "markdown-rewrite"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        for item in ctx.book.iter() {
            if let BookItem::Chapter(ref chapter) = *item {
                let dir = ctx.destination
                    .join(&chapter.path.parent().expect("at least filename"));
                create_dir_all(&dir)?;
                let output_file = dir.join(&chapter.path.file_name().expect("a filename to be present"));
                let mut fout = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&output_file)?;
                fout.write_all(chapter.content.as_bytes())?;
                eprintln!("Wrote markdown file at '{}'.", chapter.path.display());
            }
        }
        Ok(())
    }
}
