use super::super::{
    config::{Config, Source as CfgSrc},
    data::Data,
    utils::deserialize_file,
};

pub fn from_cfg_opt(config: &Option<(Config, CfgSrc)>) -> Data {
    if let Some((
        Config {
            local_repo_location,
            ..
        },
        _,
    )) = config
    {
        match deserialize_file(&local_repo_location) {
            Ok(data) => data,
            Err(error) => {
                eprintln!(
                    "Failed to load {}: {}",
                    local_repo_location.to_string_lossy(),
                    error
                );
                Data::default()
            }
        }
    } else {
        Data::default()
    }
}
