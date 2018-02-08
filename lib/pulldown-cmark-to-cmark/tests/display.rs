extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use pulldown_cmark::Event;
use pulldown_cmark_to_cmark::{EventDisplay, ItemDisplay, ItemType};

fn s(e: Event) -> String {
    format!("{}", EventDisplay(&e))
}

mod start {
    use pulldown_cmark::Event::*;
    use pulldown_cmark::Tag::*;
    use super::{s, ItemDisplay, ItemType};

    #[test]
    fn paragraph() {
        assert_eq!(s(Start(Paragraph)), "")
    }
    #[test]
    fn rule() {
        assert_eq!(s(Start(Rule)), "---")
    }
    #[test]
    fn header1() {
        assert_eq!(s(Start(Header(1))), "# ")
    }
    #[test]
    fn header2() {
        assert_eq!(s(Start(Header(2))), "## ")
    }
    #[test]
    fn blockquote() {
        assert_eq!(s(Start(BlockQuote)), "")
    }
    #[test]
    fn codeblock() {
        assert_eq!(s(Start(CodeBlock("asdf".into()))), "```asdf")
    }
    #[test]
    fn list_unordered() {
        assert_eq!(s(Start(List(None))), "")
    }
    #[test]
    fn list_ordered() {
        assert_eq!(s(Start(List(Some(1)))), "")
    }
    #[test]
    fn item() {
        assert_eq!(s(Start(Item)), "")
    }
    #[test]
    fn item_ordered_1() {
        assert_eq!(format!("{}", ItemDisplay(ItemType::Ordered(1))), "1. ");
    }
    #[test]
    fn item_ordered_2() {
        assert_eq!(format!("{}", ItemDisplay(ItemType::Ordered(2))), "2. ");
    }
    #[test]
    fn item_unordered() {
        assert_eq!(format!("{}", ItemDisplay(ItemType::Unordered)), "* ");
    }
    #[test]
    fn footnote_definition() {
        assert_eq!(s(Start(FootnoteDefinition("asdf".into()))), "[^asdf]: ")
    }
    #[test]
    fn emphasis() {
        assert_eq!(s(Start(Emphasis)), "*")
    }
    #[test]
    fn strong() {
        assert_eq!(s(Start(Strong)), "**")
    }
    #[test]
    fn code() {
        assert_eq!(s(Start(Code)), "`")
    }
    #[test]
    fn link() {
        assert_eq!(s(Start(Link("uri".into(), "title".into()))), "[")
    }
    #[test]
    fn link_without_title() {
        assert_eq!(s(Start(Link("uri".into(), "".into()))), "[")
    }
    #[test]
    fn image() {
        assert_eq!(s(Start(Image("uri".into(), "title".into()))), "![")
    }
    #[test]
    fn image_without_title() {
        assert_eq!(s(Start(Image("uri".into(), "".into()))), "![")
    }
}

mod end {
    use pulldown_cmark::Event::*;
    use pulldown_cmark::Tag::*;
    use super::s;

    #[test]
    fn emphasis() {
        assert_eq!(s(End(Emphasis)), "*")
    }
    #[test]
    fn strong() {
        assert_eq!(s(End(Strong)), "**")
    }
    #[test]
    fn code() {
        assert_eq!(s(End(Code)), "`")
    }
    #[test]
    fn link() {
        assert_eq!(s(End(Link("/uri".into(), "title".into()))), "](/uri \"title\")")
    }
    #[test]
    fn link_without_title() {
        assert_eq!(s(End(Link("/uri".into(), "".into()))), "](/uri)")
    }
    #[test]
    fn image() {
        assert_eq!(s(End(Image("/uri".into(), "title".into()))), "](/uri \"title\")")
    }
    #[test]
    fn image_without_title() {
        assert_eq!(s(End(Image("/uri".into(), "".into()))), "](/uri)")
    }
}

#[test]
fn footnote_reference() {
    assert_eq!(s(Event::FootnoteReference("asdf".into())), "[^asdf]")
}
