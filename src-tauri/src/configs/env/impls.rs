use super::Env;
use check_elevation::is_elevated;
use std::error::Error;
use tokio::process::Command;

impl Default for Env {
    fn default() -> Self {
        Self {
            debug: cfg!(debug_assertions),
            elevated: is_elevated().unwrap_or(false),
            system_drive_letter: std::env::var("windir")
                .map(|s| s[..1].to_string())
                .unwrap_or_else(|_| "C".to_string()),
            user_sid: String::new(),
            username: std::env::var("USERNAME").unwrap_or_else(|_| "user".to_string()),
        }
    }
}

impl Env {
    pub async fn read() -> Result<Env, Box<dyn Error + Send + Sync>> {
        let mut env = Env::default();
        env.user_sid = env.get_user_sid().await?;
        Ok(env)
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
