use crate::SUPPORTED_EXTENSIONS;
use std::error::Error;
use windows_registry::CURRENT_USER;

pub fn register_context_menu() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_path = std::env::current_exe()?.to_str().unwrap_or("").to_owned();

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
            .set_string("", format!(r#""{}" preview "%1""#, app_path))?;
    }

    Ok(())
}

pub fn unregister_context_menu() -> Result<(), Box<dyn Error + Send + Sync>> {
    for ext in SUPPORTED_EXTENSIONS {
        let base_path = format!(
            "Software\\Classes\\SystemFileAssociations\\.{}\\shell\\AppPorter",
            ext
        );
        CURRENT_USER.remove_tree(&base_path)?;
    }
    Ok(())
}

pub fn check_and_fix_context_menu() -> Result<bool, Box<dyn Error + Send + Sync>> {
    let app_path = std::env::current_exe()?.to_str().unwrap_or("").to_owned();
    let expected_command = format!(r#""{}" preview "%1""#, app_path);
    let expected_display_name = "Install using AppPorter";

    let mut any_exists = false;
    let mut needs_fix = false;

    for ext in SUPPORTED_EXTENSIONS {
        let base_path = format!(
            "Software\\Classes\\SystemFileAssociations\\.{}\\shell\\AppPorter",
            ext
        );

        // Check if registry key exists
        if let Ok(shell_key) = CURRENT_USER.open(&base_path) {
            any_exists = true;

            // Check display name
            let current_display_name = shell_key.get_string("").unwrap_or_default();
            if current_display_name != expected_display_name {
                needs_fix = true;
                break;
            }

            // Check icon path
            let current_icon = shell_key.get_string("Icon").unwrap_or_default();
            if current_icon != app_path {
                needs_fix = true;
                break;
            }

            // Check command
            let command_path = format!("{}\\command", base_path);
            if let Ok(command_key) = CURRENT_USER.open(&command_path) {
                let current_command = command_key.get_string("").unwrap_or_default();
                if current_command != expected_command {
                    needs_fix = true;
                    break;
                }
            } else {
                // Command key doesn't exist
                needs_fix = true;
                break;
            }
        }
    }

    if needs_fix {
        register_context_menu()?;
    }

    Ok(any_exists)
}
