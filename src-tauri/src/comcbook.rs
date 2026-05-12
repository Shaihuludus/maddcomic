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
    match comic_extension(path) {
        "cbz" => load_cbz_info(path),
        "cbr" => load_cbr_info(path),
        _ => Err("Unsupported comic format. Please select a .cbz or .cbr file.".to_string()),
    }
}

#[tauri::command]
pub fn load_comic_page(comic_path: String, page_index: usize) -> Result<ComicPage, String> {
    let path = Path::new(&comic_path);
    match comic_extension(path) {
        "cbz" => load_cbz_page(path, page_index),
        "cbr" => load_cbr_page(path, page_index),
        _ => Err("Unsupported comic format. Please select a .cbz or .cbr file.".to_string()),
    }
}

fn comic_extension(path: &Path) -> &str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .as_deref()
    {
        Some("cbz") => "cbz",
        Some("cbr") => "cbr",
        _ => "",
    }
}

// ── CBZ ──────────────────────────────────────────────────────────────────────

fn open_cbz_images(path: &Path) -> Result<(ZipArchive<File>, Vec<(String, usize)>), String> {
    let file = File::open(path).map_err(|e| format!("Failed to open archive: {e}"))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Failed to read CBZ archive: {e}"))?;

    let mut entries: Vec<(String, usize)> = (0..archive.len())
        .filter_map(|i| {
            let entry = archive.by_index(i).ok()?;
            if entry.is_file() && is_image_file(entry.name()) {
                Some((entry.name().to_string(), i))
            } else {
                None
            }
        })
        .collect();

    if entries.is_empty() {
        return Err("No image pages found in selected comic.".to_string());
    }

    entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    Ok((archive, entries))
}

fn load_cbz_info(path: &Path) -> Result<ComicInfo, String> {
    let (mut archive, entries) = open_cbz_images(path)?;
    let (name, index) = &entries[0];
    let data_url = read_cbz_entry_as_data_url(&mut archive, *index, name)?;
    Ok(ComicInfo {
        file_count: entries.len(),
        first_page_data_url: data_url,
        first_page_name: name.clone(),
    })
}

fn load_cbz_page(path: &Path, page_index: usize) -> Result<ComicPage, String> {
    let (mut archive, entries) = open_cbz_images(path)?;
    if page_index >= entries.len() {
        return Err(format!(
            "Page index out of range. Requested {page_index}, but comic has {} pages.",
            entries.len()
        ));
    }
    let (name, index) = &entries[page_index];
    let data_url = read_cbz_entry_as_data_url(&mut archive, *index, name)?;
    Ok(ComicPage {
        page_data_url: data_url,
        page_name: name.clone(),
    })
}

fn read_cbz_entry_as_data_url(archive: &mut ZipArchive<File>, index: usize, name: &str) -> Result<String, String> {
    let mut entry = archive.by_index(index).map_err(|e| format!("Failed to read entry: {e}"))?;
    let mut bytes = Vec::new();
    entry.read_to_end(&mut bytes).map_err(|e| format!("Failed to decode entry: {e}"))?;
    Ok(bytes_to_data_url(&bytes, name))
}

// ── CBR ──────────────────────────────────────────────────────────────────────

fn collect_cbr_images(path: &Path) -> Result<Vec<String>, String> {
    let archive = unrar::Archive::new(path)
        .open_for_listing()
        .map_err(|e| e.to_string())?;

    let mut names: Vec<String> = archive
        .filter_map(|e| e.ok())
        .filter(|e| is_image_file(&e.filename.to_string_lossy()))
        .map(|e| e.filename.to_string_lossy().into_owned())
        .collect();

    if names.is_empty() {
        return Err("No image pages found in selected comic.".to_string());
    }

    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    Ok(names)
}

fn read_cbr_entry(path: &Path, target: &str) -> Result<Vec<u8>, String> {
    let mut archive = unrar::Archive::new(path)
        .open_for_processing()
        .map_err(|e| e.to_string())?;

    loop {
        match archive.read_header().map_err(|e| e.to_string())? {
            None => break,
            Some(header) => {
                let name = header.entry().filename.to_string_lossy().into_owned();
                archive = if name == target {
                    let (data, _) = header.read().map_err(|e| e.to_string())?;
                    return Ok(data);
                } else {
                    header.skip().map_err(|e| e.to_string())?
                };
            }
        }
    }

    Err(format!("Entry '{target}' not found in archive"))
}

fn load_cbr_info(path: &Path) -> Result<ComicInfo, String> {
    let names = collect_cbr_images(path)?;
    let data = read_cbr_entry(path, &names[0])?;
    Ok(ComicInfo {
        file_count: names.len(),
        first_page_data_url: bytes_to_data_url(&data, &names[0]),
        first_page_name: names[0].clone(),
    })
}

fn load_cbr_page(path: &Path, page_index: usize) -> Result<ComicPage, String> {
    let names = collect_cbr_images(path)?;
    if page_index >= names.len() {
        return Err(format!(
            "Page index out of range. Requested {page_index}, but comic has {} pages.",
            names.len()
        ));
    }
    let name = &names[page_index];
    let data = read_cbr_entry(path, name)?;
    Ok(ComicPage {
        page_data_url: bytes_to_data_url(&data, name),
        page_name: name.clone(),
    })
}

// ── Shared helpers ────────────────────────────────────────────────────────────

fn bytes_to_data_url(bytes: &[u8], name: &str) -> String {
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    format!("data:{};base64,{encoded}", mime_type_from_name(name))
}

fn is_image_file(name: &str) -> bool {
    matches!(
        name.rsplit('.').next().map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "avif")
    )
}

fn mime_type_from_name(name: &str) -> &'static str {
    match name.rsplit('.').next().map(|ext| ext.to_ascii_lowercase()).as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("bmp") => "image/bmp",
        Some("avif") => "image/avif",
        _ => "application/octet-stream",
    }
}
