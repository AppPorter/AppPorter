use crate::format_result;
use crate::settings::*;
use std::result;
use std::{error::Error, result::Result};

#[tauri::command]
pub fn execute_command<T>(command: String, arg: T) -> Result<String, tauri::Error> {
    let result: Result<Settings, Box<dyn Error>> = match command.as_str() {
        "LoadSettings" => load_settings(),
        _ => Err("Unknown command".into()),
    };
    format_result(result)
}
