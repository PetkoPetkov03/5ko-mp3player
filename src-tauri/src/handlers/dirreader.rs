use std::fs::{self, ReadDir};
use std::env;
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use std::sync::Mutex;


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

fn check_if_empty() -> bool {
    let mut start = START.lock().unwrap().is_none();

    return start;
}

fn pop() -> Result<String, ()> {
    if check_if_empty() {
        return Err(());
    }

    let mut start = START.lock().unwrap();
    let next = start.take().unwrap().next;
    let path = start.take().unwrap().action;

    *start = next;

    return Ok(path);
}

#[tauri::command]
pub fn dir_reader_fn() -> ReadDir {
    let current_dir = current_dir_display().as_path().display().to_string();

    let items_in_dir = fs::read_dir(current_dir).unwrap();
    
    return items_in_dir;
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
pub fn dir_parent() {
    let current_dir = current_dir_display().as_path().display().to_string();
    push(current_dir);
    let parent_dir = current_dir_display().parent().unwrap().display().to_string();

    let _ = env::set_current_dir(parent_dir);
}