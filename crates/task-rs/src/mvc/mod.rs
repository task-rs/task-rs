use super::config::Config;
use super::data::Manifest;

#[derive(Debug)]
pub struct State {
    pub manifest: Manifest,
    pub config: Config,
}
