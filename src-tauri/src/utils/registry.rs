use crate::configs::App;
use anyhow::Result;
use std::env;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

pub fn create_registry_entries(config: &App) -> Result<()> {
    let key = if config.details.current_user_only {
        CURRENT_USER.create(format!(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.info.name
        ))?
    } else {
        LOCAL_MACHINE.create(format!(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.info.name
        ))?
    };

    key.set_string("Comments", "Installed with AppPorter")?;
    key.set_string("DisplayIcon", &config.details.full_path)?;
    key.set_string("DisplayName", &config.details.info.name)?;
    key.set_string("DisplayVersion", &config.details.info.version)?;
    key.set_string("InstallLocation", &config.details.install_path)?;
    key.set_u32("NoModify", 1)?;
    key.set_u32("NoRemove", 0)?;
    key.set_u32("NoRepair", 1)?;
    key.set_string("Publisher", &config.details.info.publisher)?;
    key.set_string(
        "UninstallString",
        format!(
            "\"{}\" uninstall {}",
            env::current_exe()?.to_string_lossy(),
            config.id
        ),
    )?;
    Ok(())
}

pub fn remove_registry_entries(app_name: &str, current_user_only: bool) -> Result<()> {
    let key;

    if current_user_only {
        key = format!(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{app_name}");
        if CURRENT_USER.open(&key).is_ok() {
            CURRENT_USER.remove_tree(&key)?;
        }
    } else {
        key = format!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{app_name}");
        if LOCAL_MACHINE.open(&key).is_ok() {
            LOCAL_MACHINE.remove_tree(&key)?;
        }
    }
    Ok(())
}
