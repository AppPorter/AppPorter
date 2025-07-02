use super::{AppValidationStatus, Library, ToolValidationStatus};
use crate::configs::ConfigFile;
use crate::configs::library::{App, Tool};
use anyhow::Result;
use uuid::Uuid;

impl Library {
    pub async fn init_app(config: &mut App) -> Result<()> {
        if config.id.is_empty() {
            config.id = Uuid::new_v4().to_string();
        }
        if config.timestamp_add.is_empty() {
            config.timestamp_add = chrono::Utc::now().to_rfc3339();
        }
        config.timestamp_update = chrono::Utc::now().to_rfc3339();
        Ok(())
    }
    pub async fn init_tool(config: &mut Tool) -> Result<()> {
        if config.id.is_empty() {
            config.id = Uuid::new_v4().to_string();
        }
        if config.timestamp_add.is_empty() {
            config.timestamp_add = chrono::Utc::now().to_rfc3339();
        }
        config.timestamp_update = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    pub async fn add_app(&mut self, config: App) -> Result<()> {
        let mut config = config;

        Library::init_app(&mut config).await?;

        config.installed = true;
        config.validation_status = AppValidationStatus {
            file_exists: true,
            registry_valid: true,
            path_exists: true,
        };

        if let Some(existing_app) = self.apps.iter_mut().find(|app| app.id == config.id) {
            *existing_app = config;
        } else {
            self.apps.push(config);
        }

        self.save().await?;
        Ok(())
    }
    pub async fn add_tool(&mut self, config: Tool) -> Result<()> {
        let mut config = config;

        Library::init_tool(&mut config).await?;

        config.installed = true;
        config.validation_status = ToolValidationStatus {
            file_exists: true,
            path_exists: true,
        };

        if let Some(existing_tool) = self.tools.iter_mut().find(|tool| tool.id == config.id) {
            *existing_tool = config;
        } else {
            self.tools.push(config);
        }

        self.save().await?;
        Ok(())
    }

    pub async fn uninstall_app(&mut self, id: &str) -> Result<()> {
        let app_index = self
            .apps
            .iter()
            .position(|existing_app| existing_app.id == id);
        if let Some(index) = app_index {
            if !self.apps[index].url.is_empty() {
                self.apps[index].installed = false;
            } else {
                self.apps.remove(index);
            }
        }
        self.save().await?;
        Ok(())
    }
    pub async fn uninstall_tool(&mut self, id: &str) -> Result<()> {
        let tool_index = self
            .tools
            .iter()
            .position(|existing_tool| existing_tool.id == id);
        if let Some(index) = tool_index {
            if !self.tools[index].url.is_empty() {
                self.tools[index].installed = false;
            } else {
                self.tools.remove(index);
            }
        }
        self.save().await?;
        Ok(())
    }
    pub async fn remove(&mut self, id: &str) -> Result<()> {
        self.apps.retain(|app| app.id != id);
        self.tools.retain(|tool| tool.id != id);
        self.save().await?;
        Ok(())
    }
}
