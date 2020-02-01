#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod cli;
mod parse;
mod types;

use std::process;
use termbook::mdbook::errors::Error;

use clap::ArgMatches;
use std::io::{stderr, Write};

fn print_causes<E, W>(e: E, mut w: W)
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

    env_logger::init();
    match matches.subcommand() {
        ("completions", Some(args)) => {
            ok_or_exit(parse::generate_completions(appc, args));
            process::exit(0);
        }
        ("play", Some(args)) => {
            let ctx = ok_or_exit(parse::playback_context_from(args));
            let mut book = ok_or_exit(termbook::load(&ctx.path, ctx.globs.clone()));
            book.with_renderer(termbook::Playback::new(ctx.chars_per_second, ctx.globs));
            ok_or_exit(book.build());
        }
        ("build", Some(args)) => {
            let ctx = ok_or_exit(parse::build_context_from(args));
            let mut book = ok_or_exit(termbook::load(&ctx.path, ctx.globs));
            if ctx.rewrite {
                book.with_renderer(termbook::Rewrite::default());
            }
            ok_or_exit(book.build());
        }
        _ => usage_and_exit(&matches),
    };
}
