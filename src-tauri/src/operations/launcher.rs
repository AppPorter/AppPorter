use anyhow::{Result, anyhow};
use tokio::process::Command;

pub async fn open_app(path: &str) -> Result<()> {
    let output = Command::new(path)
        .creation_flags(0x08000000)
        .output()
        .await?;
    if !output.stderr.is_empty() {
        return Err(anyhow!(
            "Failed to open app: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

pub async fn open_folder(path: &str) -> Result<()> {
    let output = Command::new("explorer")
        .arg(path)
        .creation_flags(0x08000000)
        .output()
        .await?;
    if !output.stderr.is_empty() {
        return Err(anyhow!(
            "Failed to open folder: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

pub async fn open_registry(name: &str, current_user_only: bool) -> Result<()> {
    let regpath = if current_user_only {
        r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall\"
    } else {
        r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\"
    };

    let ps_command = format!(
        r#"
        New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Applets\Regedit" -Name "LastKey" -Value "{regpath}{name}" -Force
        Start-Process regedit
        "#,
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
        return Err(anyhow!(
            "Failed to open registry: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}
