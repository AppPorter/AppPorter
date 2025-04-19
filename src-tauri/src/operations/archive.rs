use crate::get_7z_path;
use std::error::Error;
use tokio::process::Command;

pub fn sanitize_path(path: &str) -> String {
    let path = path.replace('/', "\\");

    // Split the path by directory separators
    let parts: Vec<&str> = path.split('\\').collect();

    // Filter out parts that could lead to path traversal
    let safe_parts: Vec<&str> = parts
        .into_iter()
        .filter(|part| {
            !part.is_empty() && *part != "." && *part != ".." && !part.contains(':')
            // Remove drive letters
        })
        .collect();

    safe_parts.join("\\")
}

// Lists contents of archive file using 7z
pub async fn get_archive_content(
    path: String,
    password: Option<String>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let password = password.unwrap_or_default();
    let output = Command::new(get_7z_path()?)
        .args([
            "l", // List contents command
            &path,
            "-y", // Yes to all prompts
            &format!("-p{}", password),
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let error_str = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        if error_str.contains("Cannot open encrypted archive. Wrong password?") {
            return Err("Wrong password".into());
        }
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    // Parse 7z output format and extract file paths
    let mut is_output_section = false;

    let mut list: Vec<String> = Vec::new();
    for line in output_str.lines() {
        if line.contains("------------------------") {
            // Toggle section when separator line is found
            is_output_section = !is_output_section;
            continue;
        }

        // Process lines only between separators and extract file paths
        if is_output_section && !line.trim().is_empty() && line.len() > 53 {
            let file_path = line[53..].trim();
            if !file_path.is_empty() {
                list.push(file_path.to_owned());
            }
        }
    }
    Ok(serde_json::to_string(&list)?)
}
