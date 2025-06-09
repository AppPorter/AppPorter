pub mod install_app;
pub mod install_tool;

pub use install_app::*;
pub use install_tool::*;

use std::{error::Error, path::Path};

// Common function to detect if extraction created a single root folder
pub async fn find_single_root_folder(
    install_path: &str,
) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let mut entries = tokio::fs::read_dir(install_path).await?;
    let mut items = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        items.push(entry.file_name().to_string_lossy().to_string());
    }

    if items.len() == 1 {
        let single_item = &items[0];
        let full_path = Path::new(install_path).join(single_item);
        if tokio::fs::metadata(full_path).await?.is_dir() {
            return Ok(Some(single_item.clone()));
        }
    }

    Ok(None)
}
