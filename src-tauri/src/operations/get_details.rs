use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use serde_json::{json, Value};
use std::error::Error;
use std::io::Read;
use systemicons::get_icon;
use tauri::{AppHandle, Emitter};
use tempfile::{tempdir, tempfile_in};
use tokio::process::Command;
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
pub struct ExePath {
    pub zip_path: String,
    pub executable_path: String,
}

pub async fn get_details(req: ExePath, app: AppHandle) -> Result<String, Box<dyn Error>> {
    // Set up temp environment
    let temp_dir = tempdir()?;
    let temp_exe_path = temp_dir.path().join(&req.executable_path);
    let executable_path = req.executable_path.clone();

    if let Some(parent) = temp_exe_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Extract executable in blocking task
    app.emit("get_details", 0)?;
    let (buffer, exe_found) =
        tokio::task::spawn_blocking(move || -> Result<(Vec<u8>, bool), String> {
            let file = std::fs::File::open(&req.zip_path)
                .map_err(|e| format!("Failed to open zip file: {}", e))?;
            let mut archive =
                ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

            let mut buffer = Vec::new();
            let found = match archive.by_name(&executable_path) {
                Ok(mut exe_file) => {
                    exe_file
                        .read_to_end(&mut buffer)
                        .map_err(|e| format!("Failed to read executable: {}", e))?;
                    true
                }
                Err(_) => false,
            };

            Ok((buffer, found))
        })
        .await
        .map_err(|e| format!("Thread error: {}", e))??;

    if !exe_found {
        return Err(format!(
            "Failed to find executable '{}' in archive",
            req.executable_path
        )
        .into());
    }

    app.emit("get_details", 1)?;

    // Write executable to temp location
    let parent_dir = temp_exe_path.parent().unwrap_or(temp_dir.path());
    let temp_file = tempfile_in(parent_dir)?;
    tokio::fs::write(&temp_exe_path, &buffer).await?;

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

    app.emit("get_details", 4)?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    // Parse metadata with fallback priority
    let output_str = String::from_utf8_lossy(&output.stdout);
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
    drop(temp_file);
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
