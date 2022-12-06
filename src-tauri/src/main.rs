#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod setup;
mod cmd;
mod clipboard;
mod event;
mod storage;
mod util;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::create_tag, 
            cmd::get_all_record,
            cmd::get_all_tags,
            cmd::delete_record,
            cmd::create_tag,
            cmd::delete_tag,
            cmd::pin_record,
            cmd::paste,
        ])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
