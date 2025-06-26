use super::Env;
use anyhow::{Result, anyhow};
use check_elevation::is_elevated;
use std::sync::OnceLock;
use tokio::process::Command;

static CACHED_ENV: OnceLock<Env> = OnceLock::new();

impl Default for Env {
    fn default() -> Self {
        Self {
            debug: cfg!(debug_assertions),
            elevated: is_elevated().unwrap_or(false),
            system_drive_letter: std::env::var("windir")
                .map(|s| s[..1].to_owned())
                .unwrap_or("C".to_owned()),
            user_sid: String::new(),
            username: std::env::var("USERNAME").unwrap_or("user".to_owned()),
        }
    }
}

impl Env {
    pub async fn read() -> Result<Env> {
        if let Some(cached) = CACHED_ENV.get() {
            return Ok(cached.clone());
        }

        let env = Env {
            user_sid: Env::get_user_sid().await?,
            ..Default::default()
        };

        let _ = CACHED_ENV.set(env.clone());

        Ok(env)
    }

    async fn get_user_sid() -> Result<String> {
        let output = Command::new("powershell")
            .args([
                "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass",
                "-Command",
                r#"Get-WmiObject Win32_UserAccount -Filter "Name='$env:USERNAME'" | Select-Object -ExpandProperty SID"#,
            ])
            .creation_flags(0x08000000)
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("{}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }
}
