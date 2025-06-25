use crate::utils::download_file;
use anyhow::Result;
use tauri::{AppHandle, Emitter};

pub async fn preview_url(app: &AppHandle, url: String) -> Result<()> {
    let timestamp = chrono::Utc::now().timestamp();
    let downloaded = download_file(&url).await?;
    app.emit("preview_url", (downloaded, timestamp, url))?;
    Ok(())
}
