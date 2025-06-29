use super::{AppValidationStatus, Library, ToolValidationStatus};
use crate::configs::ConfigFile;
use crate::configs::library::{App, Tool};
use anyhow::Result;

impl Library {
    pub async fn update_app_list_from_config(&mut self, config: App) -> Result<()> {
        let mut config = config.clone();

        config.installed = true;
        config.validation_status = AppValidationStatus {
            file_exists: true,
            registry_valid: true,
            path_exists: true,
        };

        if let Some(existing_app) = self
            .apps
            .iter_mut()
            .find(|app| app.timestamp == config.timestamp)
        {
            *existing_app = config;
        } else {
            self.apps.push(config);
        }

        self.save().await?;
        Ok(())
    }

    pub async fn update_tool_list_from_config(&mut self, config: Tool) -> Result<()> {
        let mut config = config.clone();

        config.installed = true;
        config.validation_status = ToolValidationStatus {
            file_exists: true,
            path_exists: true,
        };

        if let Some(existing_tool) = self
            .tools
            .iter_mut()
            .find(|tool| tool.timestamp == config.timestamp)
        {
            *existing_tool = config;
        } else {
            self.tools.push(config);
        }

        self.save().await?;
        Ok(())
    }

    pub async fn update_app_list_after_uninstall(&mut self, timestamp: i64) -> Result<()> {
        let app_index = self
            .apps
            .iter()
            .position(|existing_app| existing_app.timestamp == timestamp);

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

    pub async fn update_tool_list_after_uninstall(&mut self, timestamp: i64) -> Result<()> {
        let tool_index = self
            .tools
            .iter()
            .position(|existing_tool| existing_tool.timestamp == timestamp);

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
}
