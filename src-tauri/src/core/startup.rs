use std::error::Error;
use windows_registry::CURRENT_USER;

pub fn set_startup() -> Result<(), Box<dyn Error + Send + Sync>> {
    let shell_key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    shell_key.set_string(
        "AppPorter",
        format!(
            "\"{}\" --silent",
            std::env::current_exe()?.to_str().unwrap_or("")
        ),
    )?;
    Ok(())
}

pub fn remove_startup() -> Result<(), Box<dyn Error + Send + Sync>> {
    let shell_key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    shell_key.remove_value("AppPorter")?;
    Ok(())
}

pub fn check_and_fix_startup() -> Result<bool, Box<dyn Error + Send + Sync>> {
    let app_path = std::env::current_exe()?.to_str().unwrap_or("").to_owned();
    let expected_value = format!("\"{}\" --silent", app_path);

    if let Ok(shell_key) = CURRENT_USER.open(r"Software\Microsoft\Windows\CurrentVersion\Run") {
        if let Ok(current_value) = shell_key.get_string("AppPorter") {
            // Startup entry exists, check if it's correct
            if current_value != expected_value {
                // Fix the incorrect value by calling set_startup
                set_startup()?;
            }
            return Ok(true);
        }
    }

    // Startup entry doesn't exist
    Ok(false)
}
