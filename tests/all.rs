extern crate termbook;

use std::path::Path;

#[test]
fn test_build_book() {
    let book_dir = Path::new(file!())
        .parent()
        .expect("directory of file")
        .join("book");
    assert_eq!(format!("{:?}", termbook::build(book_dir)), "Ok(())");
}
