use super::config::Config;
use super::data::Data;

#[derive(Debug)]
pub struct State {
    pub manifest: Data,
    pub config: Config,
}
