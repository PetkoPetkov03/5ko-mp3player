use super::dirreadertypes;

use std::fs::{self};
use std::env;
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone)]
struct Actions {
    action: String,
    next: Option<Box<Actions>>,
}

lazy_static! {
    static ref START: Mutex<Option<Box<Actions>>> = Mutex::new(None);
}

fn push(action: String) {
    let mut start = START.lock().unwrap();
    let temp = start.take();

    let new_action = Actions { action: action, next: temp };

    *start = Some(Box::new(new_action));
}

fn pop() -> Result<String, ()> {
    let mut start = START.lock().unwrap();

    if let Some(ref node) = *start {
        let path = node.action.clone();
        *start = node.next.clone();
        Ok(path)
    } else {
        Err(())
    }
}



#[tauri::command]
pub fn dir_reader_fn() -> Vec<dirreadertypes::FileStruct> {
    let current_dir = current_dir_display().as_path().display().to_string();

    let items_in_dir = fs::read_dir(current_dir).unwrap();
    
    let mut vector_paths: Vec<dirreadertypes::FileStruct> = Vec::new();

    for path in items_in_dir {
        let pathcheck = &path.unwrap();
        if pathcheck.file_type().unwrap().is_dir() {
            let folder = dirreadertypes::FileStruct { 
                file_path: pathcheck.path().display().to_string(),
                file_type: dirreadertypes::FileType::FOLDER
            };
            
            vector_paths.push(folder);
        }else if pathcheck.file_type().unwrap().is_file() {
            if let Some(checkedpath) = pathcheck.path().extension() {
                if checkedpath == "mp3" {
                    let file = dirreadertypes::FileStruct {
                        file_path: pathcheck.path().display().to_string(),
                        file_type: dirreadertypes::FileType::MP3
                    };

                    vector_paths.push(file);
                }
            }
        }
    }

    return vector_paths;
}

#[tauri::command]
pub fn dir_hopper(dir: String) -> Result<(), ()>{
    let string_to_path = Path::new(&dir);
    let current_dir_result = env::set_current_dir(&string_to_path);


    
    if current_dir_result.is_err() {
        return Err(())
    }else {
        return Ok(());
    }
}

#[tauri::command]
pub fn current_dir_display() -> PathBuf {
    let current_dir = env::current_dir().unwrap();

    return current_dir;
}

#[tauri::command]
pub fn dir_parent() -> Result<(), ()>{
    let current_dir = current_dir_display().as_path().display().to_string();
    push(current_dir);
    if current_dir_display().parent().is_none() {
        return Err(());
    }
    let parent_dir = current_dir_display().parent().unwrap().display().to_string();

    let _ = env::set_current_dir(parent_dir);

    return Ok(());
}

#[tauri::command]
pub fn dir_previous_action() -> Result<(), ()> {
    let poped = pop();

    if poped.is_err() {
        return Err(());
    }

    let _ = env::set_current_dir(poped.unwrap());
    return Ok(());
}