mod handler;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handler::greeting_api,
            handler::dirreader_api,
            handler::hop_dir_api,
            handler::display_current_dir_api
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
