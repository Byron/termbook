extern crate termbook;
use std::env;
use std::path::PathBuf;

fn main() {
    let root_dir = env::args()
        .skip(1)
        .next()
        .expect("book root directory as first argument");

    termbook::build(PathBuf::from(root_dir)).expect("valid book");
}
