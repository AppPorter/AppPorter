use std::{env, error::Error, fs, io, path::PathBuf};
use tokio::process::Command;

// Returns path to 7z.exe, extracts both 7z.exe and 7z.dll from resources if needed
// The files will be stored in the temp directory: %TEMP%\AppPorter\
pub fn get_7z_path() -> Result<PathBuf, io::Error> {
    let temp_dir = env::temp_dir().join("AppPorter");
    fs::create_dir_all(&temp_dir)?;

    // Extract and verify 7z.dll
    let seven_zip_dll_bytes = include_bytes!("../../resources/7z.dll");
    let seven_zip_dll_path = temp_dir.join("7z.dll");
    if !seven_zip_dll_path.exists()
        || fs::metadata(&seven_zip_dll_path)?.len() != seven_zip_dll_bytes.len() as u64
    {
        fs::write(&seven_zip_dll_path, seven_zip_dll_bytes)?;
    }

    // Extract and verify 7z.exe
    let seven_zip_bytes = include_bytes!("../../resources/7z.exe");
    let seven_zip_path = temp_dir.join("7z.exe");
    if !seven_zip_path.exists()
        || fs::metadata(&seven_zip_path)?.len() != seven_zip_bytes.len() as u64
    {
        fs::write(&seven_zip_path, seven_zip_bytes)?;
    }

    Ok(seven_zip_path)
}

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
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
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
    Ok(list)
}
