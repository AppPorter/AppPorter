use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose::STANDARD};
use dirs;
use std::path::Path;
use systemicons::get_icon;
use tokio::fs;

pub async fn convert_icon_to_base64(path: &str) -> Result<String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err(anyhow!("Icon file does not exist"));
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .ok_or(anyhow!("Failed to get file extension"))?;

    match extension.as_str() {
        "exe" => {
            let raw_icon = get_icon(&path.to_string_lossy(), 64)
                .map_err(|e| anyhow!("Failed to extract icon from exe: {:?}", e))?;
            Ok(format!(
                "data:image/png;base64,{}",
                STANDARD.encode(&raw_icon)
            ))
        }
        "ico" => {
            let icon_data = tokio::fs::read(&path).await?;
            Ok(format!(
                "data:image/x-icon;base64,{}",
                STANDARD.encode(&icon_data)
            ))
        }
        "png" => {
            let icon_data = tokio::fs::read(&path).await?;
            Ok(format!(
                "data:image/png;base64,{}",
                STANDARD.encode(&icon_data)
            ))
        }
        _ => Err(anyhow!(
            "Unsupported icon format. Only .ico, .png, and .exe files are supported"
        )),
    }
}

pub async fn convert_base64_to_ico(base64_data: &str, filename: &str) -> Result<String> {
    let icons_dir = dirs::config_local_dir()
        .ok_or(anyhow!("Failed to get local config directory"))?
        .join("AppPorter")
        .join("icons");

    fs::create_dir_all(&icons_dir).await?;

    let base64_clean = if base64_data.starts_with("data:") {
        base64_data
            .split(',')
            .nth(1)
            .ok_or(anyhow!("Invalid base64 data URL format"))?
            .to_owned()
    } else {
        base64_data.to_owned()
    };

    let image_data = STANDARD
        .decode(&base64_clean)
        .map_err(|e| anyhow!("Failed to decode base64 data: {}", e))?;

    let output_filename = if filename.ends_with(".ico") {
        filename
    } else {
        &format!("{}.ico", filename)
    };
    let output_path = icons_dir.join(output_filename);

    tokio::task::spawn_blocking({
        let output_path = output_path.clone();
        move || -> Result<()> {
            let img = image::load_from_memory(&image_data)?;

            let resized_img = img.resize(32, 32, image::imageops::FilterType::Lanczos3);

            let rgba_img = resized_img.to_rgba8();

            let mut ico_dir = ico::IconDir::new(ico::ResourceType::Icon);
            let ico_image = ico::IconImage::from_rgba_data(32, 32, rgba_img.into_raw());
            ico_dir.add_entry(ico::IconDirEntry::encode(&ico_image)?);

            let mut file = std::fs::File::create(&output_path)?;
            ico_dir.write(&mut file)?;

            Ok(())
        }
    })
    .await??;

    Ok(output_path.to_string_lossy().to_string())
}
