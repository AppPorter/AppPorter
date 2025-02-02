use config::Config as ConfigBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, path::PathBuf};

pub mod app_list;
pub mod settings;

#[async_trait::async_trait]
pub trait ConfigFile: DeserializeOwned + Serialize + Default + Clone + Send + 'static {
    fn get_filename() -> &'static str;

    async fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
        let exe_dir = tokio::fs::canonicalize(
            std::env::current_exe()?
                .parent()
                .ok_or("Failed to get exe directory")?
                .to_path_buf(),
        )
        .await?;
        Ok(exe_dir.join("configs").join(Self::get_filename()))
    }

    async fn create_default() -> Result<Self, Box<dyn Error>> {
        let default_config = Self::default();
        let config_path = Self::get_file_path().await?;

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let content = tokio::task::spawn_blocking({
            let config = default_config.clone();
            move || toml::to_string_pretty(&config)
        })
        .await??;

        tokio::fs::write(config_path, content).await?;
        Ok(default_config)
    }

    async fn read() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_file_path().await?;

        if !tokio::fs::try_exists(&config_path).await? {
            return Self::create_default().await;
        }

        let content = tokio::fs::read_to_string(&config_path).await?;
        let config = tokio::task::spawn_blocking(move || {
            ConfigBuilder::builder()
                .add_source(config::File::from_str(&content, config::FileFormat::Toml))
                .build()?
                .try_deserialize::<Self>()
        })
        .await??;

        Ok(config)
    }

    async fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_file_path().await?;
        let config_str = tokio::task::spawn_blocking({
            let config = self.clone();
            move || toml::to_string_pretty(&config)
        })
        .await??;

        tokio::fs::write(config_path, config_str).await?;
        Ok(())
    }
}
