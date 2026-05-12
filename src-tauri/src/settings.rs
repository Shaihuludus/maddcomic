use serde::Serialize;
use std::fs;
use tauri::Manager;

#[derive(Serialize)]
pub struct LanguageMeta {
    code: String,
    name: String,
}

#[tauri::command]
pub fn list_languages(app: tauri::AppHandle) -> Result<Vec<LanguageMeta>, String> {
    let locales_dir = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?
        .join("locales");

    let mut languages: Vec<LanguageMeta> = fs::read_dir(&locales_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| {
            let path = e.ok()?.path();
            if path.extension()?.to_str()? != "json" {
                return None;
            }
            let code = path.file_stem()?.to_string_lossy().into_owned();
            let content = fs::read_to_string(&path).ok()?;
            let json: serde_json::Value = serde_json::from_str(&content).ok()?;
            let name = json["_name"].as_str().unwrap_or(&code).to_string();
            Some(LanguageMeta { code, name })
        })
        .collect();

    languages.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(languages)
}

#[tauri::command]
pub fn load_language(app: tauri::AppHandle, code: String) -> Result<serde_json::Value, String> {
    let path = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?
        .join("locales")
        .join(format!("{code}.json"));

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}
