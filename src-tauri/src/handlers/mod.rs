mod greeting;
mod dirreader;
mod dirreadertypes;


#[tauri::command]
pub fn greeting_api() -> &'static str {
    return greeting::greeting_fn();
}

#[tauri::command]
pub fn dirreader_api() -> Vec<dirreadertypes::FileStruct> {
    return dirreader::dir_reader_fn();
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

#[tauri::command]
pub fn previous_action_api() {
    let _  = dirreader::dir_previous_action();
}