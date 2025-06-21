pub mod install_app;
pub mod install_tool;
pub mod installer_mode;

pub use install_app::*;
pub use install_tool::*;
pub use installer_mode::*;

use std::{error::Error, path::Path};

// Function to flatten nested single folders up to 3 levels deep
pub async fn flatten_nested_folders(
    install_path: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for _ in 0..3 {
        let mut entries = tokio::fs::read_dir(install_path).await?;
        let mut items = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            items.push(entry.file_name().to_string_lossy().to_string());
        }

        // If there's only one item and it's a directory, move its contents up
        if items.len() == 1 {
            let single_item = &items[0];
            let single_item_path = Path::new(install_path).join(single_item);

            if tokio::fs::metadata(&single_item_path).await?.is_dir() {
                // Create a temporary directory to avoid conflicts
                let temp_dir = Path::new(install_path).join("__temp_flatten__");
                tokio::fs::rename(&single_item_path, &temp_dir).await?;

                // Move all contents from temp directory to install path
                let mut temp_entries = tokio::fs::read_dir(&temp_dir).await?;
                while let Some(entry) = temp_entries.next_entry().await? {
                    let source = entry.path();
                    let target = Path::new(install_path).join(entry.file_name());
                    tokio::fs::rename(source, target).await?;
                }

                // Remove the now empty temp directory
                tokio::fs::remove_dir(&temp_dir).await?;
            } else {
                // If it's not a directory, stop flattening
                break;
            }
        } else {
            // If there's more than one item, stop flattening
            break;
        }
    }

    Ok(())
}
