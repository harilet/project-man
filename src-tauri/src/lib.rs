use std::{
    net::{Shutdown, TcpStream},
    sync::OnceLock,
    thread,
    time::Duration,
};

use ollama_rs::generation::chat::ChatMessage;
use tauri::{AppHandle, Emitter};

mod utils;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_project_struture,
            get_staged_files,
            get_file_diff,
            get_all_local_models,
            send_message,
            get_recent_projects,
            set_projects,
            start_ollama_server_check,
            get_current_branch_name,
            set_ollama_setting,
            get_ollama_setting,
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
async fn send_message(
    app: AppHandle,
    model: String,
    messages: Vec<String>,
    history: Vec<ChatMessage>,
) -> String {
    let mut t_messages = vec![];
    let mut t_history = history.clone();

    for message in messages {
        let chat_message = ChatMessage::user(message);
        t_messages.push(chat_message.clone());
        t_history.push(chat_message);
    }

    app.emit("get-history", t_history.clone()).unwrap();
    let result = utils::l_ollama::send_message(model, t_messages.clone(), history).await;
    match result {
        Ok(data) => {
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

#[tauri::command]
async fn get_recent_projects(app: AppHandle) -> Vec<String> {
    match utils::db::init_db().await {
        Ok(_file_diff) => {}
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            return vec![];
        }
    }

    match utils::db::get_recent_projects().await {
        Ok(data) => {
            return data;
        }
        Err(e) => {
            app.emit("app-error", e.to_string()).unwrap();
            return vec![];
        }
    }
}

#[tauri::command]
async fn set_projects(app: AppHandle, name: String, path: String) {
    println!("{name}:{path}");
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
