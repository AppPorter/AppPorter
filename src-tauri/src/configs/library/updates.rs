use super::{AppValidationStatus, Library, ToolValidationStatus};
use crate::configs::ConfigFile;
use crate::configs::library::{App, Tool, ToolConfig, ToolDetails, ToolPaths};
use crate::operations::install::install_app::AppInstallConfig;
use crate::operations::install::install_tool::ToolInstallConfig;
use anyhow::Result;

impl Library {
    pub async fn update_app_list_from_config(
        &mut self,
        config: AppInstallConfig<'_>,
        full_path: &str,
        full_path_directory: &str,
        timestamp: i64,
    ) -> Result<()> {
        let mut updated_details = config.details;
        updated_details.paths.full_path = full_path.to_owned();
        updated_details.config.full_path_directory = full_path_directory.to_owned();
        updated_details.validation_status = AppValidationStatus {
            file_exists: true,
            registry_valid: true,
            path_exists: true,
        };

        let new_app = App {
            timestamp: if config.timestamp != 0 {
                config.timestamp
            } else {
                timestamp
            },
            installed: true,
            details: updated_details,
            url: config.url.unwrap_or_default().to_owned(),
        };

        if config.timestamp != 0 {
            // Update existing app with matching timestamp
            if let Some(existing_app) = self
                .apps
                .iter_mut()
                .find(|app| app.timestamp == config.timestamp)
            {
                existing_app.installed = true;
                existing_app.details = new_app.details;
            }
        } else {
            // Remove existing similar app and add new one
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
        }

        self.save().await?;
        Ok(())
    }

    pub async fn update_tool_list_from_config(
        &mut self,
        config: ToolInstallConfig<'_>,
        install_path: &str,
        timestamp: i64,
    ) -> Result<()> {
        let tool_timestamp = if config.timestamp != 0 {
            config.timestamp
        } else {
            timestamp
        };

        let details = ToolDetails {
            name: config.details.name,
            config: ToolConfig {
                archive_password: config.password.unwrap_or_default().to_owned(),
                add_to_path: false,
                archive_path_directory: String::new(),
                full_path_directory: String::new(),
            },
            paths: ToolPaths {
                parent_install_path: config.details.paths.parent_install_path,
                install_path: install_path.to_owned(),
            },
            validation_status: ToolValidationStatus {
                file_exists: true,
                path_exists: true,
            },
        };

        let new_tool = Tool {
            timestamp: tool_timestamp,
            installed: true,
            details,
            url: config.url.unwrap_or_default().to_owned(),
        };

        if config.timestamp != 0 {
            // Update existing tool with matching timestamp
            if let Some(existing_tool) = self
                .tools
                .iter_mut()
                .find(|tool| tool.timestamp == config.timestamp)
            {
                existing_tool.installed = true;
                existing_tool.details = new_tool.details;
            } else {
                // If tool doesn't exist, add it as new
                self.tools.push(new_tool);
            }
        } else {
            // Remove existing similar tool and add new one
            self.tools.retain(|existing_tool| {
                let mut tool1 = existing_tool.clone();
                let mut tool2 = new_tool.clone();
                tool1.timestamp = 0;
                tool2.timestamp = 0;
                tool1 != tool2
            });
            self.tools.push(new_tool);
        }

        self.save().await?;
        Ok(())
    }

    /// Update app list by either marking as uninstalled or removing completely
    pub async fn update_app_list_after_uninstall(&mut self, timestamp: i64) -> Result<()> {
        // Find the app to be uninstalled
        let app_index = self
            .apps
            .iter()
            .position(|existing_app| existing_app.timestamp == timestamp);

        if let Some(index) = app_index {
            // If the app has a URL, just mark it as not installed
            // Otherwise, remove it completely from the list
            if !self.apps[index].url.is_empty() {
                self.apps[index].installed = false;
            } else {
                self.apps.remove(index);
            }
        }

        self.save().await?;
        Ok(())
    }

    /// Update tool list by either marking as uninstalled or removing completely
    pub async fn update_tool_list_after_uninstall(&mut self, timestamp: i64) -> Result<()> {
        // Find the tool to be uninstalled
        let tool_index = self
            .tools
            .iter()
            .position(|existing_tool| existing_tool.timestamp == timestamp);

        if let Some(index) = tool_index {
            // If the tool has a URL, just mark it as not installed
            // Otherwise, remove it completely from the list
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
