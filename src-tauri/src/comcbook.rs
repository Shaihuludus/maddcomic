use base64::Engine;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

#[derive(Serialize)]
pub struct ComicInfo {
    file_count: usize,
    first_page_data_url: String,
    first_page_name: String,
}

#[derive(Serialize)]
pub struct ComicPage {
    page_data_url: String,
    page_name: String,
}

#[tauri::command]
pub fn open_comic_file() -> Option<String> {
    rfd::FileDialog::new()
        .add_filter("Comic archives", &["cbz", "cbr"])
        .pick_file()
        .map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn load_comic_info(comic_path: String) -> Result<ComicInfo, String> {
    let path = Path::new(&comic_path);

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "cbz" => load_cbz_info(path),
        "cbr" => Err("CBR preview is not supported yet. Please use CBZ for preview.".to_string()),
        _ => Err("Unsupported comic format. Please select a .cbz or .cbr file.".to_string()),
    }
}

#[tauri::command]
pub fn load_comic_page(comic_path: String, page_index: usize) -> Result<ComicPage, String> {
    let path = Path::new(&comic_path);

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "cbz" => load_cbz_page(path, page_index),
        "cbr" => Err("CBR preview is not supported yet. Please use CBZ for preview.".to_string()),
        _ => Err("Unsupported comic format. Please select a .cbz or .cbr file.".to_string()),
    }
}

fn load_cbz_info(path: &Path) -> Result<ComicInfo, String> {
    let file = File::open(path).map_err(|error| format!("Failed to open archive: {error}"))?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("Failed to read CBZ archive: {error}"))?;

    let mut image_entries: Vec<(String, usize)> = Vec::new();

    for index in 0..archive.len() {
        let entry = archive
            .by_index(index)
            .map_err(|error| format!("Failed to read archive entry: {error}"))?;

        if entry.is_file() && is_image_file(entry.name()) {
            image_entries.push((entry.name().to_string(), index));
        }
    }

    if image_entries.is_empty() {
        return Err("No image pages found in selected comic.".to_string());
    }

    image_entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    let (first_page_name, first_page_index) = image_entries[0].clone();
    let mut first_page = archive
        .by_index(first_page_index)
        .map_err(|error| format!("Failed to read first page: {error}"))?;

    let mut bytes = Vec::new();
    first_page
        .read_to_end(&mut bytes)
        .map_err(|error| format!("Failed to decode first page: {error}"))?;

    let mime_type = mime_type_from_name(&first_page_name);
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);

    Ok(ComicInfo {
        file_count: image_entries.len(),
        first_page_data_url: format!("data:{mime_type};base64,{encoded}"),
        first_page_name,
    })
}

fn load_cbz_page(path: &Path, page_index: usize) -> Result<ComicPage, String> {
    let file = File::open(path).map_err(|error| format!("Failed to open archive: {error}"))?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("Failed to read CBZ archive: {error}"))?;

    let mut image_entries: Vec<(String, usize)> = Vec::new();

    for index in 0..archive.len() {
        let entry = archive
            .by_index(index)
            .map_err(|error| format!("Failed to read archive entry: {error}"))?;

        if entry.is_file() && is_image_file(entry.name()) {
            image_entries.push((entry.name().to_string(), index));
        }
    }

    if image_entries.is_empty() {
        return Err("No image pages found in selected comic.".to_string());
    }

    image_entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    if page_index >= image_entries.len() {
        return Err(format!(
            "Page index out of range. Requested {page_index}, but comic has {} pages.",
            image_entries.len()
        ));
    }

    let (page_name, archive_index) = image_entries[page_index].clone();
    let mut page = archive
        .by_index(archive_index)
        .map_err(|error| format!("Failed to read page: {error}"))?;

    let mut bytes = Vec::new();
    page.read_to_end(&mut bytes)
        .map_err(|error| format!("Failed to decode page: {error}"))?;

    let mime_type = mime_type_from_name(&page_name);
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);

    Ok(ComicPage {
        page_data_url: format!("data:{mime_type};base64,{encoded}"),
        page_name,
    })
}

fn is_image_file(name: &str) -> bool {
    matches!(
        name.rsplit('.').next().map(|ext| ext.to_ascii_lowercase()),
        Some(ext)
            if matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "avif"
            )
    )
}

fn mime_type_from_name(name: &str) -> &'static str {
    match name
        .rsplit('.')
        .next()
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("bmp") => "image/bmp",
        Some("avif") => "image/avif",
        _ => "application/octet-stream",
    }
}