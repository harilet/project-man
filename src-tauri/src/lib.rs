use ollama_rs::generation::chat::ChatMessage;
use tauri::{AppHandle, Emitter};

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
            get_all_local_models,
            send_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn get_project_struture(app: AppHandle, location: String) -> Vec<String> {
    match utils::git::get_project_struture(location) {
        Ok(struture) => struture,
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            vec![]
        }
    }
}

#[tauri::command]
fn get_staged_files(app: AppHandle, location: String) -> Vec<String> {
    match utils::git::get_staged_files(location) {
        Ok(files) => files,
        Err(e) => {
            println!("{:#?}", e);
            app.emit("app-error", e.to_string()).unwrap();
            vec![]
        }
    }
}

#[tauri::command]
fn get_file_diff(app: AppHandle, location: String, file: String) -> String {
    match utils::git::get_file_diff(location, file) {
        Ok(file_diff) => file_diff,
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            "".to_string()
        }
    }
}

#[tauri::command]
async fn get_all_local_models(app: AppHandle) -> Vec<String> {
    match utils::l_ollama::get_all_local_models().await {
        Ok(file_diff) => file_diff,
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            vec![]
        }
    }
}

#[tauri::command]
async fn send_message(
    app: AppHandle,
    model: String,
    messages: Vec<String>,
    history: Vec<ChatMessage>,
) -> String {
    let mut t_messages = vec![];
    for message in messages {
        t_messages.push(ChatMessage::user(message));
    }
    let result = utils::l_ollama::send_message(model, t_messages, history).await;
    match result {
        Ok((data, history_response)) => {
            let mut t_history = history_response;
            t_history.push(data.message.clone());
            app.emit("get-history", t_history).unwrap();
            data.message.content
        }
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            "".to_string()
        }
    }
}
