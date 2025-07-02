use super::Library;
use crate::configs::{App, ConfigFile, Tool};
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
        library.validate_installs().await?;
        library.save().await?;
        Ok(library)
    }

    pub async fn has_link(&self, url: &str) -> bool {
        self.apps.iter().any(|app| app.url == url) || self.tools.iter().any(|tool| tool.url == url)
    }

    pub async fn get_app(&self, id: &str) -> Option<App> {
        if let Some(app) = self.apps.iter().find(|app| app.installed && app.id == id) {
            return Some(app.clone());
        }
        None
    }

    pub async fn get_tool(&self, id: &str) -> Option<Tool> {
        if let Some(tool) = self
            .tools
            .iter()
            .find(|tool| tool.installed && tool.id == id)
        {
            return Some(tool.clone());
        }
        None
    }
}
