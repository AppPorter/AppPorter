pub mod command;
pub mod installation;
pub mod settings;

pub fn handle_error<T>(result: Result<T, Box<dyn std::error::Error>>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();
            #[cfg(debug_assertions)]
            eprintln!("{}", e);
        }
    }
}

pub fn format_result<T: serde::Serialize>(
    result: Result<T, Box<dyn std::error::Error>>,
) -> Result<String, tauri::Error> {
    let output = match result {
        Ok(value) => serde_json::to_string(&value)?,
        Err(error) => error.to_string(),
    };
    Ok(output)
}
