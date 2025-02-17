use crate::configs::settings::Settings;
use crate::configs::ConfigFile;
use std::error::Error;

pub mod get_details;
pub mod installation;

pub use get_details::*;
pub use installation::*;

/// Elevates or reverts elevation of the application
pub async fn elevate(revert: bool) -> Result<(), Box<dyn Error>> {
    let (operation, settings) = if !revert {
        (
            "Set-ItemProperty -Path $regPath -Name $programPath -Value $adminFlag",
            Settings::read().await?,
        )
    } else {
        (
            "Remove-ItemProperty -Path $regPath -Name $programPath -ErrorAction SilentlyContinue",
            Settings::read().await?,
        )
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

    let output = tokio::process::Command::new("powershell")
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

/// Validates if a given path is a valid directory
pub async fn validate_path(path: String) -> Result<String, Box<dyn Error>> {
    if !is_valid_path_format(&path) {
        return Err("Invalid drive letter or path format".into());
    }

    match tokio::fs::metadata(&path).await {
        Ok(metadata) if metadata.is_dir() => Ok("Path is valid".into()),
        Ok(_) => Err("Path exists but is not a directory".into()),
        Err(e) => Err(format!("Directory does not exist: {}", e).into()),
    }
}

fn is_valid_path_format(path: &str) -> bool {
    let chars: Vec<char> = path.chars().collect();
    chars.get(0).map_or(false, |c| c.is_ascii_alphabetic())
        && chars.get(1).map_or(false, |c| *c == ':')
        && chars.get(2).map_or(false, |c| *c == '\\')
}
