mod handlers;
fn main() {
    tauri::Builder::default()
        .invoke_handler(
            tauri::generate_handler![
                handlers::greeting_api,
                handlers::dirreader_api,
                handlers::hop_dir_api,
                handlers::display_current_dir_api,
                handlers::parent_dir_api
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
