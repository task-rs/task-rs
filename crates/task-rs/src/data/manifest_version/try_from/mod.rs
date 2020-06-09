pub mod error;

pub use error::Error;

use super::{ManifestVersion, MANIFEST_VERSION, N};
use core::convert::TryFrom;

impl TryFrom<N> for ManifestVersion {
    type Error = Error;

    fn try_from(n: N) -> Result<Self, Self::Error> {
        if n == MANIFEST_VERSION {
            Ok(ManifestVersion)
        } else {
            Err(Error(n))
        }
    }
}
