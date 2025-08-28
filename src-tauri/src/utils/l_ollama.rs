use ollama_rs::{
    coordinator::Coordinator,
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse},
    Ollama,
};
use std::{env, error::Error, fs, path::Path};
use tauri::Url;

use crate::utils::git;

pub(crate) async fn get_all_local_models() -> Result<Vec<String>, Box<dyn Error>> {
    let ollama = Ollama::from_url(Url::parse("http://localhost:11434")?);
    let res = ollama.list_local_models().await?;
    let mut local_models = vec![];
    for local_model in res {
        local_models.push(local_model.name);
    }

    Ok(local_models)
}

pub(crate) async fn send_message(
    model: String,
    messages: Vec<ChatMessage>,
) -> Result<ChatMessageResponse, Box<dyn Error>> {
    let mut history = vec![];

    let ollama = Ollama::from_url(Url::parse("http://localhost:11434")?);

    let mut coordinator = Coordinator::new(ollama, model.clone(), history)

        .add_tool(get_file)
        .add_tool(get_file_diff);
    let res = coordinator.chat(messages.to_owned(),)
        .await?;

    Ok(res)
}

/// Get file contents from a file path.
///
/// * file_path - The file path to read from.
#[ollama_rs::function]
async fn get_file(file_path: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("get_file: {file_path}");
    let file_contents = fs::read_to_string(&file_path)?;
    Ok(file_contents)
}

/// Get change diff of a file from a file path.
///
/// * file_path - The file path to read from(relative path).
/// * repo - The current repo we are using(absolute path).
#[ollama_rs::function]
async fn get_file_diff(
    file_path: String,
    repo: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("get_file_diff: {repo}::{file_path}");
    let file_contents = git::get_file_diff(repo,file_path).unwrap();
    Ok(file_contents)
}
