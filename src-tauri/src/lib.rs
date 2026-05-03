// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod comcbook;

use comcbook::{load_comic_info, load_comic_page, open_comic_file};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_comic_file,
            load_comic_info,
            load_comic_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
