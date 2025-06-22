use base64::{engine::general_purpose::STANDARD, Engine};
use std::{error::Error, path::Path};
use systemicons::get_icon;

// Converts an icon file to base64 data URL
pub async fn convert_icon_to_base64(path: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let path = Path::new(&path);

    // Check if file exists
    if !path.exists() {
        return Err("Icon file does not exist".into());
    }

    // Check file extension
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "exe" => {
            // For EXE files, extract icon using systemicons
            let raw_icon = get_icon(&path.to_string_lossy(), 64).unwrap_or_default();
            if raw_icon.is_empty() {
                return Err("Failed to extract icon from EXE file".into());
            }
            Ok(format!(
                "data:image/png;base64,{}",
                STANDARD.encode(&raw_icon)
            ))
        }
        "ico" => {
            // For ICO files, read directly and encode
            let icon_data = tokio::fs::read(&path).await?;
            Ok(format!(
                "data:image/x-icon;base64,{}",
                STANDARD.encode(&icon_data)
            ))
        }
        "png" => {
            // For PNG files, read directly and encode
            let icon_data = tokio::fs::read(&path).await?;
            Ok(format!(
                "data:image/png;base64,{}",
                STANDARD.encode(&icon_data)
            ))
        }
        _ => Err("Unsupported icon format. Only .ico, .png, and .exe files are supported".into()),
    }
}
