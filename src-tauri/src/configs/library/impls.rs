use super::Library;
use crate::configs::ConfigFile;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

#[async_trait::async_trait]
impl ConfigFile for Library {
    fn get_file_path() -> Result<PathBuf> {
        Ok(dirs::config_local_dir()
            .ok_or_else(|| anyhow!("Failed to get local config directory"))?
            .join("AppPorter")
            .join("Library.json"))
    }
}

impl Library {
    pub async fn load() -> Result<Library> {
        let mut library = Library::read().await?;
        library.sync_from_registry().await?;
        library.remove_duplicates();
        library.validate_installs().await?;

        library.save().await?;

        Ok(library)
    }
}
