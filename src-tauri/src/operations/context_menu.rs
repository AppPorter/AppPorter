use crate::SUPPORTED_EXTENSIONS;
use std::error::Error;
use windows_registry::CURRENT_USER;

pub fn register_context_menu() -> Result<String, Box<dyn Error + Send + Sync>> {
    let app_path = std::env::current_exe()?.to_str().unwrap_or("").to_string();

    for ext in SUPPORTED_EXTENSIONS {
        let base_path = format!(
            "Software\\Classes\\SystemFileAssociations\\.{}\\shell\\AppPorter",
            ext
        );

        let shell_key = CURRENT_USER.create(&base_path)?;
        shell_key.set_string("", "Install using AppPorter")?;
        shell_key.set_string("Icon", &app_path)?;

        CURRENT_USER
            .create(format!("{}\\command", base_path))?
            .set_string("", &format!("\"{}\" install \"%1\"", app_path))?;
    }

    Ok("".to_owned())
}

pub fn unregister_context_menu() -> Result<String, Box<dyn Error + Send + Sync>> {
    for ext in SUPPORTED_EXTENSIONS {
        let base_path = format!(
            "Software\\Classes\\SystemFileAssociations\\.{}\\shell\\AppPorter",
            ext
        );
        CURRENT_USER.remove_tree(&base_path)?;
    }
    Ok("".to_owned())
}
