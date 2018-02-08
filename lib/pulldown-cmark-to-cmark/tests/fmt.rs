extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark_to_cmark::fmt::cmark;
use pulldown_cmark::{Options, Parser};

fn fmt(s: &str) -> String {
    let mut buf = String::new();
    cmark(Parser::new_ext(s, Options::all()), &mut buf, None).unwrap();
    buf
}

mod lazy_newlines {
    use super::fmt as f;

    #[test]
    fn after_headline() {
        assert_eq!(f("## headline"), "## headline")
    }
}
