pub mod command;

use command::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "todo-yaml")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}
