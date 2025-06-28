use crate::operations::extract_archive_files;
use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use tauri::AppHandle;

pub async fn run_installer(
    zip_path: &str,
    executable_path: &str,
    password: &str,
    app: &AppHandle,
) -> Result<()> {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let temp_dir = std::env::temp_dir()
        .join("AppPorter")
        .join(format!("installer_{timestamp}"));
    std::fs::create_dir_all(&temp_dir)?;

    extract_archive_files(
        zip_path,
        &temp_dir.to_string_lossy(),
        app,
        password,
        "installer_extract_progress",
    )
    .await?;

    let exe_full_path = temp_dir.join(executable_path.replace("/", "\\"));

    if !exe_full_path.exists() {
        return Err(anyhow!("Executable not found: {}", exe_full_path.display()));
    }

    let output = Command::new(&exe_full_path)
        .current_dir(
            exe_full_path
                .parent()
                .ok_or(anyhow!("Failed to get parent directory"))?,
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    std::fs::remove_dir_all(&temp_dir)?;

    if output.stderr.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "Installer failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
