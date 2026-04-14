use ollama_rs::generation::chat::{ChatMessage, MessageRole};
use std::{
    net::{Shutdown, TcpStream},
    sync::OnceLock,
    thread,
    time::Duration,
};
use tauri::{AppHandle, Emitter};
mod utils;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn get_app_handle() -> &'static AppHandle {
    APP_HANDLE.get().expect("APP_HANDLE not initialized")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_staged_files,
            get_file_diff,
            get_all_local_models,
            get_recent_projects,
            set_projects,
            start_ollama_server_check,
            get_current_branch_name,
            set_ollama_setting,
            get_ollama_setting,
            get_unstaged_files,
            get_unstaged_file_diff,
            add_file_index,
            remove_file_index,
            send_message,
            get_saved_messages,
            add_saved_messages,
            delete_saved_message,
            get_project_tree,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
fn get_unstaged_files(app: AppHandle, location: String) -> Vec<String> {
    match utils::git::get_unstaged_files(location) {
        Ok(files) => files,
        Err(e) => {
            println!("{:#?}", e);
            app.emit("app-error", e.to_string()).unwrap();
            vec![]
        }
    }
}

#[tauri::command]
fn get_file_diff(app: AppHandle, location: String, file: String, is_unified: bool) -> String {
    match utils::git::get_file_diff(location, file, is_unified) {
        Ok(file_diff) => file_diff.join("\n"),
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            "".to_string()
        }
    }
}

#[tauri::command]
fn get_unstaged_file_diff(app: AppHandle, location: String, file: String) -> String {
    match utils::git::get_unstaged_file_diff(location, file) {
        Ok(file_diff) => file_diff.join("\n"),
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
async fn get_recent_projects(app: AppHandle) -> Vec<String> {
    match utils::db::init_db().await {
        Ok(_file_diff) => {}
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            return vec![];
        }
    }

    return match utils::db::get_recent_projects().await {
        Ok(data) => data,
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            vec![]
        }
    };
}

#[tauri::command]
async fn set_projects(app: AppHandle, name: String, path: String) {
    match utils::db::set_projects(name, path).await {
        Ok(_) => {}
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
        }
    }
}

#[tauri::command]
async fn start_ollama_server_check(app: AppHandle) {
    thread::spawn(move || loop {
        match TcpStream::connect("localhost:11434") {
            Ok(connection) => {
                app.emit("ollama-server-status", "live").unwrap();
                connection.shutdown(Shutdown::Both).unwrap();
            }
            Err(_) => {
                app.emit("ollama-server-status", "offline").unwrap();
            }
        }
        thread::sleep(Duration::from_secs(10));
    });
}

#[tauri::command]
async fn get_current_branch_name(app: AppHandle, location: String) -> String {
    match utils::git::get_current_branch_name(location) {
        Ok(branch) => branch,
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
            "".to_string()
        }
    }
}

#[tauri::command]
async fn get_ollama_setting(app: AppHandle) -> String {
    match utils::db::get_ollama_setting().await {
        Ok(data) => match serde_json::to_string(&data) {
            Ok(json_data) => json_data,
            Err(e) => {
                app.emit("app-error", e.to_string()).unwrap();
                println!("{:#?}", e);
                "{}".to_string()
            }
        },
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
            "{}".to_string()
        }
    }
}

#[tauri::command]
async fn set_ollama_setting(app: AppHandle, name: String, value: String) {
    match utils::db::set_ollama_setting(name, value).await {
        Ok(_) => {}
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
        }
    }
}

#[tauri::command]
async fn add_file_index(app: AppHandle, location: String, path: String) {
    match utils::git::add_file_index(location, path) {
        Ok(_) => {}
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
        }
    }
}

#[tauri::command]
async fn remove_file_index(app: AppHandle, location: String, path: String) {
    match utils::git::remove_file_index(location, path) {
        Ok(_) => {}
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
        }
    }
}

#[tauri::command]
async fn send_message(
    app: AppHandle,
    message: Vec<ChatMessage>,
    history: Vec<ChatMessage>,
) -> ChatMessage {
    println!("message: {:#?}", message);
    println!("history: {:#?}", history);
    match utils::l_ollama::send_message(message, history).await {
        Ok(response) => {
            println!("{:#?}", response.message);
            return response.message;
        }
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            println!("{:#?}", e);
            return ChatMessage::new(MessageRole::User, "".to_string());
        }
    }
}

#[tauri::command]
async fn get_saved_messages(app: AppHandle) -> Vec<String> {
    match utils::db::get_saved_messages().await {
        Ok(data) => data,
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            ["{}".to_string()].to_vec()
        }
    }
}

#[tauri::command]
async fn add_saved_messages(app: AppHandle, message: String) -> Vec<String> {
    let app_clone = app.clone();
    match utils::db::add_saved_messages(message).await {
        Ok(data) => data,
        Err(e) => {
            app_clone.emit("app-error", e.to_string()).unwrap();
            ["{}".to_string()].to_vec()
        }
    }
}

#[tauri::command]
async fn delete_saved_message(app: AppHandle, message: String) -> Vec<String> {
    let app_clone = app.clone();
    match utils::db::delete_saved_message(message).await {
        Ok(data) => data,
        Err(e) => {
            app_clone.emit("app-error", e.to_string()).unwrap();
            ["{}".to_string()].to_vec()
        }
    }
}

#[tauri::command]
async fn get_project_tree(app: AppHandle, location: String) -> Vec<String> {
    match utils::git::get_project_tree(location) {
        Ok(tree) => {
            println!("{:#?}", tree);
            tree
        }
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            vec![]
        }
    }
}
