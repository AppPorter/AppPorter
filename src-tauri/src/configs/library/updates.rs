use super::{AppValidationStatus, Library, ToolValidationStatus};
use crate::configs::ConfigFile;
use crate::configs::library::{App, Tool};
use anyhow::Result;

impl Library {
    pub async fn update_app_list_from_config(&mut self, config: App) -> Result<()> {
        let mut config = config.clone();

        config.details.validation_status = AppValidationStatus {
            file_exists: true,
            registry_valid: true,
            path_exists: true,
        };

        let new_app = App {
            timestamp: config.timestamp,
            installed: true,
            details: config.details,
            url: config.url,
        };

        if let Some(existing_app) = self
            .apps
            .iter_mut()
            .find(|app| app.timestamp == config.timestamp)
        {
            existing_app.installed = true;
            existing_app.details = new_app.details.clone();
        }

        self.apps.retain(|existing_app| {
            let mut app1 = existing_app.clone();
            let mut app2 = new_app.clone();
            app1.timestamp = 0;
            app2.timestamp = 0;
            app1.details.info.version = String::new();
            app2.details.info.version = String::new();
            app1 != app2
        });

        self.apps.push(new_app);
        self.save().await?;
        Ok(())
    }

    pub async fn update_tool_list_from_config(&mut self, config: Tool) -> Result<()> {
        let mut config = config.clone();

        config.details.validation_status = ToolValidationStatus {
            file_exists: true,
            path_exists: true,
        };

        let new_tool = Tool {
            timestamp: config.timestamp,
            installed: true,
            details: config.details,
            url: config.url,
        };

        if let Some(existing_tool) = self
            .tools
            .iter_mut()
            .find(|tool| tool.timestamp == config.timestamp)
        {
            existing_tool.installed = true;
            existing_tool.details = new_tool.details.clone();
        }

        self.tools.retain(|existing_tool| {
            let mut tool1 = existing_tool.clone();
            let mut tool2 = new_tool.clone();
            tool1.timestamp = 0;
            tool2.timestamp = 0;
            tool1 != tool2
        });

        self.tools.push(new_tool);
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
