use std::fs::{self, ReadDir};
use std::env;
use std::path::Path;

#[tauri::command]
pub fn dir_reader_fn(dir: String) -> ReadDir {
    let paths = fs::read_dir(dir).unwrap();

    return paths;
}

#[tauri::command]
pub fn dir_hopper(dir: String) -> Result<(), ()>{
    let string_to_path = Path::new(&dir);
    let current_dir_result = env::set_current_dir(&string_to_path);
    
    if current_dir_result.is_err() {
        println!("err");
        panic!("{:?}", current_dir_result.err());
    }else {
        println!("ok");
        return Ok(());
    }
}

#[tauri::command]
pub fn current_dir_display() -> String {
    let current_dir = env::current_dir().unwrap().as_path().display().to_string();

    return current_dir;
}