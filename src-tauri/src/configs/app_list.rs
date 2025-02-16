use crate::configs::ConfigFile;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppList {
    #[serde(default)]
    pub links: Vec<AppLink>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppLink {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub status: String,
}

#[async_trait::async_trait]
impl ConfigFile for AppList {
    fn get_filename() -> &'static str {
        "AppList.toml"
    }

    async fn create_default() -> Result<Self, Box<dyn Error>> {
        let default_list = Self { links: Vec::new() };
        let config_path = Self::get_file_path().await?;

        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let content = tokio::task::spawn_blocking({
            let config = default_list.clone();
            move || toml::to_string_pretty(&config)
        })
        .await??;

        tokio::fs::write(config_path, content).await?;
        Ok(default_list)
    }
}

impl AppList {
    pub fn add_link(&mut self, url: String) {
        let new_link = AppLink {
            url,
            timestamp: chrono::Utc::now().timestamp(),
            status: "pending".to_string(),
        };
        self.links.push(new_link);
    }

    pub fn has_link(&self, url: &str) -> bool {
        self.links.iter().any(|link| link.url == url)
    }
}
