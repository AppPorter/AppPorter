use crate::configs::env::Env;
use anyhow::{Result, anyhow};
use mslnk::ShellLink;
use std::path::Path;
use tokio::fs;

pub fn create_desktop_shortcut(shell_link: &ShellLink, app_name: &str) -> Result<()> {
    shell_link.create_lnk(format!(
        r"{}\{}.lnk",
        dirs::desktop_dir()
            .ok_or(anyhow!("Failed to get desktop directory"))?
            .to_string_lossy(),
        app_name
    ))?;
    Ok(())
}

pub async fn create_start_menu_shortcut(
    shell_link: &ShellLink,
    current_user_only: bool,
    app_name: &str,
) -> Result<()> {
    let env = Env::read().await?;
    let lnk_path = if current_user_only {
        format!(
            r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            env.system_drive_letter, env.username, app_name
        )
    } else {
        format!(
            r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            env.system_drive_letter, app_name
        )
    };
    shell_link.create_lnk(lnk_path)?;
    Ok(())
}

pub async fn remove_desktop_shortcut(app_name: &str) -> Result<()> {
    if let Some(desktop_dir) = dirs::desktop_dir() {
        let desktop_shortcut = desktop_dir.join(format!("{app_name}.lnk"));
        if desktop_shortcut.exists() {
            fs::remove_file(desktop_shortcut).await?;
        }
    }
    Ok(())
}

pub async fn remove_start_menu_shortcut(current_user_only: bool, app_name: &str) -> Result<()> {
    let env = Env::read().await?;
    let start_menu_path = if current_user_only {
        format!(
            r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            env.system_drive_letter, env.username, app_name
        )
    } else {
        format!(
            r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            env.system_drive_letter, app_name
        )
    };
    let start_menu_shortcut = Path::new(&start_menu_path);
    if start_menu_shortcut.exists() {
        fs::remove_file(start_menu_shortcut).await?;
    }
    Ok(())
}

pub async fn remove_custom_icon(app_name: &str, id: &str) -> Result<()> {
    let icons_dir = dirs::config_local_dir()
        .ok_or(anyhow!("Failed to get local config directory"))?
        .join("AppPorter")
        .join("icons");

    let icon_filename = format!("{app_name}-{id}.ico");
    let icon_path = icons_dir.join(&icon_filename);

    if icon_path.exists() {
        fs::remove_file(icon_path).await?;
    }
    Ok(())
}
