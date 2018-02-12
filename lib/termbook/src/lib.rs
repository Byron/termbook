extern crate mdbook;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use mdbook::MDBook;
use mdbook::errors::Result as MdBookResult;
use std::path::Path;

mod render;
mod preprocess;

pub use render::*;
pub use preprocess::*;

pub fn new(dir: &Path) -> MdBookResult<MDBook> {
    let mut md = MDBook::load(dir)?;
    md.with_preprecessor(RunShellScript);
    Ok(md)
}
