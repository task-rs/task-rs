use pipe_trait::*;
use std::path::PathBuf;

pub fn config_file() -> Option<PathBuf> {
    dirs::config_dir()?
        .pipe(|dir| dir.join("task-rs/config.yaml"))
        .pipe(Some)
}
