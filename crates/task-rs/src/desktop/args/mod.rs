pub mod command;
pub mod get;

pub use command::Command;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "task-rs")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}
