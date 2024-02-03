#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod response;
mod storage;
mod rpa;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            if !app::init_app_dir() {
                panic!("Failed to initialize app directory");
            }
            storage::check();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            rpa::connector::get_connector
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
