extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark_to_cmark::fmt::{cmark, State};
use pulldown_cmark::{Event, Options, Parser, Tag};

fn fmt(s: &str) -> (String, State) {
    let mut buf = String::new();
    let s = cmark(Parser::new_ext(s, Options::all()), &mut buf, None).unwrap();
    (buf, s)
}

fn fmte(e: &[Event], s: State) -> (String, State) {
    let mut buf = String::new();
    let s = cmark(e.iter(), &mut buf, Some(s)).unwrap();
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

#[test]
fn it_applies_newlines_before_start_before_any_start_tag() {
    assert_eq!(
        fmte(
            &[Event::Start(Tag::Paragraph), Event::Text("h".into())],
            State {
                newlines_before_start: 2,
            }
        ),
        (
            "\n\nh".into(),
            State {
                newlines_before_start: 0,
            }
        )
    )
}
