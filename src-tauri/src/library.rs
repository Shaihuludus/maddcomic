use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderEntry {
    pub path: String,
}

fn folders_file_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("folders.json"))
}

#[derive(Serialize)]
pub struct ComicFile {
    pub path: String,
    pub name: String,
}

#[tauri::command]
pub fn list_comics_in_folder(folder_path: String) -> Result<Vec<ComicFile>, String> {
    let entries = fs::read_dir(&folder_path).map_err(|e| e.to_string())?;
    let mut comics: Vec<ComicFile> = entries
        .filter_map(|e| {
            let e = e.ok()?;
            let path = e.path();
            if !path.is_file() { return None; }
            let ext = path.extension()?.to_str()?.to_ascii_lowercase();
            if !matches!(ext.as_str(), "cbz" | "cbr") { return None; }
            Some(ComicFile {
                path: path.to_string_lossy().into_owned(),
                name: e.file_name().to_string_lossy().into_owned(),
            })
        })
        .collect();
    comics.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(comics)
}

#[tauri::command]
pub fn open_folder_dialog() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn load_folders(app: tauri::AppHandle) -> Result<Vec<FolderEntry>, String> {
    let path = folders_file_path(&app)?;
    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e.to_string()),
    };
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_folders(app: tauri::AppHandle, folders: Vec<FolderEntry>) -> Result<(), String> {
    let path = folders_file_path(&app)?;
    let content = serde_json::to_string_pretty(&folders).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}
