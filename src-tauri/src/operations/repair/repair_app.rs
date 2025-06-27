use crate::{configs::Library, utils::*};
use anyhow::{Result, anyhow};
use mslnk::ShellLink;

pub async fn repair_app(timestamp: i64) -> Result<()> {
    let library = Library::load().await?;
    let config = library
        .apps
        .iter()
        .find(|app| app.timestamp == timestamp)
        .ok_or(anyhow!("App not found in library"))?;

    if config.details.config.create_desktop_shortcut {
        remove_desktop_shortcut(&config.details.info.name).await?;
    }

    if config.details.config.create_start_menu_shortcut {
        remove_start_menu_shortcut(
            config.details.config.current_user_only,
            &config.details.info.name,
        )
        .await?;
    }

    if config.details.config.custom_icon {
        remove_custom_icon(&config.details.info.name, timestamp).await?;
    }

    if config.details.config.create_registry_key {
        remove_registry_entries(
            &config.details.info.name,
            config.details.config.current_user_only,
        )?;
    }

    if config.details.config.add_to_path {
        remove_from_path(
            &config.details.config.full_path_directory,
            config.details.config.current_user_only,
        )?;
    }

    let mut shell_link = ShellLink::new(&config.details.paths.full_path)?;
    if config.details.config.custom_icon {
        let icon_path = convert_base64_to_ico(
            &config.details.info.icon,
            &format!("{}-{}", config.details.info.name, timestamp),
        )
        .await?;
        shell_link.set_icon_location(Some(icon_path));
    }

    if config.details.config.create_desktop_shortcut {
        create_desktop_shortcut(&shell_link, &config.details.info.name)?;
    }

    if config.details.config.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &shell_link,
            config.details.config.current_user_only,
            &config.details.info.name,
        )
        .await?;
    }

    if config.details.config.create_registry_key {
        create_registry_entries(config)?;
    }

    if config.details.config.add_to_path {
        add_to_path(
            &config.details.config.full_path_directory,
            config.details.config.current_user_only,
        )?;
    }

    Ok(())
}
