use std::error::Error;
use windows_registry::CURRENT_USER;

pub fn set_startup() -> Result<String, Box<dyn Error + Send + Sync>> {
    let shell_key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    shell_key.set_string(
        "AppPorter",
        format!("\"{}\"", std::env::current_exe()?.to_str().unwrap_or("")),
    )?;
    Ok("".to_owned())
}

pub fn remove_startup() -> Result<String, Box<dyn Error + Send + Sync>> {
    let shell_key = CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    shell_key.remove_value("AppPorter")?;
    Ok("".to_owned())
}
