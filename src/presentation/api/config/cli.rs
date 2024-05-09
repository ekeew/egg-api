use std::path::PathBuf;

use clap::Command;

pub struct Args {
    pub config: PathBuf,
}

pub fn create_command() -> Command {
    clap::command!()
        .arg(clap::arg!(
            config: -c --config <FILE> "Specify the config file"
        ).required(true))
}

pub fn parse_args(args: std::env::Args) -> Args {
    let command = create_command();
    let matches = command.get_matches_from(args);
    Args {
        config: matches.get_one::<String>("config").unwrap().into()
    }
}
