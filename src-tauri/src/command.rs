use crate::installation::{get_details, installation, ExePath, InstallationConfig};
use crate::settings::load_settings;
use std::result::Result;

#[tauri::command]
pub fn execute_command(command: String, arg: Option<String>) -> Result<String, tauri::Error> {
    let result = match command.as_str() {
        "LoadSettings" => load_settings(),
        "GetDetails" => get_details(serde_json::from_str::<ExePath>(&arg.unwrap_or_default())?),
        "Installation" => installation(serde_json::from_str::<InstallationConfig>(
            &arg.unwrap_or_default(),
        )?),
        _ => Err("Unknown command".into()),
    };

    let output = match result {
        Ok(value) => value,
        Err(error) => serde_json::json!({
            "error": error.to_string()
        })
        .to_string(),
    };

    Ok(output)
}
