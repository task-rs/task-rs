use super::super::config::Config;
use super::super::data::Manifest;

#[derive(Debug)]
pub struct App {
    pub manifest: Manifest,
    pub config: Config,
}
