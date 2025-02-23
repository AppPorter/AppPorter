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
    println!("get_details: {:#?}", req);
    // Create temp directory
    let temp_dir = tempdir()?;
    let temp_exe_path = temp_dir.path().join(&req.executable_path);
    let executable_path = req.executable_path.clone();

    // Create parent directories if they don't exist
    if let Some(parent) = temp_exe_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Do synchronous zip operations in a blocking task
    let (buffer, exe_found) =
        tokio::task::spawn_blocking(move || -> Result<(Vec<u8>, bool), String> {
            let file = std::fs::File::open(&req.zip_path)
                .map_err(|e| format!("Failed to open zip file: {}", e))?;
            let mut archive =
                ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;
            let mut buffer = Vec::new();

            let result = match archive.by_name(&executable_path) {
                Ok(mut exe_file) => {
                    exe_file
                        .read_to_end(&mut buffer)
                        .map_err(|e| format!("Failed to read executable: {}", e))?;
                    Ok((buffer, true))
                }
                Err(_) => Ok((Vec::new(), false)),
            };
            result
        })
        .await
        .map_err(|e| format!("Thread join error: {}", e))??;

    if !exe_found {
        return Err(format!(
            "Failed to find executable '{}' in archive",
            req.executable_path
        )
        .into());
    }

    app.emit("get_details", 1)?;

    // Write to temporary file in the temp directory
    let parent_dir = temp_exe_path.parent().unwrap_or(temp_dir.path());
    let temp_file = tempfile_in(parent_dir)?;
    tokio::fs::write(&temp_exe_path, &buffer).await?;

    app.emit("get_details", 2)?;

    let raw_icon = get_icon(&temp_exe_path.to_string_lossy(), 64).unwrap_or_default();
    let icon_base64 = STANDARD.encode(&raw_icon);
    let icon_data_url = format!("data:image/png;base64,{}", icon_base64);
    // Prepare PowerShell command
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
                filename = [System.IO.Path]::GetFileNameWithoutExtension($file_path);
            }})
        }}"#,
        temp_exe_path.to_string_lossy().replace("'", "''")
    );

    app.emit("get_details", 3)?;

    // Execute PowerShell command
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

    app.emit("get_details", 4)?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let details: Value = serde_json::from_str(&output_str)?;
    println!("get_details details: {:#?}", details);
    // Process the data in Rust with new priority logic
    let filename = details["filename"].as_str().unwrap_or_default();

    // Get software name based on priority
    let product_name = details["product_name"]
        .as_str()
        .and_then(|s| {
            if !s.trim().is_empty() {
                Some(s.trim())
            } else {
                None
            }
        })
        .or_else(|| {
            details["file_description"].as_str().and_then(|s| {
                if !s.trim().is_empty() {
                    Some(s.trim())
                } else {
                    None
                }
            })
        })
        .or_else(|| {
            details["original_filename"].as_str().and_then(|s| {
                if !s.trim().is_empty() {
                    Some(s.trim().trim_end_matches(".exe").trim())
                } else {
                    None
                }
            })
        })
        .unwrap_or(filename);

    // Get version based on priority
    let version = details["product_version"]
        .as_str()
        .and_then(|s| {
            if !s.trim().is_empty() {
                Some(s.trim())
            } else {
                None
            }
        })
        .or_else(|| {
            details["file_version"].as_str().and_then(|s| {
                if !s.trim().is_empty() {
                    Some(s.trim())
                } else {
                    None
                }
            })
        })
        .unwrap_or_default();

    // Get copyright information
    let copyright = details["copyright"]
        .as_str()
        .and_then(|s| {
            if !s.trim().is_empty() {
                Some(s.trim())
            } else {
                None
            }
        })
        .unwrap_or_default();

    let response = json!([product_name, version, copyright, icon_data_url]);

    println!("get_details response: {:#?}", response);

    // Keep temp_dir and temp_file in scope until all operations are complete
    drop(temp_file);
    drop(temp_dir);

    Ok(response.to_string())
}
