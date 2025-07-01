use anyhow::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use windows_registry::CURRENT_USER;

static THEME_MONITORING_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn start_theme_monitoring(app_handle: AppHandle) -> Result<()> {
    if THEME_MONITORING_ACTIVE.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let app_handle = Arc::new(app_handle);
    thread::spawn(move || {
        let mut last_color = get_system_accent_color().unwrap_or("ff8c00".to_owned());
        while THEME_MONITORING_ACTIVE.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
            if let Ok(current_color) = get_system_accent_color() {
                if current_color != last_color {
                    last_color = current_color.clone();
                    let _ = app_handle.emit("theme-color-changed", &current_color);
                }
            }
        }
        THEME_MONITORING_ACTIVE.store(false, Ordering::SeqCst);
    });
    Ok(())
}

pub fn stop_theme_monitoring() -> Result<()> {
    THEME_MONITORING_ACTIVE.store(false, Ordering::SeqCst);
    Ok(())
}

pub fn get_system_accent_color() -> Result<String> {
    let accent_color = CURRENT_USER
        .open(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent")?
        .get_u32("StartColorMenu")?;
    let accent_color_str = format!("{accent_color:08x}");
    let (b, g, r) = (
        &accent_color_str[2..4],
        &accent_color_str[4..6],
        &accent_color_str[6..8],
    );
    Ok(format!("#{r}{g}{b}"))
}
