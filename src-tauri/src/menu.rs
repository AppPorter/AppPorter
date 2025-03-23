use check_elevation::is_elevated;
use std::io;
use windows_registry::{CLASSES_ROOT, CURRENT_USER};

const SUPPORTED_EXTENSIONS: [&str; 8] = ["zip", "7z", "rar", "tar", "gz", "bz2", "xz", "cab"];

pub fn register_context_menu() -> io::Result<()> {
    is_elevated()?;
    let app_path = std::env::current_exe()?.to_str().unwrap_or("").to_string();

    for ext in SUPPORTED_EXTENSIONS {
        let prog_id = CLASSES_ROOT
            .open(format!(".{}", ext))?
            .get_string("")
            .unwrap_or_default();

        if !prog_id.is_empty() {
            let base_path = format!("Software\\Classes\\{}\\shell\\AppPorter", prog_id);
            let shell_key = CURRENT_USER.create(&base_path)?;
            println!("{:?}", shell_key);
            shell_key.set_string("", "Open with AppPorter")?;
            shell_key.set_string("Icon", &app_path)?;
            CURRENT_USER
                .create(format!("{}\\command", base_path))?
                .set_string("", &format!("\"{}\" \"%1\"", app_path))?;
        }
    }
    Ok(())
}

pub fn unregister_context_menu() -> io::Result<()> {
    is_elevated()?;

    for ext in SUPPORTED_EXTENSIONS {
        if let Ok(prog_id) = CLASSES_ROOT.open(format!(".{}", ext))?.get_string("") {
            if !prog_id.is_empty() {
                let base_path = format!("Software\\Classes\\{}\\shell\\AppPorter", prog_id);
                let _ = delete_key(&format!("{}\\command", base_path));
                let _ = delete_key(&base_path);
            }
        }
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
