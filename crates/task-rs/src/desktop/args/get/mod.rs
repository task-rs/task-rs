use super::Args;
use std::env::args;
use structopt::*;

type Status = super::super::status::Status<Args>;

pub fn from_env() -> Status {
    Args::from_iter_safe(args()).map_err(|error| match error.kind {
        clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
            println!("{}", error.message);
            0
        }
        _ => {
            println!("{}", error.message);
            1
        }
    })
}

impl Args {
    pub fn from_env() -> Status {
        from_env()
    }
}
