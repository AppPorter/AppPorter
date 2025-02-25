use crate::configs::ConfigFile;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppList {
    #[serde(default)]
    pub links: Vec<App>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct App {
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub installed: bool,
    #[serde(default)]
    pub details: InstalledApp,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct InstalledApp {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub publisher: String,
    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub install_path: String,
    #[serde(default)]
    pub executable_path: String,

    #[serde(default)]
    pub current_user_only: bool,
    #[serde(default)]
    pub create_desktop_shortcut: bool,
    #[serde(default)]
    pub create_start_menu_shortcut: bool,
    #[serde(default)]
    pub create_registry_key: bool,
}

#[async_trait::async_trait]
impl ConfigFile for AppList {
    fn get_filename() -> &'static str {
        "AppList.json"
    }
}

impl AppList {
    pub fn add_link(&mut self, url: String) {
        self.links.push(App {
            timestamp: chrono::Utc::now().timestamp(),
            url,
            ..Default::default()
        });
    }

    pub fn has_link(&self, url: &str) -> bool {
        self.links.iter().any(|link| link.url == url)
    }
}

pub async fn load_app_list() -> Result<String, Box<dyn Error>> {
    Ok(serde_json::to_string(&AppList::read().await?)?)
}

pub async fn save_app_list(app_list: AppList) -> Result<String, Box<dyn Error>> {
    app_list.save().await?;
    Ok("AppList saved successfully".to_owned())
}
