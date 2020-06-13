use super::Args;

#[derive(Debug)]
pub struct App {
    pub args: Args,
}

mod config;
mod flags;
mod init;
mod main;
mod run;
mod settings;
