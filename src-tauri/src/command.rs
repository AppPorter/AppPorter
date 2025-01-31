use crate::{
    elevate,
    installation::{get_details, installation, ExePath, InstallationConfig},
    settings::load_settings,
    validate_path,
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
    ValidatePath { path: String },
}

#[tauri::command(async)]
pub async fn execute_command(command: Command, app: AppHandle) -> Result<String, String> {
    match command {
        Command::LoadSettings => load_settings().await,
        Command::GetDetails { path } => get_details(path, app),
        Command::Installation { config } => installation(config, app).await,
        Command::Elevate { revert } => elevate(revert).await.map(|_| "Success".to_string()),
        Command::ValidatePath { path } => validate_path(path),
    }
    .map_err(|e| e.to_string())
}
