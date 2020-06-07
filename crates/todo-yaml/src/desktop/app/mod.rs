use super::super::data::Manifest;
use super::Config;

#[derive(Debug)]
pub struct App {
    pub manifest: Manifest,
    pub config: Config,
}
