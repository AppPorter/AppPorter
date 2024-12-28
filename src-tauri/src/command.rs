use crate::result_process;
use crate::settings::*;
use serde::Deserialize;
use std::{error::Error, result::Result};

#[derive(Deserialize)]
pub enum Commands {
    ReadSettings,
}

#[tauri::command]
pub fn execute_command(command: Commands) -> Result<(), tauri::Error> {
    let result: Result<Settings, Box<dyn Error>> = match command {
        Commands::ReadSettings => read_settings(),
    };
    result_process(result);
    Ok(())
}
