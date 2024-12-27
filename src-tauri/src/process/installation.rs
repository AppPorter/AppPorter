use mslnk::ShellLink; //https://crates.io/crates/mslnk
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::Path, result::Result};
use zip::read::ZipArchive; //https://crates.io/crates/zip
use zip_extensions::read::ZipArchiveExtensions; //https://crates.io/crates/zip-extensions

fn installation() -> Result<(), Box<dyn Error>> {
    let file = File::open(r"")?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(r"")?;

    let key_path = format!(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
        r""
    );
    let key = windows_registry::LOCAL_MACHINE.create(key_path)?;
    key.set_string("Comments", "")?;
    key.set_string("DisplayIcon", "")?;
    key.set_string("DisplayName", "")?;
    key.set_string("DisplayVersion", "")?;
    key.set_u32("EstimatedSize", 0)?;
    key.set_string("InstallLocation", "")?;
    key.set_u32("NoModify", 0)?;
    key.set_u32("NoRemove", 0)?;
    key.set_u32("NoRepair", 0)?;
    key.set_string("Publisher", "")?;
    key.set_string("UninstallString", "")?;

    ShellLink::new(r"")?.create_lnk(r"")?;
    ShellLink::new(r"")?.create_lnk(r"")?;

    Ok(())
}
