pub mod install_app;
pub mod install_tool;
pub mod installer_mode;

use futures_util::future::BoxFuture;
pub use install_app::*;
pub use install_tool::*;
pub use installer_mode::*;

use std::{error::Error, path::Path};
use tokio::fs;

// Function to flatten nested single folders up to 3 levels deep and find executable
pub async fn flatten_nested_folders(
    install_path: &str,
    original_exe_path: Option<&str>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    for _ in 0..3 {
        let mut entries = fs::read_dir(install_path).await?;
        let mut items = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            items.push(entry.file_name().to_string_lossy().to_string());
        }

        // If there's only one item and it's a directory, move its contents up
        if items.len() == 1 {
            let single_item = &items[0];
            let single_item_path = Path::new(install_path).join(single_item);

            if fs::metadata(&single_item_path).await?.is_dir() {
                // Create a temporary directory to avoid conflicts
                let temp_dir = Path::new(install_path).join("__temp_flatten__");
                fs::rename(&single_item_path, &temp_dir).await?;

                // Move all contents from temp directory to install path
                let mut temp_entries = fs::read_dir(&temp_dir).await?;
                while let Some(entry) = temp_entries.next_entry().await? {
                    let source = entry.path();
                    let target = Path::new(install_path).join(entry.file_name());
                    fs::rename(source, target).await?;
                }

                // Remove the now empty temp directory
                fs::remove_dir(&temp_dir).await?;
            } else {
                // If it's not a directory, stop flattening
                break;
            }
        } else {
            // If there's more than one item, stop flattening
            break;
        }
    }

    // Find the executable file after flattening
    if let Some(exe_path) = original_exe_path {
        let exe_name = Path::new(exe_path)
            .file_name()
            .ok_or("Failed to get executable file name")?
            .to_string_lossy()
            .to_string();

        let actual_exe_path = find_executable_in_directory(install_path, &exe_name)
            .await?
            .ok_or("Executable file not found after flattening")?;

        Ok(actual_exe_path)
    } else {
        // For tools that don't specify an executable, just return the install path
        Ok(install_path.to_owned())
    }
}

// Helper function to recursively find executable file
fn find_executable_in_directory<'a>(
    dir_path: &'a str,
    exe_name: &'a str,
) -> BoxFuture<'a, Result<Option<String>, Box<dyn Error + Send + Sync>>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if path.is_file() && file_name.eq_ignore_ascii_case(exe_name) {
                return Ok(Some(path.to_string_lossy().to_string()));
            } else if path.is_dir() {
                if let Ok(Some(found_path)) =
                    find_executable_in_directory(&path.to_string_lossy(), exe_name).await
                {
                    return Ok(Some(found_path));
                }
            }
        }

        Ok(None)
    })
}
