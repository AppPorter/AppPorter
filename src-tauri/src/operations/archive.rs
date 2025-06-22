use std::error::Error;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{env, fs};
use tauri::{AppHandle, Emitter};
use tokio::process::Command;

// Returns path to 7z.exe, extracts both 7z.exe and 7z.dll from resources if needed
// The files will be stored in the temp directory: %TEMP%\AppPorter\
pub fn get_7z_path() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let current_exe = env::current_exe()?;
    let current_dir = current_exe.parent().ok_or("Failed to get exe directory")?;

    let files = vec![
        (
            "7z.dll",
            include_bytes!("../../resources/7z.dll").as_slice(),
        ),
        (
            "7z.exe",
            include_bytes!("../../resources/7z.exe").as_slice(),
        ),
    ];

    for (filename, bytes) in &files {
        let file_path = current_dir.join(filename);
        if !file_path.exists() || fs::metadata(&file_path)?.len() != bytes.len() as u64 {
            fs::write(&file_path, bytes)?;
        }
    }

    Ok(current_dir.join("7z.exe"))
}

pub fn sanitize_path(path: &str) -> String {
    path.replace('/', "\\")
        .split('\\')
        .filter(|part| !part.is_empty() && *part != "." && *part != ".." && !part.contains(':'))
        .collect::<Vec<_>>()
        .join("\\")
}

// Lists contents of archive file using 7z
pub async fn get_archive_content(
    path: String,
    password: Option<String>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let output = Command::new(get_7z_path()?)
        .args([
            "l", // List contents command
            &path,
            "-y", // Yes to all prompts
            &format!("-p{}", password.unwrap_or_default()),
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        if error_str.contains("Cannot open encrypted archive. Wrong password?") {
            return Err("Wrong password".into());
        }
        return Err(error_str.into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    Ok(parse_7z_list_output(&output_str))
}

// Common file extraction function for both apps and tools
pub async fn extract_archive_files(
    zip_path: &str,
    install_path: &str,
    app: AppHandle,
    password: Option<&str>,
    event_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path_7z = get_7z_path()?;
    let password_arg = password.map(|p| format!("-p{}", p)).unwrap_or_default();

    // Validate archive first
    let output = Command::new(&path_7z)
        .args(["l", zip_path, "-y", &password_arg])
        .creation_flags(0x08000000)
        .output()
        .await?;

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        return Err(
            if error_str.contains("Cannot open encrypted archive. Wrong password?") {
                "Wrong password".into()
            } else {
                "Failed to list archive contents".into()
            },
        );
    }

    // Validate paths for security
    let paths = parse_7z_list_output(&String::from_utf8_lossy(&output.stdout));
    let canonical_install_path = fs::canonicalize(install_path)?;

    for path in &paths {
        let target_path = Path::new(install_path).join(sanitize_path(path));
        if let Ok(canonical_path) =
            fs::canonicalize(target_path.parent().unwrap_or(Path::new(install_path)))
        {
            if !canonical_path.starts_with(&canonical_install_path) {
                return Err(format!(
                    "Security violation: Path traversal detected in archive: {}",
                    path
                )
                .into());
            }
        }
    }

    // Extract archive
    let output_dir = format!("-o{}", install_path);
    let mut extract_args = vec![
        "-bsp2", // set output stream
        "x",     // Extract files with full paths
        zip_path,
        &output_dir,
        "-y",   // Yes to all prompts
        "-aoa", // Overwrite all existing files without prompt
        "-snl", // Disable symbolic links
    ];
    extract_args.push(&password_arg);

    let mut child = std::process::Command::new(&path_7z)
        .args(extract_args)
        .creation_flags(0x08000000)
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stderr = child.stderr.take().unwrap();
    let mut buffer = [0; 1024];
    let app_clone = app.clone();
    let event_name = event_name.to_string();

    let handle = std::thread::spawn(move || {
        while let Ok(n) = stderr.read(&mut buffer) {
            if n == 0 {
                break;
            }
            if let Ok(output) = String::from_utf8(buffer[..n].to_vec()) {
                if let Some(percent_str) = output.split('%').next() {
                    if let Ok(percent) = percent_str.trim().parse::<u32>() {
                        let _ = app_clone.emit(&event_name, percent);
                    }
                }
            }
        }
    });

    let status = child.wait()?;
    if !status.success() {
        return Err("7-Zip extraction failed".into());
    }
    handle.join().unwrap();

    Ok(())
}

// Common function to parse 7z list output
pub fn parse_7z_list_output(output: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut is_output_section = false;

    for line in output.lines() {
        if line.contains("------------------------") {
            // Toggle output section when separator line is found
            is_output_section = !is_output_section;
            continue;
        }

        // Only process lines between separators
        if is_output_section {
            // 7z output format: date time attr size compressed_size filename
            // The filename starts at column 53 (0-indexed)
            if line.len() >= 53 {
                let filename = line[53..].trim();
                if !filename.is_empty() {
                    // Check if it's a directory by looking at the attr column
                    // Attr starts at column 20 and is 5 characters long
                    let mut final_filename = filename.to_string();
                    if line.len() >= 25 {
                        let attr = &line[20..25];
                        // If attr starts with 'D', it's a directory
                        if attr.starts_with('D') && !final_filename.ends_with('\\') {
                            final_filename.push('\\');
                        }
                    }
                    result.push(final_filename);
                }
            }
        }
    }
    result
}
