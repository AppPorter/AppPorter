use std::{error::Error, process::Command};

use settings::Settings;

pub mod command;
pub mod installation;
pub mod settings;

pub fn elevate(revert: bool) -> Result<(), Box<dyn Error>> {
    let operation: &str;
    if !revert {
        operation = "Set-ItemProperty -Path $regPath -Name $programPath -Value $adminFlag";
    } else {
        operation =
            "Remove-ItemProperty -Path $regPath -Name $programPath -ErrorAction SilentlyContinue";
    }
    let settings = Settings::read()?;
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
        .output()?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}
