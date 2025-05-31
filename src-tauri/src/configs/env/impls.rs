use super::Env;
use crate::configs::ConfigFile;
use check_elevation::is_elevated;
use std::error::Error;
use tokio::process::Command;

#[async_trait::async_trait]
impl ConfigFile for Env {
    fn get_filename() -> &'static str {
        "Env.json"
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
    pub async fn initialization(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.debug = cfg!(debug_assertions);
        self.elevated = is_elevated()?;
        self.user_sid = self.get_user_sid().await?;
        self.system_drive_letter = std::env::var("windir")?[..1].to_string();
        self.username = std::env::var("USERNAME")?;
        self.save().await?;
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
