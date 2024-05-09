use std::path::PathBuf;

use clap::Command;

use crate::presentation::api::errors::AppError;

pub struct Args {
    pub config: PathBuf,
}

fn create_command() -> Command {
    clap::command!()
        .arg(clap::arg!(
            config: -c --config <FILE> "Specify the config file"
        ).required(true))
}

pub fn parse_args(args: std::env::Args) -> Result<Args, AppError> {
    let command = create_command();
    let matches = command.get_matches_from(args);
    Ok(Args {
        config: matches.get_one::<String>("config").ok_or(AppError::ParsingError)?.into()
    })
}
