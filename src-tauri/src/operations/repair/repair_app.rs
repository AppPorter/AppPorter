use crate::{configs::Library, utils::*};
use anyhow::{Result, anyhow};
use mslnk::ShellLink;

pub async fn repair_app(id: &str) -> Result<()> {
    let library = Library::load().await?;
    let config = library
        .get_app(id)
        .await
        .ok_or(anyhow!("App with ID {} not found", id))?;

    if config.details.config.create_desktop_shortcut {
        remove_desktop_shortcut(&config.details.info.name).await?;
    }

    if config.details.config.create_start_menu_shortcut {
        remove_start_menu_shortcut(config.details.current_user_only, &config.details.info.name)
            .await?;
    }

    if config.details.config.custom_icon {
        remove_custom_icon(&config.details.info.name, id).await?;
    }

    if config.details.config.create_registry_key {
        remove_registry_entries(&config.details.info.name, config.details.current_user_only)?;
    }

    if config.details.config.add_to_path.0 {
        remove_from_path(
            &config.details.config.add_to_path.1,
            config.details.current_user_only,
        )?;
    }

    let mut shell_link = ShellLink::new(&config.details.full_path)?;
    if config.details.config.custom_icon {
        let icon_path = convert_base64_to_ico(
            &config.details.info.icon,
            &format!("{}-{}", config.details.info.name, id),
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
            config.details.current_user_only,
            &config.details.info.name,
        )
        .await?;
    }

    if config.details.config.create_registry_key {
        create_registry_entries(&config)?;
    }

    if config.details.config.add_to_path.0 {
        add_to_path(
            &config.details.config.add_to_path.1,
            config.details.current_user_only,
        )?;
    }

    Ok(())
}
