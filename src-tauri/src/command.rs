use crate::installation::get_details;
use crate::settings::*;
use std::result::Result;

#[tauri::command]
pub fn execute_command(command: String, arg: Option<Vec<String>>) -> Result<String, tauri::Error> {
    let result = match command.as_str() {
        "LoadSettings" => load_settings(),
        "GetDetails" => get_details(arg.unwrap_or_default()),
        _ => Err("Unknown command".into()),
    };
    let output = match result {
        Ok(value) => value,
        Err(error) => error.to_string(),
    };
    Ok(output)
}
