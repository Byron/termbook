extern crate atty;
extern crate globset;
pub extern crate mdbook;
extern crate mdcat;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;
extern crate syntect;

use globset::{Glob, GlobSet, GlobSetBuilder};
use mdbook::MDBook;
use std::path::Path;
use mdbook::book::Chapter;

mod rewrite;
mod playback;
mod preprocess;

pub use rewrite::*;
pub use preprocess::*;
pub use playback::*;

fn exclude_chapter(globs: &GlobSet, chapter: &Chapter) -> bool {
    if !globs.is_empty() && !globs.is_match(&Path::new(&chapter.name)) {
        let mut is_match = false;
        if let Some(ref section_number) = chapter.number {
            let section_number = format!("{}", section_number);
            is_match = globs.is_match(&Path::new(&section_number))
        }
        if !is_match {
            return true;
        }
    }
    false
}

fn globset_from_strings(globs: &[String]) -> Result<GlobSet, mdbook::errors::Error> {
    globs
        .iter()
        .filter_map(|s| Glob::new(s).ok())
        .fold(GlobSetBuilder::new(), |mut b, g| {
            b.add(g);
            b
        })
        .build()
        .map_err(|e| mdbook::errors::Error::from(format!("{}", e)))
}

/// Open and load an `mdbook` at the given `dir`ectory.
/// The `RunCodeBlocks` preprocessor will be added to it.
pub fn load(dir: &Path, globs: Vec<String>) -> mdbook::errors::Result<MDBook> {
    let mut md = MDBook::load(dir)?;
    md.with_preprocessor(RunCodeBlocks::new(globs));
    Ok(md)
}
