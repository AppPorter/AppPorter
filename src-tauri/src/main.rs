#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows",
    target_os = "windows"
)]

fn main() {
    app_porter_lib::run()
}
