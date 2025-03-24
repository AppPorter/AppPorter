pub mod get_details;
pub mod installation;
pub mod uninstallation;

use crate::configs::ConfigFile;
use crate::get_7z_path;
use crate::{configs::settings::Settings, CHANNEL};
pub use get_details::*;
pub use installation::*;
use std::error::Error;
use tauri::{AppHandle, Emitter};
use tokio::process::Command;
pub use uninstallation::*;

// Modifies Windows registry to enable/disable application elevation privileges
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

pub async fn validate_path(path: String) -> Result<String, Box<dyn Error>> {
    fn is_valid_path_format(path: &str) -> bool {
        let chars: Vec<char> = path.chars().collect();
        chars.first().is_some_and(|c| c.is_ascii_alphabetic())
            && chars.get(1).is_some_and(|c| *c == ':')
            && chars.get(2).is_some_and(|c| *c == '\\')
    }

    if !is_valid_path_format(&path) {
        return Err("Invalid drive letter or path format".into());
    }

    let normalized_path = path
        .chars()
        .fold(String::new(), |mut acc, c| {
            if c == '\\' {
                if !acc.ends_with('\\') {
                    acc.push(c);
                }
            } else {
                acc.push(c);
            }
            acc
        })
        .trim_end_matches('\\')
        .to_string();

    match tokio::fs::metadata(&normalized_path).await {
        Ok(metadata) if metadata.is_dir() => Ok(normalized_path),
        Ok(_) => Err("Path exists but is not a directory".into()),
        Err(e) => Err(format!("Directory does not exist: {}", e).into()),
    }
}

// Lists contents of archive file using 7z
pub async fn get_archive_content(path: String) -> Result<String, Box<dyn Error>> {
    let output = Command::new(get_7z_path()?)
        .args([
            "l", // List contents command
            &path, "-y", // Yes to all prompts
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    // Parse 7z output format and extract file paths
    let mut is_output_section = false;
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    let mut list: Vec<String> = Vec::new();
    for line in output_str.lines() {
        if line.contains("------------------------") {
            // Toggle section when separator line is found
            is_output_section = !is_output_section;
            continue;
        }

        // Process lines only between separators and extract file paths
        if is_output_section && !line.trim().is_empty() && line.len() > 53 {
            let file_path = line[53..].trim();
            if !file_path.is_empty() {
                list.push(file_path.to_owned());
            }
        }
    }
    Ok(serde_json::to_string(&list)?)
}

// Helper function to sanitize paths by removing potentially dangerous components
pub fn sanitize_path(path: &str) -> String {
    let path = path.replace('/', "\\");

    // Split the path by directory separators
    let parts: Vec<&str> = path.split('\\').collect();

    // Filter out parts that could lead to path traversal
    let safe_parts: Vec<&str> = parts
        .into_iter()
        .filter(|part| {
            !part.is_empty() && *part != "." && *part != ".." && !part.contains(':')
            // Remove drive letters
        })
        .collect();

    safe_parts.join("\\")
}

pub async fn open(path: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new(path)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok("".to_owned())
}

pub async fn open_folder(path: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("explorer")
        .arg("/select,")
        .arg(path)
        .creation_flags(0x08000000)
        .output()
        .await?;
    if !output.stderr.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok("".to_owned())
}

pub async fn check_path_empty(path: &str) -> Result<String, Box<dyn Error>> {
    // Check if directory exists
    if let Ok(mut entries) = tokio::fs::read_dir(path).await {
        // Check if directory has any contents
        if entries.next_entry().await?.is_some() {
            return Err("Installation directory is not empty".into());
        }
    }

    Ok("".to_string())
}

pub async fn open_registry(name: &str, current_user_only: bool) -> Result<String, Box<dyn Error>> {
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

pub async fn cli(app: AppHandle) -> Result<String, Box<dyn Error>> {
    let mut receiver = CHANNEL.1.resubscribe();
    loop {
        if let Ok(msg) = receiver.recv().await {
            app.emit("install", &msg)?;
        }
    }
}
