pub mod env;
pub mod library;
pub mod settings;

#[allow(ambiguous_glob_reexports)]
pub use env::*;
#[allow(ambiguous_glob_reexports)]
pub use library::*;
use serde::{Serialize, de::DeserializeOwned};
#[allow(ambiguous_glob_reexports)]
pub use settings::*;
use std::{error::Error, path::PathBuf};

#[async_trait::async_trait]
pub trait ConfigFile: DeserializeOwned + Serialize + Default + Clone + Send + 'static {
    fn get_file_path() -> Result<PathBuf, Box<dyn Error + Send + Sync>>;

    async fn create_default() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let default_config = Self::default();
        let config_path = Self::get_file_path()?;

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&config_path, serde_json::to_string_pretty(&default_config)?).await?;
        Ok(default_config)
    }

    async fn read() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let config_path = Self::get_file_path()?;

        if !tokio::fs::try_exists(&config_path).await? {
            return Self::create_default().await;
        }

        let content = tokio::fs::read_to_string(&config_path).await?;
        Ok(serde_json::from_str(&content)?)
    }

    async fn save(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let config_path = Self::get_file_path()?;
        tokio::fs::write(&config_path, serde_json::to_string_pretty(&self)?).await?;
        Ok(())
    }
}
