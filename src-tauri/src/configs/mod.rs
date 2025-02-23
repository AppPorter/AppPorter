use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, path::PathBuf};

pub mod app_list;
pub mod settings;

#[async_trait::async_trait]
pub trait ConfigFile: DeserializeOwned + Serialize + Default + Clone + Send + 'static {
    fn get_filename() -> &'static str;

    async fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
        let exe_dir = std::env::current_exe()?
            .parent()
            .ok_or("Failed to get exe directory")?
            .to_path_buf();
        Ok(exe_dir.join("configs").join(Self::get_filename()))
    }

    async fn create_default() -> Result<Self, Box<dyn Error>> {
        let default_config = Self::default();
        let config_path = Self::get_file_path().await?;

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&config_path, serde_json::to_string_pretty(&default_config)?).await?;
        Ok(default_config)
    }

    async fn read() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_file_path().await?;

        if !tokio::fs::try_exists(&config_path).await? {
            return Self::create_default().await;
        }

        let content = tokio::fs::read_to_string(&config_path).await?;
        Ok(serde_json::from_str(&content)?)
    }

    async fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_file_path().await?;
        tokio::fs::write(&config_path, serde_json::to_string_pretty(&self)?).await?;
        Ok(())
    }
}
