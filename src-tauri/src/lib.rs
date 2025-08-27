mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_project_struture,
            get_staged_files,
            get_file_diff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn get_project_struture(location: String) {
    match utils::git::get_project_struture(location {
        Ok(_) => {},
        Err(_) => {},
    }
}

#[tauri::command]
fn get_staged_files(location: String) {}

#[tauri::command]
fn get_file_diff(location: String, file: String) {}
