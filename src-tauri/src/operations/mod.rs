use crate::configs::settings::Settings;
use crate::configs::ConfigFile;
use crate::get_7z_path;
use std::error::Error;

pub mod get_details;
pub mod installation;

pub use get_details::*;
pub use installation::*;
use tokio::process::Command;

/// Modifies Windows registry for application elevation privileges
pub async fn elevate(revert: bool) -> Result<String, Box<dyn Error>> {
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
    Ok("".to_owned())
}

/// Validates if a path exists and is a directory
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

/// Checks if string has valid Windows path format (drive letter:\..)
fn is_valid_path_format(path: &str) -> bool {
    let chars: Vec<char> = path.chars().collect();
    chars.first().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.get(1).is_some_and(|c| *c == ':')
        && chars.get(2).is_some_and(|c| *c == '\\')
}

pub async fn get_archive_content(path: String) -> Result<String, Box<dyn Error>> {
    let output = Command::new(get_7z_path()?)
        .args([
            "l", // Extract without full paths
            &path, "-y",   // Auto yes to all prompts
            "-aoa", // Overwrite all existing files without prompt
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    let mut is_output_section = false;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    let mut list: Vec<String> = Vec::new();
    for line in output_str.lines() {
        if line.contains("------------------------") {
            // Toggle output state when separator line is found
            is_output_section = !is_output_section;
            continue;
        }

        // Only process lines between separators
        if is_output_section {
            if let Some(last_field) = line.split_whitespace().last() {
                println!("{}", last_field);
                list.push(last_field.to_owned());
            }
        }
    }

    Ok(serde_json::to_string(&list)?)
}
