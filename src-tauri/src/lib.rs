pub mod command;
pub mod installation;
pub mod settings;

pub fn result_process<T>(result: Result<T, Box<dyn std::error::Error>>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();
            #[cfg(debug_assertions)]
            eprintln!("{}", e);
        }
    }
}

pub fn result_send<T: std::fmt::Debug>(
    result: Result<T, Box<dyn std::error::Error>>,
    channel: tauri::ipc::Channel<String>,
) {
    let output = match result {
        Ok(result) => format!("{:#?}", result),
        Err(error) => error.to_string(),
    };
    channel.send(output);
}
