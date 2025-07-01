use crate::configs::{ConfigFile, library::*};
use crate::utils::convert_base64_to_ico;
use crate::utils::path::{add_to_path, remove_from_path};
use crate::utils::registry::{create_registry_entries, remove_registry_entries};
use crate::utils::shortcuts::{
    create_desktop_shortcut, create_start_menu_shortcut, remove_custom_icon,
    remove_desktop_shortcut, remove_start_menu_shortcut,
};
use anyhow::{Result, anyhow};
use mslnk::ShellLink;
use std::path::Path;

pub async fn modify_app(new_app: App, timestamp: i64) -> Result<()> {
    let mut library = Library::load().await?;
    let old_app = library
        .apps
        .iter()
        .find(|a| a.timestamp == timestamp)
        .ok_or(anyhow!("App not found in library"))?
        .clone();

    if old_app.details.install_path != new_app.details.install_path
        && Path::new(&old_app.details.install_path).exists()
    {
        tokio::fs::create_dir_all(&new_app.details.install_path).await?;
        fs_extra::dir::move_dir(
            &old_app.details.install_path,
            &new_app.details.install_path,
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true),
        )?;
    }

    if old_app.details.config.add_to_path != new_app.details.config.add_to_path {
        if old_app.details.config.add_to_path.0 {
            remove_from_path(
                &old_app.details.config.add_to_path.1,
                old_app.details.current_user_only,
            )?;
        }
        if new_app.details.config.add_to_path.0 {
            add_to_path(
                &new_app.details.config.add_to_path.1,
                new_app.details.current_user_only,
            )?;
        }
    }

    if old_app.details.config.create_desktop_shortcut
        != new_app.details.config.create_desktop_shortcut
    {
        if old_app.details.config.create_desktop_shortcut {
            remove_desktop_shortcut(&old_app.details.info.name).await?;
        }
        if new_app.details.config.create_desktop_shortcut {
            let mut shell_link = ShellLink::new(&new_app.details.full_path)?;
            if new_app.details.config.custom_icon {
                let icon_path = convert_base64_to_ico(
                    &new_app.details.info.icon,
                    &format!("{}-{}", new_app.details.info.name, new_app.timestamp),
                )
                .await?;
                shell_link.set_icon_location(Some(icon_path));
            }
            create_desktop_shortcut(&shell_link, &new_app.details.info.name)?;
        }
    }

    if old_app.details.config.create_start_menu_shortcut
        != new_app.details.config.create_start_menu_shortcut
        || old_app.details.current_user_only != new_app.details.current_user_only
    {
        if old_app.details.config.create_start_menu_shortcut {
            remove_start_menu_shortcut(
                old_app.details.current_user_only,
                &old_app.details.info.name,
            )
            .await?;
        }
        if new_app.details.config.create_start_menu_shortcut {
            let mut shell_link = ShellLink::new(&new_app.details.full_path)?;
            if new_app.details.config.custom_icon {
                let icon_path = convert_base64_to_ico(
                    &new_app.details.info.icon,
                    &format!("{}-{}", new_app.details.info.name, new_app.timestamp),
                )
                .await?;
                shell_link.set_icon_location(Some(icon_path));
            }
            create_start_menu_shortcut(
                &shell_link,
                new_app.details.current_user_only,
                &new_app.details.info.name,
            )
            .await?;
        }
    }

    if old_app.details.config.custom_icon != new_app.details.config.custom_icon
        || old_app.details.info.icon != new_app.details.info.icon
    {
        if old_app.details.config.custom_icon {
            remove_custom_icon(&old_app.details.info.name, old_app.timestamp).await?;
        }
        if new_app.details.config.custom_icon {
            let _ = convert_base64_to_ico(
                &new_app.details.info.icon,
                &format!("{}-{}", new_app.details.info.name, new_app.timestamp),
            )
            .await?;
        }
    }

    if old_app.details.config.create_registry_key != new_app.details.config.create_registry_key {
        if old_app.details.config.create_registry_key {
            remove_registry_entries(
                &old_app.details.info.name,
                old_app.details.current_user_only,
            )?;
        }
        if new_app.details.config.create_registry_key {
            create_registry_entries(&new_app)?;
        }
    }

    if let Some(app) = library.apps.iter_mut().find(|a| a.timestamp == timestamp) {
        *app = new_app;
    }
    library.save().await?;
    Ok(())
}
