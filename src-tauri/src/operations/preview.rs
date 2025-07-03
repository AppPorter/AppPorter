use crate::utils::download_file;
use anyhow::Result;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

pub async fn preview_url(app: &AppHandle, url: &str) -> Result<()> {
    let downloaded = download_file(url).await?;
    app.emit(
        "preview_url",
        (
            downloaded,
            Uuid::new_v4().to_string(),
            chrono::Utc::now().to_rfc3339(),
        ),
    )?;
    Ok(())
}
