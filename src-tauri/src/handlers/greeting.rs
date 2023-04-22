#[tauri::command]
pub fn greeting_fn() -> &'static str {
    return "Welcome to 5ko-mp3player!";
}