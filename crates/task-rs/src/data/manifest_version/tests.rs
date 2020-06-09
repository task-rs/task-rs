use super::{ManifestVersion, MANIFEST_VERSION};
use serde_yaml::{from_str, to_string};

#[test]
fn try_from_success() {
    let text = format!("{}", MANIFEST_VERSION);
    assert_eq!(from_str::<ManifestVersion>(&text).unwrap(), ManifestVersion);
}

#[test]
fn try_from_failure() {
    assert_eq!(
        from_str::<ManifestVersion>("1234").unwrap_err().to_string(),
        "Expecting manifest version to be 0, but received 1234",
    );
}

#[test]
fn into() {
    assert_eq!(
        to_string(&ManifestVersion).unwrap(),
        format!("---\n{}", MANIFEST_VERSION),
    );
}
