#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate termbook;

mod cli;
mod parse;
mod types;

use std::process;
use termbook::Error;

use clap::ArgMatches;
use std::io::{stderr, Write};

pub fn print_causes<E, W>(e: E, mut w: W)
where
    E: Into<Error>,
    W: Write,
{
    let e = e.into();
    let causes = e.iter().collect::<Vec<_>>();
    let num_causes = causes.len();
    for (index, cause) in causes.iter().enumerate() {
        if index == 0 {
            writeln!(w, "{}", cause).ok();
            if num_causes > 1 {
                writeln!(w, "Caused by: ").ok();
            }
        } else {
            writeln!(w, " {}: {}", num_causes - index, cause).ok();
        }
    }
}

fn usage_and_exit(args: &ArgMatches) -> ! {
    println!("{}", args.usage());
    process::exit(1)
}

fn ok_or_exit<T, E>(r: Result<T, E>) -> T
where
    E: Into<Error>,
{
    match r {
        Ok(r) => r,
        Err(e) => {
            write!(stderr(), "error: ").ok();
            print_causes(e, stderr());
            process::exit(1);
        }
    }
}

fn main() {
    let app = cli::app();
    let appc = app.clone();
    let matches = app.get_matches();
    match matches.subcommand() {
        ("completions", Some(args)) => {
            ok_or_exit(parse::generate_completions(appc, &args));
            process::exit(0);
        }
        ("build", Some(args)) => {
            let ctx = ok_or_exit(parse::build_context_from(&args));
            let mut book = ok_or_exit(termbook::new(&ctx.path));
            if ctx.rewrite {
                book.with_renderer(termbook::Rewrite::new());
            }
            ok_or_exit(book.build());
        }
        _ => usage_and_exit(&matches),
    };
}
