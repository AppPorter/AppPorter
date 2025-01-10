use crate::installation::{get_details, installation, ExePath, InstallationConfig};
use crate::settings::load_settings;
use serde_json::json;
use tauri::AppHandle;

#[tauri::command(async)]
pub async fn execute_command(
    command: String,
    arg: Option<String>,
    app: AppHandle,
) -> Result<String, String> {
    let result = match command.as_str() {
        "LoadSettings" => load_settings(),
        "GetDetails" => {
            let exe_path = serde_json::from_str::<ExePath>(&arg.ok_or("Arguments not found")?)
                .map_err(|e| e.to_string())?;
            get_details(exe_path, app)
        }
        "Installation" => {
            let config =
                serde_json::from_str::<InstallationConfig>(&arg.ok_or("Arguments not found")?)
                    .map_err(|e| e.to_string())?;
            installation(config, app)
        }
        _ => Err("Unknown command".into()),
    };

    match result {
        Ok(value) => Ok(value),
        Err(error) => Ok(json!({
            "error": error.to_string()
        })
        .to_string()),
    }
}
