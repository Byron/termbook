pub extern crate mdbook;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use mdbook::MDBook;
use std::path::Path;

mod render;
mod preprocess;

pub use render::*;
pub use preprocess::*;

/// Open and load an `mdbook` at the given `dir`ectory.
/// The `RunCodeBlocks` preprocessor will be added to it.
pub fn load(dir: &Path) -> mdbook::errors::Result<MDBook> {
    let mut md = MDBook::load(dir)?;
    md.with_preprecessor(RunCodeBlocks);
    Ok(md)
}
