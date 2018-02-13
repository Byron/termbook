pub extern crate mdbook;
extern crate mdcat;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;
extern crate syntect;

use mdbook::MDBook;
use std::path::Path;

mod rewrite;
mod playback;
mod preprocess;

pub use rewrite::*;
pub use preprocess::*;
pub use playback::*;

/// Open and load an `mdbook` at the given `dir`ectory.
/// The `RunCodeBlocks` preprocessor will be added to it.
pub fn load(dir: &Path) -> mdbook::errors::Result<MDBook> {
    let mut md = MDBook::load(dir)?;
    md.with_preprecessor(RunCodeBlocks);
    Ok(md)
}
