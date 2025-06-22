use super::Env;
use crate::configs::ConfigFile;
use check_elevation::is_elevated;
use std::{error::Error, path::PathBuf};
use tokio::process::Command;

#[async_trait::async_trait]
impl ConfigFile for Env {
    fn get_file_path() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        Ok(dirs::config_local_dir()
            .ok_or("Failed to get local config directory")?
            .join("AppPorter")
            .join("Env.json"))
    }
}

impl Default for Env {
    fn default() -> Self {
        let system_drive = std::env::var("windir")
            .map(|s| s[..1].to_string())
            .unwrap_or_else(|_| "C".to_string());
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "user".to_string());
        Self {
            first_run: true,
            debug: cfg!(debug_assertions),
            elevated: is_elevated().unwrap_or(false),
            system_drive_letter: system_drive.clone(),
            user_sid: String::new(),
            username: username.clone(),
        }
    }
}

impl Env {
    pub async fn initialization() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut env = Env::read().await?;

        env.debug = cfg!(debug_assertions);
        env.elevated = is_elevated()?;
        env.user_sid = env.get_user_sid().await?;
        env.system_drive_letter = std::env::var("windir")?[..1].to_string();
        env.username = std::env::var("USERNAME")?;

        env.save().await?;
        Ok(())
    }

    async fn get_user_sid(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let output = Command::new("powershell")
            .args([
                "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass",
                "-Command",
                r#"Get-WmiObject Win32_UserAccount -Filter "Name='$env:USERNAME'" | Select-Object -ExpandProperty SID"#,
            ])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
            .output()
            .await?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
