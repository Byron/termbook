use clap::{App, Arg};

use std::env;

lazy_static! {
    static ref SHELL: Result<String, env::VarError> = env::var("SHELL");
}

pub fn app<'a, 'b>() -> App<'a, 'b> {
    let app: App = app_from_crate!();
    let completions = App::new("completions")
        .about("generate completions for supported shell")
        .arg({
            let arg = Arg::with_name("shell").required(SHELL.is_err()).help(
                "The name of the shell, or the path to the shell as exposed by the \
                 $SHELL variable.",
            );
            if let Ok(shell) = SHELL.as_ref() {
                arg.default_value(shell)
            } else {
                arg
            }
        });
    let book_path = Arg::with_name("path")
        .required(false)
        .value_name("path-to-mdbook")
        .help(
            "The path to the mdbook to render. If unset, the current working directory \
             is expected to contain an mdbook configuration file.",
        );
    let build = App::new("build")
        .about(
            "Build the `mdbook` compatible book in the current working directory \
             or in the given location.",
        )
        .arg(
            Arg::with_name("rewrite")
                .long("rewrite")
                .short("r")
                .required(false)
                .help(
                    "If set, the 'rewrite' output will be used to render the mdbook in addition to default renderers, \
                     which will write the preprocessor output directly back to the destination directory.\
                     \
                     It's useful to review the preprocessor result.",
                ),
        )
        .arg(book_path.clone());

    let playback = App::new("play")
        .help("Playback documentation by emulating a fast human typist.")
        .arg(
            Arg::with_name("cps")
                .long("characters-per-second")
                .required(false)
                .default_value("50")
                .help("The amount of characters printed per second."),
        )
        .arg(book_path);

    app.subcommand(build)
        .subcommand(playback)
        .subcommand(completions)
}
