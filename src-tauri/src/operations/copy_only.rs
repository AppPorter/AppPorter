use crate::get_7z_path;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Stdio;
use tauri::{AppHandle, Emitter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CopyOnlyConfig {
    pub zip_path: String,
    pub password: Option<String>,
    pub extract_path: String,
}

pub async fn copy_only(
    config: CopyOnlyConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let zip_path = config.zip_path.clone();
    let extract_path = config.extract_path.clone();

    // Notify frontend preparing
    app.emit("copyonly", 0)?;

    // Ensure extract directory exists
    if !Path::new(&extract_path).exists() {
        tokio::fs::create_dir_all(&extract_path).await?;
    }

    // Prepare 7z extraction args
    let path_7z = get_7z_path()?;
    let arg = format!("-o{}", extract_path);
    let mut args = vec![
        "-bsp2", // output stream for progress
        "x",     // extract with full paths
        &zip_path, &arg, "-y",   // yes to all
        "-aoa", // overwrite all
        "-snl", // disable symbolic links
    ];

    let mut pw = String::new();
    if let Some(pass) = &config.password {
        pw = format!("-p{}", pass);
    }
    args.push(&pw);

    let mut child = std::process::Command::new(&path_7z)
        .args(&args)
        .creation_flags(0x08000000)
        .stderr(Stdio::piped())
        .spawn()?;

    // Spawn thread to read progress and emit events
    if let Some(mut stderr) = child.stderr.take() {
        let app_clone = app.clone();
        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            while let Ok(n) = stderr.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                if let Ok(output) = String::from_utf8(buffer[..n].to_vec()) {
                    if output.contains('%') {
                        if let Ok(percent) = output
                            .trim()
                            .split('%')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .parse::<u32>()
                        {
                            let _ = app_clone.emit("copyonly_extract", percent);
                        }
                    }
                }
            }
        });
    }

    let status = child.wait()?;
    if !status.success() {
        return Err("7-Zip extraction failed".into());
    }

    // Notify frontend completed
    app.emit("copyonly", 101)?;

    Ok(extract_path)
}
