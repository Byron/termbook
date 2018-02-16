use clap::{App, ArgMatches, Shell};
use std::path::Path;
use std::io::stdout;
use std::str::FromStr;
use std::env::current_dir;
use termbook::mdbook::errors::Error;

use types::{BuildContext, PlaybackContext};

pub fn generate_completions(mut app: App, args: &ArgMatches) -> Result<(), Error> {
    let shell = args.value_of("shell")
        .ok_or_else(|| "expected 'shell' argument".into())
        .map(|s| {
            Path::new(s)
                .file_name()
                .map(|f| {
                    f.to_str()
                        .expect("os-string to str conversion to work for filename")
                })
                .unwrap_or(s)
        })
        .and_then(|s| {
            Shell::from_str(s).map_err(|_| Error::from(format!("The shell '{}' is unsupported", s)))
        })?;
    let app_name = app.get_name().to_owned();
    app.gen_completions_to(app_name, shell, &mut stdout());
    Ok(())
}

pub fn playback_context_from(args: &ArgMatches) -> Result<PlaybackContext, Error> {
    Ok(PlaybackContext {
        globs: args.values_of("selector")
            .map(|v| v.map(String::from).collect())
            .unwrap_or(Vec::new()),
        chars_per_second: args.value_of("cps")
            .expect("at least default")
            .parse()
            .map_err(|e| Error::from(format!("{}", e)))?,
        path: args.value_of_os("path")
            .map(Path::new)
            .map(Into::into)
            .unwrap_or_else(|| current_dir().expect("current dir available")),
    })
}
pub fn build_context_from(args: &ArgMatches) -> Result<BuildContext, Error> {
    Ok(BuildContext {
        path: args.value_of_os("path")
            .map(Path::new)
            .map(Into::into)
            .unwrap_or_else(|| current_dir().expect("current dir available")),
        rewrite: args.is_present("rewrite"),
    })
}
