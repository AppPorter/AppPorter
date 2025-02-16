use crate::configs::ConfigFile;
use crate::configs::{
    app_list::{App, AppList, InstalledApp},
    settings::Settings,
};
use mslnk::ShellLink;
use serde::Deserialize;
use std::{collections::HashSet, error::Error};
use tauri::{AppHandle, Emitter};
use tokio::{fs::File, io::AsyncWriteExt};
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
pub struct InstallationConfig {
    zip_path: String,
    details: InstalledApp,
}

pub async fn installation(
    installation_config: InstallationConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error>> {
    let zip_path = installation_config.zip_path.clone();
    let app_path = format!(
        r"{}\{}",
        installation_config.details.install_path,
        installation_config.details.name.replace(" ", "-")
    );

    // Do synchronous zip operations in a blocking task
    let (file_count, single_root) =
        tokio::task::spawn_blocking(move || -> Result<(usize, Option<String>), String> {
            let file = std::fs::File::open(&zip_path)
                .map_err(|e| format!("Failed to open zip file: {}", e))?;
            let mut archive =
                ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

            let total_files = archive.len();
            let mut root_entries = HashSet::new();

            for i in 0..archive.len() {
                let file = archive
                    .by_index(i)
                    .map_err(|e| format!("Failed to read zip entry: {}", e))?;
                let name = file.name();
                let root = name.split('/').next().unwrap_or("");
                if !root.is_empty() {
                    root_entries.insert(root.to_string());
                }
            }

            let single_root = if root_entries.len() == 1 {
                Some(root_entries.into_iter().next().unwrap())
            } else {
                None
            };

            Ok((total_files, single_root))
        })
        .await
        .map_err(|e| format!("Thread join error: {}", e))??;

    app.emit("installation", 0)?;

    let mut last_progress = -1;
    // Extract files in chunks to avoid holding the ZipArchive for too long
    for i in 0..file_count {
        let zip_path = installation_config.zip_path.clone();
        let app_path = app_path.clone();
        let single_root = single_root.clone();

        let file_data =
            tokio::task::spawn_blocking(move || -> Result<(String, Vec<u8>), String> {
                let file = std::fs::File::open(&zip_path)
                    .map_err(|e| format!("Failed to open zip file: {}", e))?;
                let mut archive = ZipArchive::new(file)
                    .map_err(|e| format!("Failed to read zip archive: {}", e))?;

                let mut zip_file = archive
                    .by_index(i)
                    .map_err(|e| format!("Failed to read zip entry: {}", e))?;

                let name = zip_file.name().to_string();
                let mut buffer = Vec::new();
                std::io::copy(&mut zip_file, &mut buffer)
                    .map_err(|e| format!("Failed to read file content: {}", e))?;

                Ok((name, buffer))
            })
            .await
            .map_err(|e| format!("Thread join error: {}", e))??;

        let (name, buffer) = file_data;
        if name.ends_with('/') {
            continue;
        }

        let outpath = if let Some(ref root) = single_root {
            if name.starts_with(&format!("{}/", root)) {
                let relative_path = name.strip_prefix(&format!("{}/", root)).unwrap();
                std::path::Path::new(&app_path).join(relative_path)
            } else {
                std::path::Path::new(&app_path).join(&name)
            }
        } else {
            std::path::Path::new(&app_path).join(&name)
        };

        if let Some(parent) = outpath.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let mut outfile = File::create(&outpath).await?;
        outfile.write_all(&buffer).await?;

        let progress = ((i as f32 + 1.0) / file_count as f32 * 100.0) as i32;
        if progress != last_progress {
            app.emit("installation_extract", progress)?;
            last_progress = progress;
        }
    }
    app.emit("installation", 101)?;

    let full_executable_path = get_full_executable_path(
        &app_path,
        &installation_config.details.executable_path,
        single_root.as_deref(),
    );

    let settings = Settings::read().await?;

    if installation_config.details.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &settings.system_drive_letter,
            &settings.username,
            &installation_config.details.name,
            &full_executable_path,
            installation_config.details.current_user_only,
        )?;
    }

    if installation_config.details.create_desktop_shortcut {
        create_desktop_shortcut(&full_executable_path, &installation_config.details.name)?;
    }

    if installation_config.details.create_registry_key {
        create_registry_entries(&installation_config, &full_executable_path, &app_path)?;
    }

    // Add new app to app list
    let mut app_list = AppList::read().await?;
    let new_app = App {
        timestamp: chrono::Utc::now().timestamp(),
        installed: true,
        details: installation_config.details,
        url: "".to_string(),
    };
    app_list.links.push(new_app);
    app_list.save().await?;

    Ok(full_executable_path)
}

fn get_full_executable_path(
    app_path: &str,
    executable_path: &str,
    single_root: Option<&str>,
) -> String {
    if let Some(root) = single_root {
        let exe_path = if executable_path.starts_with(&format!("{}/", root)) {
            executable_path
                .strip_prefix(&format!("{}/", root))
                .unwrap_or(executable_path)
                .to_string()
        } else {
            executable_path.to_string()
        };
        format!(r"{}\{}", app_path, exe_path.replace("/", r"\"))
    } else {
        format!(r"{}\{}", app_path, executable_path.replace("/", r"\"))
    }
}

fn create_start_menu_shortcut(
    system_drive: &str,
    username: &str,
    app_name: &str,
    executable_path: &str,
    current_user_only: bool,
) -> Result<(), Box<dyn Error>> {
    let lnk_path = if current_user_only {
        format!(
            r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            system_drive, username, app_name
        )
    } else {
        format!(
            r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            system_drive, app_name
        )
    };
    ShellLink::new(executable_path)?.create_lnk(lnk_path)?;
    Ok(())
}

fn create_desktop_shortcut(executable_path: &str, app_name: &str) -> Result<(), Box<dyn Error>> {
    ShellLink::new(executable_path)?.create_lnk(format!(
        r"{}\{}.lnk",
        dirs::desktop_dir()
            .ok_or("Failed to get desktop directory")?
            .to_string_lossy()
            .to_string(),
        app_name
    ))?;
    Ok(())
}

fn create_registry_entries(
    config: &InstallationConfig,
    executable_path: &str,
    app_path: &str,
) -> Result<(), Box<dyn Error>> {
    let key = if config.details.current_user_only {
        CURRENT_USER.create(format!(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.name
        ))?
    } else {
        LOCAL_MACHINE.create(format!(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.name
        ))?
    };

    key.set_string("Comments", "Installed with AppPorter")?;
    key.set_string("DisplayIcon", executable_path)?;
    key.set_string("DisplayName", &config.details.name)?;
    key.set_string("DisplayVersion", &config.details.version)?;
    key.set_string("InstallLocation", app_path)?;
    key.set_u32("NoModify", 1)?;
    key.set_u32("NoRemove", 0)?;
    key.set_u32("NoRepair", 1)?;
    key.set_string("Publisher", &config.details.publisher)?;
    key.set_string(
        "UninstallString",
        &std::env::current_exe()?.to_string_lossy().to_string(),
    )?;
    Ok(())
}
