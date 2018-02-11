use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use super::MdBookResult;
use mdbook::book::Book;

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
