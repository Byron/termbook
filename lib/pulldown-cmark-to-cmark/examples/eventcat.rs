extern crate pulldown_cmark;

use pulldown_cmark::Parser;
use std::env;
use std::io::Read;
use std::fs::File;
use std::ffi::OsString;

fn main() {
    let path = env::args_os()
        .skip(1)
        .next()
        .expect("First argument is markdown file to display");

    let md = read_to_string(path);
    for event in Parser::new_ext(&md, pulldown_cmark::Options::all()) {
        println!("{:?}", event)
    }
}

fn read_to_string(path: OsString) -> String {
    let mut file = File::open(&path).expect("file to exist for reading");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("file to be readable");
    buf
}
