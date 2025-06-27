use anyhow::Result;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

pub fn add_to_path(path_directory: &str, current_user_only: bool) -> Result<()> {
    let (key, path_key) = if current_user_only {
        (CURRENT_USER.create("Environment")?, "Path")
    } else {
        (
            LOCAL_MACHINE
                .create(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")?,
            "path",
        )
    };

    let current_path = key.get_string(path_key)?;

    if !current_path
        .split(';')
        .any(|p| p.trim() == path_directory.trim())
    {
        let new_path = format!("{current_path};{path_directory}");
        key.set_expand_string(path_key, new_path)?;
    }

    Ok(())
}

pub fn remove_from_path(path_to_remove: &str, current_user_only: bool) -> Result<()> {
    if current_user_only {
        let key = CURRENT_USER.create("Environment")?;
        if let Ok(current_path) = key.get_string("Path") {
            let new_path = current_path
                .split(';')
                .filter(|p| p.trim() != path_to_remove.trim())
                .collect::<Vec<&str>>()
                .join(";");

            key.set_expand_string("Path", new_path)?;
        }
    } else {
        let key = LOCAL_MACHINE
            .create(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")?;
        if let Ok(current_path) = key.get_string("path") {
            let new_path = current_path
                .split(';')
                .filter(|p| p.trim() != path_to_remove.trim())
                .collect::<Vec<&str>>()
                .join(";");

            key.set_expand_string("path", new_path)?;
        }
    }
    Ok(())
}
