mod greeting;
mod dirreader;

#[tauri::command]
pub fn greeting_api() -> &'static str {
    return greeting::greeting_fn();
}

#[tauri::command]
pub fn dirreader_api(dir: String) -> Vec<String> {
    let paths = dirreader::dir_reader_fn(dir);

    let mut path_array= Vec::new();

    for path in paths {
        let cloned_path = path.unwrap().path().display().to_string().clone();
        path_array.push(cloned_path);
    }

    return path_array;
}

#[tauri::command]
pub fn hop_dir_api(dir: String) -> Result<(), ()> {
    return dirreader::dir_hopper(dir);
}

#[tauri::command]
pub fn display_current_dir_api() -> String {
    return dirreader::current_dir_display();
}