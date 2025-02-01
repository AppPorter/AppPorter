use std::error::Error;

use settings::Settings;

pub mod command;
pub mod installation;
pub mod settings;

pub async fn elevate(revert: bool) -> Result<(), Box<dyn Error>> {
    let operation: &str;
    if !revert {
        operation = "Set-ItemProperty -Path $regPath -Name $programPath -Value $adminFlag";
    } else {
        operation =
            "Remove-ItemProperty -Path $regPath -Name $programPath -ErrorAction SilentlyContinue";
    }
    let settings = Settings::read().await?;
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
        .output()
        .await?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}

pub async fn validate_path(path: String) -> Result<String, Box<dyn Error>> {
    // Check drive letter and basic path format
    if !path
        .chars()
        .nth(0)
        .map_or(false, |c| c.is_ascii_alphabetic())
        || !path.chars().nth(1).map_or(false, |c| c == ':')
        || !path.chars().nth(2).map_or(false, |c| c == '\\')
    {
        return Err("Invalid drive letter or path format".into());
    }

    // Check if path exists and is accessible using tokio's fs
    match tokio::fs::metadata(&path).await {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok("Path is valid".into())
            } else {
                Err("Path exists but is not a directory".into())
            }
        }
        Err(e) => Err(format!("Directory does not exist. {}", e).into()),
    }
}
