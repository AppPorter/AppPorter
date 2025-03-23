use check_elevation::is_elevated;
use std::io;
use windows_registry::CURRENT_USER;

const SUPPORTED_EXTENSIONS: [&str; 8] = ["zip", "7z", "rar", "tar", "gz", "bz2", "xz", "cab"];

pub fn register_context_menu() -> io::Result<()> {
    println!("Starting context menu registration...");
    is_elevated()?;
    let app_path = std::env::current_exe()?.to_str().unwrap_or("").to_string();
    println!("Application path: {}", app_path);

    for ext in SUPPORTED_EXTENSIONS {
        println!("Processing extension: .{}", ext);
        let base_path = format!(
            "Software\\Classes\\SystemFileAssociations\\.{}\\shell\\AppPorter",
            ext
        );
        println!("Creating registry key: {}", base_path);

        let shell_key = CURRENT_USER.create(&base_path)?;
        shell_key.set_string("", "Install using AppPorter")?;
        shell_key.set_string("Icon", &app_path)?;

        println!("Adding command for extension .{}", ext);
        CURRENT_USER
            .create(format!("{}\\command", base_path))?
            .set_string("", &format!("\"{}\" \"%1\"", app_path))?;
    }

    println!("Context menu registration completed successfully");
    Ok(())
}

pub fn unregister_context_menu() -> io::Result<()> {
    is_elevated()?;

    for ext in SUPPORTED_EXTENSIONS {
        let base_path = format!(
            "Software\\Classes\\SystemFileAssociations\\.{}\\shell\\AppPorter",
            ext
        );
        let _ = delete_key(&format!("{}\\command", base_path));
        let _ = delete_key(&base_path);
    }
    Ok(())
}

fn delete_key(path: &str) -> io::Result<()> {
    if let Some((parent_path, last_part)) = path.rsplit_once('\\') {
        if let Ok(parent_key) = CURRENT_USER.open(parent_path) {
            if parent_key.create(last_part).is_ok() {
                parent_key.set_string(last_part, "")?;
            }
        }
    }
    Ok(())
}
