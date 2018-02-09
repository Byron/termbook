extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark_to_cmark::fmt::{cmark, State};
use pulldown_cmark::{Event, Options, Parser, Tag};

fn fmts(s: &str) -> (String, State<'static>) {
    let mut buf = String::new();
    let s = cmark(Parser::new_ext(s, Options::all()), &mut buf, None).unwrap();
    (buf, s)
}

fn fmtes(e: &[Event], s: State<'static>) -> (String, State<'static>) {
    let mut buf = String::new();
    let s = cmark(e.iter(), &mut buf, Some(s)).unwrap();
    (buf, s)
}

fn fmte(e: &[Event]) -> (String, State<'static>) {
    let mut buf = String::new();
    let s = cmark(e.iter(), &mut buf, None).unwrap();
    (buf, s)
}

mod lazy_newlines {
    use super::{fmte, fmts};
    use super::{Event, State, Tag};

    #[test]
    fn after_emphasis_there_is_no_newline() {
        for t in &[
            Tag::Emphasis,
            Tag::Strong,
            Tag::Code,
            Tag::List(None),
            Tag::List(Some(1)),
            Tag::BlockQuote,
            Tag::Link("".into(), "".into()),
            Tag::Image("".into(), "".into()),
            Tag::FootnoteDefinition("".into()),
        ] {
            assert_eq!(
                fmte(&[Event::End(t.clone())]).1,
                State {
                    newlines_before_start: 0,
                    ..Default::default()
                }
            )
        }
    }

    #[test]
    fn after_anything_else_it_has_one_newline() {
        for e in &[
            Event::End(Tag::CodeBlock("".into())),
            Event::End(Tag::Item),
            Event::End(Tag::Rule),
            Event::End(Tag::TableRow),
            Event::End(Tag::TableHead),
            Event::End(Tag::Table(vec![])),
        ] {
            assert_eq!(
                fmte(&[e.clone()]).1,
                State {
                    newlines_before_start: 1,
                    ..Default::default()
                }
            )
        }
    }

    #[test]
    fn after_some_types_it_has_multiple_newlines() {
        for md in &["paragraph", "## headline"] {
            assert_eq!(
                fmts(md),
                (
                    String::from(*md),
                    State {
                        newlines_before_start: 2,
                        ..Default::default()
                    }
                )
            )
        }
    }
}

#[test]
fn it_applies_newlines_before_start_before_html() {
    assert_eq!(
        fmtes(
            &[Event::Html("<e>".into())],
            State {
                newlines_before_start: 2,
                ..Default::default()
            }
        ),
        (
            "\n\n<e>".into(),
            State {
                newlines_before_start: 0,
                ..Default::default()
            }
        )
    )
}

#[test]
fn it_applies_newlines_before_start_before_any_start_tag() {
    assert_eq!(
        fmtes(
            &[Event::Start(Tag::Paragraph), Event::Text("h".into())],
            State {
                newlines_before_start: 2,
                ..Default::default()
            }
        ),
        (
            "\n\nh".into(),
            State {
                newlines_before_start: 0,
                ..Default::default()
            }
        )
    )
}

mod padding {
    use super::{fmtes, Event, State, Tag};

    #[test]
    fn is_used_before_newlines() {
        assert_eq!(
            fmtes(
                &[Event::Start(Tag::Paragraph), Event::Text("h".into())],
                State {
                    newlines_before_start: 2,
                    padding: vec!["  ".into()],
                    ..Default::default()
                }
            ),
            (
                "\n  \n  h".into(),
                State {
                    newlines_before_start: 0,
                    padding: vec!["  ".into()],
                    ..Default::default()
                }
            )
        )
    }
}

mod list {
    use super::{fmte, fmtes, fmts, Event, State, Tag};

    #[test]
    fn it_pushes_one_item_to_the_lists_stack_for_each_start_list() {
        assert_eq!(
            fmte(&[
                Event::Start(Tag::List(None)),
                Event::Start(Tag::List(Some(42)))
            ]).1,
            State {
                list_stack: vec![None, Some(42)],
                padding: vec!["  ".into()],
                ..Default::default()
            }
        )
    }

    #[test]
    fn all_but_the_first_list_end_pop_the_padding() {
        assert_eq!(
            fmtes(
                &[
                    Event::End(Tag::List(None)),
                    Event::End(Tag::List(Some(444))),
                    Event::End(Tag::List(None))
                ],
                State {
                    list_stack: vec![None, Some(444), None],
                    padding: vec!["foo".into(), String::from("bar").into(), "baz".into()],
                    ..Default::default()
                }
            ).1,
            State {
                padding: vec!["foo".into()],
                ..Default::default()
            }
        )
    }

    #[test]
    fn all_but_the_first_list_append_to_the_padding() {
        assert_eq!(
            fmte(&[
                Event::Start(Tag::List(None)),
                Event::Start(Tag::List(Some(444))),
                Event::Start(Tag::List(None))
            ]).1,
            State {
                list_stack: vec![None, Some(444), None],
                padding: vec![String::from("  ").into(), "     ".into()],
                ..Default::default()
            }
        )
    }

    #[test]
    fn it_pops_one_item_from_the_lists_stack_for_each_end_list() {
        assert_eq!(
            fmtes(
                &[Event::End(Tag::List(None))],
                State {
                    list_stack: vec![None, None],
                    ..Default::default()
                }
            ).1,
            State {
                list_stack: vec![None],
                ..Default::default()
            }
        )
    }

    #[test]
    fn ordered_and_unordered() {
        assert_eq!(
            fmts("1. *b*\n   * *b*"),
            (
                "1. *b*\n   * *b*".into(),
                State {
                    newlines_before_start: 2,
                    ..Default::default()
                }
            )
        )
    }

    #[test]
    fn unordered() {
        assert_eq!(
            fmts("* a\n* b"),
            (
                "* a\n* b".into(),
                State {
                    newlines_before_start: 1,
                    ..Default::default()
                }
            )
        )
    }

    #[test]
    fn ordered() {
        assert_eq!(
            fmts("2. a\n2. b"),
            (
                "2. a\n2. b".into(),
                State {
                    newlines_before_start: 1,
                    ..Default::default()
                }
            )
        )
    }
}
