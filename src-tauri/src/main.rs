#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows",
    target_os = "windows"
)]

use config::Config;
use serde::Deserialize;
use std::{error::Error, result::Result};

#[derive(Debug, Deserialize)]
struct Settings {}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();
            #[cfg(debug_assertions)]
            eprintln!("{}", e);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() -> Result<(), Box<dyn Error>> {
    println!(
        "{:#?}",
        Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()?
            .try_deserialize::<Settings>()?
    );
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())?;
    Ok(())
}
