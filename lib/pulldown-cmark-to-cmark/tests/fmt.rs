extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark_to_cmark::fmt::{cmark, State};
use pulldown_cmark::{Options, Parser};

fn fmt(s: &str) -> (String, State) {
    let mut buf = String::new();
    let s = cmark(Parser::new_ext(s, Options::all()), &mut buf, None).unwrap();
    (buf, s)
}

mod lazy_newlines {
    use super::fmt as f;
    use super::State;

    #[test]
    fn after_headline() {
        assert_eq!(
            f("## headline"),
            (
                "## headline".into(),
                State {
                    newlines_before_start: 2,
                }
            )
        )
    }
}
