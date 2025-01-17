use crate::{
    elevate,
    installation::{get_details, installation, ExePath, InstallationConfig},
    settings::load_settings,
};
use serde::Deserialize;
use tauri::AppHandle;

#[derive(Deserialize)]
#[serde(tag = "name")]
pub enum Command {
    LoadSettings,
    GetDetails { path: ExePath },
    Installation { config: InstallationConfig },
    Elevate { revert: bool },
}

#[tauri::command(async)]
pub async fn execute_command(command: Command, app: AppHandle) -> Result<String, String> {
    match command {
        Command::LoadSettings => load_settings(),
        Command::GetDetails { path } => get_details(path, app),
        Command::Installation { config } => installation(config, app),
        Command::Elevate { revert } => elevate(revert).map(|_| "Success".to_string()),
    }
    .map_err(|e| e.to_string())
}
