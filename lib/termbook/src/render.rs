use mdbook::renderer::{RenderContext, Renderer};
use super::MdBookResult;

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
