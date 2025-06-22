use base64::{Engine, engine::general_purpose::STANDARD};
use dirs;
use std::{error::Error, path::Path};
use systemicons::get_icon;
use tokio::fs;

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

// Converts base64 icon data to physical ico file
pub async fn convert_base64_to_ico(
    base64_data: String,
    filename: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Create the icons directory path
    let icons_dir = dirs::config_local_dir()
        .ok_or("Failed to get local config directory")?
        .join("AppPorter")
        .join("icons");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&icons_dir).await?;

    // Remove data URL prefix if present
    let base64_clean = if base64_data.starts_with("data:") {
        base64_data
            .split(',')
            .nth(1)
            .ok_or("Invalid base64 data URL format")?
            .to_string()
    } else {
        base64_data
    };

    // Decode base64 data
    let image_data = STANDARD
        .decode(&base64_clean)
        .map_err(|e| format!("Failed to decode base64 data: {}", e))?;

    // Determine output file path
    let output_filename = if filename.ends_with(".ico") {
        filename
    } else {
        format!("{}.ico", filename)
    };
    let output_path = icons_dir.join(&output_filename);

    // Try to process as different image formats and convert to ICO
    tokio::task::spawn_blocking({
        let image_data = image_data.clone();
        let output_path = output_path.clone();
        move || -> Result<(), Box<dyn Error + Send + Sync>> {
            // Try to load the image using the image crate
            let img = image::load_from_memory(&image_data)?;

            // Resize to 32x32 if needed (common ICO size)
            let resized_img = img.resize(32, 32, image::imageops::FilterType::Lanczos3);

            // Convert to RGBA8
            let rgba_img = resized_img.to_rgba8();

            // Create ICO file
            let mut ico_dir = ico::IconDir::new(ico::ResourceType::Icon);
            let ico_image = ico::IconImage::from_rgba_data(32, 32, rgba_img.into_raw());
            ico_dir.add_entry(ico::IconDirEntry::encode(&ico_image)?);

            // Write to file
            let mut file = std::fs::File::create(&output_path)?;
            ico_dir.write(&mut file)?;

            Ok(())
        }
    })
    .await??;

    Ok(output_path.to_string_lossy().to_string())
}
