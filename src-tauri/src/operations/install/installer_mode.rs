use crate::operations::extract_archive_files;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use tauri::AppHandle;

pub async fn run_installer(
    zip_path: String,
    executable_path: String,
    password: Option<String>,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Create temporary directory for extraction using timestamp
    let timestamp = chrono::Utc::now().timestamp_millis();
    let temp_dir = std::env::temp_dir()
        .join("AppPorter")
        .join(format!("installer_{}", timestamp));
    std::fs::create_dir_all(&temp_dir)?;

    // Extract the entire archive to temp directory
    extract_archive_files(
        &zip_path,
        &temp_dir.to_string_lossy(),
        app.clone(),
        password.as_deref(),
        "installer_extract_progress",
    )
    .await?;

    // Build the full path to the executable
    let exe_full_path = temp_dir.join(executable_path.replace("/", "\\"));

    if !exe_full_path.exists() {
        return Err(format!("Executable not found: {}", exe_full_path.display()).into());
    }

    // Run the installer executable
    let output = Command::new(&exe_full_path)
        .current_dir(exe_full_path.parent().unwrap_or(Path::new(".")))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    // Clean up temporary directory
    std::fs::remove_dir_all(&temp_dir)?;

    if output.stderr.is_empty() {
        Ok("Installer completed successfully".to_string())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Installer failed: {}", error_msg).into())
    }
}
