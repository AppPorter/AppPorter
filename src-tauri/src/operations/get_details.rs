use crate::get_7z_path;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use serde_json::{json, Value};
use std::error::Error;
use std::path::Path;
use systemicons::get_icon;
use tauri::{AppHandle, Emitter};
use tempfile::tempdir;
use tokio::process::Command;

#[derive(Deserialize, Debug, Clone)]
pub struct ExePath {
    pub zip_path: String,
    pub executable_path: String,
}

pub async fn get_details(input: ExePath, app: AppHandle) -> Result<String, Box<dyn Error>> {
    app.emit("get_details", 1)?;
    // Set up temp environment
    let temp_dir = tempdir()?;
    let temp_exe_path = temp_dir.path().join(&input.executable_path);

    if let Some(parent) = temp_exe_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    };
    let path_7z = get_7z_path()?;
    // Extract specific file using 7z.exe
    let output1 = Command::new(path_7z)
        .args([
            "e", // Extract without full paths
            &input.zip_path,
            &input.executable_path,
            &format!("-o{}", temp_dir.path().to_str().unwrap_or(".")), // Output directory flag
            "-y",                                                      // Auto yes to all prompts
            "-aoa", // Overwrite all existing files without prompt
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;
    if !output1.status.success() {
        return Err(String::from_utf8_lossy(&output1.stderr).into());
    }

    // Check if file was extracted
    let extracted_file = Path::new(temp_dir.path()).join(
        Path::new(&input.executable_path)
            .file_name()
            .unwrap_or_default(),
    );

    if !extracted_file.exists() {
        return Err(format!(
            "Failed to find executable '{}' in archive",
            input.executable_path
        )
        .into());
    }

    app.emit("get_details", 2)?;

    // Extract icon as data URL
    let raw_icon = get_icon(&temp_exe_path.to_string_lossy(), 64).unwrap_or_default();
    let icon_data_url = format!("data:image/png;base64,{}", STANDARD.encode(&raw_icon));

    // Build PowerShell command for file metadata extraction
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
        temp_exe_path.to_string_lossy().replace("'", "''")
    );

    app.emit("get_details", 3)?;

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

    app.emit("get_details", 4)?;

    if !output2.status.success() {
        return Err(String::from_utf8_lossy(&output2.stderr).into());
    }

    // Parse metadata with fallback priority
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

    // Clean up temporary resources
    drop(temp_dir);

    Ok(json!([product_name, version, copyright, icon_data_url]).to_string())
}

// Helper function to get valid non-empty string value
fn get_valid_str(value: &Value) -> Option<&str> {
    value
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim())
}
