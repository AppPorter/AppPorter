use crate::configs::{env::Env, ConfigFile};
use std::error::Error;
use tokio::process::Command;

// Modifies Windows registry to enable/disable application elevation privileges
pub async fn elevate(revert: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    let env = Env::read().await?;

    let operation = if !revert {
        "Set-ItemProperty -Path $regPath -Name $programPath -Value $adminFlag"
    } else {
        "Remove-ItemProperty -Path $regPath -Name $programPath -ErrorAction SilentlyContinue"
    };

    let ps_command = format!(
        r#"
        $programPath = "{}"
        $regPath = "Registry::HKEY_USERS\{}\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers"
        $adminFlag = "~ RUNASADMIN"
        {}
        "#,
        std::env::current_exe()?.to_string_lossy(),
        env.user_sid,
        operation
    );

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &ps_command,
        ])
        .creation_flags(0x08000000)
        .output()
        .await?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}
