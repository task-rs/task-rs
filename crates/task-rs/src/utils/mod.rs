use pipe_trait::*;
use std::path::PathBuf;

pub fn config_file() -> Option<PathBuf> {
    dirs::config_dir()?
        .pipe(|dir| dir.join("task-rs/config.yaml"))
        .pipe(Some)
}

pub fn ui_state_file() -> Option<PathBuf> {
    dirs::cache_dir()?
        .pipe(|dir| dir.join("task-rs/ui-state.yaml"))
        .pipe(Some)
}
