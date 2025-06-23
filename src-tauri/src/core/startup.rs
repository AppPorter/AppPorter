use anyhow::{Result, anyhow};
use windows_registry::CURRENT_USER;

pub fn set_startup() -> Result<()> {
    let shell_key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    shell_key.set_string(
        "AppPorter",
        format!(
            "\"{}\" --silent",
            std::env::current_exe()?
                .to_str()
                .ok_or(anyhow!("Failed to get current exe path"))?
        ),
    )?;
    Ok(())
}

pub fn remove_startup() -> Result<()> {
    let shell_key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    shell_key.remove_value("AppPorter")?;
    Ok(())
}

pub fn check_and_fix_startup() -> Result<bool> {
    let current_exe = std::env::current_exe()?;
    let app_path = current_exe
        .to_str()
        .ok_or(anyhow!("Failed to get current exe path"))?;
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
