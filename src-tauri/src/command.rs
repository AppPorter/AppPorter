use crate::installation::setup::preview_zip;
use crate::settings::*;
use std::result::Result;

#[tauri::command]
pub fn execute_command(command: String, arg: Option<String>) -> Result<String, tauri::Error> {
    let result = match command.as_str() {
        "LoadSettings" => load_settings(),
        "PreviewZip" => preview_zip(&arg.unwrap_or_default()),
        _ => Err("Unknown command".into()),
    };
    let output = match result {
        Ok(value) => value,
        Err(error) => error.to_string(),
    };
    Ok(output)
}
