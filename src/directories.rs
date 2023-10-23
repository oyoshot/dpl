use etcetera::{choose_base_strategy, BaseStrategy};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

pub struct DplDir {
    config_dir: PathBuf,
}

impl DplDir {
    fn new() -> Option<DplDir> {
        let strategy = choose_base_strategy().ok()?;
        let config_dir = strategy.config_dir().join("dpl");
        Some(DplDir { config_dir })
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
}

pub static PROJECT_DIRS: Lazy<DplDir> =
    Lazy::new(|| DplDir::new().expect("Could not get home directory"));
