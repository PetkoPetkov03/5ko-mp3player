mod greeting;
mod dirreader;

#[tauri::command]
pub fn greeting_api() -> &'static str {
    return greeting::greeting_fn();
}

#[tauri::command]
pub fn dirreader_api() -> Vec<String> {
    let dir_read = dirreader::dir_reader_fn();
    let mut vector_paths: Vec<String> = Vec::new();

    for path in dir_read {
        vector_paths.push(path.unwrap().path().display().to_string());
    }

    return vector_paths;
}

#[tauri::command]
pub fn hop_dir_api(dir: String) -> Result<(), ()> {
    return dirreader::dir_hopper(dir);
}

#[tauri::command]
pub fn display_current_dir_api() -> String {
    return dirreader::current_dir_display().as_path().display().to_string();
}

#[tauri::command]
pub fn parent_dir_api() {
    let _ = dirreader::dir_parent();
}