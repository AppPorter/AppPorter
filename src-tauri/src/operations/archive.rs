use anyhow::{Result, anyhow};
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{env, fs};
use tauri::{AppHandle, Emitter};
use tokio::process::Command;

pub fn get_7z_path() -> Result<PathBuf> {
    let current_exe = env::current_exe()?;
    let current_dir = current_exe
        .parent()
        .ok_or_else(|| anyhow!("Failed to get exe directory"))?;

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

pub async fn get_archive_content(path: &str, password: Option<&str>) -> Result<Vec<String>> {
    let output = Command::new(get_7z_path()?)
        .args([
            "l",
            path,
            "-y",
            &format!("-p{}", password.ok_or(anyhow!("Failed to get password"))?),
        ])
        .creation_flags(0x08000000)
        .output()
        .await?;

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        if error_str.contains("Cannot open encrypted archive. Wrong password?") {
            return Err(anyhow!("Wrong password"));
        }
        return Err(anyhow!("{}", error_str));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(parse_7z_list_output(&output_str))
}

pub async fn extract_archive_files(
    zip_path: &str,
    install_path: &str,
    app: &AppHandle,
    password: &str,
    event_name: &str,
) -> Result<()> {
    let path_7z = get_7z_path()?;
    let password_arg = format!("-p{password}");

    let output = Command::new(&path_7z)
        .args(["l", zip_path, "-y", &password_arg])
        .creation_flags(0x08000000)
        .output()
        .await?;

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        return Err(
            if error_str.contains("Cannot open encrypted archive. Wrong password?") {
                anyhow!("Wrong password")
            } else {
                anyhow!("Failed to list archive contents")
            },
        );
    }

    let paths = parse_7z_list_output(&String::from_utf8_lossy(&output.stdout));
    let canonical_install_path = fs::canonicalize(install_path)?;

    for path in &paths {
        let target_path = Path::new(install_path).join(sanitize_path(path));
        if let Ok(canonical_path) =
            fs::canonicalize(target_path.parent().unwrap_or(Path::new(install_path)))
        {
            if !canonical_path.starts_with(&canonical_install_path) {
                return Err(anyhow!(
                    "Security violation: Path traversal detected in archive: {}",
                    path
                ));
            }
        }
    }

    let output_dir = format!("-o{install_path}");
    let mut extract_args = vec!["-bsp2", "x", zip_path, &output_dir, "-y", "-aoa", "-snl"];

    if !password.is_empty() {
        extract_args.push(&password_arg);
    }

    let mut child = std::process::Command::new(&path_7z)
        .args(extract_args)
        .creation_flags(0x08000000)
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stderr = child
        .stderr
        .take()
        .ok_or(anyhow!("Failed to capture stderr"))?;
    let mut buffer = [0; 1024];
    let app_clone = app.clone();
    let event_name = event_name.to_owned();

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
        return Err(anyhow!("7-Zip extraction failed"));
    }
    handle.join().map_err(|_| anyhow!("Thread join failed"))?;

    Ok(())
}

pub fn parse_7z_list_output(output: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut is_output_section = false;

    for line in output.lines() {
        if line.contains("------------------------") {
            is_output_section = !is_output_section;
            continue;
        }

        if is_output_section && line.len() >= 53 {
            let filename = line[53..].trim();
            if !filename.is_empty() {
                let mut final_filename = filename.to_owned();
                if line.len() >= 25 {
                    let attr = &line[20..25];

                    if attr.starts_with('D') && !final_filename.ends_with('\\') {
                        final_filename.push('\\');
                    }
                }
                result.push(final_filename);
            }
        }
    }
    result
}
