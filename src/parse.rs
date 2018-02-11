use failure::{Error, ResultExt};
use clap::{App, ArgMatches, Shell};
use std::path::Path;
use failure::err_msg;
use std::io::stdout;
use std::str::FromStr;
use std::env::current_dir;

use types::BuildContext;

pub fn generate_completions(mut app: App, args: &ArgMatches) -> Result<(), Error> {
    let shell = args.value_of("shell")
        .ok_or_else(|| err_msg("expected 'shell' argument"))
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
            Shell::from_str(s)
                .map_err(err_msg)
                .context(format!("The shell '{}' is unsupported", s))
                .map_err(Into::into)
        })?;
    let app_name = app.get_name().to_owned();
    app.gen_completions_to(app_name, shell, &mut stdout());
    Ok(())
}

pub fn build_context_from(args: &ArgMatches) -> Result<BuildContext, Error> {
    Ok(BuildContext {
        path: args.value_of_os("path")
            .map(Path::new)
            .map(Into::into)
            .unwrap_or_else(|| current_dir().expect("current dir available")),
        rewrite: args.value_of("rewrite").map_or(false, |_| true),
    })
}
