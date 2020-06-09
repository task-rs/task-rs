pub mod try_from;

use serde::{Deserialize, Serialize};

type N = u16;
pub const MANIFEST_VERSION: N = 0;

#[derive(Debug, Default, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
#[serde(try_from = "N", into = "N")]
pub struct ManifestVersion;

impl Into<N> for ManifestVersion {
    fn into(self) -> N {
        MANIFEST_VERSION
    }
}

#[cfg(test)]
mod tests;
