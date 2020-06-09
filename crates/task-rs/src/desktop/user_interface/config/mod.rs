pub mod source;

pub use source::Source;

use super::super::super::config::Config as Cfg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub source: Source,
    pub config: Option<Cfg>,
}
