use super::sanitize_path;
use crate::get_7z_path;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::path::Path;
use systemicons::get_icon;
use tempfile::tempdir;
use tokio::process::Command;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExePath {
    pub zip_path: String,
    pub executable_path: String,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExeDetails {
    pub product_name: String,
    pub version: String,
    pub copyright: String,
    pub icon_data_url: String,
}

// Extracts metadata from an executable file within a zip archive
pub async fn get_details(input: ExePath) -> Result<ExeDetails, Box<dyn Error + Send + Sync>> {
    let temp_dir = tempdir()?;

    // Sanitize the executable path to prevent directory traversal
    let sanitized_path = sanitize_path(&input.executable_path);
    let temp_exe_path = temp_dir.path().join(&sanitized_path);

    if let Some(parent) = temp_exe_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    };

    // Validate that the path doesn't escape the temp directory
    let temp_dir_canonical = std::fs::canonicalize(temp_dir.path())?;
    let parent_canonical = if let Some(parent) = temp_exe_path.parent() {
        std::fs::canonicalize(parent).ok()
    } else {
        Some(temp_dir_canonical.clone())
    };

    // Ensure path is within temp directory
    if let Some(parent_path) = parent_canonical {
        if !parent_path.starts_with(&temp_dir_canonical) {
            return Err(format!(
                "Security violation: Path traversal detected in executable path: {}",
                input.executable_path
            )
            .into());
        }
    }

    // Extract specific file using 7z.exe

    let dir = format!("-o{}", temp_dir.path().to_str().unwrap_or_default());

    let mut args = vec![
        "e", // Extract without full paths
        &input.zip_path,
        &input.executable_path,
        &dir,   // Output directory flag
        "-y",   // Auto yes to all prompts
        "-aoa", // Overwrite all existing files without prompt
        "-snl", // Disable symbolic links
    ]; // Add password if provided

    let mut pw = String::new();
    if let Some(password) = &input.password {
        pw = format!("-p{}", password);
    }
    args.push(&pw);

    let output1 = Command::new(get_7z_path()?)
        .args(args)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;
    if !output1.status.success() {
        let error_str = String::from_utf8_lossy(&output1.stderr).to_string();
        if error_str.contains("Cannot open encrypted archive. Wrong password?") {
            return Err("Wrong password".into());
        }
        return Err(String::from_utf8_lossy(&output1.stderr).into());
    }

    // Check if file was extracted
    let file_name = Path::new(&sanitized_path).file_name().unwrap_or_default();

    let extracted_file = temp_dir.path().join(file_name);

    if !extracted_file.exists() {
        return Err(format!(
            "Failed to find executable '{}' in archive",
            input.executable_path
        )
        .into());
    }

    // Extract icon as data URL
    let raw_icon = get_icon(&extracted_file.to_string_lossy(), 64).unwrap_or_default();
    let icon_data_url = format!("data:image/png;base64,{}", STANDARD.encode(&raw_icon));

    // Build PowerShell command for file metadata extraction with path sanitization
    let extracted_file_str = extracted_file.to_string_lossy();
    let safe_path_for_ps = extracted_file_str.replace("'", "''");
    let ps_command = format!(
        r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8;
        $ErrorActionPreference = 'Stop';
        try {{
            $file_path = '{}';
            $version_info = (Get-Item $file_path).VersionInfo;
            Write-Output (ConvertTo-Json -Compress @{{
                product_name = $version_info.ProductName;
                file_description = $version_info.FileDescription;
                original_filename = $version_info.OriginalFilename;
                product_version = $version_info.ProductVersion;
                file_version = $version_info.FileVersion;
                copyright = $version_info.LegalCopyright;
                filename = [System.IO.Path]::GetFileNameWithoutExtension($file_path);
            }})
        }} catch {{
            Write-Output (ConvertTo-Json -Compress @{{
                error = $_.Exception.Message;
            }})
        }}"#,
        safe_path_for_ps
    );

    // Execute PowerShell to get file metadata
    let output2 = Command::new("powershell")
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

    if !output2.status.success() {
        return Err(String::from_utf8_lossy(&output2.stderr).into());
    }

    let output_str = String::from_utf8_lossy(&output2.stdout);
    let details: Value = serde_json::from_str(&output_str)?;

    // Get valid non-empty string value with fallbacks
    let product_name = get_valid_str(&details["product_name"])
        .or_else(|| get_valid_str(&details["file_description"]))
        .or_else(|| {
            get_valid_str(&details["original_filename"]).map(|s| s.trim_end_matches(".exe"))
        })
        .or_else(|| get_valid_str(&details["filename"]))
        .unwrap_or_default();

    let version = get_valid_str(&details["product_version"])
        .or_else(|| get_valid_str(&details["file_version"]))
        .unwrap_or_default();

    let copyright = get_valid_str(&details["copyright"]).unwrap_or_default();

    drop(temp_dir);

    Ok(ExeDetails {
        product_name: product_name.to_owned(),
        version: version.to_owned(),
        copyright: copyright.to_owned(),
        icon_data_url,
    })
}

fn get_valid_str(value: &Value) -> Option<&str> {
    value
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim())
}
