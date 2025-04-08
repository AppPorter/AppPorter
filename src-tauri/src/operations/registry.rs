use crate::configs::settings::Settings;
use crate::configs::ConfigFile;
use std::error::Error;
use tokio::process::Command;

// Modifies Windows registry to enable/disable application elevation privileges
pub async fn elevate(revert: bool) -> Result<String, Box<dyn Error + Send + Sync>> {
    let settings = Settings::read().await?;

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
        settings.user_sid,
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
    Ok("".to_owned())
}

pub async fn open_registry(
    name: &str,
    current_user_only: bool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let regpath = if current_user_only {
        r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall\"
    } else {
        r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\"
    };

    let ps_command = format!(
        r#"
        New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Applets\Regedit" -Name "LastKey" -Value {}{} -Force
        Start-Process regedit
        "#,
        regpath, name
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
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok("".to_owned())
}
