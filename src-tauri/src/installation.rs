use base64::{engine::general_purpose::STANDARD, Engine};
use mslnk::ShellLink;
use serde::Deserialize;
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::process::Command;
use systemicons::get_icon;
use tauri::{AppHandle, Emitter};
use tempfile::Builder;
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
pub struct ExePath {
    pub zip_path: String,
    pub executable_path: String,
}

pub fn get_details(req: ExePath, app: AppHandle) -> Result<String, Box<dyn Error>> {
    let timer = std::time::Instant::now();
    // Create temp directory
    let temp_dir = Builder::new().prefix("appporter").tempdir()?;
    let temp_exe_path = temp_dir.path().join(&req.executable_path);
    // Read zip file
    let file = File::open(&req.zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    // Extract only the target executable
    if let Ok(mut exe_file) = archive.by_name(&req.executable_path) {
        // Create parent directories if they don't exist
        if let Some(parent) = temp_exe_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Extract the exe file
        let mut outfile = File::create(&temp_exe_path)?;
        app.emit("get_details", 1)?;
        std::io::copy(&mut exe_file, &mut outfile)?;
        app.emit("get_details", 2)?;
    } else {
        return Err(format!(
            "Failed to find executable '{}' in archive",
            req.executable_path
        )
        .into());
    }
    let raw_icon = get_icon(&temp_exe_path.to_string_lossy(), 64).unwrap_or_default();
    let icon_base64 = STANDARD.encode(&raw_icon);
    let icon_data_url = format!("data:image/png;base64,{}", icon_base64);
    // Prepare PowerShell command
    let ps_command = format!(
        r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8;
        $ErrorActionPreference = 'Stop';
        try {{
            $file_path = '{}';
            $version_info = (Get-Item $file_path).VersionInfo;
            
            $product_name = if ($version_info.ProductName -and $version_info.ProductName.Trim()) {{ 
                $version_info.ProductName.Trim() 
            }} else {{ 
                [System.IO.Path]::GetFileNameWithoutExtension($file_path) 
            }};
            
            $product_version = if ($version_info.ProductVersion -and $version_info.ProductVersion.Trim()) {{ 
                $version_info.ProductVersion.Trim() 
            }} else {{ 
            }};
            
            $copyright = if ($version_info.LegalCopyright -and $version_info.LegalCopyright.Trim()) {{ 
                $version_info.LegalCopyright.Trim()
            }} else {{ 
                '' 
            }};
            
            Write-Output (ConvertTo-Json -Compress @{{
                product_name = $product_name;
                product_version = $product_version;
                copyright = $copyright;
            }})
        }} catch {{
            Write-Output (ConvertTo-Json -Compress @{{
                product_name = [System.IO.Path]::GetFileNameWithoutExtension($file_path);
                product_version = '1.0.0';
                copyright = '';
                error = $_.Exception.Message;
            }})
        }}"#,
        temp_exe_path.to_string_lossy().replace("'", "''")
    );
    app.emit("get_details", 3)?;
    // Execute PowerShell command
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &ps_command,
        ])
        .output()?;
    app.emit("get_details", 4)?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let details: Value = serde_json::from_str(&output_str)?;

    let response = json!([
        details["product_name"]
            .as_str()
            .unwrap_or("Unknown Application"),
        details["product_version"].as_str().unwrap_or("1.0.0"),
        details["copyright"].as_str().unwrap_or(""),
        icon_data_url
    ]);
    Ok(response.to_string())
}

#[derive(Deserialize, Debug)]
pub struct InstallationConfig {
    zip_path: String,

    current_user_only: bool,
    create_desktop_shortcut: bool,
    create_registry_key: bool,
    create_start_menu_shortcut: bool,
    install_path: String,

    executable_path: String,

    app_icon: String,
    app_name: String,
    app_publisher: String,
    app_version: String,
}

pub fn installation(
    installation_config: InstallationConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error>> {
    let file = File::open(installation_config.zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let app_path = format!(
        r"{}\{}",
        installation_config.install_path,
        installation_config.app_name.replace(" ", "-")
    );

    // Check if the zip has only one root directory
    let mut root_entries = std::collections::HashSet::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name();
        let root = name.split('/').next().unwrap_or("");
        if !root.is_empty() {
            root_entries.insert(root.to_string());
        }
    }

    // If there's exactly one root directory, we'll extract contents directly
    let single_root = if root_entries.len() == 1 {
        Some(root_entries.into_iter().next().unwrap())
    } else {
        None
    };

    // Extract files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name();

        // Skip if it's a directory entry
        if name.ends_with('/') {
            continue;
        }

        let outpath = if let Some(root) = &single_root {
            // Remove the root directory from the path
            if name.starts_with(&format!("{}/", root)) {
                let relative_path = name.strip_prefix(&format!("{}/", root)).unwrap();
                std::path::Path::new(&app_path).join(relative_path)
            } else {
                std::path::Path::new(&app_path).join(name)
            }
        } else {
            std::path::Path::new(&app_path).join(name)
        };

        if let Some(parent) = outpath.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut outfile = std::fs::File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;
    }

    let full_executable_path = if let Some(root) = &single_root {
        // If the executable path starts with the root directory, remove it
        let exe_path = if installation_config
            .executable_path
            .starts_with(&format!("{}/", root))
        {
            installation_config
                .executable_path
                .strip_prefix(&format!("{}/", root))
                .unwrap_or(&installation_config.executable_path)
                .to_string()
        } else {
            installation_config.executable_path.clone()
        };
        format!(r"{}\{}", app_path, exe_path.replace("/", r"\"))
    } else {
        format!(
            r"{}\{}",
            app_path,
            installation_config.executable_path.replace("/", r"\")
        )
    };

    let settings = crate::settings::Settings::read()?;
    if installation_config.create_start_menu_shortcut {
        let lnk_path = if installation_config.current_user_only {
            format!(
                r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
                settings.system_drive_letter, settings.username, installation_config.app_name
            )
        } else {
            format!(
                r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
                settings.system_drive_letter, installation_config.app_name
            )
        };
        ShellLink::new(&full_executable_path)?.create_lnk(lnk_path)?;
    }

    if installation_config.create_desktop_shortcut {
        ShellLink::new(&full_executable_path)?.create_lnk(format!(
            r"{}\{}.lnk",
            dirs::desktop_dir()
                .ok_or("Failed to get desktop directory")?
                .to_string_lossy()
                .to_string(),
            installation_config.app_name
        ))?;
    }
    if installation_config.create_registry_key {
        let key: windows_registry::Key;
        if installation_config.current_user_only {
            key = windows_registry::CURRENT_USER.create(format!(
                r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                installation_config.app_name
            ))?;
        } else {
            key = windows_registry::LOCAL_MACHINE.create(format!(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                installation_config.app_name
            ))?;
        }

        key.set_string("Comments", "Installed with AppPorter")?;
        key.set_string("DisplayIcon", &full_executable_path)?;
        key.set_string("DisplayName", &installation_config.app_name)?;
        key.set_string("DisplayVersion", &installation_config.app_version)?;
        key.set_string("InstallLocation", &app_path)?;
        key.set_u32("NoModify", 1)?;
        key.set_u32("NoRemove", 0)?;
        key.set_u32("NoRepair", 1)?;
        key.set_string("Publisher", &installation_config.app_publisher)?;
        key.set_string(
            "UninstallString",
            &std::env::current_exe()?.to_string_lossy().to_string(),
        )?;
    }
    Ok("".to_owned())
}
