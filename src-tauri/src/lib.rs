// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod comcbook;
mod library;
mod settings;

use comcbook::{load_comic_info, load_comic_page, open_comic_file};
use library::{list_comics_in_folder, load_folders, open_folder_dialog, save_folders};
use settings::{list_languages, load_language};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_comic_file,
            load_comic_info,
            load_comic_page,
            open_folder_dialog,
            load_folders,
            save_folders,
            list_comics_in_folder,
            list_languages,
            load_language
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
