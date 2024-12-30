use super::super::settings::*;
use mslnk::ShellLink;
use std::{error::Error, fs::File, result::Result};
use zip::read::ZipArchive;

fn installation(zip_path: String, settings: Installation) -> Result<(), Box<dyn Error>> {
    let install_mode: InstallMode;
    let specific_settings = match settings.installation.install_mode {
        InstallMode::AllUsers => {
            install_mode = InstallMode::AllUsers;
            settings.installation.all_users
        }
        InstallMode::CurrentUser => {
            install_mode = InstallMode::CurrentUser;
            settings.installation.current_user
        }
    };

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(&specific_settings.install_path)?;

    if specific_settings.create_registry_key {
        let key: windows_registry::Key;
        match install_mode {
            InstallMode::AllUsers => {
                key = windows_registry::LOCAL_MACHINE.create(format!(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                    r"_"
                ))?;
            }
            InstallMode::CurrentUser => {
                key = windows_registry::CURRENT_USER.create(format!(
                    r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                    r"_"
                ))?;
            }
        };

        if specific_settings.registry.create_comments {
            key.set_string("Comments", "")?;
        }
        if specific_settings.registry.create_display_icon {
            key.set_string("DisplayIcon", "")?;
        }
        if specific_settings.registry.create_display_name {
            key.set_string("DisplayName", "")?;
        }
        if specific_settings.registry.create_display_version {
            key.set_string("DisplayVersion", "")?;
        }
        if specific_settings.registry.create_estimated_size {
            key.set_u32("EstimatedSize", 0)?;
        }
        if specific_settings.registry.create_install_location {
            key.set_string("InstallLocation", "")?;
        }
        if specific_settings.registry.create_no_modify {
            key.set_u32("NoModify", 0)?;
        }
        if specific_settings.registry.create_no_remove {
            key.set_u32("NoRemove", 0)?;
        }
        if specific_settings.registry.create_no_repair {
            key.set_u32("NoRepair", 0)?;
        }
        if specific_settings.registry.create_publisher {
            key.set_string("Publisher", "")?;
        }
        if specific_settings.registry.create_uninstall_string {
            key.set_string("UninstallString", "")?;
        }
    }
    if specific_settings.create_start_menu_shortcut && matches!(install_mode, InstallMode::AllUsers)
    {
        ShellLink::new(&specific_settings.install_path)?.create_lnk(format!(
            r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs",
            settings.system_drive_letter
        ))?;
    }

    if specific_settings.create_start_menu_shortcut
        && matches!(install_mode, InstallMode::CurrentUser)
    {
        ShellLink::new(&specific_settings.install_path)?.create_lnk(format!(
            r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs",
            settings.system_drive_letter, settings.username,
        ))?;
    }

    if specific_settings.create_desktop_shortcut {
        ShellLink::new(specific_settings.install_path)?.create_lnk(
            dirs::desktop_dir()
                .ok_or("Failed to get desktop directory")?
                .to_string_lossy()
                .to_string(),
        )?;
    }

    Ok(())
}
